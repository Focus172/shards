home := justfile_directory()
rushi := home / "rushi"
stardust := home / "stardust"
shards := home / "shards"

ext := if os() == "macos" {
	"dylib"
} else if os_family() == "unix" {
	"so"
} else {
	"dll"
}

# Runs the build process
default: build

# zig build --build-file {{stardust}}/build.zig
# convert .a file to so
# cp "{{stardust}}/zig-out/lib/libstardust.{{ext}}" "{{home}}/libs/"

# Builds and stores all the modules
build: header
    cargo b
    cp "{{home}}/target/debug/libshards_sys.{{ext}}" "{{home}}/libs/libshards.{{ext}}"
    cp "{{home}}/target/debug/librushi.{{ext}}" "{{home}}/libs/"
    cp "{{home}}/target/debug/librush_shard.{{ext}}" "{{home}}/libs/librush.{{ext}}"

header:
    # cargo b --manifest-path '{{home}}/Cargo.toml'
    cp '{{home}}/crates/shards-sys/bindings.h' libs/libshards.h

# Runs the code
run: build
    cargo run --bin shards

# Runs the code with optimizations enabled
release:
    cargo run --release --manifest-path '{{shards}}/Cargo.toml'
