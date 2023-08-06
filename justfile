home := justfile_directory()
rushi := home / "rushi"
shards := home / "shards"
shards-types := home/ "shards-types"

ext := if os() == "macos" {
	"dylib"
} else if os_family() == "unix" {
	"so"
} else {
	"dll"
}

# Runs the build process
default: build

# Builds and stores all the modules
build:
	cargo build --manifest-path '{{rushi}}/Cargo.toml'
	cp "{{rushi}}/target/debug/librushi.{{ext}}" "{{home}}/libs/"
	cargo build --manifest-path '{{shards}}/Cargo.toml'

# Builds and stores all the modules in release mode
build-release:
	cargo build --release --manifest-path '{{rushi}}/Cargo.toml'
	cp "{{rushi}}/target/release/librushi.dylib" "{{home}}/libs/"
	cargo build --release --manifest-path '{{shards}}/Cargo.toml'

# Formats the all the modules
fmt:
	cargo fmt --manifest-path '{{rushi}}/Cargo.toml'
	cargo fmt --manifest-path '{{shards-types}}/Cargo.toml'
	cargo fmt --manifest-path '{{shards}}/Cargo.toml'

# Lints the all the modules
clippy:
	cargo clippy --manifest-path '{{rushi}}/Cargo.toml'
	cargo clippy --manifest-path '{{shards-types}}/Cargo.toml'
	cargo clippy --manifest-path '{{shards}}/Cargo.toml'

# Runs the code
run args="": build
	cargo run --manifest-path '{{shards}}/Cargo.toml' -- {{args}}

# Runs the code with optimizations enabled
run-release: build-release
	cargo run --release --manifest-path '{{shards}}/Cargo.toml'

# vim: ft=make
