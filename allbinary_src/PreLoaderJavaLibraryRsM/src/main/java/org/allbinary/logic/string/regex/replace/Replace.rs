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
// package org.allbinary.logic.string.regex.replace
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::J2SEUtil::J2SEUtil;
use crate::org::allbinary::globals::AppUrlGlobals::AppUrlGlobals;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Replace {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub j2seUtil: Option<Rc<RefCell<J2SEUtil>>>,
    pub REPLACERS_: String,
    pub ALL: String,
    pub FOUND_KEY: String,
    pub hashMap: Option<Rc<RefCell<HashMap>>>,
}


impl Replace {
    

    pub fn new_struct2(mut key: String, mut value: String) -> Self {
        let mut this = Self {
            logUtil: None,
            j2seUtil: None,
            REPLACERS_: String::new(),
            ALL: String::new(),
            FOUND_KEY: String::new(),
            hashMap: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.j2seUtil = J2SEUtil::getInstance();
        this.REPLACERS_ = String::from("Replacers: ");
        this.ALL = String::from("all");
        this.FOUND_KEY = String::from("found key total: ");
        this.hashMap = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(this.hashMap).put(vec![key.clone()], vec![value.clone()]);
        this
    }

    pub fn new2(mut key: String, mut value: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(key, value))))
    }
    

    pub fn new_struct2(mut keys: Vec<String>, mut values: Vec<String>) -> Self {
        let mut this = Self {
            logUtil: None,
            j2seUtil: None,
            REPLACERS_: String::new(),
            ALL: String::new(),
            FOUND_KEY: String::new(),
            hashMap: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.j2seUtil = J2SEUtil::getInstance();
        this.REPLACERS_ = String::from("Replacers: ");
        this.ALL = String::from("all");
        this.FOUND_KEY = String::from("found key total: ");
        this.hashMap = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(this.hashMap).put(keys.clone(), values.clone());
        this
    }

    pub fn new2(mut keys: Vec<String>, mut values: Vec<String>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(keys, values))))
    }
    

    pub fn new_struct1(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            j2seUtil: None,
            REPLACERS_: String::new(),
            ALL: String::new(),
            FOUND_KEY: String::new(),
            hashMap: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.j2seUtil = J2SEUtil::getInstance();
        this.REPLACERS_ = String::from("Replacers: ");
        this.ALL = String::from("all");
        this.FOUND_KEY = String::from("found key total: ");
        this.hashMap = hashMap.clone();
        this
    }

    pub fn new1(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(hashMap))))
    }
    

    pub fn all(&mut self, mut replace: String) -> String {
        return self.upTo(replace.clone(), i32::MIN.clone());
    }
    

    pub fn upTo(&mut self, mut replace: String, mut total: i32) -> String {
        let mut newStringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut totalNumberOfReplaces: i32 = 0;
        let mut keys: Vec<String> = Vec::new();
        let mut values: Vec<String> = Vec::new();
        let mut keyArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(self.j2seUtil).getHashMapAsArray(self.hashMap.clone());
        let mut size: i32 = (keyArray.len() as i32).clone();
        for index2 in (0..size) {
            keys = (keyArray[(index2) as usize]).clone() /* as  */;
            values = (jcall!(self.hashMap).get(keys.clone())).clone() /* as  */;
            let mut foundTotal: i64 = 0_i64;
            let mut index: i32 = 0;
            let mut nextKey: String = String::new();
            let mut end: i32 = 0;
            let mut nextSecondKey: String = String::new();
            let mut begin2: i32 = 0;
            while index < (replace.chars().count() as i32) {
                let mut begin: i32 = jcall!(replace).indexOf(keys[(0) as usize].clone(), index.clone());
                if begin != -1 {
                    foundTotal += 1;
                    if foundTotal % 100_i64 == 0_i64 {
                        println!("{}", format!("{}{}", self.FOUND_KEY, foundTotal));
                    }
                    nextKey = keys[(0) as usize].clone();
                    end = begin + (nextKey.chars().count() as i32);
                    jcall!(newStringBuffer).delete(0, jcall!(newStringBuffer).length());
                    jcall!(newStringBuffer).append(replace[(0) as usize..(begin) as usize].to_string());
                    jcall!(newStringBuffer).append(values[(0) as usize].clone());
                    index = jcall!(newStringBuffer).length();
                    jcall!(newStringBuffer).append(replace[(end) as usize..((replace.chars().count() as i32)) as usize].to_string());
                    replace = jcall!(newStringBuffer).toString();
                    if (keys.len() as i32) > 1 {
                        nextSecondKey = keys[(1) as usize].clone();
                        begin2 = jcall!(replace).indexOf(nextSecondKey.clone(), index.clone());
                        if begin2 != -1 {
                            let mut end2: i32 = begin2 + (nextSecondKey.chars().count() as i32);
                            jcall!(newStringBuffer).delete(0, jcall!(newStringBuffer).length());
                            jcall!(newStringBuffer).append(replace[(0) as usize..(begin2) as usize].to_string());
                            jcall!(newStringBuffer).append(values[(1) as usize].clone());
                            index = jcall!(newStringBuffer).length();
                            jcall!(newStringBuffer).append(replace[(end2) as usize..((replace.chars().count() as i32)) as usize].to_string());
                            replace = jcall!(newStringBuffer).toString();
                        }
                    }
                    totalNumberOfReplaces += 1;
                    if total != i32::MIN && totalNumberOfReplaces >= total {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        if totalNumberOfReplaces > 0 {
            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).REPLACE.clone())) {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("Total Number Of Replaces: "), totalNumberOfReplaces), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.ALL.clone());
            }
        }
        return replace.clone();
    }
    

    pub fn line(&mut self, mut replace: String) -> String {
        let mut newStringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut totalNumberOfReplaces: i32 = 0;
        let mut key: String = String::new();
        let mut value: String = String::new();
        let mut keyArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(self.j2seUtil).getHashMapAsArray(self.hashMap.clone());
        let mut size: i32 = (keyArray.len() as i32).clone();
        for index2 in (0..size) {
            key = (keyArray[(index2) as usize]).clone() /* as String */;
            value = (jcall!(self.hashMap).get(key.clone())).clone() /* as String */;
            let mut index: i32 = 0;
            while index < (replace.chars().count() as i32) {
                jcall!(newStringBuffer).delete(0, jcall!(newStringBuffer).length());
                let mut begin: i32 = jcall!(replace).indexOf(key.clone(), index.clone());
                if begin != -1 {
                    println!("{}", self.FOUND_KEY.clone());
                    let mut end: i32 = jcall!(replace).indexOf('\n', begin + (key.chars().count() as i32)) + 1;
                    if end >= 0 {
                        jcall!(newStringBuffer).append(replace[(0) as usize..(begin) as usize].to_string());
                        index = jcall!(newStringBuffer).length();
                        jcall!(newStringBuffer).append(replace[(end) as usize..((replace.chars().count() as i32)) as usize].to_string());
                        replace = jcall!(newStringBuffer).toString();
                        totalNumberOfReplaces += 1;
                    }
                } else {
                    break;
                }
            }
        }
        if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).REPLACE.clone())) {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("Total Number Of Replaces: "), totalNumberOfReplaces), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.ALL.clone());
        }
        return replace.clone();
    }
    

    pub fn main(mut args: Vec<String>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
            let mut appUrlGlobals: Option<Rc<RefCell<AppUrlGlobals>>> = AppUrlGlobals::new();
            jcall!(appUrlGlobals).setWebappPath(String::from("G:/mnt/bc/mydev/work/allbinary_src/ToolsJavaLibrary/tools"));
            URLGLOBALS::init(appUrlGlobals.clone());
            let mut testString: String = String::from("\"super gun\" big tank \"goodgunriflegun");
            let mut replace: Option<Rc<RefCell<Replace>>> = Replace::new2(String::from("gun"), String::from("abcdefghijklmnopqrstuxwxyzabcdefghijklmnopqrstuxwxyz"));
            println!("{}", format!("{}{}", String::from("Old String: "), testString));
            testString = jcall!(replace).all(testString.clone());
            println!("{}", format!("{}{}", String::from("New String: "), testString));
            let mut replaceCat: Option<Rc<RefCell<Replace>>> = Replace::new2(String::from("abcdefghijklmnopqrstuxwxyzabcdefghijklmnopqrstuxwxyz"), jcall!(stringUtil).EMPTY_STRING.clone());
            testString = jcall!(replaceCat).all(testString.clone());
            println!("{}", format!("{}{}", String::from("New String: "), testString));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            }
        }
    }
}

