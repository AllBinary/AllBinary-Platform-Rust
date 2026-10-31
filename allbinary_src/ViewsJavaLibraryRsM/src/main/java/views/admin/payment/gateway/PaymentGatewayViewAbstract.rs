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
use crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData;
use crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayPrimaryKey::PaymentGatewayPrimaryKey;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PaymentGatewayViewAbstract {
    pub base: TransformInfoHttpComposite,
    pub paymentGatewayPrimaryKey: Option<Rc<RefCell<PaymentGatewayPrimaryKey>>>,
}


impl PaymentGatewayViewAbstract {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: TransformInfoHttpComposite::new_struct(transformInfoInterface.clone()),
            paymentGatewayPrimaryKey: None,
        };
        // super(...) is in the struct literal above
        { let java_arguments = ((jcall!(this.getPageContext()).getRequest()).clone() /* as HttpServletRequest */,); this.getFormData(java_arguments.0) };
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    pub fn getFormData(&mut self, mut request: Option<Rc<RefCell<HttpServletRequest>>>) {
        let mut storeName: String = jcall!(self.getWeblisketSession()).getStoreName();
        let mut gatewayName: String = jcall!(request).getParameter(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_NAME.with(Clone::clone)).toString());
        self.setPaymentGatewayPrimaryKey(PaymentGatewayPrimaryKey::new(storeName.clone(), gatewayName.clone()));
    }
    

    pub fn getPaymentGatewayPrimaryKey(&mut self) -> Option<Rc<RefCell<PaymentGatewayPrimaryKey>>> {
        return self.paymentGatewayPrimaryKey.clone();
    }
    

    pub fn setPaymentGatewayPrimaryKey(&mut self, mut paymentGatewayPrimaryKey: Option<Rc<RefCell<PaymentGatewayPrimaryKey>>>) {
        self.paymentGatewayPrimaryKey = paymentGatewayPrimaryKey.clone();
    }
}
impl std::ops::Deref for PaymentGatewayViewAbstract {
    type Target = TransformInfoHttpComposite;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PaymentGatewayViewAbstract {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait PaymentGatewayViewAbstractDyn: crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpCompositeDyn {
    fn as_PaymentGatewayViewAbstract(&self) -> &PaymentGatewayViewAbstract;
    fn as_PaymentGatewayViewAbstract_mut(&mut self) -> &mut PaymentGatewayViewAbstract;
    fn getFormData(&mut self, mut request: Option<Rc<RefCell<HttpServletRequest>>>) { PaymentGatewayViewAbstract::getFormData(self.as_PaymentGatewayViewAbstract_mut(), request) }
    fn getPaymentGatewayPrimaryKey(&mut self) -> Option<Rc<RefCell<PaymentGatewayPrimaryKey>>> { PaymentGatewayViewAbstract::getPaymentGatewayPrimaryKey(self.as_PaymentGatewayViewAbstract_mut()) }
    fn setPaymentGatewayPrimaryKey(&mut self, mut paymentGatewayPrimaryKey: Option<Rc<RefCell<PaymentGatewayPrimaryKey>>>) { PaymentGatewayViewAbstract::setPaymentGatewayPrimaryKey(self.as_PaymentGatewayViewAbstract_mut(), paymentGatewayPrimaryKey) }
}
impl PaymentGatewayViewAbstractDyn for PaymentGatewayViewAbstract {
    fn as_PaymentGatewayViewAbstract(&self) -> &PaymentGatewayViewAbstract { self }
    fn as_PaymentGatewayViewAbstract_mut(&mut self) -> &mut PaymentGatewayViewAbstract { self }
}

impl crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpCompositeDyn for PaymentGatewayViewAbstract {
    fn as_TransformInfoHttpComposite(&self) -> &crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &self.base }
    fn as_TransformInfoHttpComposite_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &mut self.base }
}


