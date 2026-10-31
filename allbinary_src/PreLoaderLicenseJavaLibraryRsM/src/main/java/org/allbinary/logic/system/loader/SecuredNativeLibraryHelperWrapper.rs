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
use crate::org::allbinary::logic::system::security::AbKeys::AbKeys;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use super::NativeLibraryHelperWrapper::NativeLibraryHelperWrapper;
use super::SecuredNativeLibraryInterface::SecuredNativeLibraryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SecuredNativeLibraryHelperWrapper {
    pub base: NativeLibraryHelperWrapper,
    pub securedNativeLibraryInterface: Option<Rc<RefCell<dyn SecuredNativeLibraryInterface>>>,
}


impl SecuredNativeLibraryHelperWrapper {
    

    pub fn new_struct(mut securedNativeLibraryInterface: Option<Rc<RefCell<dyn SecuredNativeLibraryInterface>>>) -> Self {
        let mut this = Self {
            base: NativeLibraryHelperWrapper::new_struct(jcall!(securedNativeLibraryInterface).getLibraryName()),
            securedNativeLibraryInterface: None,
        };
        // super(...) is in the struct literal above
        this.securedNativeLibraryInterface = securedNativeLibraryInterface.clone();
        this
    }

    pub fn new(mut securedNativeLibraryInterface: Option<Rc<RefCell<dyn SecuredNativeLibraryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(securedNativeLibraryInterface))))
    }
    

    // throws LicensingException
    pub fn unlock(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) -> bool {
        return { let java_receiver = self.securedNativeLibraryInterface.clone(); let java_arguments = (jcall!(AbKeys::getInstance()).getKey(abeClientInformation.clone(), jcall!(self.securedNativeLibraryInterface).getName()),); let java_result = jcall!(java_receiver).unlock(java_arguments.0); java_result };
    }
}
impl std::ops::Deref for SecuredNativeLibraryHelperWrapper {
    type Target = NativeLibraryHelperWrapper;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SecuredNativeLibraryHelperWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::system::loader::NativeLibraryHelperWrapper::NativeLibraryHelperWrapperDyn for SecuredNativeLibraryHelperWrapper {
    fn as_NativeLibraryHelperWrapper(&self) -> &crate::org::allbinary::logic::system::loader::NativeLibraryHelperWrapper::NativeLibraryHelperWrapper { &self.base }
    fn as_NativeLibraryHelperWrapper_mut(&mut self) -> &mut crate::org::allbinary::logic::system::loader::NativeLibraryHelperWrapper::NativeLibraryHelperWrapper { &mut self.base }
}


