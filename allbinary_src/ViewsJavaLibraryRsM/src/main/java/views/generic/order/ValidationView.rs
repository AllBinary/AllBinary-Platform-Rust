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
// package views.generic.order
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryFactory::OrderHistoryFactory;
use crate::org::allbinary::data::tables::user::commerce::inventory::order::OrderItemsEntity::OrderItemsEntity;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::validate::ValidationComponentInterface::ValidationComponentInterface;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::OrderView::OrderView;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ValidationView {
    pub base: OrderView,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

// impl ValidationComponentInterface for ValidationView with the methods of the trait

impl ValidationView {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: OrderView::new_struct(transformInfoInterface.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut valid: Option<bool> = true;
            if match (&jcall!(self.order).isIdValid(), &false) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                return false;
            } else {
                self.orderHistory = OrderHistoryFactory::getInstance(jcall!(self.order).getId());
                if self.orderHistory.is_none() || match (&jcall!(self.orderHistory).isValid(), &false) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    return false;
                }
                let mut orderItems: Option<Rc<RefCell<OrderItemsEntity>>> = OrderItemsEntity::new();
                self.basketReview = jcall!(orderItems).getBasketReview(jcall!(self.order).getId());
                if self.basketReview.is_none() || match (&jcall!(self.basketReview).isValid(), &false) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    return false;
                }
            }
            return valid.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to validate form"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
    

    pub fn validationInfo(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            if match (&jcall!(self.order).isIdValid(), &false) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(stringBuffer).append(jcall!(self.order).getIdValidationInfo());
            } else {
                self.orderHistory = OrderHistoryFactory::getInstance(jcall!(self.order).getId());
                if self.orderHistory.is_none() || match (&jcall!(self.orderHistory).isValid(), &false) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    jcall!(stringBuffer).append(String::from("Order History data error for: "));
                    jcall!(stringBuffer).append(jcall!(self.order).getId());
                    jcall!(stringBuffer).append(String::from("<br />"));
                }
                let mut orderItems: Option<Rc<RefCell<OrderItemsEntity>>> = OrderItemsEntity::new();
                self.basketReview = jcall!(orderItems).getBasketReview(jcall!(self.order).getId());
                if self.basketReview.is_none() {
                    jcall!(stringBuffer).append(String::from("Order Items data error - does not exist for: "));
                    jcall!(stringBuffer).append(jcall!(self.order).getId());
                    jcall!(stringBuffer).append(String::from("<br />"));
                } else if match (&jcall!(self.basketReview).isValid(), &false) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    jcall!(stringBuffer).append(String::from("Order Items data error - not valid for: "));
                    jcall!(stringBuffer).append(jcall!(self.order).getId());
                    jcall!(stringBuffer).append(String::from("<br />"));
                }
            }
            return jcall!(stringBuffer).toString();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to generate validation error info"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("validationInfo()"), crate::java_into_object(e.clone()));
            }
            return String::from("Error Validating Form");
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
impl std::ops::Deref for ValidationView {
    type Target = OrderView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ValidationView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::views::generic::order::OrderView::OrderViewDyn for ValidationView {
    fn as_OrderView(&self) -> &crate::views::generic::order::OrderView::OrderView { &self.base }
    fn as_OrderView_mut(&mut self) -> &mut crate::views::generic::order::OrderView::OrderView { &mut self.base }
}
impl crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn for ValidationView {
    fn as_HttpStoreComponentView(&self) -> &crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &self.base.base }
    fn as_HttpStoreComponentView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &mut self.base.base }
}
impl crate::views::HttpComponentView::HttpComponentViewDyn for ValidationView {
    fn as_HttpComponentView(&self) -> &crate::views::HttpComponentView::HttpComponentView { &self.base.base.base }
    fn as_HttpComponentView_mut(&mut self) -> &mut crate::views::HttpComponentView::HttpComponentView { &mut self.base.base.base }
    fn view(&mut self) -> String { crate::views::generic::order::OrderView::OrderView::view(crate::views::generic::order::OrderView::OrderViewDyn::as_OrderView_mut(self)) }
}


