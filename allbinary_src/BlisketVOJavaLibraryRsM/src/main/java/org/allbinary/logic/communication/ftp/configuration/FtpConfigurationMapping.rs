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
// package org.allbinary.logic.communication.ftp.configuration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::data::tables::TableMappingInterface::TableMappingInterface;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use super::FtpConfigurationInterface::FtpConfigurationInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FtpConfigurationMapping {
    pub ftpConfigurationInterface: Option<Rc<RefCell<dyn FtpConfigurationInterface>>>,
}

impl TableMappingInterface for FtpConfigurationMapping {
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getKey(self) }
    fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::toVector(self) }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { Self::toHashMap(self) }
}

impl FtpConfigurationMapping {
    

    pub fn new_struct(mut ftpConfigurationInterface: Option<Rc<RefCell<dyn FtpConfigurationInterface>>>) -> Self {
        let mut this = Self {
            ftpConfigurationInterface: None,
        };
        this.ftpConfigurationInterface = ftpConfigurationInterface.clone();
        this
    }

    pub fn new(mut ftpConfigurationInterface: Option<Rc<RefCell<dyn FtpConfigurationInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(ftpConfigurationInterface))))
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(FtpConfigurationData_SERVER.with(Clone::clone).clone(), jcall!(self.ftpConfigurationInterface).getServer());
        jcall!(hashMap).put(FtpConfigurationData_USERNAME.with(Clone::clone).clone(), jcall!(self.ftpConfigurationInterface).getUserName());
        jcall!(hashMap).put(FtpConfigurationData_PASSWORD.with(Clone::clone).clone(), jcall!(self.ftpConfigurationInterface).getPassword());
        jcall!(hashMap).put(FtpConfigurationData_PATH.with(Clone::clone).clone(), jcall!(self.ftpConfigurationInterface).getPath());
        return hashMap.clone();
    }
    

    // throws Exception
    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return FtpConfigurationData_NAME.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(vector).add(crate::java_into_object(jcall!(self.ftpConfigurationInterface).getServer()));
        jcall!(vector).add(crate::java_into_object(jcall!(self.ftpConfigurationInterface).getUserName()));
        jcall!(vector).add(crate::java_into_object(jcall!(self.ftpConfigurationInterface).getPassword()));
        jcall!(vector).add(crate::java_into_object(jcall!(self.ftpConfigurationInterface).getPath()));
        return vector.clone();
    }
}

