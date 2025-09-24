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
exe_path = ./e
exe_test_path = ./t
other_flags = -Wall -std=c++23
src_paths = $(wildcard **/*.cpp)
src_test_paths = $(wildcard test/*.cpp)
compile_command = $(compiler) $(include_paths) $(library_paths) $(library_names) $(framework_names) -o $(exe_path) $(other_flags) $(src_paths)

.PHONY: default
default: all

# Target "all" is expected by CLion
.PHONY: all
all: setup build

.PHONY: setup
setup: compile_commands.json

.PHONY: run
run: build
	$(exe_path)

.PHONY: build
build:
	$(compile_command) -g -O0

.PHONY: build-release
build-release:
	$(compile_command) -O3

compile_commands.json:
	echo '[{"directory": "$(PWD)", "command": "$(compile_command)", "file": "$(src_paths)"}]' > compile_commands.json

# Target "clean" is expected by CLion
.PHONY: clean
clean:
	rm -rf $(exe_path) $(exe_path).dSYM compile_commands.json .cache/

.PHONY: build-test
build-test:
	$(compiler) $(include_paths) $(library_paths) $(library_names) $(framework_names) -o $(exe_test_path) $(other_flags) $(src_test_paths) -g -O0

.PHONY: test
test: build-test
	$(exe_test_path)

