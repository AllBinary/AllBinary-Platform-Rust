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
// package views.admin.payment.gateways
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::data::tables::user::commerce::money::payment::gateway::PaymentGatewayEntityFactory::PaymentGatewayEntityFactory;
use crate::org::allbinary::data::tables::user::commerce::money::payment::gateway::PaymentGatewayEntityInterface::PaymentGatewayEntityInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::views::admin::payment::gateway::PaymentGatewayViewAbstract::PaymentGatewayViewAbstract;
use super::PaymentGatewaysView::PaymentGatewaysView;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SelectExistingView {
    pub base: PaymentGatewayViewAbstract,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl SelectExistingView {
    

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
    

    // throws Exception
    pub fn view(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut paymentGatewayEntityInterface: Option<Rc<RefCell<dyn PaymentGatewayEntityInterface>>> = PaymentGatewayEntityFactory::getInstance().map(|reference| reference as Rc<RefCell<dyn PaymentGatewayEntityInterface>>);
            let mut existingGateways: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(paymentGatewayEntityInterface).findPaymentTypeVectorByStore(jcall!(self.getPaymentGatewayPrimaryKey()).getStoreName());
            return jcall!(PaymentGatewaysView::new(self.getTransformInfoInterface(), existingGateways.clone())).view();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("view"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
}
impl std::ops::Deref for SelectExistingView {
    type Target = PaymentGatewayViewAbstract;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SelectExistingView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::views::admin::payment::gateway::PaymentGatewayViewAbstract::PaymentGatewayViewAbstractDyn for SelectExistingView {
    fn as_PaymentGatewayViewAbstract(&self) -> &crate::views::admin::payment::gateway::PaymentGatewayViewAbstract::PaymentGatewayViewAbstract { &self.base }
    fn as_PaymentGatewayViewAbstract_mut(&mut self) -> &mut crate::views::admin::payment::gateway::PaymentGatewayViewAbstract::PaymentGatewayViewAbstract { &mut self.base }
}
impl crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpCompositeDyn for SelectExistingView {
    fn as_TransformInfoHttpComposite(&self) -> &crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &self.base.base }
    fn as_TransformInfoHttpComposite_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &mut self.base.base }
}


