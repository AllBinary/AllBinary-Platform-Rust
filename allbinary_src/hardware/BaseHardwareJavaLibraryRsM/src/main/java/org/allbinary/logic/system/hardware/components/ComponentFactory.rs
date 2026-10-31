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
// package org.allbinary.logic.system.hardware.components
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::Component::Component;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ComponentFactory {
    pub VGA: Option<Rc<RefCell<Component>>>,
    pub IDE: Option<Rc<RefCell<Component>>>,
    pub SCSI: Option<Rc<RefCell<Component>>>,
    pub BRIDGE: Option<Rc<RefCell<Component>>>,
    pub ETHERNET: Option<Rc<RefCell<Component>>>,
    pub USB: Option<Rc<RefCell<Component>>>,
    pub MULTIMEDIA: Option<Rc<RefCell<Component>>>,
    pub PCI: Option<Rc<RefCell<Component>>>,
    pub RAID: Option<Rc<RefCell<Component>>>,
    pub SMBUS: Option<Rc<RefCell<Component>>>,
    pub PIC: Option<Rc<RefCell<Component>>>,
    pub Class: Option<Rc<RefCell<Component>>>,
    pub UNKNOWN: Option<Rc<RefCell<Component>>>,
}

thread_local! { pub static ComponentFactory_instance: Option<Rc<RefCell<ComponentFactory>>> = ComponentFactory::new(); }

impl ComponentFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            VGA: None,
            IDE: None,
            SCSI: None,
            BRIDGE: None,
            ETHERNET: None,
            USB: None,
            MULTIMEDIA: None,
            PCI: None,
            RAID: None,
            SMBUS: None,
            PIC: None,
            Class: None,
            UNKNOWN: None,
        };
        this.VGA = Component::new(String::from("VGA"));
        this.IDE = Component::new(String::from("IDE"));
        this.SCSI = Component::new(String::from("SCSI"));
        this.BRIDGE = Component::new(String::from("bridge"));
        this.ETHERNET = Component::new(String::from("Ethernet"));
        this.USB = Component::new(String::from("USB"));
        this.MULTIMEDIA = Component::new(String::from("Multimedia"));
        this.PCI = Component::new(String::from("PCI"));
        this.RAID = Component::new(String::from("RAID"));
        this.SMBUS = Component::new(String::from("SMBus"));
        this.PIC = Component::new(String::from("PIC"));
        this.Class = Component::new(String::from("Class"));
        this.UNKNOWN = Component::new(jcall!(CommonStrings::getInstance()).UNKNOWN.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ComponentFactory>>> {
        return ComponentFactory_instance.with(Clone::clone).clone();
    }
}

