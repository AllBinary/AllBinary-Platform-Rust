// Generated Code Do Not Modify
// package org.bouncycastle.jce.provider
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::security::Provider::Provider;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BouncyCastleProvider {
    pub base: Provider,
}

pub static mut BouncyCastleProvider_info: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("BouncyCastle Security Provider v1.41"));
pub static mut BouncyCastleProvider_PROVIDER_NAME: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("BC"));

impl BouncyCastleProvider {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Provider::new_struct3(unsafe { BouncyCastleProvider_PROVIDER_NAME.clone() }.clone(), 1.41, unsafe { BouncyCastleProvider_info.clone() }.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setParameter(&mut self, mut parameterName: String, mut parameter: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
}
impl std::ops::Deref for BouncyCastleProvider {
    type Target = Provider;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BouncyCastleProvider {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



