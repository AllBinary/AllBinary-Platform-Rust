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
// package views.generic.address.shipping
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontFactory::StoreFrontFactory;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData;
use crate::org::allbinary::business::user::address::StreetAddress::StreetAddress;
use crate::org::allbinary::business::user::commerce::money::tax::TaxFactory::TaxFactory;
use crate::org::allbinary::business::user::commerce::money::tax::modules::TaxModuleInterface::TaxModuleInterface;
use crate::org::allbinary::data::tables::user::address::shipping::ShippingAddressesEntity::ShippingAddressesEntity;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::validate::ValidationComponentInterface::ValidationComponentInterface;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::ShippingAddressView::ShippingAddressView;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AddTaxValidationView {
    pub base: ShippingAddressView,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

// impl ValidationComponentInterface for AddTaxValidationView with the methods of the trait

impl AddTaxValidationView {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: ShippingAddressView::new_struct(transformInfoInterface.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    // throws Exception
    pub fn isValid(&mut self) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.streetAddress = StreetAddress::new1(self.getRequest());
            let mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>> = StoreFrontFactory::getInstance(jcall!(self.getTransformInfoInterface()).getStoreName());
            let mut taxInterface: Option<Rc<RefCell<dyn TaxModuleInterface>>> = jcall!(TaxFactory::getInstance()).getInstance2(self.abeClientInformation.clone(), storeFrontInterface.clone());
            if taxInterface.is_none() {
                return false;
            }
            if match (&jcall!(taxInterface).isValid(self.streetAddress.clone(), storeFrontInterface.clone()), &false) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                return false;
            }
            let mut billingAddressesEntity: Option<Rc<RefCell<ShippingAddressesEntity>>> = ShippingAddressesEntity::new(jcall!(self.getWeblisketSession()).getUserName());
            let mut streetAddressList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(billingAddressesEntity).get();
            if streetAddressList.is_none() {
                return false;
            }
            let mut size: i32 = jcall!(streetAddressList).size();
            for index in (0..size) {
                let mut aStreetAddress: Option<Rc<RefCell<StreetAddress>>> = { let java_object = (jcall!(streetAddressList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<StreetAddress>>>>().expect("ClassCastException").clone()) };
                if jcall!(jcall!(aStreetAddress).getName()).compareTo(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_TAX.with(Clone::clone).clone()) == 0 {
                    return false;
                }
            }
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to validate"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
    

    // throws Exception
    pub fn validationInfo(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            let mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>> = StoreFrontFactory::getInstance(jcall!(self.getTransformInfoInterface()).getStoreName());
            let mut taxInterface: Option<Rc<RefCell<dyn TaxModuleInterface>>> = jcall!(TaxFactory::getInstance()).getInstance2(self.abeClientInformation.clone(), storeFrontInterface.clone());
            if taxInterface.is_none() {
                jcall!(stringBuffer).append(String::from("Unable to Load Tax Component<br/>"));
            } else {
                if match (&jcall!(taxInterface).isValid(self.streetAddress.clone(), storeFrontInterface.clone()), &false) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    jcall!(stringBuffer).append(String::from("Unable to validate address with Tax Component<br/>"));
                }
            }
            let mut billingAddressesEntity: Option<Rc<RefCell<ShippingAddressesEntity>>> = ShippingAddressesEntity::new(jcall!(self.getWeblisketSession()).getUserName());
            let mut streetAddressList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(billingAddressesEntity).get();
            let mut size: i32 = jcall!(streetAddressList).size();
            for index in (0..size) {
                let mut aStreetAddress: Option<Rc<RefCell<StreetAddress>>> = { let java_object = (jcall!(streetAddressList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<StreetAddress>>>>().expect("ClassCastException").clone()) };
                if jcall!(jcall!(aStreetAddress).getName()).compareTo(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_TAX.with(Clone::clone).clone()) == 0 {
                    jcall!(stringBuffer).append(String::from("Not a valid tax location<br/>"));
                }
            }
            return jcall!(stringBuffer).toString();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to create validateInfo"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("validationInfo()"), crate::java_into_object(e.clone()));
            }
            return String::from("Error Creating ValidationInfo");
            }
        }
    }
    

    pub fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    pub fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return None;
    }
}
impl std::ops::Deref for AddTaxValidationView {
    type Target = ShippingAddressView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AddTaxValidationView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::views::generic::address::shipping::ShippingAddressView::ShippingAddressViewDyn for AddTaxValidationView {
    fn as_ShippingAddressView(&self) -> &crate::views::generic::address::shipping::ShippingAddressView::ShippingAddressView { &self.base }
    fn as_ShippingAddressView_mut(&mut self) -> &mut crate::views::generic::address::shipping::ShippingAddressView::ShippingAddressView { &mut self.base }
}
impl crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn for AddTaxValidationView {
    fn as_HttpStoreComponentView(&self) -> &crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &self.base.base }
    fn as_HttpStoreComponentView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &mut self.base.base }
}
impl crate::views::HttpComponentView::HttpComponentViewDyn for AddTaxValidationView {
    fn as_HttpComponentView(&self) -> &crate::views::HttpComponentView::HttpComponentView { &self.base.base.base }
    fn as_HttpComponentView_mut(&mut self) -> &mut crate::views::HttpComponentView::HttpComponentView { &mut self.base.base.base }
    fn view(&mut self) -> String { crate::views::generic::address::shipping::ShippingAddressView::ShippingAddressView::view(crate::views::generic::address::shipping::ShippingAddressView::ShippingAddressViewDyn::as_ShippingAddressView_mut(self)) }
}


