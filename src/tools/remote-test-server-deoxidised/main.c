// A simplified replacement for the Ruts compiler's test server utility.
// The original can be in the rustc repository in src/tools/remote-test-server/src/main.rs
// We need a version that can run reliably on CHERI, and we don't yet have Rust's std library working.
// Thus, this C port.
//
// This version is designed for simplicity and brevity, so we've dropped multithreading support.
// We also only support Linux and BSD.
// Everything here is blocking, one client is served at a time.
// This will degrade performance, but it make this program much easier to implement correctly!

#include <dirent.h>
#include <fcntl.h>
#include <netinet/in.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

const char *const working_directory = "/tmp/work";
const char *const test_directory = "/tmp/work/test0";
const char *const temporary_directory = "/tmp/work/tmp";

#define COMMAND_LENGTH 4

// This macro declares a type to hold dynamically sized arrays of things of type ITEM_TYPE.
// In other words, this macro does templating.
// The resulting type can be initialised to all-zero to create an empty array.
// The resulting type will be called NAME and come with these declarations:
// - NAME_push(self, item) add an item to the end of the an array
// - NAME_pop(self) remove and return last item (there must an item) from an array
// - NAME_append(self, items, count) concatenate an array of items to the end of an array
// - NAME_clear(self) remove all items from an array
// - NAME_nulled(self) return vector as NULL terminated array valid until modification of the vector
#define VECTOR(ITEM_TYPE, NAME) \
	typedef struct { \
		size_t capacity; \
		ITEM_TYPE *data; \
		size_t length; \
	} NAME; \
	void NAME ## _expand(NAME *const vector, const size_t length) { \
		vector->capacity *= 2; \
		if (vector->capacity < length) { \
			vector->capacity = length; \
		} \
		vector->data = realloc(vector->data, vector->capacity*sizeof(ITEM_TYPE)); \
		if (vector->data == NULL) { \
			fprintf(stderr, "failed to allocate memory\n"); \
			abort(); \
		} \
	} \
	void NAME ## _push(NAME *const vector, ITEM_TYPE item) { \
		if (vector->length == vector->capacity) { \
			NAME ## _expand(vector, vector->length+1); \
		} \
		vector->data[vector->length] = item; \
		vector->length ++; \
	} \
	ITEM_TYPE NAME ## _pop(NAME *const vector) { \
		if (vector->length == 0) { \
			fprintf(stderr, "pop from empty vector\n"); \
			abort(); \
		} \
		vector->length --; \
		return vector->data[vector->length]; \
	} \
	void NAME ## _append(NAME *const vector, const ITEM_TYPE *const items, const size_t length) { \
		const size_t offset = vector->length; \
		vector->length += length; \
		if (vector->length > vector->capacity) { \
			NAME ## _expand(vector, vector->length); \
		} \
		memcpy(&vector->data[offset], items, length*sizeof(ITEM_TYPE)); \
	} \
	void NAME ## _clear(NAME *const vector) { \
		vector->length = 0; \
	} \
	ITEM_TYPE *NAME ## _nulled(NAME *const vector) { \
		if (vector->length == 0 || vector->data[vector->length-1] != 0) { \
			if (vector->capacity < vector->length+1) { \
				NAME ## _expand(vector, vector->length+1); \
			} \
			vector->data[vector->length] = 0; \
		} \
		return vector->data; \
	}

// Declare some vector types we'll need.
VECTOR(char, string)
VECTOR(char*, string_vector)

// Buffered wrapper around a socket.
typedef struct {
	uint8_t data[1024];
	// Number of bytes left in buffer (after offset).
	size_t length;
	// Number of bytes of buffer already read (instead of shifting remaining data).
	size_t offset;
	int socket;
} readbuffer;
readbuffer readbuffer_new(const int socket) {
	readbuffer buffer;
	buffer.length = 0;
	buffer.offset = 0;
	buffer.socket = socket;
	return buffer;
}
// If the buffer is empty, wait for data to become available and copy it into the buffer.
// Crash on error.
void readbuffer_fill(readbuffer *const buffer) {
	if (buffer->length == 0) {
		const ssize_t read = recv(buffer->socket, &buffer->data[0], sizeof(buffer->data), 0);
		if (read == 0) {
			fprintf(stderr, "socket closed unexpectedly\n");
			abort();
		} else if (read <= 0) {
			perror("failed to receive data from network");
			abort();
		}
		buffer->offset = 0;
		buffer->length = read;
	}
}
// Iterate over some number of bytes, divided into buffer-sized chunks.
// Return `true` if there was a chunk to read, `false` if there was nothing left.
// The returned chunk pointer is into the buffer, so is only valid until a readbuffer method is called, and does not need freeing.
// See implementation of `readbuffer_read()` for usage example.
//
// `data_out` is used to return a pointer to the next chunk.
// `length_out` is used to return the length of the next chunk.
// `remaining_length` specifies the number of bytes to read in total, and is updated to account for the returned chunk.
bool readbuffer_chunks(readbuffer *const buffer, const uint8_t **const data_out, size_t *const length_out, size_t *const remaining_length) {
	if (remaining_length == NULL) abort();
	if (*remaining_length == 0) {
		*data_out = NULL;
		*length_out = 0;
		return false;
	}

	readbuffer_fill(buffer);

	*data_out = &buffer->data[buffer->offset];
	*length_out = buffer->length;
	if (*length_out > *remaining_length) *length_out = *remaining_length;
	*remaining_length -= *length_out;
	buffer->offset += *length_out;
	buffer->length -= *length_out;
	return true;
}
// Fill an array of bytes, crash on error.
void readbuffer_read(readbuffer *const buffer, uint8_t *const into, size_t length) {
	size_t offset = 0;
	const uint8_t *chunk_data = NULL;
	size_t chunk_length = 0;
	while (readbuffer_chunks(buffer, &chunk_data, &chunk_length, &length)) {
		memcpy(&into[offset], chunk_data, chunk_length);
		offset += chunk_length;
	}
}
// Return the next byte without removing it from the buffer, crash on error.
uint8_t readbuffer_peek(readbuffer *const buffer) {
	readbuffer_fill(buffer);
	return buffer->data[buffer->offset];
}
// Fill a `string` with data received before a terminating byte, crash on error.
// The terminating byte is not included.
// The `string` is not cleared before appending data to it.
void readbuffer_read_until(readbuffer *const buffer, string *const into, uint8_t until) {
	uint8_t byte;
	while (true) {
		readbuffer_read(buffer, &byte, 1);
		if (byte == until) break;
		string_push(into, byte);
	}
}
// Decode a 32 bit unsigned integer, crash on error.
uint32_t readbuffer_read_u32(readbuffer *const buffer) {
	uint8_t bytes[4];
	readbuffer_read(buffer, &bytes[0], sizeof(bytes));
	return ((uint32_t)bytes[0] << 24) |
		((uint32_t)bytes[1] << 16) |
		((uint32_t)bytes[2] << 8) |
		(uint32_t)bytes[3];
}

// Receive a file from the client (aka `recv()` originally), crash on error.
string upload_file_path; // static to allow re-use and not worrying about free()ing
char *upload_file(readbuffer *buffer, const char *const base_path) {
	// Read NULL terminated filename, truncate it to a maximum length, and append it to the path.
	// Truncation matches original behaviour, though given it's for filesystem compatbility it shouldn't matter much.
	const size_t max_length = 50;
	const size_t base_length = strlen(base_path);
	string_clear(&upload_file_path);
	string_append(&upload_file_path, base_path, base_length);
	string_push(&upload_file_path, '/');
	readbuffer_read_until(buffer, &upload_file_path, 0);
	if (upload_file_path.length-base_length-1 > max_length) upload_file_path.length = base_length+1+max_length;
	char *const file_path = string_nulled(&upload_file_path);
printf("HACK upload_file() path=%s\n", file_path);

	// Read length of file.
	size_t length = readbuffer_read_u32(buffer);
printf("HACK upload_file() length=%lu\n", length);

	// Create file at calculated path.
	const int file = open(file_path, O_CREAT|O_TRUNC|O_WRONLY, 0755);
	if (file < 0) {
		perror("failed to open file for writing");
		abort();
	}

	// Read data and write it to open file.
	// We're already buffering chunks so we can get away without fopen/fwrite/fclose's buffering.
	const uint8_t *chunk_data = NULL;
	size_t chunk_length = 0;
	while (readbuffer_chunks(buffer, &chunk_data, &chunk_length, &length)) {
printf("HACK upload_file() read input bytes=%lu/%lu\n", chunk_length, length);
		if (write(file, chunk_data, chunk_length) != (ssize_t)chunk_length) {
			fprintf(stderr, "failed to write to file\n");
			abort();
		}
	}
printf("HACK upload_file() end of bytes\n");

	// Tidy up.
	if (close(file) != 0) {
		perror("failed to close file");
		abort();
	}
printf("HACK upload_file() file uploaded\n");

	return file_path;
}

// Write contents of buffer to a socket, crash on error.
void send_buffer(const int socket, const uint8_t *const buffer, const size_t length) {
	if (send(socket, buffer, length, 0) != (ssize_t)length) {
		perror("failed to send data");
		abort();
	}
}

// Forward a chunk of output from a pipe connected to a child process to the network, crash on error.
// Return `true` at end of file.
// `pipe_number` identifies whether the output is stdout or stderr for the client.
bool send_output(const int socket, const uint8_t pipe_number, const int pipe) {
	uint8_t buffer[1024];
	const ssize_t bytes = read(pipe, &buffer, sizeof(buffer));
	if (bytes < 0) {
		perror("failed to read from child process");
		abort();
	}
	const uint8_t packet[5] = {pipe_number, bytes >> 24, bytes >> 16, bytes >> 8, bytes};
	send_buffer(socket, &packet[0], sizeof(packet));
	if (bytes == 0) {
		return true;
	} else {
		send_buffer(socket, &buffer[0], bytes);
		return false;
	}
}

// Recursively delete a directory and all files in it, crash on error.
string remove_directory_path; // static to allow re-use and not worrying about free()ing
void _remove_directory(void) {
	DIR *directory = opendir(string_nulled(&remove_directory_path));
	if (directory == NULL) {
		perror("failed to open temporary directory");
		abort();
	}

	const size_t path_length = remove_directory_path.length;
	string_push(&remove_directory_path, '/');

	while (true) {
		// Get next item to delete and append it to the path.
		const struct dirent *const entry = readdir(directory);
		if (entry == NULL) break;
		if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0) continue;
		string_append(&remove_directory_path, entry->d_name, strlen(entry->d_name));

		// Recursively delete item, reusing path buffer for recursion.
printf("HACK _remove_directory() remove %s\n", string_nulled(&remove_directory_path));
		if (entry->d_type == DT_DIR) {
printf("HACK _remove_directory() recurse for %s\n", string_nulled(&remove_directory_path));
			// `entry` becomes invalid after this call due to how `readdir()` allocates it.
			_remove_directory();
		} else if (unlink(string_nulled(&remove_directory_path)) != 0) {
			perror("failed to remove temporary file");
			abort();
		}

		// Remove item from path so we can recycle it on future iterations.
		remove_directory_path.length = path_length+1;
	}

	// Remove all added suffixes from path so it stays as it was when we were called.
	remove_directory_path.length = path_length;

	// Close and delete directory.
	if (closedir(directory) != 0) {
		perror("failed to close temporary directory");
		abort();
	}
	if (rmdir(string_nulled(&remove_directory_path)) != 0) {
		perror("failed to delete temporary directory");
		abort();
	}
printf("HACK _remove_directory() removed directory %s\n", string_nulled(&remove_directory_path));
}
void remove_directory(const char *const path) {
	string_clear(&remove_directory_path);
	string_append(&remove_directory_path, path, strlen(path));
	_remove_directory();
}

// Receive a test binary and related files, run the test, transmit results, crash on error.
// (aka `handle_run()` originally)
string_vector run_args; // static to allow re-use and not worrying about free()ing
string run_args_strings; // static to allow re-use and not worrying about free()ing
string_vector run_envs; // static to allow re-use and not worrying about free()ing
string run_envs_strings; // static to allow re-use and not worrying about free()ing
void run_test(readbuffer *const buffer) {
	// Create directory to put test data in.
	// This will be tidied up after running the test to save space.
	if (mkdir(test_directory, 0755) != 0) {
		perror("failed to create test directory");
		abort();
	}
printf("HACK run_test() temporary directory created\n");

	// Read list of NULL terminated arguments.
	// We store the argument data into one big long string with a NULL at the end of each argument.
	// We store pointers to the argument data into a list to pass to `execve()`.
	// The first entry is a special case, a dummy which will later be replaced by the path to the program.
	string_vector_clear(&run_args);
	string_vector_push(&run_args, NULL);
	string_clear(&run_args_strings);
	while (true) {
		const size_t start = run_args_strings.length;
		readbuffer_read_until(buffer, &run_args_strings, 0);
		if (run_args_strings.length == start) break; // empty argument marks end of list
		string_push(&run_args_strings, 0);
		string_vector_push(&run_args, &run_args_strings.data[start]);
	}
printf("HACK run_test() arguments added\n");

	// Do pretty much the same thing for environment variable key-value pairs.
	// We do some extra processing to format the pairs as needed by `execve()`.
	string_vector_clear(&run_envs);
	string_clear(&run_envs_strings);
	while (true) {
		const size_t start = run_envs_strings.length;
		readbuffer_read_until(buffer, &run_envs_strings, 0);
		if (run_envs_strings.length == start) break; // empty variable name marks end of list
		if (strcmp(&string_nulled(&run_envs_strings)[start], "LD_LIBRARY_PATH") == 0) {
			// This shouldn't happen, I think, so we'll make sure it doesn't
			fprintf(stderr, "unexpected LD_LIBRARY_PATH in environment\n");
			abort();
		}
		string_push(&run_envs_strings, '=');
		readbuffer_read_until(buffer, &run_envs_strings, 0);
		string_push(&run_envs_strings, '\0');
		string_vector_push(&run_envs, &run_envs_strings.data[start]);
	}
printf("HACK run_test() environment variables added\n");

	// Add library paths.
	{
		const size_t start = run_envs_strings.length;
		const char *const library_path_env = "LD_LIBRARY_PATH";
		string_append(&run_envs_strings, library_path_env, strlen(library_path_env));
		string_push(&run_envs_strings, '=');
		string_append(&run_envs_strings, working_directory, strlen(working_directory));
		string_push(&run_envs_strings, ':');
		string_append(&run_envs_strings, test_directory, strlen(test_directory));
		const char *const os_paths = getenv("LD_LIBRARY_PATH");
		if (os_paths != NULL) {
			string_push(&run_envs_strings, ':');
			string_append(&run_envs_strings, os_paths, strlen(os_paths));
		}
		string_push(&run_envs_strings, '\0');
		string_vector_push(&run_envs, &run_envs_strings.data[start]);
	}

	// Add temporary directory.
	{
		const size_t start = run_envs_strings.length;
		const char *const temporary_directory_env = "RUST_TEST_TMPDIR";
		string_append(&run_envs_strings, temporary_directory_env, strlen(temporary_directory_env));
		string_push(&run_envs_strings, '=');
		string_append(&run_envs_strings, temporary_directory, strlen(temporary_directory));
		string_push(&run_envs_strings, '\0');
		string_vector_push(&run_envs, &run_envs_strings.data[start]);
	}
printf("HACK run_test() extra environment variables inserted\n");

	// Read list of dynamic libraries.
	while (readbuffer_peek(buffer) != 0) {
		upload_file(buffer, test_directory);
	}
	uint8_t byte = 0;
	readbuffer_read(buffer, &byte, 1);
	if (byte != 0) abort();
printf("HACK run_test() dynamic libraries copied\n");

	// Upload binary.
	char *const binary_path = upload_file(buffer, test_directory);

	// Replace dummy args[0] now we have the path to the binary.
	run_args.data[0] = binary_path;
printf("HACK run_test() binary is %s\n", binary_path);

	// Run test program.
	int stdout_pipe[2] = {-1, -1};
	int stderr_pipe[2] = {-1, -1};
	if (pipe(stdout_pipe) != 0 || pipe(stderr_pipe) != 0) {
		perror("failed to create subprocess output pipes");
		abort();
	}
	const pid_t child_pid = fork();
	if (child_pid == -1) {
		perror("failed to fork");
		abort();
	} else if (child_pid == 0) {
		// Child process
		if (close(stdout_pipe[0]) != 0 || close(stderr_pipe[0]) != 0) {
			perror("failed to close parent end of output pipes");
			abort();
		}
printf("HACK run_test() closed parent end of pipes\n");
printf("HACK run_test() loading %s\n", binary_path);
		if (dup2(stdout_pipe[1], STDOUT_FILENO) != STDOUT_FILENO) {
			perror("failed to attach stdout");
			abort();
		}
		if (dup2(stderr_pipe[1], STDERR_FILENO) != STDERR_FILENO) {
			perror("failed to attach stderr");
			abort();
		}
		execve(binary_path, string_vector_nulled(&run_args), string_vector_nulled(&run_envs));
		perror("failed to exec new process");
		abort();
	} else {
		// Parent process
		if (close(stdout_pipe[1]) != 0 || close(stderr_pipe[1]) != 0) {
			perror("failed to close child end of output pipes");
			abort();
		}
printf("HACK run_test() closed child end of pipes\n");

		// Transmit output.
		int end_descriptor = stderr_pipe[0] > stdout_pipe[0] ? stderr_pipe[0] : stdout_pipe[0];
		end_descriptor += 1;
		bool stdout_open = true;
		bool stderr_open = true;
		while (stdout_open || stderr_open) {
			fd_set read_descriptors;
			FD_ZERO(&read_descriptors);
			if (stdout_open) FD_SET(stdout_pipe[0], &read_descriptors);
			if (stderr_open) FD_SET(stderr_pipe[0], &read_descriptors);
			if (select(end_descriptor, &read_descriptors, NULL, NULL, NULL) < 0) {
				perror("failed to wait for data from test program");
				abort();
			}
			if (FD_ISSET(stdout_pipe[0], &read_descriptors) && send_output(buffer->socket, 0, stdout_pipe[0])) {
				stdout_open = false;
			}
			if (FD_ISSET(stderr_pipe[0], &read_descriptors) && send_output(buffer->socket, 1, stderr_pipe[0])) {
				stderr_open = false;
			}
		}
printf("HACK run_test() end of test\n");

		// Transmit result.
		int status = 0;
		if (waitpid(child_pid, &status, 0) != child_pid) {
			perror("falied to wait for subprocess to terminate");
			abort();
		}
printf("HACK run_test() child terminated\n");
		if (close(stdout_pipe[0]) != 0 || close(stderr_pipe[0]) != 0) {
			perror("failed to close output pipes");
			abort();
		}
		if (WIFEXITED(status)) {
			// Child exited with an exit code.
			const uint32_t code = WEXITSTATUS(status);
			const uint8_t packet[5] = {0, code >> 24, code >> 16, code >> 8, code};
			send_buffer(buffer->socket, &packet[0], sizeof(packet));
printf("HACK run_test() sent exit code\n");
		} else if (WIFSIGNALED(status)) {
			// Child exited after signal.
			const uint32_t signal = WTERMSIG(status);
			const uint8_t packet[5] = {1, signal >> 24, signal >> 16, signal >> 8, signal};
			send_buffer(buffer->socket, &packet[0], sizeof(packet));
printf("HACK run_test() sent signal\n");
		} else {
			fprintf(stderr, "child process stopped unexpectedly\n");
			abort();
		}
	}

	remove_directory(test_directory);
printf("HACK run_test() removed temporary directory\n");
}

// Open first socket to listen to new connections.
// Crash on failure, return socket on success.
int start_listening() {
	// Open listening socket.
	const int root_socket = socket(PF_INET, SOCK_STREAM, 0);
	if (root_socket < 0) {
		perror("failed to listen");
		abort();
	}

	// Set flags to force file to close on exec() so we don't confuse child processes.
	// This can be done via socket() on modern Linux, but we do it the old way for FreeBSD compatibility.
	if (fcntl(root_socket, F_SETFD, FD_CLOEXEC) != 0) {
		perror("failed to set socket flags");
		abort();
	}

	// Set port for the socket and listen on all interfaces.
	struct sockaddr_in address;
	memset(&address, 0, sizeof(address));
	address.sin_family = AF_INET;
	address.sin_addr.s_addr = htons(INADDR_ANY);
	address.sin_port = htons(12345);
	if (bind(root_socket, (struct sockaddr*)&address, sizeof(address)) != 0) {
		perror("failed to choose listening port and interface");
		abort();
	}

	// Mark socket as used to accept new connections.
	// We allow a fairly large number of queued connection requests because we'll be blocking a lot.
	if (listen(root_socket, 256) != 0) {
		perror("failed to setup socket for listening");
		abort();
	}

	return root_socket;
}

// Wait for a client to connect.
// Crash on failure, return socket on success.
int accept_client(const int root_socket) {
	// Open socket from client.
	const int socket = accept(root_socket, NULL, NULL);
	if (socket < 0) {
		perror("failed to accept connection");
		abort();
	}

	// Set flags to force file to close on exec() so we don't confuse child processes.
	// This can be done via socket() on modern Linux, but we do it the old way for FreeBSD compatibility.
	if (fcntl(root_socket, F_SETFD, FD_CLOEXEC) != 0) {
		perror("failed to set socket flags");
		abort();
	}

	return socket;
}

// Return true if two buffers contain the same data.
// Both buffers are assumed to be the same length.
bool compare_bytes(const uint8_t *const a, const uint8_t *const b, const size_t length) {
	size_t index;
	for (index = 0; index < length; index ++) {
		if (a[index] != b[index]) return false;
	}
	return true;
}

// Read command from client and call whichever function it corresponds to.
// Crash on error.
void handle(readbuffer *const buffer) {
	uint8_t command[COMMAND_LENGTH] = {0};
	readbuffer_read(buffer, &command[0], sizeof(command));
	if (compare_bytes(command, (const uint8_t*)"ping", COMMAND_LENGTH)) {
printf("HACK handle() ping\n");
		send_buffer(buffer->socket, (const uint8_t*)"pong", 4);
	} else if (compare_bytes(command, (const uint8_t*)"push", COMMAND_LENGTH)) {
printf("HACK handle() push\n");
		upload_file(buffer, working_directory);
		send_buffer(buffer->socket, (const uint8_t*)"ack ", 4);
	} else if (compare_bytes(command, (const uint8_t*)"run ", COMMAND_LENGTH)) {
printf("HACK handle() run\n");
		run_test(buffer);
	} else {
		fprintf(stderr, "invalid command received\n");
		abort();
	}
}

int main(int argc, char *argv[]) {
	(void)argc;
	(void)argv;

	if (mkdir(working_directory, 0755) != 0 || mkdir(temporary_directory, 0755) != 0) {
		perror("failed to create temporary directory");
		abort();
	}
printf("HACK main() created temporary directories\n");

	const int root_socket = start_listening();
printf("HACK main() opened port\n");
	while (true) {
		const int socket = accept_client(root_socket);
printf("HACK main() accepted client\n");
		readbuffer buffer = readbuffer_new(socket);
		handle(&buffer);
		if (close(socket) != 0) {
			perror("failed to close socket");
			abort();
		}
printf("HACK main() processed request\n");
	}
}
