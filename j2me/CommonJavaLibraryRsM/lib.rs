// Crate root for the generated Rust sources.
// The generated code uses paths such as crate::org::allbinary::string::CommonSeps so build.rs generates the module tree from the Java package directories.
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

include!(concat!(env!("OUT_DIR"), "/modules.rs"));

// A Java static final field: pub static X: crate::JavaStatic<T> = crate::JavaStatic::new(|| init), read like a thread local with X.with(Clone::clone).
// Java has one value of a static field for every thread (a thread_local! would give each Java Thread its own singletons).
// The value is made on first use like a Java class initializer. Java code only runs while its thread holds the Java lock
// (java::lang::JavaLock of the runtime), so one thread at a time uses the value.
pub struct JavaStatic<T: 'static> {
    cell: std::cell::LazyCell<T, fn() -> T>,
}

unsafe impl<T: 'static> Sync for JavaStatic<T> {}

impl<T: 'static> JavaStatic<T> {
    pub const fn new(init: fn() -> T) -> Self {
        Self { cell: std::cell::LazyCell::new(init) }
    }

    pub fn with<R, F: FnOnce(&T) -> R>(&'static self, f: F) -> R {
        f(std::cell::LazyCell::force(&self.cell))
    }
}
