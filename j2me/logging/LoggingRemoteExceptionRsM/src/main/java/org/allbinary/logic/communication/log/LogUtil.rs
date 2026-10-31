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
// package org.allbinary.logic.communication.log
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::IOException::IOException;
use crate::org::apache::xmlrpc::XmlRpcException::XmlRpcException;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::canvas::SpecialMessageUtil::SpecialMessageUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::Log::Log;
use super::XmlRpcRemoteLogClient::XmlRpcRemoteLogClient;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//ActualPlatform
pub struct LogUtil {
    pub MESSAGE: String,
    pub isFirstException: bool,
    pub timeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>,
    pub stringBuffer: Option<Rc<RefCell<StringMaker>>>,
}

thread_local! { pub static LogUtil_instance: Option<Rc<RefCell<LogUtil>>> = LogUtil::new(); }

impl LogUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<LogUtil>>> {
        return LogUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            MESSAGE: String::new(),
            isFirstException: false,
            timeDelayHelper: None,
            abeClientInformation: None,
            stringBuffer: None,
        };
        this.MESSAGE = String::from("message");
        this.isFirstException = true;
        this.timeDelayHelper = TimeDelayHelper::new(200000).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.stringBuffer = StringMaker::new();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn putL(&mut self, mut log: Option<Rc<RefCell<Log>>>) {
        if log.is_none() {
            return;
        }
        let mut specialMessage: String = jcall!(log).getSpecialMessage();
        let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(log).getObject();
        let mut functionName: String = jcall!(log).getFunctionName();
        let mut exception: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(log).getThrowable();
        self.put(specialMessage.clone(), anyType.clone(), functionName.clone(), exception.clone());
    }
    

    pub fn putF<T1, T2, T3>(&mut self, specialMessage: T1, anyType: T2, functionName: T3) {
    }
    
    

    pub fn put(&mut self, mut specialMessage: String, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut functionName: String, mut exception: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
        if exception.is_some() {
            if jcall!(std::any::type_name_of_val(&exception).to_string()).compareTo(jcall!(std::any::type_name::<XmlRpcException>()).getName()) == 0 {
                return;
            }
            if jcall!(std::any::type_name_of_val(&exception).to_string()).compareTo(jcall!(std::any::type_name::<IOException>()).getName()) == 0 {
                return;
            }
            if self.isFirstException || jcall!(self.timeDelayHelper).isTimeTNT() {
                let mut className: String = jcall!(CommonStrings::getInstance()).EMPTY.clone();
                self.isFirstException = false;
                if anyType.is_some() && true {
                    className = std::any::type_name_of_val(&anyType).to_string().clone();
                }
                let mut message: String = jcall!(LogFormatUtil::LogFormatUtil::getInstance()).get(className.clone(), functionName.clone(), specialMessage.clone(), crate::java_into_object(exception.clone()));
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    println!("{}", String::from("Eeeek"));
                    if self.abeClientInformation.is_none() {
                        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
                    }
                    let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = jcall!(self.abeClientInformation).toHashtable();
                    { let java_receiver = self.stringBuffer.clone(); let java_arguments = (0, jcall!(self.stringBuffer).length(),); jcall!(java_receiver).delete(java_arguments.0, java_arguments.1); };
                    jcall!(self.stringBuffer).append(message.clone());
                    { let java_receiver = self.stringBuffer.clone(); let java_arguments = (jcall!(CommonSeps::getInstance()).SPACE.clone(),); jcall!(java_receiver).append(java_arguments.0); };
                    { let java_receiver = self.stringBuffer.clone(); let java_arguments = (jcall!(SpecialMessageUtil::getInstance()).get(),); jcall!(java_receiver).append(java_arguments.0); };
                    jcall!(hashtable).put(crate::java_into_object(self.MESSAGE.clone()), crate::java_into_object(jcall!(self.stringBuffer).toString()));
                    jcall!(XmlRpcRemoteLogClient::new(self.abeClientInformation.clone())).get(hashtable.clone());
                })) {
                    Ok(_) => {}
                    Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                    println!("{}", String::from("Exception"));
                    eprintln!("{:?}", e);
                    }
                }
            }
        }
    }
}

