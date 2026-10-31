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
// package views.business.context.modules.storefront.customizer.generic.page
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::data::tables::transform::info::TransformInfoEntity::TransformInfoEntity;
use crate::org::allbinary::data::tables::transform::info::TransformInfoEntityBuilder::TransformInfoEntityBuilder;
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::data::tree::dom::document::XmlDocumentHelper::XmlDocumentHelper;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::validate::ValidationComponentInterface::ValidationComponentInterface;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfo;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpInterface::TransformInfoHttpInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfigInterface::TransformInfoObjectConfigInterface;
use crate::org::allbinary::logic::visual::transform::template::customizer::page::PageValidation::PageValidation;
use crate::org::allbinary::logic::visual::transform::template::customizer::widgets::title::TitleData::TitleData;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::views::business::context::modules::storefront::customizer::CustomizerUtil::CustomizerUtil;
use super::PageView::PageView;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PageValidationView {
    pub base: PageView,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

// impl ValidationComponentInterface for PageValidationView with the methods of the trait

impl PageValidationView {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: PageView::new_struct(transformInfoInterface.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("View Name: "), jcall!(transformInfoInterface).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("PageViewValidation()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut isValid: Option<bool> = true;
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone());
            }
            let mut transformInfoEntityInterface: Option<Rc<RefCell<TransformInfoEntity>>> = TransformInfoEntityBuilder::getInstance();
            let mut pageObjectConfigInterface: Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>> = jcall!(self.getTransformInfoInterface()).getObjectConfigInterface();
            let mut allViewsToBeModifiedVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(pageObjectConfigInterface).getGroupTransforms();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Views To Be Modified: "), jcall!(allViewsToBeModifiedVector).size()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("get(transformInfoInterface)"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            let mut size: i32 = jcall!(allViewsToBeModifiedVector).size();
            for index in (0..size) {
                let mut transformInfoInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfoDyn>>> = { let java_object = (jcall!(allViewsToBeModifiedVector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfoDyn>>>>().expect("ClassCastException").clone()) };
                let mut viewNameOfViewToBeModified: String = jcall!(transformInfoInterface).getName();
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                    let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                    jcall!(stringBuffer).append(jcall!(self.getTransformInfoInterface()).getName());
                    jcall!(stringBuffer).append(String::from(" is modifying view: "));
                    jcall!(stringBuffer).append(viewNameOfViewToBeModified.clone());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("insert()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                let mut httpTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoHttpInterface>>> = (self.getTransformInfoInterface()).clone() /* as TransformInfoHttpInterface */;
                let mut specifiedTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>> = jcall!(transformInfoEntityInterface).get(viewNameOfViewToBeModified.clone(), jcall!(httpTransformInfoInterface).getPropertiesHashMap(), jcall!(httpTransformInfoInterface).getPageContext());
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                    let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                    jcall!(stringBuffer).append(jcall!(self.getTransformInfoInterface()).getName());
                    jcall!(stringBuffer).append(String::from(" is adding data to view: "));
                    jcall!(stringBuffer).append(viewNameOfViewToBeModified.clone());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("insert()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
                let mut title: String = jcall!(jcall!(specifiedTransformInfoInterface).getName()).substring(jcall!(jcall!(self.getTransformInfoInterface()).getStoreName()).length());
                if (title.cmp(&String::from("index")) as i32) == 0 {
                    jcall!(hashMap).put(jcall!(TitleData::getInstance()).TEXT.clone(), format!("{}{}", jcall!(self.getTransformInfoInterface()).getStoreName(), String::from(" - Home Page")));
                } else {
                    jcall!(hashMap).put(jcall!(TitleData::getInstance()).TEXT.clone(), format!("{}{}", format!("{}{}", jcall!(self.getTransformInfoInterface()).getStoreName(), String::from(" -")), title));
                }
                let mut pageValidation: Option<Rc<RefCell<PageValidation>>> = PageValidation::new1(hashMap.clone());
                if match (&jcall!(pageValidation).isValid(), &false) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    isValid = false;
                }
                if match (&isValid, &true) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    let mut document: Option<Rc<RefCell<Document>>> = DomDocumentHelper::create();
                    jcall!(document).appendChild(jcall!(pageValidation).toXmlNode(document.clone()));
                    let mut documentString: String = XmlDocumentHelper::toString(document.clone());
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                        jcall!(stringBuffer).append(viewNameOfViewToBeModified.clone());
                        jcall!(stringBuffer).append(String::from(" is changing data in "));
                        jcall!(stringBuffer).append(jcall!(jcall!(specifiedTransformInfoInterface).getDataFilePath()).toString());
                        jcall!(stringBuffer).append(String::from(" to the following data:\n"));
                        jcall!(stringBuffer).append(documentString.clone());
                        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    }
                    jcall!(CustomizerUtil::getInstance()).write(specifiedTransformInfoInterface.clone(), documentString.clone());
                }
            }
            return isValid.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to validate"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
    

    pub fn validationInfo(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            let mut transformInfoEntityInterface: Option<Rc<RefCell<TransformInfoEntity>>> = TransformInfoEntityBuilder::getInstance();
            let mut objectConfig: Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>> = jcall!(self.getTransformInfoInterface()).getObjectConfigInterface();
            let mut httpTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoHttpInterface>>> = (self.getTransformInfoInterface()).clone() /* as TransformInfoHttpInterface */;
            let mut allViewsToBeModifiedVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(objectConfig).getGroupTransforms();
            let mut size: i32 = jcall!(allViewsToBeModifiedVector).size();
            for index in (0..size) {
                let mut transformInfoInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfoDyn>>> = { let java_object = (jcall!(allViewsToBeModifiedVector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfoDyn>>>>().expect("ClassCastException").clone()) };
                let mut viewNameOfViewToBeModified: String = jcall!(transformInfoInterface).getName();
                let mut specifiedTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>> = jcall!(transformInfoEntityInterface).get(viewNameOfViewToBeModified.clone(), jcall!(httpTransformInfoInterface).getPropertiesHashMap(), jcall!(httpTransformInfoInterface).getPageContext());
                let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
                let mut title: String = jcall!(jcall!(specifiedTransformInfoInterface).getName()).substring(jcall!(jcall!(self.getTransformInfoInterface()).getStoreName()).length());
                if (title.cmp(&String::from("index")) as i32) == 0 {
                    jcall!(hashMap).put(jcall!(TitleData::getInstance()).TEXT.clone(), format!("{}{}", jcall!(self.getTransformInfoInterface()).getStoreName(), String::from(" - Home Page")));
                } else {
                    jcall!(hashMap).put(jcall!(TitleData::getInstance()).TEXT.clone(), format!("{}{}", format!("{}{}", jcall!(self.getTransformInfoInterface()).getStoreName(), String::from(" -")), title));
                }
                let mut pageValidation: Option<Rc<RefCell<PageValidation>>> = PageValidation::new1(hashMap.clone());
                if match (&jcall!(pageValidation).isValid(), &false) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    jcall!(stringBuffer).append(format!("{}{}", String::from("TransformInfo Name for PageValidation:"), jcall!(specifiedTransformInfoInterface).getName()));
                    jcall!(stringBuffer).append(format!("{}{}", String::from("PageValidation:"), hashMap));
                    jcall!(stringBuffer).append(format!("{}{}", String::from("PageValidation Info:"), jcall!(pageValidation).validationInfo()));
                }
            }
            return jcall!(stringBuffer).toString();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to generate validation error info"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("validationInfo()"), crate::java_into_object(e.clone()));
            }
            return String::from("An auto generated page name was invalid.");
            }
        }
    }
    

    pub fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    pub fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return None;
    }
}
impl std::ops::Deref for PageValidationView {
    type Target = PageView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PageValidationView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::views::business::context::modules::storefront::customizer::generic::page::PageView::PageViewDyn for PageValidationView {
    fn as_PageView(&self) -> &crate::views::business::context::modules::storefront::customizer::generic::page::PageView::PageView { &self.base }
    fn as_PageView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::customizer::generic::page::PageView::PageView { &mut self.base }
}
impl crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpCompositeDyn for PageValidationView {
    fn as_TransformInfoHttpComposite(&self) -> &crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &self.base.base.base.base }
    fn as_TransformInfoHttpComposite_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &mut self.base.base.base.base }
}
impl crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn for PageValidationView {
    fn as_HttpStoreComponentView(&self) -> &crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &self.base.base }
    fn as_HttpStoreComponentView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &mut self.base.base }
}
impl crate::views::HttpComponentView::HttpComponentViewDyn for PageValidationView {
    fn as_HttpComponentView(&self) -> &crate::views::HttpComponentView::HttpComponentView { &self.base.base.base }
    fn as_HttpComponentView_mut(&mut self) -> &mut crate::views::HttpComponentView::HttpComponentView { &mut self.base.base.base }
    fn view(&mut self) -> String { crate::views::business::context::modules::storefront::customizer::generic::page::PageView::PageView::view(crate::views::business::context::modules::storefront::customizer::generic::page::PageView::PageViewDyn::as_PageView_mut(self)) }
}


