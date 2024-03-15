home := justfile_directory()
shards := home / "shards"
parsers := home / "parsers"
rushi := parsers / "rushi"
stardust := parsers / "stardust"

# Cdylib extension name
ext := if os() == "macos" { "dylib" } else if os_family() == "unix" { "so" } else { "dll" }

default:
    @just -l

# convert .a file to so

# Builds and stores all the modules
build: && header
    cargo b
    @# this happens before the rest of the parsers are built so they can link
    @# against this
    cp "{{home}}/target/debug/libshards_sys.{{ext}}" "{{home}}/lib/libshards.{{ext}}"
    # zig build --build-file {{stardust}}/build.zig
    cp "{{home}}/target/debug/librushi.{{ext}}" "{{home}}/lib/"
    cp "{{home}}/target/debug/librush_shard.{{ext}}" "{{home}}/lib/librush.{{ext}}"
    # cp "{{stardust}}/zig-out/lib/libstardust.{{ext}}" "{{home}}/libs/"

header:
    mv '{{home}}/crates/libshards-core/bindings.h' lib/libshards.h

# Runs the code
run: build
    cargo run --bin shards
