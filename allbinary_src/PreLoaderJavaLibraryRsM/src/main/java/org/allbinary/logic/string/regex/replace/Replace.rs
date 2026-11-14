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
use super::*;
use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::J2SEUtil::J2SEUtil;
use crate::org::allbinary::globals::AppUrlGlobals::AppUrlGlobals;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes;
use crate::java::lang::Exception::*;

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
    pub hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>,
}


impl Replace {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

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
        this.hashMap = { let java_value = (jcall!(StdUtil::getInstance()).createHashMap()).clone(); java_value };
        jcall!(this.hashMap).put(crate::java_into_object(vec![key.clone()]), crate::java_into_object(vec![value.clone()]));
        this
    }

    pub fn new2(mut key: String, mut value: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(key, value))))
    }
    

    pub fn new_struct2_StringArray_StringArray(mut keys: Vec<String>, mut values: Vec<String>) -> Self {
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
        this.hashMap = { let java_value = (jcall!(StdUtil::getInstance()).createHashMap()).clone(); java_value };
        jcall!(this.hashMap).put(crate::java_into_object(keys.clone()), crate::java_into_object(values.clone()));
        this
    }

    pub fn new2_StringArray_StringArray(mut keys: Vec<String>, mut values: Vec<String>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2_StringArray_StringArray(keys, values))))
    }
    

    pub fn new_struct1(mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Self {
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

    pub fn new1(mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(hashMap))))
    }
    

    pub fn all(&mut self, mut replace: String) -> String {
        return { let java_arguments = (replace.clone(), i32::MIN.clone(),); self.upTo(java_arguments.0, java_arguments.1) };
    }
    

    pub fn upTo(&mut self, mut replace: String, mut total: i32) -> String {
        let mut newStringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut totalNumberOfReplaces: i32 = 0;
        let mut keys: Vec<String> = Vec::new();
        let mut values: Vec<String> = Vec::new();
        let mut keyArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(self.j2seUtil).getHashMapAsArray(self.hashMap.clone());
        let mut size: i32 = (keyArray.len() as i32).clone();
        for index2 in (0..size) {
            keys = { let java_object = (keyArray[(index2) as usize]).clone(); java_object.map(|object| object.borrow().downcast_ref::<Vec<String>>().cloned().expect("ClassCastException")).unwrap_or_default() };
            values = { let java_value = ({ let java_object = (jcall!(self.hashMap).get(crate::java_into_object(keys.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<Vec<String>>().cloned().expect("ClassCastException")).unwrap_or_default() }).clone(); java_value };
            let mut foundTotal: i64 = 0_i64;
            let mut index: i32 = 0;
            let mut nextKey: String = String::new();
            let mut end: i32 = 0;
            let mut nextSecondKey: String = String::new();
            let mut begin2: i32 = 0;
            while index < (replace.chars().count() as i32) {
                let mut begin: i32 = { let java_from = (index).max(0) as usize; replace.get(java_from..).and_then(|java_rest| java_rest.find(keys[(0) as usize])).map(|index| (index + java_from) as i32).unwrap_or(-1) };
                if begin != -1 {
                    foundTotal += 1;
                    if foundTotal % 100_i64 == 0_i64 {
                        println!("{}", format!("{}{}", self.FOUND_KEY, foundTotal));
                    }
                    nextKey = { let java_value = (keys[(0) as usize]).clone(); java_value }.clone();
                    end = { let java_value = (begin + (nextKey.chars().count() as i32)).clone(); java_value };
                    jcall!(newStringBuffer).delete(0, jcall!(newStringBuffer).length());
                    jcall!(newStringBuffer).append(replace[(0) as usize..(begin) as usize].to_string());
                    jcall!(newStringBuffer).append(values[(0) as usize].clone());
                    index = { let java_value = (jcall!(newStringBuffer).length()).clone(); java_value };
                    jcall!(newStringBuffer).append(replace[(end) as usize..((replace.chars().count() as i32)) as usize].to_string());
                    replace = { let java_value = (jcall!(newStringBuffer).toString()).clone(); java_value };
                    if (keys.len() as i32) > 1 {
                        nextSecondKey = { let java_value = (keys[(1) as usize]).clone(); java_value }.clone();
                        begin2 = { let java_value = ({ let java_from = (index).max(0) as usize; replace.get(java_from..).and_then(|java_rest| java_rest.find(&nextSecondKey)).map(|index| (index + java_from) as i32).unwrap_or(-1) }).clone(); java_value };
                        if begin2 != -1 {
                            let mut end2: i32 = begin2 + (nextSecondKey.chars().count() as i32);
                            jcall!(newStringBuffer).delete(0, jcall!(newStringBuffer).length());
                            jcall!(newStringBuffer).append(replace[(0) as usize..(begin2) as usize].to_string());
                            jcall!(newStringBuffer).append(values[(1) as usize].clone());
                            index = { let java_value = (jcall!(newStringBuffer).length()).clone(); java_value };
                            jcall!(newStringBuffer).append(replace[(end2) as usize..((replace.chars().count() as i32)) as usize].to_string());
                            replace = { let java_value = (jcall!(newStringBuffer).toString()).clone(); java_value };
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
            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).REPLACE.clone(); java_field }.clone())) {
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
            key = { let java_object = (keyArray[(index2) as usize]).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
            value = { let java_value = ({ let java_object = (jcall!(self.hashMap).get(crate::java_into_object(key.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
            let mut index: i32 = 0;
            while index < (replace.chars().count() as i32) {
                jcall!(newStringBuffer).delete(0, jcall!(newStringBuffer).length());
                let mut begin: i32 = { let java_from = (index).max(0) as usize; replace.get(java_from..).and_then(|java_rest| java_rest.find(&key)).map(|index| (index + java_from) as i32).unwrap_or(-1) };
                if begin != -1 {
                    println!("{}", self.FOUND_KEY.clone());
                    let mut end: i32 = { let java_from = (begin + (key.chars().count() as i32)).max(0) as usize; replace.get(java_from..).and_then(|java_rest| java_rest.find('\n')).map(|index| (index + java_from) as i32).unwrap_or(-1) } + 1;
                    if end >= 0 {
                        jcall!(newStringBuffer).append(replace[(0) as usize..(begin) as usize].to_string());
                        index = { let java_value = (jcall!(newStringBuffer).length()).clone(); java_value };
                        jcall!(newStringBuffer).append(replace[(end) as usize..((replace.chars().count() as i32)) as usize].to_string());
                        replace = { let java_value = (jcall!(newStringBuffer).toString()).clone(); java_value };
                        totalNumberOfReplaces += 1;
                    }
                } else {
                    break;
                }
            }
        }
        if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).REPLACE.clone(); java_field }.clone())) {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("Total Number Of Replaces: "), totalNumberOfReplaces), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.ALL.clone());
        }
        return replace.clone();
    }
    

    pub fn main(mut args: Vec<String>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
            let mut appUrlGlobals: Option<Rc<RefCell<AppUrlGlobals>>> = AppUrlGlobals::new();
            jcall!(appUrlGlobals).setWebappPath(String::from("G:/mnt/bc/mydev/work/allbinary_src/ToolsJavaLibrary/tools"));
            URLGLOBALS::init(appUrlGlobals.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::globals::UrlGlobalsInterface::UrlGlobalsInterfaceDyn>>));
            let mut testString: String = String::from("\"super gun\" big tank \"goodgunriflegun");
            let mut replace: Option<Rc<RefCell<Replace>>> = Replace::new2(String::from("gun"), String::from("abcdefghijklmnopqrstuxwxyzabcdefghijklmnopqrstuxwxyz"));
            println!("{}", format!("{}{}", String::from("Old String: "), testString));
            testString = { let java_value = (jcall!(replace).all(testString.clone())).clone(); java_value };
            println!("{}", format!("{}{}", String::from("New String: "), testString));
            let mut replaceCat: Option<Rc<RefCell<Replace>>> = Replace::new2(String::from("abcdefghijklmnopqrstuxwxyzabcdefghijklmnopqrstuxwxyz"), { let java_field = jcall!(stringUtil).EMPTY_STRING.clone(); java_field }.clone());
            testString = { let java_value = (jcall!(replaceCat).all(testString.clone())).clone(); java_value };
            println!("{}", format!("{}{}", String::from("New String: "), testString));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            }
        }
    }
}

