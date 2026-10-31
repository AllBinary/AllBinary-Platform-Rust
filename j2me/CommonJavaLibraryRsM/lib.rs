// Crate root for the generated Rust sources.
// The generated code uses paths such as crate::org::allbinary::string::CommonSeps so build.rs generates the module tree from the Java package directories.
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

include!(concat!(env!("OUT_DIR"), "/modules.rs"));
