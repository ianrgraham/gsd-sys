# gsd-sys
This crate presents an FFI to GSD's native C API. The GSD (General Simulation Data)
file format is the native file format of [HOOMD-blue](https://glotzerlab.engin.umich.edu/hoomd-blue/)
and is used to store and analyze HOOMD-blue simulations. Though it's also a perfectly good binary file
format for chunk-like data, so why not wrap Rust around it?

Original documentation for the API can be found 
[here](https://gsd.readthedocs.io/en/stable/c-api.html).

## Usage
To add `gsd-sys` as a dependency in `Cargo.toml`, please clone the repository
onto your system and use it as a path dependency (until I get around to
publishing it on crates.io alongside a high-level API):

```toml
[dependencies]
gsd-sys = { path = "path/to/gsd-sys" }
```
