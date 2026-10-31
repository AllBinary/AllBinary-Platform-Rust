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
// package org.allbinary.logic.system.hardware.android
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::Closeable::Closeable;
// use crate::java::io::FileReader::FileReader;
// use crate::java::io::LineNumberReader::LineNumberReader;
use crate::org::allbinary::logic::io::NullCloseable::NullCloseable;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::hardware::HardwareInterface::HardwareInterface;
use crate::org::allbinary::logic::system::hardware::components::android::UnknownHardware::UnknownHardware;
use crate::org::allbinary::logic::system::hardware::components::interfaces::HardwareComponentInterface::HardwareComponentInterface;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AndroidHardware {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub componentInterfaceVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub PROC_BUS_INPUT_DIRECTORY: String,
    pub DEVICES: String,
    pub MINHARDWARE: i32,
}

impl HardwareInterface for AndroidHardware {
    fn getComponent(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn HardwareComponentInterface>>> { Self::getComponent(self, index) }
    fn toString(&mut self) -> String { Self::toString(self) }
    fn compareTo(&mut self, mut hardwareInterface: Option<Rc<RefCell<dyn HardwareInterface>>>) -> bool { Self::compareTo(self, hardwareInterface) }
    fn difference(&mut self, mut hardwareInterface: Option<Rc<RefCell<dyn HardwareInterface>>>) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> { Self::difference(self, hardwareInterface) }
}

impl AndroidHardware {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            componentInterfaceVector: None,
            PROC_BUS_INPUT_DIRECTORY: String::new(),
            DEVICES: String::new(),
            MINHARDWARE: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.componentInterfaceVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.PROC_BUS_INPUT_DIRECTORY = String::from("/proc/bus/input/");
        this.DEVICES = format!("{}{}", this.PROC_BUS_INPUT_DIRECTORY, String::from("devices/"));
        this.MINHARDWARE = 3;
        this.init(this.DEVICES.clone());
        let mut size: i32 = jcall!(this.componentInterfaceVector).size();
        if size < this.MINHARDWARE {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Not Enough Data For A Valid License On Linux"))));
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn init(&mut self, mut filePath: String) {
        let mut lineNumberReader: Option<Rc<RefCell<Closeable>>> = crate::org::allbinary::logic::io::NullCloseable::NullCloseable_NULL_CLOSEABLE.with(Clone::clone).clone();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            lineNumberReader = self.get(filePath.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Hardware Data: "), self.toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone(), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            panic!("{:?}", e);
            }
        }
        { // finally
            if lineNumberReader.is_some() {
                jcall!(lineNumberReader).close();
            }
        }
    }
    

    // throws Exception
    pub fn get(&mut self, mut filePath: String) -> Option<Rc<RefCell<LineNumberReader>>> {
        self.componentInterfaceVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut pciFile: Option<Rc<RefCell<FileReader>>> = FileReader::new(filePath.clone());
        let mut lineNumberReader: Option<Rc<RefCell<LineNumberReader>>> = LineNumberReader::new(pciFile.clone());
        jcall!(self.logUtil).putF(String::from("File Found"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).CONSTRUCTOR.clone());
        let mut nextLine: String = jcall!(lineNumberReader).readLine();
        while true {
            nextLine = jcall!(lineNumberReader).readLine();
            jcall!(self.componentInterfaceVector).add(crate::java_into_object(UnknownHardware::new(nextLine.clone())));
        }
        return lineNumberReader.clone();
    }
    

    pub fn getComponent(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn HardwareComponentInterface>>> {
        return { let java_object = jcall!(self.componentInterfaceVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn HardwareComponentInterface>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuilder: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut size: i32 = jcall!(self.componentInterfaceVector).size();
        for index in (0..size) {
            let mut componentInterface: Option<Rc<RefCell<dyn HardwareComponentInterface>>> = { let java_object = jcall!(self.componentInterfaceVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn HardwareComponentInterface>>>>().expect("ClassCastException").clone()) };
            jcall!(stringBuilder).append(jcall!(componentInterface).toString());
            jcall!(stringBuilder).append(jcall!(CommonSeps::getInstance()).NEW_LINE.clone());
        }
        return jcall!(stringBuilder).toString();
    }
    

    pub fn compareTo(&mut self, mut hardwareInterface: Option<Rc<RefCell<dyn HardwareInterface>>>) -> bool {
        return true;
    }
    

    pub fn difference(&mut self, mut hardwareInterface: Option<Rc<RefCell<dyn HardwareInterface>>>) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        return jcall!(StdUtil::getInstance()).NULL_TABLE.clone();
    }
}

