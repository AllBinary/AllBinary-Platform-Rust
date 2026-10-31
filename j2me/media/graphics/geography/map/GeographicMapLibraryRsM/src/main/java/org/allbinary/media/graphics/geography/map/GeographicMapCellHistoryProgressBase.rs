// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
// package org.allbinary.media.graphics.geography.map
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GeographicMapCellHistoryProgressBase {
}

thread_local! { pub static GeographicMapCellHistoryProgressBase_instance: Option<Rc<RefCell<dyn GeographicMapCellHistoryProgressBaseDyn>>> = GeographicMapCellHistoryProgressBase::new().map(|reference| reference as Rc<RefCell<dyn GeographicMapCellHistoryProgressBaseDyn>>); }

impl GeographicMapCellHistoryProgressBase {
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn GeographicMapCellHistoryProgressBaseDyn>>> {
        return GeographicMapCellHistoryProgressBase_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn isAnyProgress(&mut self) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}
pub trait GeographicMapCellHistoryProgressBaseDyn {
    fn as_GeographicMapCellHistoryProgressBase(&self) -> &GeographicMapCellHistoryProgressBase;
    fn as_GeographicMapCellHistoryProgressBase_mut(&mut self) -> &mut GeographicMapCellHistoryProgressBase;
    fn init(&mut self) { GeographicMapCellHistoryProgressBase::init(self.as_GeographicMapCellHistoryProgressBase_mut()) }
    fn isAnyProgress(&mut self) -> bool { GeographicMapCellHistoryProgressBase::isAnyProgress(self.as_GeographicMapCellHistoryProgressBase_mut()) }
}
impl GeographicMapCellHistoryProgressBaseDyn for GeographicMapCellHistoryProgressBase {
    fn as_GeographicMapCellHistoryProgressBase(&self) -> &GeographicMapCellHistoryProgressBase { self }
    fn as_GeographicMapCellHistoryProgressBase_mut(&mut self) -> &mut GeographicMapCellHistoryProgressBase { self }
}


