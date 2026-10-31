// 
// AllBinary Open License Version 1 
// Copyright (c) 2026 AllBinary 
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
// package org.allbinary.logic.io.file
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

pub struct CommonDataFileStrings {
    pub XML: String,
    pub _XML: String,
    pub JSON: String,
    pub _JSON: String,
    pub XSL: String,
    pub _XSL: String,
    pub JAVA: String,
    pub _JAVA: String,
    pub KT: String,
    pub _KT: String,
    pub TS: String,
    pub _TS: String,
    pub JS: String,
    pub _JS: String,
    pub CPP: String,
    pub _CPP: String,
    pub H: String,
    pub _H: String,
    pub PY: String,
    pub _PY: String,
    pub GO: String,
    pub _GO: String,
    pub CS: String,
    pub _CS: String,
    pub ADB: String,
    pub _ADB: String,
    pub ADS: String,
    pub _ADS: String,
    pub ADA: String,
    pub _ADA: String,
    pub RUST: String,
    pub _RUST: String,
    pub C: String,
    pub _C: String,
    pub CSHARP: String,
    pub _CSHARP: String,
    pub ERL: String,
    pub _ERL: String,
    pub HS: String,
    pub _HS: String,
    pub HX: String,
    pub _HX: String,
    pub LUA: String,
    pub _LUA: String,
    pub PAS: String,
    pub _PAS: String,
    pub ML: String,
    pub _ML: String,
    pub PL: String,
    pub _PL: String,
    pub RB: String,
    pub _RB: String,
    pub ZIG: String,
    pub _ZIG: String,
    pub UNCRYPTED_EXTENSION: String,
    pub ENCRYPTED_EXTENSION: String,
}

thread_local! { pub static CommonDataFileStrings_instance: Option<Rc<RefCell<CommonDataFileStrings>>> = CommonDataFileStrings::new(); }

impl CommonDataFileStrings {

    pub fn new_struct() -> Self {
        let mut this = Self {
            XML: String::new(),
            _XML: String::new(),
            JSON: String::new(),
            _JSON: String::new(),
            XSL: String::new(),
            _XSL: String::new(),
            JAVA: String::new(),
            _JAVA: String::new(),
            KT: String::new(),
            _KT: String::new(),
            TS: String::new(),
            _TS: String::new(),
            JS: String::new(),
            _JS: String::new(),
            CPP: String::new(),
            _CPP: String::new(),
            H: String::new(),
            _H: String::new(),
            PY: String::new(),
            _PY: String::new(),
            GO: String::new(),
            _GO: String::new(),
            CS: String::new(),
            _CS: String::new(),
            ADB: String::new(),
            _ADB: String::new(),
            ADS: String::new(),
            _ADS: String::new(),
            ADA: String::new(),
            _ADA: String::new(),
            RUST: String::new(),
            _RUST: String::new(),
            C: String::new(),
            _C: String::new(),
            CSHARP: String::new(),
            _CSHARP: String::new(),
            ERL: String::new(),
            _ERL: String::new(),
            HS: String::new(),
            _HS: String::new(),
            HX: String::new(),
            _HX: String::new(),
            LUA: String::new(),
            _LUA: String::new(),
            PAS: String::new(),
            _PAS: String::new(),
            ML: String::new(),
            _ML: String::new(),
            PL: String::new(),
            _PL: String::new(),
            RB: String::new(),
            _RB: String::new(),
            ZIG: String::new(),
            _ZIG: String::new(),
            UNCRYPTED_EXTENSION: String::new(),
            ENCRYPTED_EXTENSION: String::new(),
        };
        this.XML = String::from("xml");
        this._XML = String::from(".xml");
        this.JSON = String::from("json");
        this._JSON = String::from(".json");
        this.XSL = String::from("xsl");
        this._XSL = String::from(".xsl");
        this.JAVA = String::from("java");
        this._JAVA = String::from(".java");
        this.KT = String::from("kt");
        this._KT = String::from(".kt");
        this.TS = String::from("ts");
        this._TS = String::from(".ts");
        this.JS = String::from("js");
        this._JS = String::from(".js");
        this.CPP = String::from("cpp");
        this._CPP = String::from(".cpp");
        this.H = String::from("h");
        this._H = String::from(".h");
        this.PY = String::from("py");
        this._PY = String::from(".py");
        this.GO = String::from("go");
        this._GO = String::from(".go");
        this.CS = String::from("cs");
        this._CS = String::from(".cs");
        this.ADB = String::from("adb");
        this._ADB = String::from(".adb");
        this.ADS = String::from("ads");
        this._ADS = String::from(".ads");
        this.ADA = String::from("ada");
        this._ADA = String::from(".ada");
        this.RUST = String::from("rs");
        this._RUST = String::from(".rs");
        this.C = String::from("c");
        this._C = String::from(".c");
        this.CSHARP = String::from("cs");
        this._CSHARP = String::from(".cs");
        this.ERL = String::from("erl");
        this._ERL = String::from(".erl");
        this.HS = String::from("hs");
        this._HS = String::from(".hs");
        this.HX = String::from("hx");
        this._HX = String::from(".hx");
        this.LUA = String::from("lua");
        this._LUA = String::from(".lua");
        this.PAS = String::from("pas");
        this._PAS = String::from(".pas");
        this.ML = String::from("ml");
        this._ML = String::from(".ml");
        this.PL = String::from("pl");
        this._PL = String::from(".pl");
        this.RB = String::from("rb");
        this._RB = String::from(".rb");
        this.ZIG = String::from("zig");
        this._ZIG = String::from(".zig");
        this.UNCRYPTED_EXTENSION = this.XML.clone();
        this.ENCRYPTED_EXTENSION = String::from("abd");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<CommonDataFileStrings>>> {
        return CommonDataFileStrings_instance.with(Clone::clone).clone();
    }
}

