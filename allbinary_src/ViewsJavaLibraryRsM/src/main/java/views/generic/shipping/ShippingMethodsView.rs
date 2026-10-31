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
// package views.generic.shipping
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontFactory::StoreFrontFactory;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::user::address::StreetAddress::StreetAddress;
use crate::org::allbinary::business::user::commerce::inventory::basket::BasketInterface::BasketInterface;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderInterface::OrderInterface;
use crate::org::allbinary::business::user::commerce::money::Money::Money;
use crate::org::allbinary::business::user::commerce::money::tax::TaxFactory::TaxFactory;
use crate::org::allbinary::business::user::commerce::shipping::ShippingMethods::ShippingMethods;
use crate::org::allbinary::business::user::commerce::shipping::ShippingMethodsData::ShippingMethodsData;
use crate::org::allbinary::business::user::commerce::shipping::modules::ShippingInterface::ShippingInterface;
use crate::org::allbinary::data::tables::user::address::StreetAddressesEntityInterface::StreetAddressesEntityInterface;
use crate::org::allbinary::data::tables::user::address::shipping::ShippingAddressesEntityFactory::ShippingAddressesEntityFactory;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
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

pub struct ShippingMethodsView {
    pub base: HttpStoreComponentView,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub shippingMethods: Option<Rc<RefCell<ShippingMethods>>>,
    pub storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>,
}

impl DomNodeInterface for ShippingMethodsView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl ShippingMethodsView {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: HttpStoreComponentView::new_struct(transformInfoInterface.clone()),
            logUtil: None,
            shippingMethods: None,
            storeFrontInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this.storeFrontInterface = StoreFrontFactory::getInstance(jcall!(this.getTransformInfoInterface()).getStoreName());
        this.shippingMethods = ShippingMethods::new(this.abeClientInformation.clone(), this.storeFrontInterface.clone());
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut shippingMethodsNode: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(crate::org::allbinary::business::user::commerce::shipping::ShippingMethodsData::ShippingMethodsData_SHIPPINGMETHODSORDERSUMMARIES.with(Clone::clone).clone());
            let mut order: Option<Rc<RefCell<dyn OrderInterface>>> = jcall!(self.getWeblisketSession()).getOrder();
            let mut basket: Option<Rc<RefCell<dyn BasketInterface>>> = jcall!(jcall!(self.getWeblisketSession()).getOrder()).getBasket();
            let mut shippingAddressesEntityInterface: Option<Rc<RefCell<dyn StreetAddressesEntityInterface>>> = jcall!(ShippingAddressesEntityFactory::getInstance()).getInstance1(jcall!(self.getWeblisketSession()).getUserName());
            let mut streetAddress: Option<Rc<RefCell<StreetAddress>>> = jcall!(shippingAddressesEntityInterface).getDefault();
            let mut shippingVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(self.shippingMethods).get();
            let mut size: i32 = jcall!(shippingVector).size();
            for index in (0..size) {
                let mut shippingMethodNode: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(crate::org::allbinary::business::user::commerce::shipping::ShippingMethodsData::ShippingMethodsData_SHIPPINGMETHODORDERSUMMARY.with(Clone::clone).clone());
                let mut shipping: Option<Rc<RefCell<dyn ShippingInterface>>> = { let java_object = jcall!(shippingVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn ShippingInterface>>>>().expect("ClassCastException").clone()) };
                let mut shippingView: Option<Rc<RefCell<dyn DomNodeInterface>>> = panic!("ClassCastException: a Rust dyn ShippingInterface cannot be cast to dyn DomNodeInterface");
                jcall!(shippingMethodNode).appendChild(jcall!(shippingView).toXmlNode(document.clone()));
                let mut shippingCost: Option<Rc<RefCell<Money>>> = jcall!(shipping).getCost(order.clone());
                let mut subTotal: Option<Rc<RefCell<Money>>> = jcall!(basket).getSubTotal();
                let mut taxRate: Option<f32> = jcall!(jcall!(TaxFactory::getInstance()).getInstance2(self.abeClientInformation.clone(), self.storeFrontInterface.clone())).getTaxRate(streetAddress.clone(), self.storeFrontInterface.clone());
                let mut tax: Option<Rc<RefCell<Money>>> = Money::new();
                let mut total: Option<Rc<RefCell<Money>>> = Money::new();
                jcall!(total).add(jcall!(shippingCost).toString());
                jcall!(total).add(jcall!(subTotal).toString());
                jcall!(tax).add(jcall!(total).toString());
                jcall!(tax).multiply(taxRate.clone());
                jcall!(total).add(jcall!(tax).toString());
                jcall!(shippingMethodNode).appendChild(ModDomHelper::createNameValueNodes(document.clone(), crate::org::allbinary::business::user::commerce::shipping::ShippingMethodsData::ShippingMethodsData_SHIPPINGCOST.with(Clone::clone).clone(), jcall!(shippingCost).toString()));
                jcall!(shippingMethodNode).appendChild(ModDomHelper::createNameValueNodes(document.clone(), crate::org::allbinary::business::user::commerce::shipping::ShippingMethodsData::ShippingMethodsData_TAXRATE.with(Clone::clone).clone(), (taxRate).expect("NullPointerException").to_string()));
                jcall!(shippingMethodNode).appendChild(ModDomHelper::createNameValueNodes(document.clone(), crate::org::allbinary::business::user::commerce::shipping::ShippingMethodsData::ShippingMethodsData_SUBTOTAL.with(Clone::clone).clone(), jcall!(subTotal).toString()));
                jcall!(shippingMethodNode).appendChild(ModDomHelper::createNameValueNodes(document.clone(), crate::org::allbinary::business::user::commerce::shipping::ShippingMethodsData::ShippingMethodsData_TAX.with(Clone::clone).clone(), jcall!(tax).toString()));
                jcall!(shippingMethodNode).appendChild(ModDomHelper::createNameValueNodes(document.clone(), crate::org::allbinary::business::user::commerce::shipping::ShippingMethodsData::ShippingMethodsData_TOTAL.with(Clone::clone).clone(), jcall!(total).toString()));
                jcall!(shippingMethodNode).appendChild(ModDomHelper::createNameValueNodes(document.clone(), jcall!(EntryData::getInstance()).DEFAULT.clone(), jcall!(jcall!(self.shippingMethods).getDefault()).getName()));
                jcall!(shippingMethodsNode).appendChild(shippingMethodNode.clone());
            }
            return shippingMethodsNode.clone();
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
    

    pub fn view(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.addDomNodeInterfaces();
            return self.base.view();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to view ShippingMethods");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPERERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("view()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
}
impl std::ops::Deref for ShippingMethodsView {
    type Target = HttpStoreComponentView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ShippingMethodsView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ShippingMethodsViewDyn: crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn {
    fn as_ShippingMethodsView(&self) -> &ShippingMethodsView;
    fn as_ShippingMethodsView_mut(&mut self) -> &mut ShippingMethodsView;
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { ShippingMethodsView::toXmlNode(self.as_ShippingMethodsView_mut(), document) }
    fn addDomNodeInterfaces(&mut self) { ShippingMethodsView::addDomNodeInterfaces(self.as_ShippingMethodsView_mut()) }
}
impl ShippingMethodsViewDyn for ShippingMethodsView {
    fn as_ShippingMethodsView(&self) -> &ShippingMethodsView { self }
    fn as_ShippingMethodsView_mut(&mut self) -> &mut ShippingMethodsView { self }
}

impl crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn for ShippingMethodsView {
    fn as_HttpStoreComponentView(&self) -> &crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &self.base }
    fn as_HttpStoreComponentView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &mut self.base }
}
impl crate::views::HttpComponentView::HttpComponentViewDyn for ShippingMethodsView {
    fn as_HttpComponentView(&self) -> &crate::views::HttpComponentView::HttpComponentView { &self.base.base }
    fn as_HttpComponentView_mut(&mut self) -> &mut crate::views::HttpComponentView::HttpComponentView { &mut self.base.base }
    fn view(&mut self) -> String { Self::view(self) }
}


