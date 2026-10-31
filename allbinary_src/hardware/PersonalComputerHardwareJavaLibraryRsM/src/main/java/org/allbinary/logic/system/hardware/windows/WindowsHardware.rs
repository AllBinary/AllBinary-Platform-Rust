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
// package org.allbinary.logic.system.hardware.windows
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::hardware::HardwareInterface::HardwareInterface;
use crate::org::allbinary::logic::system::hardware::components::ComponentFactory::ComponentFactory;
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
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::util::ABHashtable::ABHashtable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WindowsHardware {
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
    pub componentFactory: Option<Rc<RefCell<ComponentFactory>>>,
}

impl HardwareInterface for WindowsHardware {
    fn getComponent(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn HardwareComponentInterface>>> { Self::getComponent(self, index) }
    fn toString(&mut self) -> String { Self::toString(self) }
    fn compareTo(&mut self, mut hardwareInterface: Option<Rc<RefCell<dyn HardwareInterface>>>) -> bool { Self::compareTo(self, hardwareInterface) }
    fn difference(&mut self, mut hardwareInterface: Option<Rc<RefCell<dyn HardwareInterface>>>) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> { Self::difference(self, hardwareInterface) }
}

impl WindowsHardware {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
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
            componentFactory: None,
        };
        this.NAME = String::from("Windows Hardware Profile");
        this.MINHARDWARE = 0;
        this.componentFactory = ComponentFactory::getInstance();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            this.componentInterfaceVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            this.videoInterfaceVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            this.hardDriveControllerInterfaceVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            this.cpuInterfaceVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            this.usbInterfaceVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            this.ethernetInterfaceVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            this.multimediaInterfaceVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            this.fireWireInterfaceVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            this.bridgeInterfaceVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            this.hardDriveInterfaceVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            this.macInterfaceVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            this.monitorInterfaceVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            if jcall!(this.componentInterfaceVector).size() < this.MINHARDWARE {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Not Enough Data For A Valid License On Windows"))));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getComponentType(&mut self, mut component: String) -> String {
        if component.find(&jcall!(jcall!(self.componentFactory).ETHERNET).toString()).map(|index| index as i32).unwrap_or(-1) >= 0 {
            return jcall!(jcall!(self.componentFactory).ETHERNET).toString();
        } else if component.find(&jcall!(jcall!(self.componentFactory).BRIDGE).toString()).map(|index| index as i32).unwrap_or(-1) >= 0 {
            return jcall!(jcall!(self.componentFactory).BRIDGE).toString();
        } else if component.find(&jcall!(jcall!(self.componentFactory).IDE).toString()).map(|index| index as i32).unwrap_or(-1) >= 0 {
            return jcall!(jcall!(self.componentFactory).IDE).toString();
        } else if component.find(&jcall!(jcall!(self.componentFactory).MULTIMEDIA).toString()).map(|index| index as i32).unwrap_or(-1) >= 0 {
            return jcall!(jcall!(self.componentFactory).MULTIMEDIA).toString();
        } else if component.find(&jcall!(jcall!(self.componentFactory).SCSI).toString()).map(|index| index as i32).unwrap_or(-1) >= 0 {
            return jcall!(jcall!(self.componentFactory).SCSI).toString();
        } else if component.find(&jcall!(jcall!(self.componentFactory).USB).toString()).map(|index| index as i32).unwrap_or(-1) >= 0 {
            return jcall!(jcall!(self.componentFactory).USB).toString();
        } else if component.find(&jcall!(jcall!(self.componentFactory).VGA).toString()).map(|index| index as i32).unwrap_or(-1) >= 0 {
            return jcall!(jcall!(self.componentFactory).VGA).toString();
        } else {
            return None;
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
}

