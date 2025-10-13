compiler ?= /opt/homebrew/Cellar/llvm/21.1.0/bin/clang++

include_paths = -I./lib/raylib/raylib/include/
library_paths = -L./lib/raylib/raylib/
library_names = -lraylib
framework_names = \
    -framework OpenGL \
    -framework Cocoa \
    -framework IOKit \
    -framework CoreVideo \
    -framework CoreFoundation \
    -framework CoreGraphics \
    -framework Foundation
other_flags = -Wall -std=c++23
exe_path_client_app = ./ce
exe_path_client_test = ./ct
src_paths_client_app = $(wildcard client/archive/*.cpp)
src_paths_client_test = $(wildcard client/archive/test/*.cpp)
compile_command_base = $(compiler) $(include_paths) $(library_paths) $(library_names) $(framework_names)
compile_command_client_app = $(compile_command_base) -o $(exe_path_client_app) $(other_flags) $(src_paths_client_app)
compile_command_client_test = $(compile_command_base) -o $(exe_path_client_test) $(other_flags) $(src_paths_client_test) -g -O0

.PHONY: default
default: all

# Target "all" is expected by CLion
.PHONY: all
all: setup build

.PHONY: setup
setup: client/compile_commands.json

.PHONY: run
run: build
	$(exe_path_client_app)

.PHONY: build
build: build-dev-client

.PHONY: build-dev-client
build-dev-client:
	$(compile_command_client_app) -g -O0
	ln -sF ce c

.PHONY: build-release
build-release: build-release-client

.PHONY: build-release-client
build-release-client:
	$(compile_command_client_app) -O3
	ln -sF ce c

.PHONY: build-test
build-test: build-test-client

.PHONY: build-test-client
build-test-client:
	$(compile_command_client_test)

.PHONY: test
test: build-test
	$(exe_path_client_test)

client/compile_commands.json:
	echo '[{"directory": "$(PWD)", "command": "$(compile_command_client_app)", "file": "$(src_paths_client_app)"}]' > client/compile_commands.json

# Target "clean" is expected by CLion
.PHONY: clean
clean:
	rm -rf \
		$(exe_path_client_app) $(exe_path_client_app).dSYM \
		c \
		$(exe_path_client_test) $(exe_path_client_test).dSYM \
		compile_commands.json .cache/

