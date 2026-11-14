//
// AllBinary Open License Version 1
// Copyright (c) 2011 AllBinary
// 
// By agreeing to this license you and any business entity you represent are
// legally bound to the AllBinary Open License Version 1 legal agreement.
// 
// You may obtain the AllBinary Open License Version 1 legal agreement from
// AllBinary or the root directory of AllBinary's AllBinary Platform repository.
// 
// Created By: Travis Berthelot
//  
// Generated Code Do Not Modify
// package org.allbinary.logic.system.loader
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::lang::reflect::Constructor::Constructor;
// use crate::java::lang::reflect::Method::Method;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;
use crate::java::lang::ClassLoader::*;
use super::NativeLibraryClassLoader::NativeLibraryClassLoader;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NativeLibraryHelperWrapper {
    pub libraryName: String,
    pub loader: Option<Rc<RefCell<ClassLoader>>>,
    pub anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>,
}


impl NativeLibraryHelperWrapper {
    

    pub fn new_struct(mut libraryName: String) -> Self {
        let mut this = Self {
            libraryName: String::new(),
            loader: None,
            anyType: None,
        };
        this.setLibraryName(libraryName.clone());
        this
    }

    pub fn new(mut libraryName: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(libraryName))))
    }
    

    // throws Exception
    pub fn load(&mut self) {
        let mut params: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = vec![None; (1) as usize];
        params[(0) as usize] = crate::java_into_object(self.getLibraryName());
        let mut classes: Vec<&'static str> = vec![""; (1) as usize];
        classes[(0) as usize] = std::any::type_name_of_val(&self.getLibraryName());
        self.loader = { let java_value = (NativeLibraryClassLoader::new({ let java_argument = jcall!(std::any::type_name_of_val(&self)).getClassLoader(); java_argument }).map(|reference| Rc::new(RefCell::new(Rc::try_unwrap(reference).ok().expect("new object is not shared").into_inner().base)))).clone(); java_value };
        let mut myClass: &'static str = jcall!(self.loader).loadClass(String::from("dynamic.NativeLibraryHelper"));
        let mut constructor: Option<Rc<RefCell<Constructor>>> = jcall!(myClass).getConstructor(classes.clone());
        self.anyType = crate::java_into_object({ let java_value = (jcall!(constructor).newInstance(params.clone())).clone(); java_value });
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        let mut method: Option<Rc<RefCell<Method>>> = jcall!(std::any::type_name_of_val(&self.anyType)).getMethod({ let java_field = jcall!(commonStrings).LOAD.clone(); java_field }.clone(), None);
        jcall!(method).invoke(self.anyType.clone(), None);
    }
    

    pub fn unload(&mut self) {
        self.anyType = None;
        self.loader = None;
        ();
        ();
    }
    

    pub fn toString(&mut self) -> String {
        return format!("{}{}", String::from("Library: "), self.getLibraryName());
    }
    

    pub fn getLibraryName(&mut self) -> String {
        return self.libraryName.clone();
    }
    

    pub fn setLibraryName(&mut self, mut libraryName: String) {
        self.libraryName = libraryName.clone();
    }
}
pub trait NativeLibraryHelperWrapperDyn: std::any::Any {
    fn as_NativeLibraryHelperWrapper(&self) -> &NativeLibraryHelperWrapper;
    fn as_NativeLibraryHelperWrapper_mut(&mut self) -> &mut NativeLibraryHelperWrapper;
    fn load(&mut self) { NativeLibraryHelperWrapper::load(self.as_NativeLibraryHelperWrapper_mut()) }
    fn unload(&mut self) { NativeLibraryHelperWrapper::unload(self.as_NativeLibraryHelperWrapper_mut()) }
    fn toString(&mut self) -> String { NativeLibraryHelperWrapper::toString(self.as_NativeLibraryHelperWrapper_mut()) }
    fn getLibraryName(&mut self) -> String { NativeLibraryHelperWrapper::getLibraryName(self.as_NativeLibraryHelperWrapper_mut()) }
    fn setLibraryName(&mut self, mut libraryName: String) { NativeLibraryHelperWrapper::setLibraryName(self.as_NativeLibraryHelperWrapper_mut(), libraryName) }
}
impl NativeLibraryHelperWrapperDyn for NativeLibraryHelperWrapper {
    fn as_NativeLibraryHelperWrapper(&self) -> &NativeLibraryHelperWrapper { self }
    fn as_NativeLibraryHelperWrapper_mut(&mut self) -> &mut NativeLibraryHelperWrapper { self }
}


