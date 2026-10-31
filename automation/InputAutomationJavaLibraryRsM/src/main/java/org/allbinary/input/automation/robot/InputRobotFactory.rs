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
// package org.allbinary.input.automation.robot
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::GraphicsDevice::GraphicsDevice;
// use crate::java::awt::GraphicsEnvironment::GraphicsEnvironment;
// use crate::java::util::Collection::Collection;
// use crate::java::util::Iterator::Iterator;
// use crate::java::util::Set::Set;
// use crate::javax::help::HelpSet::HelpSet;
// use crate::javax::help::event::HelpSetEvent::HelpSetEvent;
// use crate::javax::help::event::HelpSetListener::HelpSetListener;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::java::help::JavaHelpSetNotifier::JavaHelpSetNotifier;
use crate::org::allbinary::logic::java::anyType::InterfaceUtil::InterfaceUtil;
use crate::org::allbinary::logic::system::loader::SecuredNativeLibraryInterface::SecuredNativeLibraryInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::InputRobotInterface::InputRobotInterface;
use super::InputRobot::InputRobot;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputRobotFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>,
    pub helpSetListenerInterface: Option<Rc<RefCell<HelpSetListener>>>,
}

thread_local! { pub static InputRobotFactory_inputRobotFactory: Option<Rc<RefCell<InputRobotFactory>>> = InputRobotFactory::new(); }

impl InputRobotFactory {
    

    // throws Exception
    pub fn getInstance() -> Option<Rc<RefCell<InputRobotFactory>>> {
        return InputRobotFactory_inputRobotFactory.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            hashtable: None,
            helpSetListenerInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.hashtable = jcall!(StdUtil::getInstance()).createHashtable();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut graphenv: Option<Rc<RefCell<GraphicsEnvironment>>> = GraphicsEnvironment::getLocalGraphicsEnvironment();
            let mut screens: Vec<Option<Rc<RefCell<GraphicsDevice>>>> = jcall!(graphenv).getScreenDevices();
            let mut inputRobotInterface: Option<Rc<RefCell<dyn InputRobotInterface>>> = None;
            for i in (0..(screens.len() as i32)) {
                inputRobotInterface = (InputRobot::new(screens[(i) as usize].clone())).clone() /* as InputRobotInterface */;
                { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Adding Robot: "), jcall!(inputRobotInterface).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getRobots"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                jcall!(this.get()).put(jcall!(inputRobotInterface).getName(), inputRobotInterface.clone());
            }
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Number Of Robots: "), jcall!(this.hashtable).size()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getRobots"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(this.logUtil).putF(jcall!(this.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
            }
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn addListener(&mut self, mut helpSetListenerInterface: Option<Rc<RefCell<HelpSetListener>>>) {
        self.helpSetListenerInterface = helpSetListenerInterface.clone();
    }
    

    // throws Exception
    pub fn add(&mut self, mut inputRobotInterface: Option<Rc<RefCell<dyn InputRobotInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Adding InputRobotInterface: "), jcall!(inputRobotInterface).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("add"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        jcall!(self.get()).put(jcall!(inputRobotInterface).getName(), inputRobotInterface.clone());
        let mut helpSet: Option<Rc<RefCell<HelpSet>>> = jcall!(inputRobotInterface).getHelpSet();
        if self.helpSetListenerInterface.is_some() {
            if helpSet.is_some() {
                if !JavaHelpSetNotifier::isNotified(helpSet.clone()) {
                    let mut helpSetEvent: Option<Rc<RefCell<HelpSetEvent>>> = HelpSetEvent::new(None /* Java this: a &mut self method has no Rc of itself */, helpSet.clone(), unsafe { crate::javax::help::event::HelpSetEvent::HelpSetEvent_HELPSET_ADDED.clone() }.clone());
                    jcall!(self.helpSetListenerInterface).helpSetAdded(helpSetEvent.clone());
                }
            } else {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Null HelpSet For: "), jcall!(inputRobotInterface).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("add"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
        } else {
            jcall!(self.logUtil).putF(String::from("No HelpSet Listener"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("add"));
        }
    }
    

    // throws Exception
    pub fn loadLibraries(&mut self) {
        let mut set: Option<Rc<RefCell<dyn Set>>> = jcall!(self.get()).keySet();
        jcall!(self.logUtil).putF(String::from("Loading Libraries"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("loadLibraries"));
        let mut nameArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(set).toArray();
        let mut size: i32 = (nameArray.len() as i32).clone();
        for index in (0..size) {
            let mut inputRobotInterface: Option<Rc<RefCell<dyn InputRobotInterface>>> = jcall!(InputRobotFactory::getInstance()).get1((nameArray[(index) as usize]).clone() /* as String */);
            InputRobotFactory::loadLibrary(inputRobotInterface.clone());
        }
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn loadLibraries1(mut collection: Option<Rc<RefCell<dyn Collection>>>) {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        jcall!(logUtil).putF(String::from("Loading Libraries"), crate::java_into_object(String::from("InputRobotFactory")), String::from("loadLibraries"));
        let mut iterator: Option<Rc<RefCell<dyn Iterator>>> = jcall!(collection).iterator();
        while jcall!(iterator).hasNext() {
            InputRobotFactory::loadLibrary((jcall!(iterator).next()).clone() /* as InputRobotInterface */);
        }
    }
    

    // throws Exception
    pub fn loadLibrary(mut inputRobotInterface: Option<Rc<RefCell<dyn InputRobotInterface>>>) {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        if InterfaceUtil::isImplemented(std::any::type_name::<SecuredNativeLibraryInterface>(), inputRobotInterface.clone()) {
            jcall!(logUtil).putF(format!("{}{}", String::from("Loading Library: "), jcall!(inputRobotInterface).getName()), crate::java_into_object(String::from("InputRobotFactory")), String::from("loadLibraries"));
            let mut securedNativeLibraryInterface: Option<Rc<RefCell<dyn SecuredNativeLibraryInterface>>> = panic!("ClassCastException: a Rust dyn InputRobotInterface cannot be cast to dyn SecuredNativeLibraryInterface");
            jcall!(securedNativeLibraryInterface).load();
        }
    }
    

    // throws Exception
    pub fn unloadLibraries(&mut self) {
        jcall!(self.logUtil).putF(String::from("Unloading Libraries"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("unloadLibraries"));
        let mut set: Option<Rc<RefCell<dyn Set>>> = jcall!(self.get()).keySet();
        let mut inputRobotInterface: Option<Rc<RefCell<dyn InputRobotInterface>>> = None;
        let mut inputRobotArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(set).toArray();
        let mut size: i32 = (inputRobotArray.len() as i32).clone();
        for index in (0..size) {
            inputRobotInterface = self.get1((inputRobotArray[(index) as usize]).clone() /* as String */);
            if InterfaceUtil::isImplemented(std::any::type_name::<SecuredNativeLibraryInterface>(), inputRobotInterface.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Unloading Library: "), jcall!(inputRobotInterface).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("unloadLibraries"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                let mut securedNativeLibraryInterface: Option<Rc<RefCell<dyn SecuredNativeLibraryInterface>>> = panic!("ClassCastException: a Rust dyn InputRobotInterface cannot be cast to dyn SecuredNativeLibraryInterface");
                jcall!(securedNativeLibraryInterface).unload();
            }
        }
    }
    

    // throws Exception
    pub fn get(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        return self.hashtable.clone();
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn get1(&mut self, mut name: String) -> Option<Rc<RefCell<dyn InputRobotInterface>>> {
        jcall!(self.logUtil).putF(format!("{}{}", String::from("Getting Robot: "), name), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getRobots"));
        return { let java_object = jcall!(self.hashtable).get(crate::java_into_object(name.clone())); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn InputRobotInterface>>>>().expect("ClassCastException").clone()) };
    }
}

