// Generated Code Do Not Modify
// package org.osgi.framework
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::util::Dictionary::Dictionary;
use super::Bundle::Bundle;
use super::ServiceRegistration::ServiceRegistration;
use super::ServiceReference::ServiceReference;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait BundleContext {
    

    fn getBundles(&mut self) -> Vec<Option<Rc<RefCell<dyn Bundle>>>>;
    

    // throws BundleException
    fn installBundle(&mut self, location: String) -> Option<Rc<RefCell<dyn Bundle>>>;
    

    fn registerService(&mut self, clazz: String, service: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, properties: Option<Rc<RefCell<Dictionary>>>) -> Option<Rc<RefCell<dyn ServiceRegistration>>>;
    

    fn getService(&mut self, reference: Option<Rc<RefCell<dyn ServiceReference>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>>;
    

    // throws InvalidSyntaxException
    fn getServiceReferences(&mut self, clazz: String, filter: String) -> Vec<Option<Rc<RefCell<dyn ServiceReference>>>>;
    

    fn getServiceReference(&mut self, clazz: String) -> Option<Rc<RefCell<dyn ServiceReference>>>;

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


