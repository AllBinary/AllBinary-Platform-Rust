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
use crate::java::lang::Exception::*;
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

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            shippingVector: None,
            defaultShippingMethodInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.defaultShippingMethodInterface = { let java_value = (jcall!(ShippingMethodsFactory::new(abeClientInformation.clone(), storeFrontInterface.clone())).getDefaultInstance()).clone(); java_value };
        this.shippingVector = { let java_value = (jcall!(ShippingMethodsFactory::new(abeClientInformation.clone(), storeFrontInterface.clone())).getInstance()).clone(); java_value };
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
            shipping = { let java_value = ({ let java_object = (jcall!(self.shippingVector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn ShippingInterface>>>>().expect("ClassCastException").clone()) }).clone(); java_value };
            if (name.cmp(&jcall!(shipping).getName()) as i32) == 0 {
                return shipping.clone();
            }
        }
        let mut error: String = format!("{}{}", String::from("Error Finding Shipping: "), name);
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SHIPPINGERROR.clone(); java_field }.clone()) {
            jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getShippingInterface(String name)"));
        }
        panic!("{:?}", format!("{}: {:?}", "Exception", (error.clone())));
    }
    

    // throws Exception
    pub fn getDefault(&mut self) -> Option<Rc<RefCell<dyn ShippingInterface>>> {
        return self.defaultShippingMethodInterface.clone();
    }
}

