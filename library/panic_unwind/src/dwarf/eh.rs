//! Parsing of GCC-style Language-Specific Data Area (LSDA)
//! For details see:
//!  * <https://refspecs.linuxfoundation.org/LSB_3.0.0/LSB-PDA/LSB-PDA/ehframechpt.html>
//!  * <https://itanium-cxx-abi.github.io/cxx-abi/exceptions.pdf>
//!  * <https://www.airs.com/blog/archives/460>
//!  * <https://www.airs.com/blog/archives/464>
//!
//! A reference implementation may be found in the GCC source tree
//! (`<root>/libgcc/unwind-c.c` as of this writing).

#![allow(non_upper_case_globals)]
#![allow(unused)]

use crate::dwarf::DwarfReader;
use core::ffi::c_void;
use core::mem;

pub const DW_EH_PE_omit: u8 = 0xFF;
pub const DW_EH_PE_absptr: u8 = 0x00;

pub const DW_EH_PE_uleb128: u8 = 0x01;
pub const DW_EH_PE_udata2: u8 = 0x02;
pub const DW_EH_PE_udata4: u8 = 0x03;
pub const DW_EH_PE_udata8: u8 = 0x04;
pub const DW_EH_PE_sleb128: u8 = 0x09;
pub const DW_EH_PE_sdata2: u8 = 0x0A;
pub const DW_EH_PE_sdata4: u8 = 0x0B;
pub const DW_EH_PE_sdata8: u8 = 0x0C;

pub const DW_EH_PE_pcrel: u8 = 0x10;
pub const DW_EH_PE_textrel: u8 = 0x20;
pub const DW_EH_PE_datarel: u8 = 0x30;
pub const DW_EH_PE_funcrel: u8 = 0x40;
pub const DW_EH_PE_aligned: u8 = 0x50;

pub const DW_EH_PE_indirect: u8 = 0x80;

#[derive(Copy, Clone)]
pub struct EHContext<'a> {
    pub ip: usize,                             // Current instruction pointer
    pub func_start: usize,                     // Address of the current function
    pub get_text_start: &'a dyn Fn() -> usize, // Get address of the code section
    pub get_data_start: &'a dyn Fn() -> usize, // Get address of the data section
}

pub enum EHAction {
    None,
    // TODO(seharris): is this really the right way to handle this?
    Cleanup(*const c_void),
    Catch(*const c_void),
    Terminate,
}

pub const USING_SJLJ_EXCEPTIONS: bool = cfg!(all(target_os = "ios", target_arch = "arm"));

pub unsafe fn find_eh_action(lsda: *const u8, context: &EHContext<'_>) -> Result<EHAction, ()> {
    if lsda.is_null() {
        return Ok(EHAction::None);
    }

    let func_start = context.func_start;
    let mut reader = DwarfReader::new(lsda);

    let start_encoding = reader.read::<u8>();
    // base address for landing pad offsets
    let lpad_base = if start_encoding != DW_EH_PE_omit {
        read_encoded_pointer(&mut reader, context, start_encoding)?
    } else {
        func_start
    };

    let ttype_encoding = reader.read::<u8>();
    if ttype_encoding != DW_EH_PE_omit {
        // Rust doesn't analyze exception types, so we don't care about the type table
        reader.read_uleb128();
    }

    let call_site_encoding = reader.read::<u8>();
    let call_site_table_length = reader.read_uleb128();
    let action_table = reader.ptr.offset(call_site_table_length as isize);
    let ip = context.ip;

    if !USING_SJLJ_EXCEPTIONS {
        while reader.ptr < action_table {
            let cs_start = read_encoded_pointer(&mut reader, context, call_site_encoding)?;
            let cs_len = read_encoded_pointer(&mut reader, context, call_site_encoding)?;
            let mut cs_lpad = read_encoded_pointer(&mut reader, context, call_site_encoding)?;
            // Set when landing pad is encoded as a pointer instead of an
            // offset from `lpad_base`.
            let mut sealed_lpad = false;

            // Handle special encoding of landing pads on Morello.
            // Landing pads are encoded as straight pointer values, either
            // among the LSDA data, or at a given offset.
            //
            // Based on changes in
            // `morello-llvm-project/libcxxabi/src/cxa_personality.cpp`
            // from Morello LLVM release 1.5 (2022-10-5).
            #[cfg(all(target_arch = "aarch64", target_abi = "purecap"))]
            {
                let pointer_align = 16;
                if cs_lpad == 0xc {
                    cs_lpad = reader.read_aligned::<*const c_void>(pointer_align) as usize;
                    sealed_lpad = true;
                } else if cs_lpad == 0xd {
                    let address = reader.read::<u64>();
                    cs_lpad = *(reader.ptr.add(address as usize) as *const *const c_void) as usize;
                    sealed_lpad = true;
                } else if cs_lpad != 0 {
                    // Invalid encoding.
                    return Err(());
                }
            }
            let cs_action = reader.read_uleb128();
            // Callsite table is sorted by cs_start, so if we've passed the ip, we
            // may stop searching.
            if ip < func_start + cs_start {
                break;
            }
            if ip < func_start + cs_start + cs_len {
                if cs_lpad == 0 {
                    return Ok(EHAction::None);
                } else {
                    let lpad = if sealed_lpad { cs_lpad } else { lpad_base + cs_lpad };
                    return Ok(interpret_cs_action(cs_action, lpad));
                }
            }
        }
        // Ip is not present in the table.  This should not happen... but it does: issue #35011.
        // So rather than returning EHAction::Terminate, we do this.
        Ok(EHAction::None)
    } else {
        // SjLj version:
        // The "IP" is an index into the call-site table, with two exceptions:
        // -1 means 'no-action', and 0 means 'terminate'.
        match ip as isize {
            -1 => return Ok(EHAction::None),
            0 => return Ok(EHAction::Terminate),
            _ => (),
        }
        let mut idx = ip;
        loop {
            let cs_lpad = reader.read_uleb128();
            let cs_action = reader.read_uleb128();
            idx -= 1;
            if idx == 0 {
                // Can never have null landing pad for sjlj -- that would have
                // been indicated by a -1 call site index.
                let lpad = (cs_lpad + 1) as usize;
                return Ok(interpret_cs_action(cs_action, lpad));
            }
        }
    }
}

fn interpret_cs_action(cs_action: u64, lpad: usize) -> EHAction {
    let lpad = int_to_program_pointer(lpad);
    if cs_action == 0 {
        // If cs_action is 0 then this is a cleanup (Drop::drop). We run these
        // for both Rust panics and foreign exceptions.
        EHAction::Cleanup(lpad)
    } else {
        // Stop unwinding Rust panics at catch_unwind.
        EHAction::Catch(lpad)
    }
}

// Rederive a capability using the program counter.
// This is required on CHERI if we want to be able to dereference landing pads.
// We generally *do* want to dereference landing pads because we want to be
// able to execute from them.
// Failing to do this will cause an error about an invalid capability from
// somewhere in libunwind.
//
// Note that this only works for pointers into loaded machine code, i.e. the
// actual program instructions.
#[cfg(target_abi = "purecap")]
fn int_to_program_pointer(pointer: usize) -> *const c_void {
    // TODO(seharris): there are intrinsics that can do this, albeit slightly less cleanly, so we should possibly use them instead of assembly.
    // TODO(seharris): don't forget to remove the `#![feature(asm)]` annotation
    //                 from the root of this crate.
    // TODO(seharris): it would be cleaner to have a shared implementation of
    //                 these intrinsics in a library, perhaps adds them to core
    //                 or something.
    // I think the intrinsics we need are:
    // - __builtin_cheri_address_set
    // - __builtin_cheri_program_counter_get
    // The extern for one is below, but I couldn't find the other.
    // pub fn new_cap_with_provenance<T>(cap: *const T, addr: ???) -> *const T {
    //     extern {
    //         #[link_name = "llvm.cheri.cap.address.set.i64"]
    //         fn __builtin_cheri_address_set(cap: ???, addr: ???) -> ???;
    //     }
    //     return unsafe { __builtin_cheri_address_set(cap as ???, addr) as *const T };
    // }

    let capability: *const c_void;
    unsafe {
        // "Convert pointer to capability offset from PCC, with null capability
        // from zero semantics".
        //
        // We assume that the current program counter has access to the whole
        // executable, and create a new capability with the target address we
        // want, and provenance and bounds from the program counter.
        asm!("cvtpz {0}, {1}", out(reg) capability, in(reg) pointer);
    }
    capability
}
#[cfg(not(target_abi = "purecap"))]
fn int_to_program_pointer(pointer: usize) -> *const c_void {
    pointer as *const c_void
}

#[inline]
fn round_up(unrounded: usize, align: usize) -> Result<usize, ()> {
    if align.is_power_of_two() { Ok((unrounded + align - 1) & !(align - 1)) } else { Err(()) }
}

unsafe fn read_encoded_pointer(
    reader: &mut DwarfReader,
    context: &EHContext<'_>,
    encoding: u8,
) -> Result<usize, ()> {
    if encoding == DW_EH_PE_omit {
        return Err(());
    }

    // DW_EH_PE_aligned implies it's an absolute pointer value
    if encoding == DW_EH_PE_aligned {
        reader.ptr = round_up(reader.ptr as usize, mem::size_of::<usize>())? as *const u8;
        return Ok(reader.read::<usize>());
    }

    let mut result = match encoding & 0x0F {
        DW_EH_PE_absptr => reader.read::<usize>(),
        DW_EH_PE_uleb128 => reader.read_uleb128() as usize,
        DW_EH_PE_udata2 => reader.read::<u16>() as usize,
        DW_EH_PE_udata4 => reader.read::<u32>() as usize,
        DW_EH_PE_udata8 => reader.read::<u64>() as usize,
        DW_EH_PE_sleb128 => reader.read_sleb128() as usize,
        DW_EH_PE_sdata2 => reader.read::<i16>() as usize,
        DW_EH_PE_sdata4 => reader.read::<i32>() as usize,
        DW_EH_PE_sdata8 => reader.read::<i64>() as usize,
        _ => return Err(()),
    };

    result += match encoding & 0x70 {
        DW_EH_PE_absptr => 0,
        // relative to address of the encoded value, despite the name
        DW_EH_PE_pcrel => reader.ptr as usize,
        DW_EH_PE_funcrel => {
            if context.func_start == 0 {
                return Err(());
            }
            context.func_start
        }
        DW_EH_PE_textrel => (*context.get_text_start)(),
        DW_EH_PE_datarel => (*context.get_data_start)(),
        _ => return Err(()),
    };

    if encoding & DW_EH_PE_indirect != 0 {
        result = *(result as *const usize);
    }

    Ok(result)
}
