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
// package org.allbinary.logic.system.hardware.components.linux
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

pub struct CpuInfo {
    pub name: String,
}

pub static CpuInfo_PROCESSOR: crate::JavaStatic<Option<Rc<RefCell<CpuInfo>>>> = crate::JavaStatic::new(|| CpuInfo::new(String::from("processor")));
pub static CpuInfo_VENDORID: crate::JavaStatic<Option<Rc<RefCell<CpuInfo>>>> = crate::JavaStatic::new(|| CpuInfo::new(String::from("vendor_id")));
pub static CpuInfo_CPUFAMILY: crate::JavaStatic<Option<Rc<RefCell<CpuInfo>>>> = crate::JavaStatic::new(|| CpuInfo::new(String::from("cpu family")));
pub static CpuInfo_MODEL: crate::JavaStatic<Option<Rc<RefCell<CpuInfo>>>> = crate::JavaStatic::new(|| CpuInfo::new(String::from("model")));
pub static CpuInfo_MODELNAME: crate::JavaStatic<Option<Rc<RefCell<CpuInfo>>>> = crate::JavaStatic::new(|| CpuInfo::new(String::from("model name")));
pub static CpuInfo_STEPPING: crate::JavaStatic<Option<Rc<RefCell<CpuInfo>>>> = crate::JavaStatic::new(|| CpuInfo::new(String::from("stepping")));
pub static CpuInfo_CPUMHZ: crate::JavaStatic<Option<Rc<RefCell<CpuInfo>>>> = crate::JavaStatic::new(|| CpuInfo::new(String::from("cpu MHz")));
pub static CpuInfo_CACHESIZE: crate::JavaStatic<Option<Rc<RefCell<CpuInfo>>>> = crate::JavaStatic::new(|| CpuInfo::new(String::from("cache size")));
pub static CpuInfo_FDIVBUG: crate::JavaStatic<Option<Rc<RefCell<CpuInfo>>>> = crate::JavaStatic::new(|| CpuInfo::new(String::from("fdiv_bug")));
pub static CpuInfo_HLTBUG: crate::JavaStatic<Option<Rc<RefCell<CpuInfo>>>> = crate::JavaStatic::new(|| CpuInfo::new(String::from("hlt_bug")));
pub static CpuInfo_F00FBUG: crate::JavaStatic<Option<Rc<RefCell<CpuInfo>>>> = crate::JavaStatic::new(|| CpuInfo::new(String::from("f00f_bug")));
pub static CpuInfo_COMABUG: crate::JavaStatic<Option<Rc<RefCell<CpuInfo>>>> = crate::JavaStatic::new(|| CpuInfo::new(String::from("coma_bug")));
pub static CpuInfo_FPU: crate::JavaStatic<Option<Rc<RefCell<CpuInfo>>>> = crate::JavaStatic::new(|| CpuInfo::new(String::from("fpu")));
pub static CpuInfo_FPUEXCEPTION: crate::JavaStatic<Option<Rc<RefCell<CpuInfo>>>> = crate::JavaStatic::new(|| CpuInfo::new(String::from("fpu_exception")));
pub static CpuInfo_CPUIDLEVEL: crate::JavaStatic<Option<Rc<RefCell<CpuInfo>>>> = crate::JavaStatic::new(|| CpuInfo::new(String::from("cache size")));
pub static CpuInfo_WP: crate::JavaStatic<Option<Rc<RefCell<CpuInfo>>>> = crate::JavaStatic::new(|| CpuInfo::new(String::from("yes")));
pub static CpuInfo_FLAGS: crate::JavaStatic<Option<Rc<RefCell<CpuInfo>>>> = crate::JavaStatic::new(|| CpuInfo::new(String::from("flags")));
pub static CpuInfo_BOGOMIPS: crate::JavaStatic<Option<Rc<RefCell<CpuInfo>>>> = crate::JavaStatic::new(|| CpuInfo::new(String::from("bogomips")));

impl CpuInfo {
    

    pub fn new_struct(mut name: String) -> Self {
        let mut this = Self {
            name: String::new(),
        };
        this.name = name.clone();
        this
    }

    pub fn new(mut name: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name))))
    }
    

    pub fn toString(&mut self) -> String {
        return self.name.clone();
    }
}

