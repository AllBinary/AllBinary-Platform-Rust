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
// package org.allbinary.business.user.commerce.shipping
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::user::commerce::shipping::modules::ShippingInterface::ShippingInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::ShippingMethodsInterface::ShippingMethodsInterface;
use super::ShippingMethodsFactory::ShippingMethodsFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ShippingMethods {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub shippingVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub defaultShippingMethodInterface: Option<Rc<RefCell<dyn ShippingInterface>>>,
}

impl ShippingMethodsInterface for ShippingMethods {
    fn get(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::get(self) }
    fn getShippingInterface(&mut self, mut name: String) -> Option<Rc<RefCell<dyn ShippingInterface>>> { Self::getShippingInterface(self, name) }
    fn getDefault(&mut self) -> Option<Rc<RefCell<dyn ShippingInterface>>> { Self::getDefault(self) }
}

impl ShippingMethods {
    

    pub fn new_struct(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            shippingVector: None,
            defaultShippingMethodInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.defaultShippingMethodInterface = jcall!(ShippingMethodsFactory::new(abeClientInformation.clone(), storeFrontInterface.clone())).getDefaultInstance();
        this.shippingVector = jcall!(ShippingMethodsFactory::new(abeClientInformation.clone(), storeFrontInterface.clone())).getInstance();
        this
    }

    pub fn new(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(abeClientInformation, storeFrontInterface))))
    }
    

    pub fn get(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.shippingVector.clone();
    }
    

    // throws Exception
    pub fn getShippingInterface(&mut self, mut name: String) -> Option<Rc<RefCell<dyn ShippingInterface>>> {
        let mut shipping: Option<Rc<RefCell<dyn ShippingInterface>>> = None;
        let mut size: i32 = jcall!(self.shippingVector).size();
        for index in (0..size) {
            shipping = { let java_object = jcall!(self.shippingVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn ShippingInterface>>>>().expect("ClassCastException").clone()) };
            if (name.cmp(&jcall!(shipping).getName()) as i32) == 0 {
                return shipping.clone();
            }
        }
        let mut error: String = format!("{}{}", String::from("Error Finding Shipping: "), name);
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SHIPPINGERROR.clone()) {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getShippingInterface(String name)"));
        }
        panic!("{:?}", format!("{}: {:?}", "Exception", (error.clone())));
    }
    

    // throws Exception
    pub fn getDefault(&mut self) -> Option<Rc<RefCell<dyn ShippingInterface>>> {
        return self.defaultShippingMethodInterface.clone();
    }
}

