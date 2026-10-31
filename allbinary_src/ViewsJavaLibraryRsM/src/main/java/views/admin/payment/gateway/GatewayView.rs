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
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
use crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayInterface::PaymentGatewayInterface;
use crate::org::allbinary::business::user::commerce::money::payment::types::BasicPaymentTypeUtil::BasicPaymentTypeUtil;
use crate::org::allbinary::data::tables::user::commerce::money::payment::gateway::PaymentGatewayEntityFactory::PaymentGatewayEntityFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use super::PaymentGatewayViewAbstract::PaymentGatewayViewAbstract;
use super::PaymentGatewayComponent::PaymentGatewayComponent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GatewayView {
    pub base: PaymentGatewayViewAbstract,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl GatewayView {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: PaymentGatewayViewAbstract::new_struct(transformInfoInterface.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    pub fn getFormData(&mut self, mut request: Option<Rc<RefCell<HttpServletRequest>>>) {
        self.base.getFormData(request.clone());
        if jcall!(StringValidationUtil::getInstance()).isEmpty(jcall!(self.getPaymentGatewayPrimaryKey()).getName()) {
            jcall!(self.getPaymentGatewayPrimaryKey()).setName(jcall!(self.getWeblisketSession()).getPaymentMethod());
        }
    }
    

    // throws Exception
    pub fn view(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut paymentGatewayInterface: Option<Rc<RefCell<dyn PaymentGatewayInterface>>> = jcall!(PaymentGatewayEntityFactory::getInstance()).getPaymentGatewayInterface(jcall!(self.getPaymentGatewayPrimaryKey()).getStoreName(), jcall!(BasicPaymentTypeUtil::getInstance()).get(jcall!(self.getPaymentGatewayPrimaryKey()).getName()));
            return jcall!(PaymentGatewayComponent::new(self.getTransformInfoInterface(), paymentGatewayInterface.clone())).view();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("edit()"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
}
impl std::ops::Deref for GatewayView {
    type Target = PaymentGatewayViewAbstract;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GatewayView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::views::admin::payment::gateway::PaymentGatewayViewAbstract::PaymentGatewayViewAbstractDyn for GatewayView {
    fn as_PaymentGatewayViewAbstract(&self) -> &crate::views::admin::payment::gateway::PaymentGatewayViewAbstract::PaymentGatewayViewAbstract { &self.base }
    fn as_PaymentGatewayViewAbstract_mut(&mut self) -> &mut crate::views::admin::payment::gateway::PaymentGatewayViewAbstract::PaymentGatewayViewAbstract { &mut self.base }
    fn getFormData(&mut self, mut request: Option<Rc<RefCell<HttpServletRequest>>>) { Self::getFormData(self, request) }
}
impl crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpCompositeDyn for GatewayView {
    fn as_TransformInfoHttpComposite(&self) -> &crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &self.base.base }
    fn as_TransformInfoHttpComposite_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &mut self.base.base }
}


