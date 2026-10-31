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
use crate::org::allbinary::data::tables::user::address::shipping::ShippingAddressesEntity::ShippingAddressesEntity;
use crate::org::allbinary::logic::control::validate::ValidationComponentInterface::ValidationComponentInterface;
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

pub struct UpdateTaxValidationView {
    pub base: ShippingAddressView,
}

// impl ValidationComponentInterface for UpdateTaxValidationView with the methods of the trait

impl UpdateTaxValidationView {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: ShippingAddressView::new_struct(transformInfoInterface.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    // throws Exception
    pub fn isValid(&mut self) -> Option<bool> {
        self.streetAddress = StreetAddress::new1(self.getRequest());
        let mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>> = StoreFrontFactory::getInstance(jcall!(self.getTransformInfoInterface()).getStoreName());
        if match (&jcall!(jcall!(TaxFactory::getInstance()).getInstance2(self.abeClientInformation.clone(), storeFrontInterface.clone())).isValid(self.streetAddress.clone(), storeFrontInterface.clone()), &false) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return false;
        }
        let mut billingAddressesEntity: Option<Rc<RefCell<ShippingAddressesEntity>>> = ShippingAddressesEntity::new(jcall!(self.getWeblisketSession()).getUserName());
        let mut streetAddressList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(billingAddressesEntity).get();
        if streetAddressList.is_none() {
            return false;
        }
        let mut count: i32 = 0;
        let mut size: i32 = jcall!(streetAddressList).size();
        for index in (0..size) {
            let mut aStreetAddress: Option<Rc<RefCell<StreetAddress>>> = { let java_object = (jcall!(streetAddressList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<StreetAddress>>>>().expect("ClassCastException").clone()) };
            if jcall!(jcall!(aStreetAddress).getName()).compareTo(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_TAX.with(Clone::clone).clone()) == 0 {
                count += 1;
            }
        }
        if count != 1 {
            return false;
        }
        return true;
    }
    

    pub fn validationInfo(&mut self) -> String {
        return String::from("Unable to update tax location.");
    }
    

    pub fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    pub fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return None;
    }
}
impl std::ops::Deref for UpdateTaxValidationView {
    type Target = ShippingAddressView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for UpdateTaxValidationView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::views::generic::address::shipping::ShippingAddressView::ShippingAddressViewDyn for UpdateTaxValidationView {
    fn as_ShippingAddressView(&self) -> &crate::views::generic::address::shipping::ShippingAddressView::ShippingAddressView { &self.base }
    fn as_ShippingAddressView_mut(&mut self) -> &mut crate::views::generic::address::shipping::ShippingAddressView::ShippingAddressView { &mut self.base }
}
impl crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn for UpdateTaxValidationView {
    fn as_HttpStoreComponentView(&self) -> &crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &self.base.base }
    fn as_HttpStoreComponentView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &mut self.base.base }
}
impl crate::views::HttpComponentView::HttpComponentViewDyn for UpdateTaxValidationView {
    fn as_HttpComponentView(&self) -> &crate::views::HttpComponentView::HttpComponentView { &self.base.base.base }
    fn as_HttpComponentView_mut(&mut self) -> &mut crate::views::HttpComponentView::HttpComponentView { &mut self.base.base.base }
    fn view(&mut self) -> String { crate::views::generic::address::shipping::ShippingAddressView::ShippingAddressView::view(crate::views::generic::address::shipping::ShippingAddressView::ShippingAddressViewDyn::as_ShippingAddressView_mut(self)) }
}


