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
// package views.business.context
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::visual::transform::TransformFactory::TransformFactory;
use crate::org::allbinary::logic::visual::transform::TransformInterface::TransformInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfo;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::logic::visual::transform::template::TransformTemplateInterface::TransformTemplateInterface;
use crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView;
use crate::views::business::context::modules::storefront::customizer::template::objectConfig::NoTemplateTransformInfoObjectConfig::NoTemplateTransformInfoObjectConfig;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RootContextView {
    pub base: HttpStoreComponentView,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

impl TransformTemplateInterface for RootContextView {
    fn getName(&mut self) -> String { Self::getName(self) }
}

impl RootContextView {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: HttpStoreComponentView::new_struct(transformInfoInterface.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("View Name: "), jcall!(transformInfoInterface).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("RootContextView()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    // throws Exception
    pub fn view(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("View Name: "), jcall!(self.getTransformInfoInterface()).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("view()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            let mut viewName: String = self.getName();
            let mut componentInterface: Option<Rc<RefCell<dyn TransformInterface>>> = jcall!(TransformFactory::getInstance()).getInstance3(self.abeClientInformation.clone(), viewName.clone(), self.getTransformInfoInterface());
            return jcall!(componentInterface).view();
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
    

    // throws Exception
    pub fn getName(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut objectConfig: Option<Rc<RefCell<dyn crate::views::business::context::modules::storefront::customizer::template::objectConfig::NoTemplateTransformInfoObjectConfig::NoTemplateTransformInfoObjectConfigDyn>>> = NoTemplateTransformInfoObjectConfig::new2(self.getTransformInfoInterface(), jcall!(jcall!(self.getTransformInfoInterface()).getObjectConfigInterface()).toXmlDoc()).map(|reference| reference as Rc<RefCell<dyn crate::views::business::context::modules::storefront::customizer::template::objectConfig::NoTemplateTransformInfoObjectConfig::NoTemplateTransformInfoObjectConfigDyn>>);
            let mut componentsVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(objectConfig).getTransforms();
            if jcall!(componentsVector).size() == 0 {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Template View Not Set - No Components"))));
            }
            if jcall!(componentsVector).size() > 1 {
                panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("To Many Root Template Views: "), jcall!(componentsVector).size()))));
            }
            let mut transformInfoInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfoDyn>>> = { let java_object = (jcall!(componentsVector).get(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfoDyn>>>>().expect("ClassCastException").clone()) };
            let mut viewName: String = jcall!(transformInfoInterface).getName();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("Root Template With View Name: "), viewName), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getName()"));
            }
            if jcall!(StringValidationUtil::getInstance()).isEmpty(viewName.clone()) {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Template View Not Set - No View Name"))));
            }
            return viewName.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getName()"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
}
impl std::ops::Deref for RootContextView {
    type Target = HttpStoreComponentView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RootContextView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::views::HttpComponentView::HttpComponentViewDyn for RootContextView {
    fn as_HttpComponentView(&self) -> &crate::views::HttpComponentView::HttpComponentView { &self.base.base }
    fn as_HttpComponentView_mut(&mut self) -> &mut crate::views::HttpComponentView::HttpComponentView { &mut self.base.base }
    fn view(&mut self) -> String { Self::view(self) }
}
impl crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpCompositeDyn for RootContextView {
    fn as_TransformInfoHttpComposite(&self) -> &crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &self.base.base.base }
    fn as_TransformInfoHttpComposite_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &mut self.base.base.base }
}
impl crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn for RootContextView {
    fn as_HttpStoreComponentView(&self) -> &crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &self.base }
    fn as_HttpStoreComponentView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &mut self.base }
}


