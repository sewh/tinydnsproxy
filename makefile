# Makefile for doing release builds of tinydnsproxy. It current
# requires Docker and an X86_64 build host.

# Check to see whether our user can run docker, or we need to elevate
# to root.
ifneq (,$(findstring Got permission denied,$(shell docker ps 2>&1)))
$(info Using sudo)
	DOCKER=sudo docker
	RMCMD=sudo rm
	CHOWNCMD=sudo chown $(shell whoami):$(shell whoami) -R target/
else
$(info Not using sudo)
	DOCKER=docker
	RMCMD=rm
	CHOWNCMD=
endif

.PHONY: tests

all: tests armv7 x86_64

clean:
	$(RMCMD) -rf target/

tests:
	$(DOCKER) run --rm -it -v "$(shell pwd)":/home/rust/src messense/rust-musl-cross:x86_64-musl cargo test

armv7: target/armv7-unknown-linux-musleabihf/release/tinydnsproxy

x86_64: target/x86_64-unknown-linux-musl/release/tinydnsproxy

target/armv7-unknown-linux-musleabihf/release/tinydnsproxy:
	$(DOCKER) run --rm -it -v "$(shell pwd)":/home/rust/src messense/rust-musl-cross:armv7-musleabihf cargo build --release
	$(CHOWNCMD)

target/x86_64-unknown-linux-musl/release/tinydnsproxy:
	$(DOCKER) run --rm -it -v "$(shell pwd)":/home/rust/src messense/rust-musl-cross:x86_64-musl cargo build --release
	$(CHOWNCMD)
