SHELL 		:= /bin/bash

PROJECTS 	:= prelude plugin-json plugin-markdown plugin-primitives \
						 plugin-yaml cli
TARGET		:= target
SRC_FILES 	:= $(shell find $(PROJECTS) -name '*.rs')
TOML 		:= $(shell find $(PROJECTS) -name '*.toml')

target/debug/cryogen: $(TOML) $(SRC_FILES)
	@cargo build --manifest-path cli/Cargo.toml

build: target/debug/cryogen

format:
	@rustfmt $(SRC_FILES) --write-mode overwrite
