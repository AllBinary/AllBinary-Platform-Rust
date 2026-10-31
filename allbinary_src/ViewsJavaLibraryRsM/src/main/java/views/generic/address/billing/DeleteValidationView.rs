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
// package views.generic.address.billing
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData;
use crate::org::allbinary::data::tables::user::address::billing::BillingAddressesEntity::BillingAddressesEntity;
use crate::org::allbinary::logic::control::validate::ValidationComponentInterface::ValidationComponentInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpInterface::TransformInfoHttpInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::BillingAddressView::BillingAddressView;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DeleteValidationView {
    pub base: BillingAddressView,
    pub value: String,
}

// impl ValidationComponentInterface for DeleteValidationView with the methods of the trait

impl DeleteValidationView {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: BillingAddressView::new_struct(transformInfoInterface.clone()),
            value: String::new(),
        };
        // super(...) is in the struct literal above
        let mut httpTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoHttpInterface>>> = (this.getTransformInfoInterface()).clone() /* as TransformInfoHttpInterface */;
        this.value = jcall!(jcall!(jcall!(httpTransformInfoInterface).getPageContext()).getRequest()).getParameter(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_ID.with(Clone::clone).clone());
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        if jcall!(self.getWeblisketSession()).getUserName().is_some() {
            return false;
        }
        let mut billingAddressesEntity: Option<Rc<RefCell<BillingAddressesEntity>>> = BillingAddressesEntity::new(jcall!(self.getWeblisketSession()).getUserName());
        self.streetAddress = jcall!(billingAddressesEntity).get1(Some((self.value.clone()) as i32));
        if self.streetAddress.is_none() {
            return false;
        }
        return jcall!(self.streetAddress).isValid();
    }
    

    pub fn validationInfo(&mut self) -> String {
        return jcall!(self.streetAddress).validationInfo();
    }
    

    pub fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    pub fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return None;
    }
}
impl std::ops::Deref for DeleteValidationView {
    type Target = BillingAddressView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for DeleteValidationView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::views::generic::address::billing::BillingAddressView::BillingAddressViewDyn for DeleteValidationView {
    fn as_BillingAddressView(&self) -> &crate::views::generic::address::billing::BillingAddressView::BillingAddressView { &self.base }
    fn as_BillingAddressView_mut(&mut self) -> &mut crate::views::generic::address::billing::BillingAddressView::BillingAddressView { &mut self.base }
}
impl crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn for DeleteValidationView {
    fn as_HttpStoreComponentView(&self) -> &crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &self.base.base }
    fn as_HttpStoreComponentView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &mut self.base.base }
}
impl crate::views::HttpComponentView::HttpComponentViewDyn for DeleteValidationView {
    fn as_HttpComponentView(&self) -> &crate::views::HttpComponentView::HttpComponentView { &self.base.base.base }
    fn as_HttpComponentView_mut(&mut self) -> &mut crate::views::HttpComponentView::HttpComponentView { &mut self.base.base.base }
    fn view(&mut self) -> String { crate::views::generic::address::billing::BillingAddressView::BillingAddressView::view(crate::views::generic::address::billing::BillingAddressView::BillingAddressViewDyn::as_BillingAddressView_mut(self)) }
}


