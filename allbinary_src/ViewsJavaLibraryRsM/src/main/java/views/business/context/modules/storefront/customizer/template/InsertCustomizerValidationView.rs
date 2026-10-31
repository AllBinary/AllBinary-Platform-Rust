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
// package views.business.context.modules.storefront.customizer.template
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::data::tables::transform::info::TransformInfoEntityBuilder::TransformInfoEntityBuilder;
use crate::org::allbinary::logic::communication::http::request::RequestParams::RequestParams;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::validate::ValidationComponentInterface::ValidationComponentInterface;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::visual::transform::TransformFactory::TransformFactory;
use crate::org::allbinary::logic::visual::transform::TransformInterface::TransformInterface;
use crate::org::allbinary::logic::visual::transform::info::GeneratorTransformInfoData::GeneratorTransformInfoData;
use crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfo;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoData::TransformInfoData;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfosData::TransformInfosData;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView;
use crate::views::business::context::modules::storefront::customizer::template::objectConfig::InsertTemplateCustomizerTransformInfoObjectConfig::InsertTemplateCustomizerTransformInfoObjectConfig;
use crate::views::business::context::modules::storefront::customizer::template::objectConfig::NoTemplateTransformInfoObjectConfig::NoTemplateTransformInfoObjectConfig;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//Sets Template Type
pub struct InsertCustomizerValidationView {
    pub base: HttpStoreComponentView,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub viewName: String,
}

// impl ValidationComponentInterface for InsertCustomizerValidationView with the methods of the trait

impl InsertCustomizerValidationView {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: HttpStoreComponentView::new_struct(transformInfoInterface.clone()),
            logUtil: None,
            viewName: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("View Name: "), jcall!(transformInfoInterface).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut requestHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(RequestParams::new1(this.getPageContext()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::communication::http::request::RequestParams::RequestParamsDyn>>)).toHashMap();
        this.viewName = (jcall!(requestHashMap).get(jcall!(TransformInfoData::getInstance()).NAME.clone())).clone() /* as String */;
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                jcall!(self.logUtil).putF(String::from("Started Validation"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone());
            }
            let mut isValid: Option<bool> = true;
            if jcall!(StringValidationUtil::getInstance()).isEmpty(self.viewName.clone()) {
                isValid = false;
            } else {
                let mut objectConfig: Option<Rc<RefCell<dyn crate::views::business::context::modules::storefront::customizer::template::objectConfig::NoTemplateTransformInfoObjectConfig::NoTemplateTransformInfoObjectConfigDyn>>> = NoTemplateTransformInfoObjectConfig::new2(self.getTransformInfoInterface(), jcall!(jcall!(self.getTransformInfoInterface()).getObjectConfigInterface()).toXmlDoc()).map(|reference| reference as Rc<RefCell<dyn crate::views::business::context::modules::storefront::customizer::template::objectConfig::NoTemplateTransformInfoObjectConfig::NoTemplateTransformInfoObjectConfigDyn>>);
                let mut componentVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(objectConfig).getGroupTransforms();
                let mut size: i32 = jcall!(componentVector).size();
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                    let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                    jcall!(stringBuffer).append(String::from("Pointing "));
                    jcall!(stringBuffer).appendint(jcall!(componentVector).size());
                    jcall!(stringBuffer).append(String::from(" Components"));
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                for index in (0..size) {
                    let mut transformInfo: Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfoDyn>>> = { let java_object = (jcall!(componentVector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfoDyn>>>>().expect("ClassCastException").clone()) };
                    let mut transformInfoName: String = jcall!(transformInfo).getName();
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                        jcall!(self.logUtil).putF(format!("{}{}", String::from("Setting Template Component with ViewName: "), transformInfoName), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone());
                    }
                    let mut componentInterface: Option<Rc<RefCell<dyn TransformInterface>>> = jcall!(TransformFactory::getInstance()).getInstance3(self.abeClientInformation.clone(), transformInfoName.clone(), self.getTransformInfoInterface());
                    let mut templateViewObjectConfig: Option<Rc<RefCell<InsertTemplateCustomizerTransformInfoObjectConfig>>> = InsertTemplateCustomizerTransformInfoObjectConfig::new2(jcall!(componentInterface).getTransformInfoInterface(), jcall!(jcall!(jcall!(componentInterface).getTransformInfoInterface()).getObjectConfigInterface()).toXmlDoc());
                    let mut storeName: String = jcall!(self.getTransformInfoInterface()).getStoreName();
                    let mut endIndex: i32 = self.viewName.find(&storeName.clone()).map(|index| index as i32).unwrap_or(-1) + (storeName.chars().count() as i32);
                    let mut storePrepend: String = self.viewName[(0) as usize..(endIndex) as usize].to_string();
                    let mut viewNamePostfix: String = self.viewName[(endIndex) as usize..((self.viewName.chars().count() as i32)) as usize].to_string();
                    let mut newViewNameStringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                    jcall!(newViewNameStringBuffer).append(storePrepend.clone());
                    let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
                    let mut transformInfosData: Option<Rc<RefCell<TransformInfosData>>> = TransformInfosData::getInstance();
                    if transformInfoName.find(&jcall!(transformInfosData).SMALL_PREVIEW.clone()).map(|index| index as i32).unwrap_or(-1) >= 0 {
                        jcall!(newViewNameStringBuffer).append(jcall!(commonSeps).SPACE.clone());
                        jcall!(newViewNameStringBuffer).append(jcall!(transformInfosData).SMALL_PREVIEW.clone());
                    } else if transformInfoName.find(&jcall!(transformInfosData).PREVIEW.clone()).map(|index| index as i32).unwrap_or(-1) >= 0 {
                        jcall!(newViewNameStringBuffer).append(jcall!(commonSeps).SPACE.clone());
                        jcall!(newViewNameStringBuffer).append(jcall!(transformInfosData).PREVIEW.clone());
                    }
                    jcall!(newViewNameStringBuffer).append(jcall!(commonSeps).SPACE.clone());
                    jcall!(newViewNameStringBuffer).append(viewNamePostfix.clone());
                    if transformInfoName.find(&crate::org::allbinary::logic::visual::transform::info::GeneratorTransformInfoData::GeneratorTransformInfoData_NAME.with(Clone::clone).clone()).map(|index| index as i32).unwrap_or(-1) >= 0 {
                        jcall!(newViewNameStringBuffer).append(jcall!(commonSeps).SPACE.clone());
                        jcall!(newViewNameStringBuffer).append(crate::org::allbinary::logic::visual::transform::info::GeneratorTransformInfoData::GeneratorTransformInfoData_NAME.with(Clone::clone).clone());
                    }
                    let mut newViewName: String = jcall!(newViewNameStringBuffer).toString();
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                        jcall!(stringBuffer).append(String::from("Template Component: "));
                        jcall!(stringBuffer).append(transformInfoName.clone());
                        jcall!(stringBuffer).append(String::from("\n Now Pointing To View Name: "));
                        jcall!(stringBuffer).append(newViewName.clone());
                        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    }
                    jcall!(templateViewObjectConfig).set(newViewName.clone());
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                        jcall!(stringBuffer).append(String::from("Template Component: "));
                        jcall!(stringBuffer).append(transformInfoName.clone());
                        jcall!(stringBuffer).append(String::from("\n with ObjectConfig: "));
                        jcall!(stringBuffer).append(jcall!(templateViewObjectConfig).toString());
                        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    }
                    jcall!(jcall!(componentInterface).getTransformInfoInterface()).setObjectConfigInterface(templateViewObjectConfig.clone());
                    let mut updatedTransformInfoHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(jcall!(componentInterface).getTransformInfoInterface()).toHashMap();
                    jcall!(TransformInfoEntityBuilder::getInstance()).update(updatedTransformInfoHashMap.clone());
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
            jcall!(stringBuffer).append(String::from("Error: Template Name Is Empty."));
            return jcall!(stringBuffer).toString();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to generate validation error info"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("validationInfo()"), crate::java_into_object(e.clone()));
            }
            return String::from("Error Getting Validation Info");
            }
        }
    }
    

    pub fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    pub fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return None;
    }
    

    // throws Exception
    pub fn view(&mut self) -> String {
        return jcall!(jcall!(jcall!(self.base.views).ValidationOnlyTempUtil).getInstance()).view(None /* Java this: a &mut self method has no Rc of itself */);
    }
}
impl std::ops::Deref for InsertCustomizerValidationView {
    type Target = HttpStoreComponentView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for InsertCustomizerValidationView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpCompositeDyn for InsertCustomizerValidationView {
    fn as_TransformInfoHttpComposite(&self) -> &crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &self.base.base.base }
    fn as_TransformInfoHttpComposite_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &mut self.base.base.base }
}
impl crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn for InsertCustomizerValidationView {
    fn as_HttpStoreComponentView(&self) -> &crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &self.base }
    fn as_HttpStoreComponentView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &mut self.base }
}
impl crate::views::HttpComponentView::HttpComponentViewDyn for InsertCustomizerValidationView {
    fn as_HttpComponentView(&self) -> &crate::views::HttpComponentView::HttpComponentView { &self.base.base }
    fn as_HttpComponentView_mut(&mut self) -> &mut crate::views::HttpComponentView::HttpComponentView { &mut self.base.base }
    fn view(&mut self) -> String { Self::view(self) }
}


