SHELL     := /bin/bash

PROJECTS  := prelude plugin-json plugin-markdown plugin-primitives \
             plugin-yaml cli
TARGET    := target
SRC_FILES := $(shell find $(PROJECTS) -name '*.rs')
TOML      := $(shell find $(PROJECTS) -name '*.toml')

all: target/debug/cryogen

target/debug/cryogen: $(TOML) $(SRC_FILES)
	cargo build

format:
	cargo fmt
