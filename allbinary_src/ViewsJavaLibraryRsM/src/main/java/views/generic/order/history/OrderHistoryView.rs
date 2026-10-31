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
// package views.generic.order.history
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderHistory::OrderHistory;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData;
use crate::org::allbinary::data::tables::user::commerce::inventory::order::OrderHistoryEntity::OrderHistoryEntity;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::globals::GLOBALS2::GLOBALS2;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OrderHistoryView {
    pub base: HttpStoreComponentView,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub request: Option<Rc<RefCell<HttpServletRequest>>>,
    pub shipped: String,
    pub partiallyShipped: String,
    pub processing: String,
    pub preprocessing: String,
    pub cancelled: String,
}

impl DomNodeInterface for OrderHistoryView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl OrderHistoryView {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: HttpStoreComponentView::new_struct(transformInfoInterface.clone()),
            logUtil: None,
            request: None,
            shipped: String::new(),
            partiallyShipped: String::new(),
            processing: String::new(),
            preprocessing: String::new(),
            cancelled: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this.request = (jcall!(this.getPageContext()).getRequest()).clone() /* as HttpServletRequest */;
        this.preprocessing = jcall!(this.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PREPROCESSINGNAME.with(Clone::clone).clone());
        this.shipped = jcall!(this.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SHIPPEDNAME.with(Clone::clone).clone());
        this.partiallyShipped = jcall!(this.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PARTIALLYSHIPPEDNAME.with(Clone::clone).clone());
        this.processing = jcall!(this.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PROCESSINGNAME.with(Clone::clone).clone());
        this.cancelled = jcall!(this.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_CANCELLEDNAME.with(Clone::clone).clone());
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_ORDERS.with(Clone::clone).clone());
            let mut orderHistoryEntity: Option<Rc<RefCell<OrderHistoryEntity>>> = OrderHistoryEntity::new();
            let mut orderReviewVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(orderHistoryEntity).getOrders(jcall!(self.getWeblisketSession()).getUserName());
            let mut size: i32 = jcall!(orderReviewVector).size();
            for index in (0..size) {
                let mut orderHistory: Option<Rc<RefCell<OrderHistory>>> = { let java_object = (jcall!(orderReviewVector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<OrderHistory>>>>().expect("ClassCastException").clone()) };
                let mut orderHistoryNode: Option<Rc<RefCell<Node>>> = jcall!(orderHistory).toXmlNode(document.clone());
                let mut orderNode: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(jcall!(orderHistory).getPaymentMethod());
                jcall!(node).appendChild(jcall!(orderHistory).toXmlNode(document.clone()));
            }
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                jcall!(self.logUtil).putF(String::from("Attempt to View a users order history"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("view"));
            }
            jcall!(node).appendChild(ModDomHelper::createNameValueNodes(document.clone(), crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PREPROCESSINGNAME.with(Clone::clone).clone(), crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PREPROCESSING.with(Clone::clone).clone()));
            jcall!(node).appendChild(ModDomHelper::createNameValueNodes(document.clone(), crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PROCESSINGNAME.with(Clone::clone).clone(), crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PROCESSING.with(Clone::clone).clone()));
            jcall!(node).appendChild(ModDomHelper::createNameValueNodes(document.clone(), crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_CANCELLEDNAME.with(Clone::clone).clone(), crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_CANCELLED.with(Clone::clone).clone()));
            jcall!(node).appendChild(ModDomHelper::createNameValueNodes(document.clone(), crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PARTIALLYSHIPPEDNAME.with(Clone::clone).clone(), crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PARTIALLYSHIPPED.with(Clone::clone).clone()));
            jcall!(node).appendChild(ModDomHelper::createNameValueNodes(document.clone(), crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SHIPPEDNAME.with(Clone::clone).clone(), crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SHIPPED.with(Clone::clone).clone()));
            jcall!(node).appendChild(ModDomHelper::createNameValueNodes(document.clone(), crate::org::allbinary::globals::GLOBALS2::GLOBALS2_VIEWNAME.with(Clone::clone).clone(), crate::org::allbinary::globals::GLOBALS2::GLOBALS2_VIEW.with(Clone::clone).clone()));
            return node.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).XSLLOGGINGERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toXmlNode"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
    

    pub fn addDomNodeInterfaces(&mut self) {
        self.addDomNodeInterface((self).clone() /* as DomNodeInterface */);
    }
    

    // throws Exception
    pub fn view(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.addDomNodeInterfaces();
            return self.base.view();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPERERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("view()"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
}
impl std::ops::Deref for OrderHistoryView {
    type Target = HttpStoreComponentView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for OrderHistoryView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait OrderHistoryViewDyn: crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn {
    fn as_OrderHistoryView(&self) -> &OrderHistoryView;
    fn as_OrderHistoryView_mut(&mut self) -> &mut OrderHistoryView;
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { OrderHistoryView::toXmlNode(self.as_OrderHistoryView_mut(), document) }
    fn addDomNodeInterfaces(&mut self) { OrderHistoryView::addDomNodeInterfaces(self.as_OrderHistoryView_mut()) }
}
impl OrderHistoryViewDyn for OrderHistoryView {
    fn as_OrderHistoryView(&self) -> &OrderHistoryView { self }
    fn as_OrderHistoryView_mut(&mut self) -> &mut OrderHistoryView { self }
}

impl crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn for OrderHistoryView {
    fn as_HttpStoreComponentView(&self) -> &crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &self.base }
    fn as_HttpStoreComponentView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &mut self.base }
}
impl crate::views::HttpComponentView::HttpComponentViewDyn for OrderHistoryView {
    fn as_HttpComponentView(&self) -> &crate::views::HttpComponentView::HttpComponentView { &self.base.base }
    fn as_HttpComponentView_mut(&mut self) -> &mut crate::views::HttpComponentView::HttpComponentView { &mut self.base.base }
    fn view(&mut self) -> String { Self::view(self) }
}


