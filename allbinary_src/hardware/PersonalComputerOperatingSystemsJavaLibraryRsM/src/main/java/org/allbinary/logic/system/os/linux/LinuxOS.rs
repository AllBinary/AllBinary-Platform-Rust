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
// package org.allbinary.logic.system.os.linux
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LinuxOS {
    pub base: GenericOperatingSystem,
}


impl LinuxOS {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: GenericOperatingSystem::new_struct(),
        };
        this.scalable = true;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn isAutoHide(&mut self) -> bool {
        return true;
    }
}
impl std::ops::Deref for LinuxOS {
    type Target = GenericOperatingSystem;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for LinuxOS {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn for LinuxOS {
    fn as_GenericOperatingSystem(&self) -> &crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem { &self.base }
    fn as_GenericOperatingSystem_mut(&mut self) -> &mut crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem { &mut self.base }
}
impl crate::org::allbinary::logic::system::os::OperatingSystemInterface::OperatingSystemInterface for LinuxOS {
    fn getName(&mut self) -> String { crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem::getName(crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn::as_GenericOperatingSystem_mut(self)) }
    fn getVersion(&mut self) -> String { crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem::getVersion(crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn::as_GenericOperatingSystem_mut(self)) }
    fn getArch(&mut self) -> String { crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem::getArch(crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn::as_GenericOperatingSystem_mut(self)) }
    fn getCharacterSet(&mut self) -> Option<Rc<RefCell<crate::org::allbinary::logic::java::character::CharacterSet::CharacterSet>>> { crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem::getCharacterSet(crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn::as_GenericOperatingSystem_mut(self)) }
    fn isOverScan(&mut self) -> bool { crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem::isOverScan(crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn::as_GenericOperatingSystem_mut(self)) }
    fn getOverScanXPercent(&mut self) -> i32 { crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem::getOverScanXPercent(crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn::as_GenericOperatingSystem_mut(self)) }
    fn getOverScanYPercent(&mut self) -> i32 { crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem::getOverScanYPercent(crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn::as_GenericOperatingSystem_mut(self)) }
    fn isScalable(&mut self) -> bool { crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem::isScalable(crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn::as_GenericOperatingSystem_mut(self)) }
    fn isAutoHide(&mut self) -> bool { Self::isAutoHide(self) }
    fn toString(&mut self) -> String { crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem::toString(crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn::as_GenericOperatingSystem_mut(self)) }
}


