# Treet

A parsing library for turning tree sitter tree into tree that can be parsed by
shards.

## Building
This project is built using nix so to do so just run:
```bash
nix build
```

## Development
by default your editor (zls) will not give you much information as it wont be 
able to find the need libraries when edition code. To fix this just run.
```
nix develop

# In Nix Shell
nvim
```
