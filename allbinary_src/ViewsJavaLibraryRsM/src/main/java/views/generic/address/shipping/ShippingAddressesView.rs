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
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
use crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData;
use crate::org::allbinary::business::user::address::StreetAddress::StreetAddress;
use crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpInterface::TransformInfoHttpInterface;
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

pub struct ShippingAddressesView {
    pub base: HttpStoreComponentView,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub request: Option<Rc<RefCell<HttpServletRequest>>>,
    pub streetAddressList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

impl DomNodeInterface for ShippingAddressesView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl ShippingAddressesView {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: HttpStoreComponentView::new_struct(transformInfoInterface.clone()),
            logUtil: None,
            request: None,
            streetAddressList: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        let mut httpTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoHttpInterface>>> = (this.getTransformInfoInterface()).clone() /* as TransformInfoHttpInterface */;
        this.request = (jcall!(jcall!(httpTransformInfoInterface).getPageContext()).getRequest()).clone() /* as HttpServletRequest */;
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    pub fn getRequest(&mut self) -> Option<Rc<RefCell<HttpServletRequest>>> {
        return self.request.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut billingAddressesNode: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_MULTIPLE.with(Clone::clone).clone());
            let mut size: i32 = jcall!(self.streetAddressList).size();
            for index in (0..size) {
                let mut streetAddress: Option<Rc<RefCell<StreetAddress>>> = { let java_object = (jcall!(self.streetAddressList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<StreetAddress>>>>().expect("ClassCastException").clone()) };
                jcall!(billingAddressesNode).appendChild(jcall!(streetAddress).toXmlNode(document.clone()));
            }
            jcall!(billingAddressesNode).appendChild(ModDomHelper::createNameValueNodes(document.clone(), crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_NUMBEROFADDRESSES.with(Clone::clone).clone(), jcall!(Some((jcall!(self.streetAddressList).size()) as i32)).toString()));
            return billingAddressesNode.clone();
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
impl std::ops::Deref for ShippingAddressesView {
    type Target = HttpStoreComponentView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ShippingAddressesView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ShippingAddressesViewDyn: crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn {
    fn as_ShippingAddressesView(&self) -> &ShippingAddressesView;
    fn as_ShippingAddressesView_mut(&mut self) -> &mut ShippingAddressesView;
    fn getRequest(&mut self) -> Option<Rc<RefCell<HttpServletRequest>>> { ShippingAddressesView::getRequest(self.as_ShippingAddressesView_mut()) }
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { ShippingAddressesView::toXmlNode(self.as_ShippingAddressesView_mut(), document) }
    fn addDomNodeInterfaces(&mut self) { ShippingAddressesView::addDomNodeInterfaces(self.as_ShippingAddressesView_mut()) }
}
impl ShippingAddressesViewDyn for ShippingAddressesView {
    fn as_ShippingAddressesView(&self) -> &ShippingAddressesView { self }
    fn as_ShippingAddressesView_mut(&mut self) -> &mut ShippingAddressesView { self }
}

impl crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn for ShippingAddressesView {
    fn as_HttpStoreComponentView(&self) -> &crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &self.base }
    fn as_HttpStoreComponentView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &mut self.base }
}
impl crate::views::HttpComponentView::HttpComponentViewDyn for ShippingAddressesView {
    fn as_HttpComponentView(&self) -> &crate::views::HttpComponentView::HttpComponentView { &self.base.base }
    fn as_HttpComponentView_mut(&mut self) -> &mut crate::views::HttpComponentView::HttpComponentView { &mut self.base.base }
    fn view(&mut self) -> String { Self::view(self) }
}


