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
// package org.allbinary.logic.system.security.licensing
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::system::hardware::HardwareInterface::HardwareInterface;
use crate::org::allbinary::logic::system::hardware::SystemHardwareFactory::SystemHardwareFactory;
use crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem;
use crate::org::allbinary::logic::system::os::NoOperatingSystem::NoOperatingSystem;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::AbeClientInformationInterface::AbeClientInformationInterface;
use super::AbeClientInformationData::AbeClientInformationData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

// This is the information sent to the license server
pub struct ClientInformation {
    pub operatingSystemInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>>,
    pub tsUtil: Option<Rc<RefCell<TsUtil>>>,
    pub name: String,
    pub shortName: String,
    pub version: String,
    pub specialName: String,
    pub hardSale: bool,
    pub licenseId: String,
    pub licenseServers: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

impl AbeClientInformationInterface for ClientInformation {
    fn init(&mut self) { Self::init(self) }
    fn getOperatingSystemInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>> { Self::getOperatingSystemInterface(self) }
    fn getName(&mut self) -> String { Self::getName(self) }
    fn getSpecialName(&mut self) -> String { Self::getSpecialName(self) }
    fn getVersion(&mut self) -> String { Self::getVersion(self) }
    fn getLicenseId(&mut self) -> String { Self::getLicenseId(self) }
    fn getLicenseServers(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getLicenseServers(self) }
    fn getLicenseServer(&mut self, mut index: i32) -> String { Self::getLicenseServer(self, index) }
    fn getNumberOfLicenseServers(&mut self) -> i32 { Self::getNumberOfLicenseServers(self) }
    fn toHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> { Self::toHashtable(self) }
    fn isSameId(&mut self, mut alicenseId: String) -> bool { Self::isSameId(self, alicenseId) }
    fn isLargerOrDifferentServerList(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> bool { Self::isLargerOrDifferentServerList(self, list) }
    fn isHardSale(&mut self) -> bool { Self::isHardSale(self) }
    fn toShortString(&mut self) -> String { Self::toShortString(self) }
}

impl ClientInformation {
    

    pub fn new_struct(mut name: String, mut version: String, mut specialName: String, mut shortName: String) -> Self {
        let mut this = Self {
            operatingSystemInterface: None,
            tsUtil: None,
            name: String::new(),
            shortName: String::new(),
            version: String::new(),
            specialName: String::new(),
            hardSale: false,
            licenseId: String::new(),
            licenseServers: None,
        };
        this.operatingSystemInterface = crate::org::allbinary::logic::system::os::NoOperatingSystem::NoOperatingSystem_NO_OPERATING_SYSTEM.with(Clone::clone).clone();
        this.tsUtil = TsUtil::getInstance();
        this.hardSale = true;
        this.licenseId = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.licenseServers = jcall!(BasicArrayListUtil::getInstance()).getImmutableInstance();
        this.name = name.clone();
        this.version = version.clone();
        this.specialName = specialName.clone();
        this.shortName = jcall!(StringMaker::new()).append(shortName.clone()).append(jcall!(CommonSeps::getInstance()).UNDERSCORE.clone()).append(version.clone()).toString();
        this.init();
        this
    }

    pub fn new(mut name: String, mut version: String, mut specialName: String, mut shortName: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, version, specialName, shortName))))
    }
    

    pub fn init(&mut self) {
    }
    

    pub fn getOperatingSystemInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>> {
        return self.operatingSystemInterface.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn getSpecialName(&mut self) -> String {
        return self.specialName.clone();
    }
    

    pub fn getVersion(&mut self) -> String {
        return self.version.clone();
    }
    

    pub fn getLicenseId(&mut self) -> String {
        return self.licenseId.clone();
    }
    

    pub fn getLicenseServers(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.licenseServers.clone();
    }
    

    pub fn getLicenseServer(&mut self, mut index: i32) -> String {
        return (jcall!(self.licenseServers).as_BasicArrayList_mut().objectArray[(index) as usize]).clone() /* as String */;
    }
    

    pub fn getNumberOfLicenseServers(&mut self) -> i32 {
        return jcall!(self.licenseServers).size();
    }
    

    pub fn toHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        let mut abeClientInformationData: Option<Rc<RefCell<AbeClientInformationData>>> = AbeClientInformationData::getInstance();
        let mut genericOperatingSystem: Option<Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>> = self.getOperatingSystemInterface();
        let mut clientInfoHashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = jcall!(StdUtil::getInstance()).createHashtable();
        jcall!(clientInfoHashtable).put(crate::java_into_object(jcall!(abeClientInformationData).NAME.clone()), crate::java_into_object(self.name.clone()));
        jcall!(clientInfoHashtable).put(crate::java_into_object(jcall!(abeClientInformationData).VERSION.clone()), crate::java_into_object(self.version.clone()));
        jcall!(clientInfoHashtable).put(crate::java_into_object(jcall!(abeClientInformationData).SPECIALNAME.clone()), crate::java_into_object(self.specialName.clone()));
        jcall!(clientInfoHashtable).put(crate::java_into_object(jcall!(abeClientInformationData).LICENSEID.clone()), crate::java_into_object(self.getLicenseId()));
        jcall!(clientInfoHashtable).put(crate::java_into_object(jcall!(abeClientInformationData).OSNAME.clone()), crate::java_into_object(jcall!(genericOperatingSystem).getName()));
        jcall!(clientInfoHashtable).put(crate::java_into_object(jcall!(abeClientInformationData).OSARCH.clone()), crate::java_into_object(jcall!(genericOperatingSystem).getArch()));
        jcall!(clientInfoHashtable).put(crate::java_into_object(jcall!(abeClientInformationData).OSVERSION.clone()), crate::java_into_object(jcall!(genericOperatingSystem).getVersion()));
        jcall!(clientInfoHashtable).put(crate::java_into_object(jcall!(abeClientInformationData).OS.clone()), crate::java_into_object(jcall!(genericOperatingSystem).toString()));
        let mut hardwareInterface: Option<Rc<RefCell<dyn HardwareInterface>>> = jcall!(SystemHardwareFactory::getInstance()).getInstance1(genericOperatingSystem.clone());
        let mut hardwareString: String = jcall!(hardwareInterface).toString();
        jcall!(clientInfoHashtable).put(crate::java_into_object(jcall!(abeClientInformationData).HARDWARE.clone()), crate::java_into_object(hardwareString.clone()));
        return clientInfoHashtable.clone();
    }
    

    pub fn isSameId(&mut self, mut alicenseId: String) -> bool {
        if { let left = &(self.getLicenseId()); let right = &(alicenseId); let mut left_units = left.encode_utf16(); let mut right_units = right.encode_utf16(); loop { match (left_units.next(), right_units.next()) { (Some(a), Some(b)) if a != b => break a as i32 - b as i32, (Some(_), Some(_)) => {}, (None, None) => break 0, _ => break left.encode_utf16().count() as i32 - right.encode_utf16().count() as i32 } } } == 0 {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn isLargerOrDifferentServerList(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> bool {
        let mut size: i32 = jcall!(list).size() - 1;
        let mut currentSize: i32 = jcall!(self.licenseServers).size() - 1;
        if size > currentSize {
            return true;
        }
        for index in (0..=size).rev() {
            let mut newServerName: String = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone() /* as String */;
            for index2 in (0..=currentSize).rev() {
                let mut nextOldServerName: String = (jcall!(self.licenseServers).as_BasicArrayList_mut().objectArray[(index) as usize]).clone() /* as String */;
                if { let left = &(newServerName); let right = &(nextOldServerName); let mut left_units = left.encode_utf16(); let mut right_units = right.encode_utf16(); loop { match (left_units.next(), right_units.next()) { (Some(a), Some(b)) if a != b => break a as i32 - b as i32, (Some(_), Some(_)) => {}, (None, None) => break 0, _ => break left.encode_utf16().count() as i32 - right.encode_utf16().count() as i32 } } } != 0 {
                    return true;
                }
            }
        }
        return false;
    }
    

    pub fn setHardSale(&mut self, mut hardSale: bool) {
        self.hardSale = hardSale.clone();
    }
    

    pub fn isHardSale(&mut self) -> bool {
        return self.hardSale.clone();
    }
    

    pub fn setOperatingSystemInterface(&mut self, mut operatingSystemInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>>) {
        self.operatingSystemInterface = operatingSystemInterface.clone();
    }
    

    pub fn setLicenseId(&mut self, mut licenseId: String) {
        self.licenseId = licenseId.clone();
    }
    

    pub fn setLicenseServers(&mut self, mut licenseServers: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        self.licenseServers = licenseServers.clone();
    }
    

    pub fn toShortString(&mut self) -> String {
        return self.shortName.clone();
    }
}
pub trait ClientInformationDyn {
    fn as_ClientInformation(&self) -> &ClientInformation;
    fn as_ClientInformation_mut(&mut self) -> &mut ClientInformation;
    fn init(&mut self) { ClientInformation::init(self.as_ClientInformation_mut()) }
    fn getOperatingSystemInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>> { ClientInformation::getOperatingSystemInterface(self.as_ClientInformation_mut()) }
    fn getName(&mut self) -> String { ClientInformation::getName(self.as_ClientInformation_mut()) }
    fn getSpecialName(&mut self) -> String { ClientInformation::getSpecialName(self.as_ClientInformation_mut()) }
    fn getVersion(&mut self) -> String { ClientInformation::getVersion(self.as_ClientInformation_mut()) }
    fn getLicenseId(&mut self) -> String { ClientInformation::getLicenseId(self.as_ClientInformation_mut()) }
    fn getLicenseServers(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { ClientInformation::getLicenseServers(self.as_ClientInformation_mut()) }
    fn getLicenseServer(&mut self, mut index: i32) -> String { ClientInformation::getLicenseServer(self.as_ClientInformation_mut(), index) }
    fn getNumberOfLicenseServers(&mut self) -> i32 { ClientInformation::getNumberOfLicenseServers(self.as_ClientInformation_mut()) }
    fn toHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> { ClientInformation::toHashtable(self.as_ClientInformation_mut()) }
    fn isSameId(&mut self, mut alicenseId: String) -> bool { ClientInformation::isSameId(self.as_ClientInformation_mut(), alicenseId) }
    fn isLargerOrDifferentServerList(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> bool { ClientInformation::isLargerOrDifferentServerList(self.as_ClientInformation_mut(), list) }
    fn setHardSale(&mut self, mut hardSale: bool) { ClientInformation::setHardSale(self.as_ClientInformation_mut(), hardSale) }
    fn isHardSale(&mut self) -> bool { ClientInformation::isHardSale(self.as_ClientInformation_mut()) }
    fn setOperatingSystemInterface(&mut self, mut operatingSystemInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>>) { ClientInformation::setOperatingSystemInterface(self.as_ClientInformation_mut(), operatingSystemInterface) }
    fn setLicenseId(&mut self, mut licenseId: String) { ClientInformation::setLicenseId(self.as_ClientInformation_mut(), licenseId) }
    fn setLicenseServers(&mut self, mut licenseServers: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { ClientInformation::setLicenseServers(self.as_ClientInformation_mut(), licenseServers) }
    fn toShortString(&mut self) -> String { ClientInformation::toShortString(self.as_ClientInformation_mut()) }
}
impl ClientInformationDyn for ClientInformation {
    fn as_ClientInformation(&self) -> &ClientInformation { self }
    fn as_ClientInformation_mut(&mut self) -> &mut ClientInformation { self }
}


