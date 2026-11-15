// Generated Code Do Not Modify
// package org.osgi.framework
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait Constants: std::any::Any + crate::java::lang::Object::JavaObjectToString {
}

pub static Constants_BUNDLE_SYMBOLICNAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone());

