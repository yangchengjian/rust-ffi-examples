ifeq ($(shell uname),Darwin)
    LDFLAGS := -Wl,-dead_strip
else
    LDFLAGS := -Wl,--gc-sections -lpthread
endif

all: target/native
	target/native

target:
	mkdir -p $@

target/native: target/main.o target/debug/libnative.a
	$(CC) -o $@ $^ $(LDFLAGS)

target/debug/libnative.a: src/lib.rs Cargo.toml
	cargo build

target/main.o: src/main.c | target
	$(CC) -o $@ -c $<

clean:
	rm -rf target
