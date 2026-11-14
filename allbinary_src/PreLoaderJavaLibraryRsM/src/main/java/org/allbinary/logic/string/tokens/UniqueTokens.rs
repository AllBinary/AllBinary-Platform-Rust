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
// package org.allbinary.logic.string.tokens
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::util::HashSet::HashSet;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::string::regex::replace::Replace::Replace;
use crate::org::allbinary::string::CommonPhoneStrings::CommonPhoneStrings;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::java::lang::Exception::*;
use super::Tokenizer::Tokenizer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UniqueTokens {
    pub specialCharacters: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}


impl UniqueTokens {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            specialCharacters: None,
        };
        this.specialCharacters = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from("!")));
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from("@")));
        { let java_receiver = this.specialCharacters.clone(); let java_arguments = (crate::java_into_object({ let java_field = jcall!(CommonPhoneStrings::getInstance()).POUND.clone(); java_field }.clone()),); jcall!(java_receiver).add(java_arguments.0); };
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from("$")));
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from("%")));
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from("^")));
        { let java_receiver = this.specialCharacters.clone(); let java_arguments = (crate::java_into_object({ let java_field = jcall!(CommonSeps::getInstance()).AMPERSAND.clone(); java_field }.clone()),); jcall!(java_receiver).add(java_arguments.0); };
        { let java_receiver = this.specialCharacters.clone(); let java_arguments = (crate::java_into_object({ let java_field = jcall!(CommonPhoneStrings::getInstance()).STAR.clone(); java_field }.clone()),); jcall!(java_receiver).add(java_arguments.0); };
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from("(")));
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from(")")));
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from("-")));
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from("_")));
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from("+")));
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from("=")));
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from("\\")));
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from("|")));
        { let java_receiver = this.specialCharacters.clone(); let java_arguments = (crate::java_into_object({ let java_field = jcall!(AbPathData::getInstance()).EXTENSION_SEP.clone(); java_field }.clone()),); jcall!(java_receiver).add(java_arguments.0); };
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from(",")));
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from("<")));
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from(">")));
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from("?")));
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from("/")));
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from("~")));
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from("`")));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getWhithoutDashesAndSkipNumberOnlyTokens(&mut self, mut stringVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<HashSet>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut hashSet: Option<Rc<RefCell<HashSet>>> = HashSet::new();
            let mut index: i32 = 0;
            let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
            while index < jcall!(stringVector).size() {
                let mut keywords: String = { let java_object = (jcall!(stringVector).get(index.clone())).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
                let mut tokenizer: Option<Rc<RefCell<Tokenizer>>> = Tokenizer::new({ let java_field = jcall!(commonSeps).COMMA.clone(); java_field }.clone());
                let mut keywordVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(tokenizer).getTokensFromString(keywords.clone(), BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
                for forIndex in (0..{ let java_bound = jcall!(keywordVector).size(); java_bound }) {
                    let mut cleanString: String = { let java_object = (jcall!(keywordVector).get(forIndex.clone())).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
                    cleanString = { let java_value = (cleanString.trim().to_string()).clone(); java_value };
                    if cleanString.find('-').map(|index| index as i32).unwrap_or(-1) != -1 {
                        cleanString = { let java_value = (jcall!(Replace::new2(String::from("-"), { let java_field = jcall!(commonSeps).SPACE.clone(); java_field }.clone())).all(cleanString.clone())).clone(); java_value };
                    }
                    jcall!(hashSet).add(cleanString.clone());
                    if cleanString.find(' ').map(|index| index as i32).unwrap_or(-1) != -1 {
                        let mut spaceTokenizer: Option<Rc<RefCell<Tokenizer>>> = Tokenizer::new({ let java_field = jcall!(commonSeps).SPACE.clone(); java_field }.clone());
                        let mut subKeywordVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(spaceTokenizer).getTokensFromString(cleanString.clone(), BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
                        for spaceIndex in (0..{ let java_bound = jcall!(subKeywordVector).size(); java_bound }) {
                            let mut subCleanString: String = { let java_object = (jcall!(subKeywordVector).get(spaceIndex.clone())).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
                            if !self.isSpecialCharacter(subCleanString.clone()) && !self.numberOnly(subCleanString.clone()) {
                                jcall!(hashSet).add(subCleanString.clone());
                            }
                        }
                    }
                }
                index += 1;
            }
            return Some(hashSet.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            panic!("{:?}", e);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn numberOnly(&mut self, mut subCleaningString: String) -> bool {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            subCleaningString.clone().parse::<i32>().unwrap();
            return Some(true);
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            return false;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn isSpecialCharacter(&mut self, mut subCleaningString: String) -> bool {
        for index in (0..{ let java_bound = jcall!(self.specialCharacters).size(); java_bound }) {
            if (subCleaningString.cmp(&{ let java_object = (jcall!(self.specialCharacters).get(index.clone())).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }) as i32) == 0 {
                return true;
            }
        }
        return false;
    }
}

