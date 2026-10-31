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
// package org.allbinary.logic.system.hardware.linux
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::FileReader::FileReader;
// use crate::java::io::LineNumberReader::LineNumberReader;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypeFactory::LogConfigTypeFactory;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::FilePathData::FilePathData;
use crate::org::allbinary::logic::io::file::directory::SubDirectory::SubDirectory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::hardware::HardwareInterface::HardwareInterface;
use crate::org::allbinary::logic::system::hardware::components::interfaces::BridgeInterface::BridgeInterface;
use crate::org::allbinary::logic::system::hardware::components::interfaces::CpuInterface::CpuInterface;
use crate::org::allbinary::logic::system::hardware::components::interfaces::EthernetInterface::EthernetInterface;
use crate::org::allbinary::logic::system::hardware::components::interfaces::FireWireInterface::FireWireInterface;
use crate::org::allbinary::logic::system::hardware::components::interfaces::HardDriveControllerInterface::HardDriveControllerInterface;
use crate::org::allbinary::logic::system::hardware::components::interfaces::HardDriveInterface::HardDriveInterface;
use crate::org::allbinary::logic::system::hardware::components::interfaces::HardwareComponentInterface::HardwareComponentInterface;
use crate::org::allbinary::logic::system::hardware::components::interfaces::MachineAccessControlAddressInterface::MachineAccessControlAddressInterface;
use crate::org::allbinary::logic::system::hardware::components::interfaces::MediaInterface::MediaInterface;
use crate::org::allbinary::logic::system::hardware::components::interfaces::MonitorInterface::MonitorInterface;
use crate::org::allbinary::logic::system::hardware::components::interfaces::UsbInterface::UsbInterface;
use crate::org::allbinary::logic::system::hardware::components::interfaces::VideoInterface::VideoInterface;
use crate::org::allbinary::logic::system::hardware::components::linux::Cpu::Cpu;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LinuxHardware {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub componentInterfaceVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub videoInterfaceVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub hardDriveControllerInterfaceVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub cpuInterfaceVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub usbInterfaceVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub ethernetInterfaceVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub multimediaInterfaceVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub fireWireInterfaceVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub bridgeInterfaceVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub hardDriveInterfaceVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub macInterfaceVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub monitorInterfaceVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub NAME: String,
    pub MINHARDWARE: i32,
}

thread_local! { pub static LinuxHardware_PCIFILE: String = String::from("/proc/pci"); }
impl HardwareInterface for LinuxHardware {
    fn getComponent(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn HardwareComponentInterface>>> { Self::getComponent(self, index) }
    fn toString(&mut self) -> String { Self::toString(self) }
    fn compareTo(&mut self, mut hardwareInterface: Option<Rc<RefCell<dyn HardwareInterface>>>) -> bool { Self::compareTo(self, hardwareInterface) }
    fn difference(&mut self, mut hardwareInterface: Option<Rc<RefCell<dyn HardwareInterface>>>) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> { Self::difference(self, hardwareInterface) }
}

impl LinuxHardware {
    

    pub fn new_struct1(mut path: String) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            componentInterfaceVector: None,
            videoInterfaceVector: None,
            hardDriveControllerInterfaceVector: None,
            cpuInterfaceVector: None,
            usbInterfaceVector: None,
            ethernetInterfaceVector: None,
            multimediaInterfaceVector: None,
            fireWireInterfaceVector: None,
            bridgeInterfaceVector: None,
            hardDriveInterfaceVector: None,
            macInterfaceVector: None,
            monitorInterfaceVector: None,
            NAME: String::new(),
            MINHARDWARE: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.NAME = String::from("Linux Hardware Profile");
        this.MINHARDWARE = 5;
        this.init(path.clone());
        this
    }

    pub fn new1(mut path: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(path))))
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            componentInterfaceVector: None,
            videoInterfaceVector: None,
            hardDriveControllerInterfaceVector: None,
            cpuInterfaceVector: None,
            usbInterfaceVector: None,
            ethernetInterfaceVector: None,
            multimediaInterfaceVector: None,
            fireWireInterfaceVector: None,
            bridgeInterfaceVector: None,
            hardDriveInterfaceVector: None,
            macInterfaceVector: None,
            monitorInterfaceVector: None,
            NAME: String::new(),
            MINHARDWARE: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.NAME = String::from("Linux Hardware Profile");
        this.MINHARDWARE = 5;
        this.init(LinuxHardware_PCIFILE.with(Clone::clone).clone());
        if jcall!(this.componentInterfaceVector).size() < this.MINHARDWARE {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Not Enough Data For A Valid License On Linux"))));
        }
        let mut cpu: Option<Rc<RefCell<Cpu>>> = Cpu::new();
        if cpu.is_some() {
            jcall!(this.cpuInterfaceVector).add(crate::java_into_object(cpu.clone()));
            jcall!(this.componentInterfaceVector).add(crate::java_into_object(cpu.clone()));
        }
        if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).OS.clone())) {
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Hardware Data: "), this.toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
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
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).OS.clone())) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Hardware Data: "), self.toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn init2(&mut self, mut lineNumberReader: Option<Rc<RefCell<LineNumberReader>>>, mut filePath: String) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.componentInterfaceVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            self.videoInterfaceVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            self.hardDriveControllerInterfaceVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            self.cpuInterfaceVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            self.usbInterfaceVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            self.ethernetInterfaceVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            self.multimediaInterfaceVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            self.fireWireInterfaceVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            self.bridgeInterfaceVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            self.hardDriveInterfaceVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            self.macInterfaceVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            self.monitorInterfaceVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            let mut pciFile: Option<Rc<RefCell<FileReader>>> = FileReader::new(filePath.clone());
            lineNumberReader = LineNumberReader::new(pciFile.clone());
            if lineNumberReader.is_none() {
                let mut fileVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(SubDirectory::getInstance()).search2(filePath.clone(), AbFile::createAbFile(jcall!(FilePathData::getInstance()).PATH_START.clone()));
                if !jcall!(fileVector).isEmpty() {
                    let mut file: Option<Rc<RefCell<AbFile>>> = { let java_object = (jcall!(fileVector).get(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<AbFile>>>>().expect("ClassCastException").clone()) };
                    lineNumberReader = LineNumberReader::new(FileReader::new(jcall!(file).getPath()));
                }
                if lineNumberReader.is_none() {
                    panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No Linux File Data"))));
                }
            }
            if lineNumberReader.is_some() {
                if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).OS.clone())) {
                    jcall!(self.logUtil).putF(String::from("PCI File Found"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).CONSTRUCTOR.clone());
                }
                let mut nextLine: String = jcall!(lineNumberReader).readLine();
                while lineNumberReader.is_some() && true {
                    if self.isNextHardware(nextLine.clone()) {
                        break;
                    }
                    nextLine = jcall!(lineNumberReader).readLine();
                }
                while self.isNextHardware(nextLine.clone()) {
                    let mut componentData: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                    if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).OS.clone())) {
                        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Found Hardware Device: "), jcall!(self.componentInterfaceVector).size()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).CONSTRUCTOR.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    }
                    jcall!(componentData).append(nextLine.clone());
                    jcall!(componentData).append(jcall!(CommonSeps::getInstance()).NEW_LINE.clone());
                    nextLine = jcall!(lineNumberReader).readLine();
                    let mut componentType: String = jcall!(PCComponentFactory::PCComponentFactory::getInstance()).getComponentType(nextLine.clone());
                    while lineNumberReader.is_some() {
                        jcall!(componentData).append(nextLine.clone());
                        jcall!(componentData).append(jcall!(CommonSeps::getInstance()).NEW_LINE.clone());
                        nextLine = jcall!(lineNumberReader).readLine();
                        if false || self.isNextHardware(nextLine.clone()) {
                            break;
                        }
                    }
                    let mut componentInterface: Option<Rc<RefCell<dyn HardwareComponentInterface>>> = jcall!(PCComponentFactory::PCComponentFactory::getInstance()).getInstance2(componentType.clone(), jcall!(componentData).toString());
                    if componentInterface.is_some() {
                        jcall!(self.componentInterfaceVector).add(crate::java_into_object(componentInterface.clone()));
                    }
                }
            } else {
                if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).OS.clone())) {
                    jcall!(self.logUtil).putF(String::from("Could not load PCI File"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).CONSTRUCTOR.clone());
                }
            }
            jcall!(lineNumberReader).close();
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
    

    pub fn getMultimedia(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn MediaInterface>>> {
        return { let java_object = jcall!(self.multimediaInterfaceVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn MediaInterface>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn getBridge(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn BridgeInterface>>> {
        return { let java_object = jcall!(self.bridgeInterfaceVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn BridgeInterface>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn getCpu(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn CpuInterface>>> {
        return { let java_object = jcall!(self.cpuInterfaceVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn CpuInterface>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn getEthernet(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn EthernetInterface>>> {
        return { let java_object = jcall!(self.ethernetInterfaceVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn EthernetInterface>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn getFireWire(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn FireWireInterface>>> {
        return { let java_object = jcall!(self.fireWireInterfaceVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn FireWireInterface>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn getHardDriveController(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn HardDriveControllerInterface>>> {
        return { let java_object = jcall!(self.hardDriveControllerInterfaceVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn HardDriveControllerInterface>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn getHardDrive(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn HardDriveInterface>>> {
        return { let java_object = jcall!(self.hardDriveInterfaceVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn HardDriveInterface>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn getMachineAccessControlAddress(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn MachineAccessControlAddressInterface>>> {
        return { let java_object = jcall!(self.macInterfaceVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn MachineAccessControlAddressInterface>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn getMonitor(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn MonitorInterface>>> {
        return { let java_object = jcall!(self.monitorInterfaceVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn MonitorInterface>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn getUsb(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn UsbInterface>>> {
        return { let java_object = jcall!(self.usbInterfaceVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn UsbInterface>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn getVideo(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn VideoInterface>>> {
        return { let java_object = jcall!(self.videoInterfaceVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn VideoInterface>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn getComponent(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn HardwareComponentInterface>>> {
        return { let java_object = jcall!(self.componentInterfaceVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn HardwareComponentInterface>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn toString(&mut self) -> String {
        let mut hardwareBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut size: i32 = jcall!(self.componentInterfaceVector).size();
        for index in (0..size) {
            let mut componentInterface: Option<Rc<RefCell<dyn HardwareComponentInterface>>> = { let java_object = jcall!(self.componentInterfaceVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn HardwareComponentInterface>>>>().expect("ClassCastException").clone()) };
            jcall!(hardwareBuffer).append(String::from("Component "));
            jcall!(hardwareBuffer).appendint(index.clone());
            jcall!(hardwareBuffer).append(String::from(": \n"));
            jcall!(hardwareBuffer).append(jcall!(componentInterface).toString());
            jcall!(hardwareBuffer).append(jcall!(CommonSeps::getInstance()).NEW_LINE.clone());
        }
        return jcall!(hardwareBuffer).toString();
    }
    

    pub fn compareTo(&mut self, mut hardwareInterface: Option<Rc<RefCell<dyn HardwareInterface>>>) -> bool {
        return true;
    }
    

    pub fn difference(&mut self, mut hardwareInterface: Option<Rc<RefCell<dyn HardwareInterface>>>) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        return jcall!(StdUtil::getInstance()).NULL_TABLE.clone();
    }
    

    pub fn isNextHardware(&mut self, mut nextLine: String) -> bool {
        if true {
            let mut index: i32 = nextLine.find(&String::from("Bus")).map(|index| index as i32).unwrap_or(-1);
            if index >= 0 && index < 4 {
                return true;
            }
        }
        return false;
    }
}

