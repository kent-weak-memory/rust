#!/usr/bin/python3
# Script to annotate LLVM IR with more readable debugging notes

import re

# Reference to a debug entry
regex_reference = re.compile(b"!dbg !([0-9]+)")
# Entity name inside a debug entry
regex_name = re.compile(b"name: \"([^\"]+)\"")
# Reference to a scope inside a debug entry
regex_scope = re.compile(b"scope: !([0-9]+)")
# Reference to a file inside a debug entry
regex_file = re.compile(b"file: !([0-9]+)")
# Filename inside a debug entry
regex_filename = re.compile(b"filename: \"([^\"]+)\"")
# Line number inside a debug entry
regex_line = re.compile(b"line: ([0-9]+)")

# Return debug entry by `uid` in IR `text` (just a ".ll" file as bytes)
def find_info(text, uid):
	prefix = b"!"+uid+b" = "
	info_start = text.find(prefix)
	if info_start < 0:
		raise ValueError("missing information")
	info_start += len(prefix)
	try:
		info_end = text.index(b"\n", info_start)
	except ValueError: # not found
		info_end = len(text)
	return text[info_start:info_end]

# Return (filename, line number, entity name) by recursively searching through
# debug entries starting at `uid`.
# `text` is the ".ll" file contents to search for debug entries.
# `uids_seens` is a set of uids that have already been used to detect cycles.
def find_scope_source(text, uid, uids_seen):
	info = find_info(text, uid)
	# File
	match = regex_filename.search(info)
	filename = None
	if match:
		filename = match.group(1)
	# Line number
	match = regex_line.search(info)
	line = None
	if match:
		line = int(match.group(1))
	# Entity name
	match = regex_name.search(info)
	name = None
	if match:
		name = match.group(1)
	# Recurse if missing information
	if filename == None or line == None or name == None:
		for regex in [regex_scope, regex_file]:
			match = regex.search(info)
			if match:
				next_uid = match.group(1)
				if not next_uid in uids_seen:
					uids_seen.add(next_uid)
					(next_filename, next_line, next_name) = find_scope_source(text, next_uid, uids_seen)
					if filename == None:
						filename = next_filename
					if line == None:
						line = next_line
					if name == None:
						name = next_name
	return (filename, line, name)

# Load input files.
input_path = "/tmp/build/args.ll" # file to annotate
debug_path = "/tmp/build/std.ll" # file containing debug entries
with open(input_path, "rb") as file:
	input_bytes = file.read()
with open(debug_path, "rb") as file:
	debug_bytes = file.read()

# Write new file with annotations.
write_path = "/tmp/build/args.ll.out"
with open(write_path, "wb") as file:
	# Find all !dbg references and annotate their lines
	last_end = 0
	for match in regex_reference.finditer(input_bytes):
		start = match.start()
		uid = match.group(1)
		# Look for filename, line number, entity name
		(filename, line, name) = find_scope_source(debug_bytes, uid, set())
		# Write preceding lines
		line_start = last_end
		try:
			line_start = input_bytes.rindex(b"\n", last_end, start)+1
		except ValueError: # not found
			pass
		file.write(input_bytes[last_end:line_start])
		last_end = line_start
		# Write annotation
		file.write(bytes("; dbg: {}:{}: {}\n".format(
			(filename or b"<no filename>").decode("utf-8"),
			line or "<no line>",
			(name or b"<no name>").decode("utf-8"),
		), "utf-8"))
	# Write leftover lines
	file.write(input_bytes[last_end:])
