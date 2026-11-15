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
// package org.allbinary.logic.system.security.licensing
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::io::InputStream::InputStream;
use crate::java::io::InputStreamReader::InputStreamReader;
use crate::org::allbinary::data::resource::ResourceUtil::ResourceUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PartnerIdentifierFileUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub FILE_NAME: String,
}

pub static PartnerIdentifierFileUtil_instance: crate::JavaStatic<Option<Rc<RefCell<PartnerIdentifierFileUtil>>>> = crate::JavaStatic::new(|| PartnerIdentifierFileUtil::new());

impl PartnerIdentifierFileUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            FILE_NAME: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.FILE_NAME = String::from("partner.txt");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PartnerIdentifierFileUtil>>> {
        return PartnerIdentifierFileUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn get(&mut self) -> String {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut resourceUtil: Option<Rc<RefCell<ResourceUtil>>> = ResourceUtil::getInstance();
            let mut inputStream: Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>> = jcall!(resourceUtil).getResourceAsStream(self.FILE_NAME.clone());
            let mut inputStreamReader: Option<Rc<RefCell<InputStreamReader>>> = InputStreamReader::new(inputStream.clone());
            let mut chars: Vec<char> = vec!['\0'; (30) as usize];
            let mut size: i32 = jcall!(inputStreamReader).read1(chars.clone());
            let mut partnerString: String = { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone();
            if size > 0 {
                partnerString = chars[(0) as usize..(0 + size) as usize].iter().collect::<String>();
            }
            jcall!(inputStreamReader).close();
            return Some(partnerString.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).GET.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            return { let java_field = jcall!(StringUtil::getInstance()).NULL_STRING.clone(); java_field }.clone();
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
}

