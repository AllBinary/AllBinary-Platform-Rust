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

thread_local! { pub static MobileClientInformation_DESC: String = String::from("Mobile"); }
thread_local! { pub static MobileClientInformation_ANDROID_DESC: String = format!("{}{}", String::from("Android"), MobileClientInformation_DESC.with(Clone::clone)); }

impl MobileClientInformation {
    

    pub fn new_struct(mut name: String, mut version: String, mut specialName: String, mut shortName: String) -> Self {
        let mut this = Self {
            base: AbeClientInformation::new_struct(name.clone(), version.clone(), jcall!(StringMaker::new()).append(specialName.clone()).append(jcall!(CommonSeps::getInstance()).SPACE.clone()).append(jcall!(PartnerIdentifierFileUtil::PartnerIdentifierFileUtil::getInstance()).get()).toString(), shortName.clone()),
        };
        // super(...) is in the struct literal above
        let mut string: String = this.getSpecialName();
        let mut number: String = string[((string.chars().count() as i32) - 1) as usize..].to_string();
        if Integer::Integer::getInteger(number.clone()).is_none() {
            this.setHardSale(false);
        }
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        if this.isHardSale() {
            jcall!(this.logUtil).putF(String::from("Use Hard Coded Selling"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).CONSTRUCTOR.clone());
        } else {
            jcall!(this.logUtil).putF(String::from("Don't Use Hard Coded Selling"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).CONSTRUCTOR.clone());
        }
        this
    }

    pub fn new(mut name: String, mut version: String, mut specialName: String, mut shortName: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, version, specialName, shortName))))
    }
    

    pub fn toHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = self.base.toHashtable();
        let mut registrationConfiguration: Option<Rc<RefCell<RegistrationConfiguration>>> = RegistrationConfiguration::getInstance();
        jcall!(hashtable).put(crate::java_into_object(jcall!(registrationConfiguration).NAME.clone()), crate::java_into_object(jcall!(registrationConfiguration).getRegistrationCode()));
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
    fn toHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> { Self::toHashtable(self) }
}
impl crate::org::allbinary::logic::system::security::licensing::AbeClientInformation::AbeClientInformationDyn for MobileClientInformation {
    fn as_AbeClientInformation(&self) -> &crate::org::allbinary::logic::system::security::licensing::AbeClientInformation::AbeClientInformation { &self.base }
    fn as_AbeClientInformation_mut(&mut self) -> &mut crate::org::allbinary::logic::system::security::licensing::AbeClientInformation::AbeClientInformation { &mut self.base }
}


