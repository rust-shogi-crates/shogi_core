# Must be run in the crate's root directory
cbindgen --config cbindgen.toml --crate shogi_core --output include/shogi_core.h
