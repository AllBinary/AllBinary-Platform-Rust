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
// package org.allbinary.midlet
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Command::Command;
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;
// use crate::javax::microedition::lcdui::Display::Display;
// use crate::javax::microedition::lcdui::Displayable::Displayable;
// use crate::javax::microedition::lcdui::NullCanvas::NullCanvas;
// use crate::javax::microedition::midlet::MIDlet::MIDlet;
// use crate::javax::microedition::midlet::MIDletStateChangeException::MIDletStateChangeException;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::system::Memory::Memory;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::util::ABHashtable::ABHashtable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryMidlet {
    pub base: MIDlet,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub SET_DISPLAY: String,
    pub SETTING_: String,
    pub _DISPLAY_: String,
    pub SETTING_NO_TITLE: String,
    pub hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>,
    pub midletDestroyed: bool,
}

pub static mut AllBinaryMidlet_NULL_ALLBINARY_MIDLET: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());
// impl CommandListener for AllBinaryMidlet with the methods of the trait

impl AllBinaryMidlet {
    

    pub fn getNullInstance() -> Option<Rc<RefCell<dyn AllBinaryMidletDyn>>> {
        if match (&unsafe { AllBinaryMidlet_NULL_ALLBINARY_MIDLET.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *AllBinaryMidlet_NULL_ALLBINARY_MIDLET = AllBinaryMidlet::new().map(|reference| reference as Rc<RefCell<dyn AllBinaryMidletDyn>>) };
        }
        return (unsafe { AllBinaryMidlet_NULL_ALLBINARY_MIDLET.clone() }).clone() /* as AllBinaryMidlet */;
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: MIDlet::new_struct(),
            logUtil: None,
            commonStrings: None,
            SET_DISPLAY: String::new(),
            SETTING_: String::new(),
            _DISPLAY_: String::new(),
            SETTING_NO_TITLE: String::new(),
            hashtable: None,
            midletDestroyed: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.SET_DISPLAY = String::from("setDisplay");
        this.SETTING_ = String::from("Setting: ");
        this._DISPLAY_ = String::from(" Display: ");
        this.SETTING_NO_TITLE = String::from("Setting: No Title, Display: ");
        this.hashtable = jcall!(StdUtil::getInstance()).createHashtable();
        jcall!(this.logUtil).putF(jcall!(this.commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("AllBinaryMidlet::AllBinaryMidlet"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setDisplay(&mut self, mut newDisplay: Option<Rc<RefCell<Displayable>>>) {
        let mut title: String = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        if !match (&newDisplay, &crate::javax::microedition::lcdui::NullCanvas::NullCanvas_NULL_CANVAS.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            title = jcall!(newDisplay).getTitle();
            if true {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(self.SETTING_.clone()).append(title.clone()).append(self._DISPLAY_.clone()).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(newDisplay.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.SET_DISPLAY.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            } else {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(self.SETTING_NO_TITLE.clone()).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(newDisplay.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.SET_DISPLAY.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
        }
        let mut display: Option<Rc<RefCell<Display>>> = self.getDisplay();
        jcall!(display).setCurrent(newDisplay.clone());
    }
    

    pub fn getDisplay(&mut self) -> Option<Rc<RefCell<Display>>> {
        return Display::getDisplay(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    pub fn getCurrentDisplayable(&mut self) -> Option<Rc<RefCell<Displayable>>> {
        return jcall!(Display::getDisplay(None /* Java this: a &mut self method has no Rc of itself */)).getCurrent();
    }
    

    pub fn setDestroyed(&mut self, mut destroyed: bool) {
        self.midletDestroyed = destroyed.clone();
    }
    

    pub fn isDestroyed(&mut self) -> bool {
        return self.midletDestroyed.clone();
    }
    

    // throws MIDletStateChangeException
    pub fn startApp(&mut self) {
        ForcedLogUtil::log(jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    pub fn pauseApp(&mut self) {
        ForcedLogUtil::log(jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    pub fn destroyAppInRunnable(&mut self, mut unconditional: bool, mut isProgress: bool) {
    }
    

    pub fn destroyApp(&mut self, mut unconditional: bool) {
        let mut METHOD_NAME: String = String::from("AllBinaryMidlet::destroyApp");
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone());
            PreLogUtil::put(Memory::getInfo(), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone());
            self.setDestroyed(true);
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn setStartStateHashtable(&mut self, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(CommonLabels::getInstance()).START_LABEL.clone()).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(hashtable.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setStartStateHashtable"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        self.hashtable = hashtable.clone();
    }
    

    // throws Exception
    pub fn getStartStateHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(CommonLabels::getInstance()).START_LABEL.clone()).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(self.hashtable.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getStartStateHashtable"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        return self.hashtable.clone();
    }
    

    // throws Exception
    pub fn getCurrentStateHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getStateHashtable"));
        return jcall!(StdUtil::getInstance()).NULL_TABLE.clone();
    }
    

    pub fn commandAction(&mut self, mut command: Option<Rc<RefCell<Command>>>, mut displayable: Option<Rc<RefCell<Displayable>>>) {
    }
}
impl std::ops::Deref for AllBinaryMidlet {
    type Target = MIDlet;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllBinaryMidlet {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AllBinaryMidletDyn {
    fn as_AllBinaryMidlet(&self) -> &AllBinaryMidlet;
    fn as_AllBinaryMidlet_mut(&mut self) -> &mut AllBinaryMidlet;
    fn setDisplay(&mut self, mut newDisplay: Option<Rc<RefCell<Displayable>>>) { AllBinaryMidlet::setDisplay(self.as_AllBinaryMidlet_mut(), newDisplay) }
    fn getDisplay(&mut self) -> Option<Rc<RefCell<Display>>> { AllBinaryMidlet::getDisplay(self.as_AllBinaryMidlet_mut()) }
    fn getCurrentDisplayable(&mut self) -> Option<Rc<RefCell<Displayable>>> { AllBinaryMidlet::getCurrentDisplayable(self.as_AllBinaryMidlet_mut()) }
    fn setDestroyed(&mut self, mut destroyed: bool) { AllBinaryMidlet::setDestroyed(self.as_AllBinaryMidlet_mut(), destroyed) }
    fn isDestroyed(&mut self) -> bool { AllBinaryMidlet::isDestroyed(self.as_AllBinaryMidlet_mut()) }
    fn startApp(&mut self) { AllBinaryMidlet::startApp(self.as_AllBinaryMidlet_mut()) }
    fn pauseApp(&mut self) { AllBinaryMidlet::pauseApp(self.as_AllBinaryMidlet_mut()) }
    fn destroyAppInRunnable(&mut self, mut unconditional: bool, mut isProgress: bool) { AllBinaryMidlet::destroyAppInRunnable(self.as_AllBinaryMidlet_mut(), unconditional, isProgress) }
    fn destroyApp(&mut self, mut unconditional: bool) { AllBinaryMidlet::destroyApp(self.as_AllBinaryMidlet_mut(), unconditional) }
    fn setStartStateHashtable(&mut self, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>) { AllBinaryMidlet::setStartStateHashtable(self.as_AllBinaryMidlet_mut(), hashtable) }
    fn getStartStateHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> { AllBinaryMidlet::getStartStateHashtable(self.as_AllBinaryMidlet_mut()) }
    fn getCurrentStateHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> { AllBinaryMidlet::getCurrentStateHashtable(self.as_AllBinaryMidlet_mut()) }
    fn commandAction(&mut self, mut command: Option<Rc<RefCell<Command>>>, mut displayable: Option<Rc<RefCell<Displayable>>>) { AllBinaryMidlet::commandAction(self.as_AllBinaryMidlet_mut(), command, displayable) }
}
impl AllBinaryMidletDyn for AllBinaryMidlet {
    fn as_AllBinaryMidlet(&self) -> &AllBinaryMidlet { self }
    fn as_AllBinaryMidlet_mut(&mut self) -> &mut AllBinaryMidlet { self }
}



