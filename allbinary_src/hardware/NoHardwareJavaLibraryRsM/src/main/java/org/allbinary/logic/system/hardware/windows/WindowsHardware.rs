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
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
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
use crate::org::allbinary::util::ABHashtable::ABHashtable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WindowsHardware {
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
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getMultimedia(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn MediaInterface>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getBridge(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn BridgeInterface>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getCpu(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn CpuInterface>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getEthernet(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn EthernetInterface>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getFireWire(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn FireWireInterface>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getHardDriveController(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn HardDriveControllerInterface>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getHardDrive(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn HardDriveInterface>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getMachineAccessControlAddress(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn MachineAccessControlAddressInterface>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getMonitor(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn MonitorInterface>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getUsb(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn UsbInterface>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getVideo(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn VideoInterface>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getComponent(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn HardwareComponentInterface>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn toString(&mut self) -> String {
        return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
    }
    

    pub fn compareTo(&mut self, mut hardwareInterface: Option<Rc<RefCell<dyn HardwareInterface>>>) -> bool {
        return true;
    }
    

    pub fn difference(&mut self, mut hardwareInterface: Option<Rc<RefCell<dyn HardwareInterface>>>) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        return jcall!(StdUtil::getInstance()).NULL_TABLE.clone();
    }
}

