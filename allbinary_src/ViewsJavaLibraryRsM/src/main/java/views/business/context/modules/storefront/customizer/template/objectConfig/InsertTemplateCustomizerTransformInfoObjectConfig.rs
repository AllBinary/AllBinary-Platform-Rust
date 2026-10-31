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
// package views.business.context.modules.storefront.customizer.template.objectConfig
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoDomNodeView::TransformInfoDomNodeView;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfosData::TransformInfosData;
use crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfigData::TransformInfoObjectConfigData;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;
use super::NoTemplateTransformInfoObjectConfig::NoTemplateTransformInfoObjectConfig;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InsertTemplateCustomizerTransformInfoObjectConfig {
    pub base: NoTemplateTransformInfoObjectConfig,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl InsertTemplateCustomizerTransformInfoObjectConfig {
    

    pub fn new_struct1(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: NoTemplateTransformInfoObjectConfig::new_struct1(transformInfoInterface.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new1(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(transformInfoInterface))))
    }
    

    pub fn new_struct2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut document: Option<Rc<RefCell<Document>>>) -> Self {
        let mut this = Self {
            base: NoTemplateTransformInfoObjectConfig::new_struct2(transformInfoInterface.clone(), document.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        { let java_arguments = ({ let java_arguments = (this.toXmlDoc(),); this.generate(java_arguments.0) },); this.setDocument(java_arguments.0) };
        this
    }

    pub fn new2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(transformInfoInterface, document))))
    }
    

    pub fn new_struct3(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut name: String, mut r#type: String) -> Self {
        let mut this = Self {
            base: NoTemplateTransformInfoObjectConfig::new_struct3(transformInfoInterface.clone(), name.clone(), r#type.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        { let java_arguments = ({ let java_arguments = (this.toXmlDoc(),); this.generate(java_arguments.0) },); this.setDocument(java_arguments.0) };
        this
    }

    pub fn new3(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut name: String, mut r#type: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct3(transformInfoInterface, name, r#type))))
    }
    

    // throws Exception
    pub fn set(&mut self, mut aParentViewName: String) {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("Setting: "), aParentViewName), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("set()"));
        }
        let mut document: Option<Rc<RefCell<Document>>> = self.toXmlDoc();
        let mut parentComponentsNodeList: Option<Rc<RefCell<NodeList>>> = jcall!(document).getElementsByTagName(jcall!(TransformInfosData::getInstance()).NAME.clone());
        if parentComponentsNodeList.is_some() && jcall!(parentComponentsNodeList).getLength() > 0 {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                jcall!(self.logUtil).putF(String::from("Replacing"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("set()"));
            }
            let mut componentNode: Option<Rc<RefCell<Node>>> = jcall!(jcall!(jcall!(parentComponentsNodeList).item(0)).getChildNodes()).item(0);
            if componentNode.is_some() {
                jcall!(document).removeChild(componentNode.clone());
            }
            jcall!(jcall!(parentComponentsNodeList).item(0)).appendChild(jcall!(TransformInfoDomNodeView::new1(aParentViewName.clone())).toXmlNode(document.clone()));
        } else {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                jcall!(self.logUtil).putF(String::from("Setting"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("set()"));
            }
            let mut objectConfigNode: Option<Rc<RefCell<Node>>> = jcall!(jcall!(document).getElementsByTagName(jcall!(TransformInfoObjectConfigData::getInstance()).NAME.clone())).item(0);
            let mut parentComponentsNode: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(jcall!(TransformInfosData::getInstance()).NAME.clone());
            jcall!(objectConfigNode).appendChild(parentComponentsNode.clone());
            let mut transformInfoDomNodeView: Option<Rc<RefCell<TransformInfoDomNodeView>>> = TransformInfoDomNodeView::new1(aParentViewName.clone());
            jcall!(parentComponentsNode).appendChild(jcall!(transformInfoDomNodeView).toXmlNode(document.clone()));
        }
    }
}
impl std::ops::Deref for InsertTemplateCustomizerTransformInfoObjectConfig {
    type Target = NoTemplateTransformInfoObjectConfig;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for InsertTemplateCustomizerTransformInfoObjectConfig {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::views::business::context::modules::storefront::customizer::template::objectConfig::NoTemplateTransformInfoObjectConfig::NoTemplateTransformInfoObjectConfigDyn for InsertTemplateCustomizerTransformInfoObjectConfig {
    fn as_NoTemplateTransformInfoObjectConfig(&self) -> &crate::views::business::context::modules::storefront::customizer::template::objectConfig::NoTemplateTransformInfoObjectConfig::NoTemplateTransformInfoObjectConfig { &self.base }
    fn as_NoTemplateTransformInfoObjectConfig_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::customizer::template::objectConfig::NoTemplateTransformInfoObjectConfig::NoTemplateTransformInfoObjectConfig { &mut self.base }
}
impl crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfig::TransformInfoObjectConfigDyn for InsertTemplateCustomizerTransformInfoObjectConfig {
    fn as_TransformInfoObjectConfig(&self) -> &crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfig::TransformInfoObjectConfig { &self.base.base }
    fn as_TransformInfoObjectConfig_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfig::TransformInfoObjectConfig { &mut self.base.base }
}


