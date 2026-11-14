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
// package org.allbinary.logic.system.hardware.components.linux
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::io::FileReader::FileReader;
// use crate::java::io::LineNumberReader::LineNumberReader;
use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypeFactory::LogConfigTypeFactory;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::FilePathData::FilePathData;
use crate::org::allbinary::logic::io::file::directory::SubDirectory::SubDirectory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::hardware::components::interfaces::CpuInterface::CpuInterface;
use crate::org::allbinary::logic::system::hardware::components::interfaces::HardwareComponentInterface::HardwareComponentInterface;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Cpu {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub CPUFILE: String,
    pub cpuHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>,
}

// impl CpuInterface for Cpu with the methods of the trait
// impl HardwareComponentInterface for Cpu with the methods of the trait

impl Cpu {
    

    pub fn new_struct1(mut filePath: String) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            CPUFILE: String::new(),
            cpuHashMap: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.CPUFILE = String::from("/proc/cpuinfo");
        this.init(filePath.clone());
        this
    }

    pub fn new1(mut filePath: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(filePath))))
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            CPUFILE: String::new(),
            cpuHashMap: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.CPUFILE = String::from("/proc/cpuinfo");
        { let java_arguments = (this.CPUFILE.clone(),); this.init(java_arguments.0) };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn init(&mut self, mut filePath: String) {
        let mut lineNumberReader: Option<Rc<RefCell<LineNumberReader>>> = None;
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.init2(lineNumberReader.clone(), filePath.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object({ let java_field = jcall!(LogConfigTypeFactory::getInstance()).OS.clone(); java_field }.clone())) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = ({ let java_argument = jcall!(StringMaker::new()).append(String::from("Cpu Data: ")).append(self.toString()).toString(); java_argument }, crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).CONSTRUCTOR.clone(); java_field }.clone(), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn init2(&mut self, mut lineNumberReader: Option<Rc<RefCell<LineNumberReader>>>, mut filePath: String) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut fileReader: Option<Rc<RefCell<FileReader>>> = FileReader::new(filePath.clone());
            lineNumberReader = LineNumberReader::new(fileReader.clone());
            self.cpuHashMap = { let java_value = (jcall!(StdUtil::getInstance()).createHashMap()).clone(); java_value };
            if lineNumberReader.is_none() {
                let mut fileVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(SubDirectory::getInstance()).search2_String_AbFile(filePath.clone(), AbFile::createAbFile({ let java_field = jcall!(FilePathData::getInstance()).PATH_START.clone(); java_field }.clone()));
                if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object({ let java_field = jcall!(LogConfigTypeFactory::getInstance()).OS.clone(); java_field }.clone())) {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Cpu File BasicArrayList Size: ")).appendint(jcall!(fileVector).size()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).CONSTRUCTOR.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                if !jcall!(fileVector).isEmpty() {
                    let mut file: Option<Rc<RefCell<AbFile>>> = { let java_object = (jcall!(fileVector).get(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<AbFile>>>>().expect("ClassCastException").clone()) };
                    lineNumberReader = { let java_value = (LineNumberReader::new(FileReader::new(jcall!(file).getPath()))).clone(); java_value };
                }
            }
            if lineNumberReader.is_some() {
                let mut nextLine: String = jcall!(lineNumberReader).readLine();
                while lineNumberReader.is_some() && true {
                    let mut index: i32 = nextLine.find(':').map(|index| index as i32).unwrap_or(-1);
                    if index >= 0 {
                        let mut key: String = nextLine[(0) as usize..(index) as usize].to_string();
                        let mut valueIndex: i32 = index + 1;
                        let mut value: String = nextLine[(valueIndex) as usize..((nextLine.chars().count() as i32)) as usize].to_string();
                        if key.find(&jcall!(CpuInfo_PROCESSOR.with(Clone::clone)).toString()).map(|index| index as i32).unwrap_or(-1) >= 0 {
                            jcall!(self.cpuHashMap).put(crate::java_into_object(CpuInfo_PROCESSOR.with(Clone::clone).clone()), crate::java_into_object(value.clone()));
                        }
                        if key.find(&jcall!(CpuInfo_VENDORID.with(Clone::clone)).toString()).map(|index| index as i32).unwrap_or(-1) >= 0 {
                            jcall!(self.cpuHashMap).put(crate::java_into_object(CpuInfo_VENDORID.with(Clone::clone).clone()), crate::java_into_object(value.clone()));
                        }
                        if key.find(&jcall!(CpuInfo_CPUFAMILY.with(Clone::clone)).toString()).map(|index| index as i32).unwrap_or(-1) >= 0 {
                            jcall!(self.cpuHashMap).put(crate::java_into_object(CpuInfo_CPUFAMILY.with(Clone::clone).clone()), crate::java_into_object(value.clone()));
                        }
                        if key.find(&jcall!(CpuInfo_MODEL.with(Clone::clone)).toString()).map(|index| index as i32).unwrap_or(-1) >= 0 {
                            jcall!(self.cpuHashMap).put(crate::java_into_object(CpuInfo_MODEL.with(Clone::clone).clone()), crate::java_into_object(value.clone()));
                        }
                        if key.find(&jcall!(CpuInfo_MODELNAME.with(Clone::clone)).toString()).map(|index| index as i32).unwrap_or(-1) >= 0 {
                            jcall!(self.cpuHashMap).put(crate::java_into_object(CpuInfo_MODELNAME.with(Clone::clone).clone()), crate::java_into_object(value.clone()));
                        }
                        if key.find(&jcall!(CpuInfo_STEPPING.with(Clone::clone)).toString()).map(|index| index as i32).unwrap_or(-1) >= 0 {
                            jcall!(self.cpuHashMap).put(crate::java_into_object(CpuInfo_STEPPING.with(Clone::clone).clone()), crate::java_into_object(value.clone()));
                        }
                        if key.find(&jcall!(CpuInfo_CPUMHZ.with(Clone::clone)).toString()).map(|index| index as i32).unwrap_or(-1) >= 0 {
                            jcall!(self.cpuHashMap).put(crate::java_into_object(CpuInfo_CPUMHZ.with(Clone::clone).clone()), crate::java_into_object(value.clone()));
                        }
                        if key.find(&jcall!(CpuInfo_CACHESIZE.with(Clone::clone)).toString()).map(|index| index as i32).unwrap_or(-1) >= 0 {
                            jcall!(self.cpuHashMap).put(crate::java_into_object(CpuInfo_CACHESIZE.with(Clone::clone).clone()), crate::java_into_object(value.clone()));
                        }
                    }
                    nextLine = { let java_value = (jcall!(lineNumberReader).readLine()).clone(); java_value };
                }
            }
        })) {
            Ok(_) => {}
            Err(error) => std::panic::resume_unwind(error),
        }
        { // finally
            if lineNumberReader.is_some() {
                jcall!(lineNumberReader).close();
            }
        }
    }
    

    pub fn getProcessor(&mut self) -> String {
        return { let java_object = (jcall!(self.cpuHashMap).get(crate::java_into_object(CpuInfo_PROCESSOR.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
    

    pub fn getVendorId(&mut self) -> String {
        return { let java_object = (jcall!(self.cpuHashMap).get(crate::java_into_object(CpuInfo_VENDORID.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
    

    pub fn getCpuFamily(&mut self) -> String {
        return { let java_object = (jcall!(self.cpuHashMap).get(crate::java_into_object(CpuInfo_CPUFAMILY.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
    

    pub fn getModel(&mut self) -> String {
        return { let java_object = (jcall!(self.cpuHashMap).get(crate::java_into_object(CpuInfo_MODEL.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
    

    pub fn getModelName(&mut self) -> String {
        return { let java_object = (jcall!(self.cpuHashMap).get(crate::java_into_object(CpuInfo_MODELNAME.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
    

    pub fn getStepping(&mut self) -> String {
        return { let java_object = (jcall!(self.cpuHashMap).get(crate::java_into_object(CpuInfo_STEPPING.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
    

    pub fn getCpuSpeed(&mut self) -> String {
        return { let java_object = (jcall!(self.cpuHashMap).get(crate::java_into_object(CpuInfo_CPUMHZ.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
    

    pub fn getCacheSize(&mut self) -> String {
        return { let java_object = (jcall!(self.cpuHashMap).get(crate::java_into_object(CpuInfo_CACHESIZE.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
    

    pub fn compareTo(&mut self, mut cpuInterface: Option<Rc<RefCell<dyn CpuInterface>>>) -> bool {
        return true;
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn compareTo_HardwareComponentInterface(&mut self, mut componentInterface: Option<Rc<RefCell<dyn HardwareComponentInterface>>>) -> bool {
        return true;
    }
    

    pub fn toString(&mut self) -> String {
        let mut commonLabels: Option<Rc<RefCell<CommonLabels>>> = CommonLabels::getInstance();
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        let mut stringMaker: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringMaker).append(jcall!(CpuInfo_PROCESSOR.with(Clone::clone)).toString()).append({ let java_field = jcall!(commonLabels).COLON_SEP.clone(); java_field }.clone()).append(self.getProcessor());
        jcall!(stringMaker).append({ let java_field = jcall!(commonSeps).NEW_LINE.clone(); java_field }.clone());
        jcall!(stringMaker).append(jcall!(CpuInfo_VENDORID.with(Clone::clone)).toString()).append({ let java_field = jcall!(commonLabels).COLON_SEP.clone(); java_field }.clone()).append(self.getVendorId());
        jcall!(stringMaker).append({ let java_field = jcall!(commonSeps).NEW_LINE.clone(); java_field }.clone());
        jcall!(stringMaker).append(jcall!(CpuInfo_CPUFAMILY.with(Clone::clone)).toString()).append({ let java_field = jcall!(commonLabels).COLON_SEP.clone(); java_field }.clone()).append(self.getCpuFamily());
        jcall!(stringMaker).append({ let java_field = jcall!(commonSeps).NEW_LINE.clone(); java_field }.clone());
        jcall!(stringMaker).append(jcall!(CpuInfo_MODEL.with(Clone::clone)).toString()).append({ let java_field = jcall!(commonLabels).COLON_SEP.clone(); java_field }.clone()).append(self.getModel());
        jcall!(stringMaker).append({ let java_field = jcall!(commonSeps).NEW_LINE.clone(); java_field }.clone());
        jcall!(stringMaker).append(jcall!(CpuInfo_MODELNAME.with(Clone::clone)).toString()).append({ let java_field = jcall!(commonLabels).COLON_SEP.clone(); java_field }.clone()).append(self.getModelName());
        jcall!(stringMaker).append({ let java_field = jcall!(commonSeps).NEW_LINE.clone(); java_field }.clone());
        jcall!(stringMaker).append(jcall!(CpuInfo_STEPPING.with(Clone::clone)).toString()).append({ let java_field = jcall!(commonLabels).COLON_SEP.clone(); java_field }.clone()).append(self.getStepping());
        jcall!(stringMaker).append({ let java_field = jcall!(commonSeps).NEW_LINE.clone(); java_field }.clone());
        jcall!(stringMaker).append(jcall!(CpuInfo_CPUMHZ.with(Clone::clone)).toString()).append({ let java_field = jcall!(commonLabels).COLON_SEP.clone(); java_field }.clone()).append(self.getCpuSpeed());
        jcall!(stringMaker).append({ let java_field = jcall!(commonSeps).NEW_LINE.clone(); java_field }.clone());
        jcall!(stringMaker).append(jcall!(CpuInfo_CACHESIZE.with(Clone::clone)).toString()).append({ let java_field = jcall!(commonLabels).COLON_SEP.clone(); java_field }.clone()).append(self.getCacheSize());
        return jcall!(stringMaker).toString();
    }
}

