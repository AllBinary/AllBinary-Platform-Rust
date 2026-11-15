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
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::security::licensing::registration::RegistrationConfiguration::RegistrationConfiguration;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::AbeClientInformation::AbeClientInformation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MobileClientInformation {
    pub base: AbeClientInformation,
}

pub static MobileClientInformation_DESC: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Mobile"));
pub static MobileClientInformation_ANDROID_DESC: crate::JavaStatic<String> = crate::JavaStatic::new(|| format!("{}{}", String::from("Android"), MobileClientInformation_DESC.with(Clone::clone)));
impl crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface for MobileClientInformation {
    fn init(&mut self) { crate::org::allbinary::logic::system::security::licensing::AbeClientInformation::AbeClientInformation::init(crate::org::allbinary::logic::system::security::licensing::AbeClientInformation::AbeClientInformationDyn::as_AbeClientInformation_mut(self)) }
    fn getOperatingSystemInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>> { crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformation::getOperatingSystemInterface(crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformationDyn::as_ClientInformation_mut(self)) }
    fn getName(&mut self) -> String { crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformation::getName(crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformationDyn::as_ClientInformation_mut(self)) }
    fn getSpecialName(&mut self) -> String { crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformation::getSpecialName(crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformationDyn::as_ClientInformation_mut(self)) }
    fn getVersion(&mut self) -> String { crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformation::getVersion(crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformationDyn::as_ClientInformation_mut(self)) }
    fn getLicenseId(&mut self) -> String { crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformation::getLicenseId(crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformationDyn::as_ClientInformation_mut(self)) }
    fn getLicenseServers(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformation::getLicenseServers(crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformationDyn::as_ClientInformation_mut(self)) }
    fn getLicenseServer(&mut self, p1: i32) -> String { crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformation::getLicenseServer(crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformationDyn::as_ClientInformation_mut(self), p1) }
    fn getNumberOfLicenseServers(&mut self) -> i32 { crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformation::getNumberOfLicenseServers(crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformationDyn::as_ClientInformation_mut(self)) }
    fn toHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>> { Self::toHashtable(self) }
    fn isSameId(&mut self, p1: String) -> bool { crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformation::isSameId(crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformationDyn::as_ClientInformation_mut(self), p1) }
    fn isLargerOrDifferentServerList(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> bool { crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformation::isLargerOrDifferentServerList(crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformationDyn::as_ClientInformation_mut(self), p1) }
    fn isHardSale(&mut self) -> bool { crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformation::isHardSale(crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformationDyn::as_ClientInformation_mut(self)) }
    fn toShortString(&mut self) -> String { crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformation::toShortString(crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformationDyn::as_ClientInformation_mut(self)) }
}

impl MobileClientInformation {
    

    pub fn new_struct(mut name: String, mut version: String, mut specialName: String, mut shortName: String) -> Self {
        let mut this = Self {
            base: AbeClientInformation::new_struct(name.clone(), version.clone(), jcall!(StringMaker::new()).append(specialName.clone()).append({ let java_field = jcall!(CommonSeps::getInstance()).SPACE.clone(); java_field }.clone()).append(jcall!(PartnerIdentifierFileUtil::PartnerIdentifierFileUtil::getInstance()).get()).toString(), shortName.clone()),
        };
        // super(...) is in the struct literal above
        let mut string: String = this.getSpecialName();
        let mut number: String = string[((string.chars().count() as i32) - 1) as usize..].to_string();
        if Integer::Integer::getInteger(number.clone()).is_none() {
            this.setHardSale(false);
        }
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        if this.isHardSale() {
            jcall!(this.logUtil).putF(String::from("Use Hard Coded Selling"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).CONSTRUCTOR.clone(); java_field }.clone());
        } else {
            jcall!(this.logUtil).putF(String::from("Don't Use Hard Coded Selling"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).CONSTRUCTOR.clone(); java_field }.clone());
        }
        this
    }

    pub fn new(mut name: String, mut version: String, mut specialName: String, mut shortName: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, version, specialName, shortName))))
    }
    

    pub fn toHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>> {
        let mut hashtable: Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>> = self.base.toHashtable();
        let mut registrationConfiguration: Option<Rc<RefCell<RegistrationConfiguration>>> = RegistrationConfiguration::getInstance();
        jcall!(hashtable).put(crate::java_into_object({ let java_field = jcall!(registrationConfiguration).NAME.clone(); java_field }.clone()), crate::java_into_object(jcall!(registrationConfiguration).getRegistrationCode()));
        return hashtable.clone();
    }
}
impl std::ops::Deref for MobileClientInformation {
    type Target = AbeClientInformation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MobileClientInformation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformationDyn for MobileClientInformation {
    fn as_ClientInformation(&self) -> &crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformation { &self.base.base }
    fn as_ClientInformation_mut(&mut self) -> &mut crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformation { &mut self.base.base }
    fn init(&mut self) { crate::org::allbinary::logic::system::security::licensing::AbeClientInformation::AbeClientInformation::init(crate::org::allbinary::logic::system::security::licensing::AbeClientInformation::AbeClientInformationDyn::as_AbeClientInformation_mut(self)) }
    fn toHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>> { Self::toHashtable(self) }
}
impl crate::org::allbinary::logic::system::security::licensing::AbeClientInformation::AbeClientInformationDyn for MobileClientInformation {
    fn as_AbeClientInformation(&self) -> &crate::org::allbinary::logic::system::security::licensing::AbeClientInformation::AbeClientInformation { &self.base }
    fn as_AbeClientInformation_mut(&mut self) -> &mut crate::org::allbinary::logic::system::security::licensing::AbeClientInformation::AbeClientInformation { &mut self.base }
}


