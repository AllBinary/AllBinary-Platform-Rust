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
use crate::org::allbinary::data::tree::dom::document::XmlDocumentHelper::XmlDocumentHelper;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::visual::transform::TransformFactory::TransformFactory;
use crate::org::allbinary::logic::visual::transform::TransformInterface::TransformInterface;
use crate::org::allbinary::logic::visual::transform::info::GeneratorTransformInfoData::GeneratorTransformInfoData;
use crate::org::allbinary::logic::visual::transform::info::RootTransformInfoData::RootTransformInfoData;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoHttp::TransformInfoHttp;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfigAndManipulatorFactory::TransformInfoObjectConfigAndManipulatorFactory;
use crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfigInterface::TransformInfoObjectConfigInterface;
use crate::org::allbinary::logic::visual::transform::template::TransformTemplateInterface::TransformTemplateInterface;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::w3c::dom::Document::Document;
use crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OverrideObjectConfigRootContextView {
    pub base: HttpStoreComponentView,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

impl crate::org::allbinary::logic::visual::transform::info::CompositeTransformInfoInterface::CompositeTransformInfoInterface for OverrideObjectConfigRootContextView {
    fn getTransformInfoInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface>>> { crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite::getTransformInfoInterface(crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpCompositeDyn::as_TransformInfoHttpComposite_mut(self)) }
}

impl OverrideObjectConfigRootContextView {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: HttpStoreComponentView::new_struct(transformInfoInterface.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("View Name: "), jcall!(transformInfoInterface).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("OverrideObjectRootContextView()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    // throws Exception
    pub fn view(&mut self) -> String {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("View Name: "), { let java_value = jcall!(self.getTransformInfoInterface()).getName(); java_value }), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("view()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            let mut rootTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>> = (TransformInfoHttp::new1((|| -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfoHttp::TransformInfoHttpDyn>>> { panic!("ClassCastException: a Rust dyn TransformInfoInterface cannot be cast to TransformInfoHttp") })()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfoHttp::TransformInfoHttpDyn>>)).clone() /* as TransformInfoInterface */;
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append({ let java_argument = jcall!(self.getTransformInfoInterface()).getStoreName(); java_argument });
            jcall!(stringBuffer).append({ let java_field = jcall!(CommonSeps::getInstance()).SPACE.clone(); java_field }.clone());
            let mut nameStart: String = jcall!(stringBuffer).toString();
            if { let java_value = jcall!(self.getTransformInfoInterface()).getName().find(&crate::org::allbinary::logic::visual::transform::info::GeneratorTransformInfoData::GeneratorTransformInfoData_NAME.with(Clone::clone).clone()).map(|index| index as i32).unwrap_or(-1); java_value } > 0 {
                jcall!(rootTransformInfoInterface).setName(format!("{}{}", nameStart, crate::org::allbinary::logic::visual::transform::info::GeneratorTransformInfoData::GeneratorTransformInfoData_NAME.with(Clone::clone)));
            } else if { let java_value = jcall!(self.getTransformInfoInterface()).getName().find(&crate::org::allbinary::logic::visual::transform::info::RootTransformInfoData::RootTransformInfoData_NAME.with(Clone::clone).clone()).map(|index| index as i32).unwrap_or(-1); java_value } > 0 {
                jcall!(rootTransformInfoInterface).setName(format!("{}{}", nameStart, crate::org::allbinary::logic::visual::transform::info::RootTransformInfoData::RootTransformInfoData_NAME.with(Clone::clone)));
            } else {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Override Currently Not Supported"))));
            }
            let mut overrideObjectConfigDocument: Option<Rc<RefCell<Document>>> = jcall!(jcall!(self.getTransformInfoInterface()).getObjectConfigInterface()).toXmlDoc();
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Retrieved OverrideObjectConfig: "), XmlDocumentHelper::toString(overrideObjectConfigDocument.clone())), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("view()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            let mut transformFactory: Option<Rc<RefCell<TransformFactory>>> = TransformFactory::getInstance();
            let mut rootComponentInterface: Option<Rc<RefCell<dyn TransformInterface>>> = jcall!(transformFactory).getInstance3(self.abeClientInformation.clone(), jcall!(rootTransformInfoInterface).getName(), rootTransformInfoInterface.clone());
            let mut transformTemplateInterface: Option<Rc<RefCell<dyn TransformTemplateInterface>>> = (|| -> Option<Rc<RefCell<dyn TransformTemplateInterface>>> { panic!("ClassCastException: a Rust dyn TransformInterface cannot be cast to dyn TransformTemplateInterface") })();
            let mut componentInterface: Option<Rc<RefCell<dyn TransformInterface>>> = jcall!(transformFactory).getInstance3(self.abeClientInformation.clone(), jcall!(transformTemplateInterface).getName(), jcall!(rootComponentInterface).getTransformInfoInterface());
            let mut transformInfoObjectConfigInterface: Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>> = jcall!(TransformInfoObjectConfigAndManipulatorFactory::getInstance()).getInstance3_AbeClientInformationInterface_TransformInfoInterface_Document(self.abeClientInformation.clone(), jcall!(componentInterface).getTransformInfoInterface(), overrideObjectConfigDocument.clone());
            jcall!(jcall!(componentInterface).getTransformInfoInterface()).setObjectConfigInterface(transformInfoObjectConfigInterface.clone());
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Set OverrideObjectConfig: "), jcall!(jcall!(jcall!(componentInterface).getTransformInfoInterface()).getObjectConfigInterface()).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("view()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            return Some(jcall!(componentInterface).view());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).TAGHELPERERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("view()"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
}
impl std::ops::Deref for OverrideObjectConfigRootContextView {
    type Target = HttpStoreComponentView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for OverrideObjectConfigRootContextView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpCompositeDyn for OverrideObjectConfigRootContextView {
    fn as_TransformInfoHttpComposite(&self) -> &crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &self.base.base.base }
    fn as_TransformInfoHttpComposite_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &mut self.base.base.base }
}
impl crate::views::HttpComponentView::HttpComponentViewDyn for OverrideObjectConfigRootContextView {
    fn as_HttpComponentView(&self) -> &crate::views::HttpComponentView::HttpComponentView { &self.base.base }
    fn as_HttpComponentView_mut(&mut self) -> &mut crate::views::HttpComponentView::HttpComponentView { &mut self.base.base }
    fn view(&mut self) -> String { Self::view(self) }
}
impl crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn for OverrideObjectConfigRootContextView {
    fn as_HttpStoreComponentView(&self) -> &crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &self.base }
    fn as_HttpStoreComponentView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &mut self.base }
}


