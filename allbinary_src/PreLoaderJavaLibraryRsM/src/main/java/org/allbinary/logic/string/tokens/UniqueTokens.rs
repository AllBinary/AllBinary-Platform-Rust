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
// use crate::java::util::HashSet::HashSet;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::string::regex::replace::Replace::Replace;
use crate::org::allbinary::string::CommonPhoneStrings::CommonPhoneStrings;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
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
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            specialCharacters: None,
        };
        this.specialCharacters = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from("!")));
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from("@")));
        { let java_receiver = this.specialCharacters.clone(); let java_arguments = (crate::java_into_object(jcall!(CommonPhoneStrings::getInstance()).POUND.clone()),); jcall!(java_receiver).add(java_arguments.0); };
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from("$")));
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from("%")));
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from("^")));
        { let java_receiver = this.specialCharacters.clone(); let java_arguments = (crate::java_into_object(jcall!(CommonSeps::getInstance()).AMPERSAND.clone()),); jcall!(java_receiver).add(java_arguments.0); };
        { let java_receiver = this.specialCharacters.clone(); let java_arguments = (crate::java_into_object(jcall!(CommonPhoneStrings::getInstance()).STAR.clone()),); jcall!(java_receiver).add(java_arguments.0); };
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from("(")));
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from(")")));
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from("-")));
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from("_")));
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from("+")));
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from("=")));
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from("\\")));
        jcall!(this.specialCharacters).add(crate::java_into_object(String::from("|")));
        { let java_receiver = this.specialCharacters.clone(); let java_arguments = (crate::java_into_object(jcall!(AbPathData::getInstance()).EXTENSION_SEP.clone()),); jcall!(java_receiver).add(java_arguments.0); };
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
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut hashSet: Option<Rc<RefCell<HashSet>>> = HashSet::new();
            let mut index: i32 = 0;
            let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
            while index < jcall!(stringVector).size() {
                let mut keywords: String = (jcall!(stringVector).get(index.clone())).clone() /* as String */;
                let mut tokenizer: Option<Rc<RefCell<Tokenizer>>> = Tokenizer::new(jcall!(commonSeps).COMMA.clone());
                let mut keywordVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(tokenizer).getTokensFromString(keywords.clone(), BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
                for forIndex in (0..jcall!(keywordVector).size()) {
                    let mut cleanString: String = (jcall!(keywordVector).get(forIndex.clone())).clone() /* as String */;
                    cleanString = cleanString.trim().to_string();
                    if cleanString.find('-').map(|index| index as i32).unwrap_or(-1) != -1 {
                        cleanString = jcall!(Replace::new2(String::from("-"), jcall!(commonSeps).SPACE.clone())).all(cleanString.clone());
                    }
                    jcall!(hashSet).add(cleanString.clone());
                    if cleanString.find(' ').map(|index| index as i32).unwrap_or(-1) != -1 {
                        let mut spaceTokenizer: Option<Rc<RefCell<Tokenizer>>> = Tokenizer::new(jcall!(commonSeps).SPACE.clone());
                        let mut subKeywordVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(spaceTokenizer).getTokensFromString(cleanString.clone(), BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
                        for spaceIndex in (0..jcall!(subKeywordVector).size()) {
                            let mut subCleanString: String = (jcall!(subKeywordVector).get(spaceIndex.clone())).clone() /* as String */;
                            if !self.isSpecialCharacter(subCleanString.clone()) && !self.numberOnly(subCleanString.clone()) {
                                jcall!(hashSet).add(subCleanString.clone());
                            }
                        }
                    }
                }
                index += 1;
            }
            return hashSet.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    pub fn numberOnly(&mut self, mut subCleaningString: String) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            subCleaningString.clone().parse::<i32>().unwrap();
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            return false;
            }
        }
    }
    

    pub fn isSpecialCharacter(&mut self, mut subCleaningString: String) -> bool {
        for index in (0..jcall!(self.specialCharacters).size()) {
            if (subCleaningString.cmp(&(jcall!(self.specialCharacters).get(index.clone())).clone() /* as String */) as i32) == 0 {
                return true;
            }
        }
        return false;
    }
}

