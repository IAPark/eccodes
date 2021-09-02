//!# Unofficial high-level safe Rust bindings to ecCodes library
//!
//!This crate contains safe high-level bindings for ecCodes library. 
//!Bindings can be considered safe mainly because all crate structures 
//!will take ownership of the data in memory before passing the raw pointer to ecCodes. 
//!**Currently only reading of GRIB files is supported.**
//!
//!Because of the ecCodes library API characteristics theses bindings are 
//!rather thick wrapper to make this crate safe and convienient to use.
//!
//!Because ecCodes supports mainly Linux platforms, this crate is not tested on other architectures.
//!
//!If you want to see more features released quicker do not hesitate 
//!to contribute and check out Github repository.
//!
//![ecCodes](https://confluence.ecmwf.int/display/ECC/ecCodes+Home) is an open-source library 
//!for reading and writing GRIB and BUFR files developed by [European Centre for Medium-Range Weather Forecasts](https://www.ecmwf.int/).
//!
//!## Usage
//!
//!### Accesing GRIB files
//!
//!This crate provide an access to GRIB file by creating a [`CodesHandle`](codes_handle::CodesHandle) and reading messages from the file.
//!
//!The [`CodesHandle`](codes_handle::CodesHandle) can be constructed in two ways.
//!
//!The main option is to use [`new_from_file()`](codes_handle::CodesHandle::new_from_file) function
//!to open a file under provided [`path`](`std::path::Path`) with filesystem, 
//!when copying whole file into memory is not desired or not necessary.
//!
//!```
//!
//!```
//!
//!Alternatively [`new_from_memory()`](codes_handle::CodesHandle::new_from_memory) function can be used
//!to access a file that is already in memory. For example, when file is downloaded from the internet 
//!and does not need to be saved on hard drive. The file must be stored in [`bytes::Bytes`](https://docs.rs/bytes/1.1.0/bytes/struct.Bytes.html).
//!
//!```
//!
//!```
//!
//!### Build options
//!
//!This crate uses [eccodes-sys](https://crates.io/crates/eccodes-sys) with default options to link ecCodes. 
//!Check `eccodes-sys` website for more details on how it links the library.
//!
//!If you would like to build ecCodes with other options simply import `eccodes-sys` 
//!along with `eccodes` in your `Cargo.toml` file and select needed features.
//!
//!For example: 
//!
//!```no_run
//![dependencies]
//!eccodes = "0.1.0"
//!eccodes-sys = { version="0.1.3", features=["build_source"] }
//!```
//!
//!### Features
//!
//!- `docs` - builds the create without linking ecCodes, particularly useful when building the documentation
//!on [docs.rs](https://docs.rs/). For more details check documentation of [eccodes-sys](https://crates.io/crates/eccodes-sys).
//!
//!To build your own crate with this crate as dependency on docs.rs without linking ecCodes add following lines to your `Cargo.toml`
//!
//!```no_run
//![package.metadata.docs.rs]
//!features = ["eccodes/docs"]
//!```
//!

pub mod codes_handle;
pub mod errors;
