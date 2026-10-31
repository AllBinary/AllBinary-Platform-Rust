// Generated Code Do Not Modify
// package org.allbinary.time
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::util::visitor::Visitor::Visitor;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::thread::ThreadObjectUtil::ThreadObjectUtil;
use super::TimeDelayHelper::TimeDelayHelper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StupidTimer {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub nullUtil: Option<Rc<RefCell<NullUtil>>>,
    pub threadObjectUtil: Option<Rc<RefCell<ThreadObjectUtil>>>,
}


impl StupidTimer {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            nullUtil: None,
            threadObjectUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.nullUtil = NullUtil::getInstance();
        this.threadObjectUtil = ThreadObjectUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn visitBool(&mut self, mut visitorInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn>>>) -> bool {
        let mut result: Option<bool> = { let java_object = jcall!(visitorInterface).visit(crate::java_into_object(jcall!(self.nullUtil).NULL_OBJECT.clone())); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<bool>().copied().or_else(|| object.downcast_ref::<Option<bool>>().copied().flatten()).expect("ClassCastException") }) };
        let mut result2: bool = result.unwrap();
        return result2.clone();
    }
    

    // throws Exception
    pub fn visit(&mut self, mut visitorInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn>>>, mut timeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>) {
        let mut tookTooLong: bool = false;
        let mut WAITING_FOR: String = String::from("Waiting for: ");
        PreLogUtil::put(jcall!(StringMaker::new()).append(WAITING_FOR.clone()).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(visitorInterface.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).VISIT.clone());
        let mut index: i32 = 0;
        while self.visitBool(visitorInterface.clone()) {
            if index % 10 == 0 {
                PreLogUtil::put(jcall!(StringMaker::new()).append(WAITING_FOR.clone()).appendint(index.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).VISIT.clone());
            }
            index += 1;
            { // synchronized (self)
                jcall!(self.threadObjectUtil).waitObjectFor(None /* Java this: a &mut self method has no Rc of itself */, 1800);
            }
            if jcall!(timeDelayHelper).isTimeTNT() {
                tookTooLong = true;
                break;
            }
        }
        if tookTooLong {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).VISIT.clone(), crate::java_into_object(format!("{}: {:?}", "Exception", (jcall!(StringMaker::new()).append(String::from("Took Too Long: ")).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(visitorInterface.clone()))).toString()))),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
        } else {
            PreLogUtil::put(jcall!(timeDelayHelper).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).VISIT.clone());
        }
    }
    

    // throws Exception
    pub fn stopWaiting(&mut self) {
        { // synchronized (self)
            jcall!(self.threadObjectUtil).notifyObject(None /* Java this: a &mut self method has no Rc of itself */);
        }
    }
}

