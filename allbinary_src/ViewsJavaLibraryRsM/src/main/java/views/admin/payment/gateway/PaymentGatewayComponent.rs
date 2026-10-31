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
// package views.admin.payment.gateway
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayInterface::PaymentGatewayInterface;
use crate::org::allbinary::business::user::commerce::money::payment::gateway::modules::gateway::PaymentGatewayDomNodeFactoryInterface::PaymentGatewayDomNodeFactoryInterface;
use crate::org::allbinary::business::user::commerce::money::payment::types::BasicPaymentType::BasicPaymentType;
use crate::org::allbinary::business::user::commerce::money::payment::types::BasicPaymentTypeUtil::BasicPaymentTypeUtil;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PaymentGatewayComponent {
    pub base: HttpStoreComponentView,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub paymentGatewayInterface: Option<Rc<RefCell<dyn PaymentGatewayInterface>>>,
}


impl PaymentGatewayComponent {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut paymentGatewayInterface: Option<Rc<RefCell<dyn PaymentGatewayInterface>>>) -> Self {
        let mut this = Self {
            base: HttpStoreComponentView::new_struct(transformInfoInterface.clone()),
            logUtil: None,
            paymentGatewayInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this.paymentGatewayInterface = paymentGatewayInterface.clone();
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut paymentGatewayInterface: Option<Rc<RefCell<dyn PaymentGatewayInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface, paymentGatewayInterface))))
    }
    

    // throws Exception
    pub fn addDomNodeInterfaces(&mut self) {
        let mut paymentType: Option<Rc<RefCell<BasicPaymentType>>> = jcall!(BasicPaymentTypeUtil::getInstance()).get(jcall!(self.paymentGatewayInterface).getName());
        let mut paymentGatewayDomNodeFactoryInterface: Option<Rc<RefCell<dyn PaymentGatewayDomNodeFactoryInterface>>> = jcall!(paymentType).getPaymentGatewayDomNodeFactoryInterface();
        let mut domNodeInterface: Option<Rc<RefCell<dyn DomNodeInterface>>> = (jcall!(paymentGatewayDomNodeFactoryInterface).getInstance(self.paymentGatewayInterface.clone())).clone() /* as DomNodeInterface */;
        self.addDomNodeInterface(domNodeInterface.clone());
    }
    

    pub fn view(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.addDomNodeInterfaces();
            return self.base.view();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to view payment gateway");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPERERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("view()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
}
impl std::ops::Deref for PaymentGatewayComponent {
    type Target = HttpStoreComponentView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PaymentGatewayComponent {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn for PaymentGatewayComponent {
    fn as_HttpStoreComponentView(&self) -> &crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &self.base }
    fn as_HttpStoreComponentView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &mut self.base }
}
impl crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpCompositeDyn for PaymentGatewayComponent {
    fn as_TransformInfoHttpComposite(&self) -> &crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &self.base.base.base }
    fn as_TransformInfoHttpComposite_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &mut self.base.base.base }
}
impl crate::views::HttpComponentView::HttpComponentViewDyn for PaymentGatewayComponent {
    fn as_HttpComponentView(&self) -> &crate::views::HttpComponentView::HttpComponentView { &self.base.base }
    fn as_HttpComponentView_mut(&mut self) -> &mut crate::views::HttpComponentView::HttpComponentView { &mut self.base.base }
    fn view(&mut self) -> String { Self::view(self) }
}


