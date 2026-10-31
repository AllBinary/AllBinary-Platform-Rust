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

thread_local! { pub static CpuInfo_PROCESSOR: Option<Rc<RefCell<CpuInfo>>> = CpuInfo::new(String::from("processor")); }
thread_local! { pub static CpuInfo_VENDORID: Option<Rc<RefCell<CpuInfo>>> = CpuInfo::new(String::from("vendor_id")); }
thread_local! { pub static CpuInfo_CPUFAMILY: Option<Rc<RefCell<CpuInfo>>> = CpuInfo::new(String::from("cpu family")); }
thread_local! { pub static CpuInfo_MODEL: Option<Rc<RefCell<CpuInfo>>> = CpuInfo::new(String::from("model")); }
thread_local! { pub static CpuInfo_MODELNAME: Option<Rc<RefCell<CpuInfo>>> = CpuInfo::new(String::from("model name")); }
thread_local! { pub static CpuInfo_STEPPING: Option<Rc<RefCell<CpuInfo>>> = CpuInfo::new(String::from("stepping")); }
thread_local! { pub static CpuInfo_CPUMHZ: Option<Rc<RefCell<CpuInfo>>> = CpuInfo::new(String::from("cpu MHz")); }
thread_local! { pub static CpuInfo_CACHESIZE: Option<Rc<RefCell<CpuInfo>>> = CpuInfo::new(String::from("cache size")); }
thread_local! { pub static CpuInfo_FDIVBUG: Option<Rc<RefCell<CpuInfo>>> = CpuInfo::new(String::from("fdiv_bug")); }
thread_local! { pub static CpuInfo_HLTBUG: Option<Rc<RefCell<CpuInfo>>> = CpuInfo::new(String::from("hlt_bug")); }
thread_local! { pub static CpuInfo_F00FBUG: Option<Rc<RefCell<CpuInfo>>> = CpuInfo::new(String::from("f00f_bug")); }
thread_local! { pub static CpuInfo_COMABUG: Option<Rc<RefCell<CpuInfo>>> = CpuInfo::new(String::from("coma_bug")); }
thread_local! { pub static CpuInfo_FPU: Option<Rc<RefCell<CpuInfo>>> = CpuInfo::new(String::from("fpu")); }
thread_local! { pub static CpuInfo_FPUEXCEPTION: Option<Rc<RefCell<CpuInfo>>> = CpuInfo::new(String::from("fpu_exception")); }
thread_local! { pub static CpuInfo_CPUIDLEVEL: Option<Rc<RefCell<CpuInfo>>> = CpuInfo::new(String::from("cache size")); }
thread_local! { pub static CpuInfo_WP: Option<Rc<RefCell<CpuInfo>>> = CpuInfo::new(String::from("yes")); }
thread_local! { pub static CpuInfo_FLAGS: Option<Rc<RefCell<CpuInfo>>> = CpuInfo::new(String::from("flags")); }
thread_local! { pub static CpuInfo_BOGOMIPS: Option<Rc<RefCell<CpuInfo>>> = CpuInfo::new(String::from("bogomips")); }

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

