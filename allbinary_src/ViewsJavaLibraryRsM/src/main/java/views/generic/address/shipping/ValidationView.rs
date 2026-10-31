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
use crate::org::allbinary::business::user::address::StreetAddress::StreetAddress;
use crate::org::allbinary::data::tables::user::address::shipping::ShippingAddressesEntity::ShippingAddressesEntity;
use crate::org::allbinary::logic::control::validate::ValidationComponentInterface::ValidationComponentInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::ShippingAddressesView::ShippingAddressesView;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ValidationView {
    pub base: ShippingAddressesView,
}

// impl ValidationComponentInterface for ValidationView with the methods of the trait

impl ValidationView {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: ShippingAddressesView::new_struct(transformInfoInterface.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        let mut billingAddressesEntity: Option<Rc<RefCell<ShippingAddressesEntity>>> = ShippingAddressesEntity::new(jcall!(self.getWeblisketSession()).getUserName());
        self.streetAddressList = jcall!(billingAddressesEntity).get();
        if self.streetAddressList.is_none() {
            return false;
        }
        let mut size: i32 = jcall!(self.streetAddressList).size();
        for index in (0..size) {
            let mut streetAddress: Option<Rc<RefCell<StreetAddress>>> = { let java_object = (jcall!(self.base.streetAddressList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<StreetAddress>>>>().expect("ClassCastException").clone()) };
            if match (&jcall!(streetAddress).isValid(), &false) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(self.base.streetAddressList).remove(crate::java_into_object(streetAddress.clone()));
            }
        }
        if jcall!(self.streetAddressList).size() < 1 {
            return false;
        }
        return true;
    }
    

    pub fn validationInfo(&mut self) -> String {
        return String::from("No Shipping Addresses to display.<br />");
    }
    

    pub fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    pub fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return None;
    }
}
impl std::ops::Deref for ValidationView {
    type Target = ShippingAddressesView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ValidationView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::views::generic::address::shipping::ShippingAddressesView::ShippingAddressesViewDyn for ValidationView {
    fn as_ShippingAddressesView(&self) -> &crate::views::generic::address::shipping::ShippingAddressesView::ShippingAddressesView { &self.base }
    fn as_ShippingAddressesView_mut(&mut self) -> &mut crate::views::generic::address::shipping::ShippingAddressesView::ShippingAddressesView { &mut self.base }
}
impl crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn for ValidationView {
    fn as_HttpStoreComponentView(&self) -> &crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &self.base.base }
    fn as_HttpStoreComponentView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &mut self.base.base }
}
impl crate::views::HttpComponentView::HttpComponentViewDyn for ValidationView {
    fn as_HttpComponentView(&self) -> &crate::views::HttpComponentView::HttpComponentView { &self.base.base.base }
    fn as_HttpComponentView_mut(&mut self) -> &mut crate::views::HttpComponentView::HttpComponentView { &mut self.base.base.base }
    fn view(&mut self) -> String { crate::views::generic::address::shipping::ShippingAddressesView::ShippingAddressesView::view(crate::views::generic::address::shipping::ShippingAddressesView::ShippingAddressesViewDyn::as_ShippingAddressesView_mut(self)) }
}


