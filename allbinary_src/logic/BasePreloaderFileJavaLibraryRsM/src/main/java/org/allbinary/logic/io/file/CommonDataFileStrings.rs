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
    pub PM: String,
    pub _PM: String,
    pub KTS: String,
    pub _KTS: String,
    pub CBL: String,
    pub _CBL: String,
    pub COB: String,
    pub _COB: String,
    pub F90: String,
    pub _F90: String,
    pub F: String,
    pub _F: String,
    pub DART: String,
    pub _DART: String,
    pub EX: String,
    pub _EX: String,
    pub EXS: String,
    pub _EXS: String,
    pub KOTLIN: String,
    pub TYPESCRIPT: String,
    pub JAVASCRIPT: String,
    pub C_PLUS_PLUS: String,
    pub PYTHON: String,
    pub C_SHARP: String,
    pub RUST_NAME: String,
    pub RUBY: String,
    pub PERL: String,
    pub OBJECT_PASCAL: String,
    pub OCAML: String,
    pub HASKELL: String,
    pub HAXE: String,
    pub ERLANG: String,
    pub COBOL: String,
    pub FORTRAN: String,
    pub ELIXIR: String,
    pub CPLUSPLUS: String,
    pub GOLANG: String,
    pub CSHARP_ALIAS: String,
    pub C23: String,
    pub PASCAL: String,
    pub OBJECT_PASCAL_ALIAS: String,
    pub KT_SHORT_NAME: String,
    pub TS_SHORT_NAME: String,
    pub JS_SHORT_NAME: String,
    pub CPP_SHORT_NAME: String,
    pub PY_SHORT_NAME: String,
    pub GO_SHORT_NAME: String,
    pub CS_SHORT_NAME: String,
    pub ADA_SHORT_NAME: String,
    pub RUST_SHORT_NAME: String,
    pub LUA_SHORT_NAME: String,
    pub C_SHORT_NAME: String,
    pub ZIG_SHORT_NAME: String,
    pub RB_SHORT_NAME: String,
    pub PL_SHORT_NAME: String,
    pub PAS_SHORT_NAME: String,
    pub ML_SHORT_NAME: String,
    pub HS_SHORT_NAME: String,
    pub HX_SHORT_NAME: String,
    pub ERL_SHORT_NAME: String,
    pub CBL_SHORT_NAME: String,
    pub F90_SHORT_NAME: String,
    pub DART_SHORT_NAME: String,
    pub EX_SHORT_NAME: String,
    pub UNCRYPTED_EXTENSION: String,
    pub ENCRYPTED_EXTENSION: String,
}

thread_local! { pub static CommonDataFileStrings_instance: Option<Rc<RefCell<CommonDataFileStrings>>> = CommonDataFileStrings::new(); }

impl CommonDataFileStrings {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

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
            PM: String::new(),
            _PM: String::new(),
            KTS: String::new(),
            _KTS: String::new(),
            CBL: String::new(),
            _CBL: String::new(),
            COB: String::new(),
            _COB: String::new(),
            F90: String::new(),
            _F90: String::new(),
            F: String::new(),
            _F: String::new(),
            DART: String::new(),
            _DART: String::new(),
            EX: String::new(),
            _EX: String::new(),
            EXS: String::new(),
            _EXS: String::new(),
            KOTLIN: String::new(),
            TYPESCRIPT: String::new(),
            JAVASCRIPT: String::new(),
            C_PLUS_PLUS: String::new(),
            PYTHON: String::new(),
            C_SHARP: String::new(),
            RUST_NAME: String::new(),
            RUBY: String::new(),
            PERL: String::new(),
            OBJECT_PASCAL: String::new(),
            OCAML: String::new(),
            HASKELL: String::new(),
            HAXE: String::new(),
            ERLANG: String::new(),
            COBOL: String::new(),
            FORTRAN: String::new(),
            ELIXIR: String::new(),
            CPLUSPLUS: String::new(),
            GOLANG: String::new(),
            CSHARP_ALIAS: String::new(),
            C23: String::new(),
            PASCAL: String::new(),
            OBJECT_PASCAL_ALIAS: String::new(),
            KT_SHORT_NAME: String::new(),
            TS_SHORT_NAME: String::new(),
            JS_SHORT_NAME: String::new(),
            CPP_SHORT_NAME: String::new(),
            PY_SHORT_NAME: String::new(),
            GO_SHORT_NAME: String::new(),
            CS_SHORT_NAME: String::new(),
            ADA_SHORT_NAME: String::new(),
            RUST_SHORT_NAME: String::new(),
            LUA_SHORT_NAME: String::new(),
            C_SHORT_NAME: String::new(),
            ZIG_SHORT_NAME: String::new(),
            RB_SHORT_NAME: String::new(),
            PL_SHORT_NAME: String::new(),
            PAS_SHORT_NAME: String::new(),
            ML_SHORT_NAME: String::new(),
            HS_SHORT_NAME: String::new(),
            HX_SHORT_NAME: String::new(),
            ERL_SHORT_NAME: String::new(),
            CBL_SHORT_NAME: String::new(),
            F90_SHORT_NAME: String::new(),
            DART_SHORT_NAME: String::new(),
            EX_SHORT_NAME: String::new(),
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
        this.PM = String::from("pm");
        this._PM = String::from(".pm");
        this.KTS = String::from("kts");
        this._KTS = String::from(".kts");
        this.CBL = String::from("cbl");
        this._CBL = String::from(".cbl");
        this.COB = String::from("cob");
        this._COB = String::from(".cob");
        this.F90 = String::from("f90");
        this._F90 = String::from(".f90");
        this.F = String::from("f");
        this._F = String::from(".f");
        this.DART = String::from("dart");
        this._DART = String::from(".dart");
        this.EX = String::from("ex");
        this._EX = String::from(".ex");
        this.EXS = String::from("exs");
        this._EXS = String::from(".exs");
        this.KOTLIN = String::from("kotlin");
        this.TYPESCRIPT = String::from("typescript");
        this.JAVASCRIPT = String::from("javascript");
        this.C_PLUS_PLUS = String::from("c++");
        this.PYTHON = String::from("python");
        this.C_SHARP = String::from("c#");
        this.RUST_NAME = String::from("rust");
        this.RUBY = String::from("ruby");
        this.PERL = String::from("perl");
        this.OBJECT_PASCAL = String::from("objectpascal");
        this.OCAML = String::from("ocaml");
        this.HASKELL = String::from("haskell");
        this.HAXE = String::from("haxe");
        this.ERLANG = String::from("erlang");
        this.COBOL = String::from("cobol");
        this.FORTRAN = String::from("fortran");
        this.ELIXIR = String::from("elixir");
        this.CPLUSPLUS = String::from("cplusplus");
        this.GOLANG = String::from("golang");
        this.CSHARP_ALIAS = String::from("csharp");
        this.C23 = String::from("c23");
        this.PASCAL = String::from("pascal");
        this.OBJECT_PASCAL_ALIAS = String::from("object pascal");
        this.KT_SHORT_NAME = String::from("Kt");
        this.TS_SHORT_NAME = String::from("Ts");
        this.JS_SHORT_NAME = String::from("Js");
        this.CPP_SHORT_NAME = String::from("Cpp");
        this.PY_SHORT_NAME = String::from("Py");
        this.GO_SHORT_NAME = String::from("Go");
        this.CS_SHORT_NAME = String::from("Cs");
        this.ADA_SHORT_NAME = String::from("Ada");
        this.RUST_SHORT_NAME = String::from("Rust");
        this.LUA_SHORT_NAME = String::from("Lua");
        this.C_SHORT_NAME = String::from("C");
        this.ZIG_SHORT_NAME = String::from("Zig");
        this.RB_SHORT_NAME = String::from("Rb");
        this.PL_SHORT_NAME = String::from("Pl");
        this.PAS_SHORT_NAME = String::from("Pas");
        this.ML_SHORT_NAME = String::from("Ml");
        this.HS_SHORT_NAME = String::from("Hs");
        this.HX_SHORT_NAME = String::from("Hx");
        this.ERL_SHORT_NAME = String::from("Erl");
        this.CBL_SHORT_NAME = String::from("Cbl");
        this.F90_SHORT_NAME = String::from("F90");
        this.DART_SHORT_NAME = String::from("Dart");
        this.EX_SHORT_NAME = String::from("Ex");
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

