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
// package org.allbinary.logic.visual.theme
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::category::CategoryData::CategoryData;
use crate::org::allbinary::business::category::CategoryFactoryInterface::CategoryFactoryInterface;
use crate::org::allbinary::business::category::store::theme::StoreThemeCategoryFactory::StoreThemeCategoryFactory;
use crate::org::allbinary::business::category::store::theme::StoreThemeCategoryInterface::StoreThemeCategoryInterface;
use crate::org::allbinary::data::tree::category::CategoryLoaderFactory::CategoryLoaderFactory;
use crate::org::allbinary::data::tree::category::CategoryLoaderInterface::CategoryLoaderInterface;
use crate::org::allbinary::data::tree::dom::DomData::DomData;
use crate::org::allbinary::data::tree::dom::DomNodeHelper::DomNodeHelper;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData;
use crate::org::allbinary::logic::control::crypt::file::CryptFileReader::CryptFileReader;
use crate::org::allbinary::logic::control::validate::ValidationInterface::ValidationInterface;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::io::path::PathUtil::PathUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::visual::transform::info::CompositeTransformInfoInterface::CompositeTransformInfoInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::logic::visual::transform::template::customizer::includes::style::css::template::retail::CssStyleValidation::CssStyleValidation;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::ThemeInterface::ThemeInterface;
use super::ThemeData::ThemeData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ThemeValidation {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>,
    pub styleValidationInterface: Option<Rc<RefCell<dyn ValidationInterface>>>,
    pub themeName: String,
    pub themePreviewImageName: String,
    pub webAppAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>,
    pub categoryAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>,
    pub fileAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>,
}

impl ThemeInterface for ThemeValidation {
    fn getName(&mut self) -> String { Self::getName(self) }
    fn getPath(&mut self) -> String { Self::getPath(self) }
    fn getPreviewImageName(&mut self) -> String { Self::getPreviewImageName(self) }
    fn getPreviewImagePath(&mut self) -> String { Self::getPreviewImagePath(self) }
}
impl crate::org::allbinary::data::tables::TableMappingInterface::TableMappingInterface for ThemeValidation {
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getKey(self) }
    fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::toVector(self) }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { Self::toHashMap(self) }
}
impl ValidationInterface for ThemeValidation {
    fn isValid(&mut self) -> Option<bool> { Self::isValid(self) }
    fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toValidationInfoNode(self, document) }
    fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toValidationInfoDoc(self) }
    fn validationInfo(&mut self) -> String { Self::validationInfo(self) }
}
impl DomNodeInterface for ThemeValidation {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}
impl CompositeTransformInfoInterface for ThemeValidation {
    fn getTransformInfoInterface(&mut self) -> Option<Rc<RefCell<dyn TransformInfoInterface>>> { Self::getTransformInfoInterface(self) }
}

impl ThemeValidation {
    

    pub fn new_struct3(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut cssStyleValidation: Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::template::customizer::includes::style::css::template::retail::CssStyleValidation::CssStyleValidationDyn>>>, mut categoryThemePath: String) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            transformInfoInterface: None,
            styleValidationInterface: None,
            themeName: String::new(),
            themePreviewImageName: String::new(),
            webAppAbPath: None,
            categoryAbPath: None,
            fileAbPath: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.transformInfoInterface = transformInfoInterface.clone();
        let mut pathData: Option<Rc<RefCell<AbPathData>>> = AbPathData::getInstance();
        let mut pathUtil: Option<Rc<RefCell<PathUtil>>> = PathUtil::getInstance();
        this.categoryAbPath = jcall!(pathUtil).removeNameFromPath(categoryThemePath.clone());
        this.themeName = jcall!(pathData).getNameFromPath(categoryThemePath.clone());
        this.styleValidationInterface = cssStyleValidation.clone().map(|reference| reference as Rc<RefCell<dyn ValidationInterface>>);
        this.init();
        this
    }

    pub fn new3(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut cssStyleValidation: Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::template::customizer::includes::style::css::template::retail::CssStyleValidation::CssStyleValidationDyn>>>, mut categoryThemePath: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct3(transformInfoInterface, cssStyleValidation, categoryThemePath))))
    }
    

    pub fn new_struct2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            transformInfoInterface: None,
            styleValidationInterface: None,
            themeName: String::new(),
            themePreviewImageName: String::new(),
            webAppAbPath: None,
            categoryAbPath: None,
            fileAbPath: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.transformInfoInterface = transformInfoInterface.clone();
        let mut categoryThemeAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new((jcall!(hashMap).get(jcall!(ThemeData::getInstance()).PATH.clone())).clone() /* as String */, jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        let mut pathData: Option<Rc<RefCell<AbPathData>>> = AbPathData::getInstance();
        let mut pathUtil: Option<Rc<RefCell<PathUtil>>> = PathUtil::getInstance();
        this.categoryAbPath = jcall!(pathUtil).removeNameFromPath(jcall!(categoryThemeAbPath).toString());
        this.themeName = jcall!(pathData).getNameFromPath(jcall!(categoryThemeAbPath).toString());
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("CategoryThemePath: "));
            jcall!(stringBuffer).append(jcall!(categoryThemeAbPath).toString());
            jcall!(stringBuffer).append(String::from(" & CategoryPath: "));
            jcall!(stringBuffer).append(jcall!(this.categoryAbPath).toString());
            jcall!(stringBuffer).append(String::from(" & ThemeName: "));
            jcall!(stringBuffer).append(this.themeName.clone());
            { let java_receiver = this.logUtil.clone(); let java_arguments = (String::from("Http Request Constructor"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(stringBuffer).toString(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        this.init();
        this
    }

    pub fn new2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(transformInfoInterface, hashMap))))
    }
    

    pub fn new_struct2(mut storeThemeCategoryInterface: Option<Rc<RefCell<dyn StoreThemeCategoryInterface>>>, mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            transformInfoInterface: None,
            styleValidationInterface: None,
            themeName: String::new(),
            themePreviewImageName: String::new(),
            webAppAbPath: None,
            categoryAbPath: None,
            fileAbPath: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.transformInfoInterface = jcall!(storeThemeCategoryInterface).getTransformInfoInterface();
        this.categoryAbPath = jcall!(storeThemeCategoryInterface).getPath();
        let mut themeNameNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(jcall!(ThemeData::getInstance()).NAME.clone(), jcall!(node).getChildNodes());
        let mut valueNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(crate::org::allbinary::data::tree::dom::DomData::DomData_VALUE.with(Clone::clone).clone(), jcall!(themeNameNode).getChildNodes());
        this.themeName = DomNodeHelper::getTextNodeValue(valueNode.clone());
        let mut previewImageNameNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(jcall!(ThemeData::getInstance()).PREVIEW_IMAGE_NAME.clone(), jcall!(node).getChildNodes());
        let mut previewImageNameValueNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(crate::org::allbinary::data::tree::dom::DomData::DomData_VALUE.with(Clone::clone).clone(), jcall!(previewImageNameNode).getChildNodes());
        this.themePreviewImageName = DomNodeHelper::getTextNodeValue(previewImageNameValueNode.clone());
        this.webAppAbPath = jcall!(storeThemeCategoryInterface).getWebAppPath();
        this.init1(storeThemeCategoryInterface.clone());
        this
    }

    pub fn new2(mut storeThemeCategoryInterface: Option<Rc<RefCell<dyn StoreThemeCategoryInterface>>>, mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(storeThemeCategoryInterface, node))))
    }
    

    // throws Exception
    pub fn init(&mut self) {
        let mut categoryFactoryInterface: Option<Rc<RefCell<dyn CategoryFactoryInterface>>> = StoreThemeCategoryFactory::new(self.getTransformInfoInterface()).map(|reference| reference as Rc<RefCell<dyn CategoryFactoryInterface>>);
        let mut categoryLoaderInterface: Option<Rc<RefCell<dyn CategoryLoaderInterface>>> = CategoryLoaderFactory::getInstance(categoryFactoryInterface.clone());
        let mut rootStoreThemeCategoryInterface: Option<Rc<RefCell<dyn StoreThemeCategoryInterface>>> = (jcall!(categoryFactoryInterface).getRootInstance()).clone() /* as StoreThemeCategoryInterface */;
        self.init1(rootStoreThemeCategoryInterface.clone());
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn init1(&mut self, mut storeThemeCategoryInterface: Option<Rc<RefCell<dyn StoreThemeCategoryInterface>>>) {
        self.fileAbPath = AbPath::new(format!("{}{}", jcall!(jcall!(storeThemeCategoryInterface).getRootFilePath()).toString(), jcall!(self.categoryAbPath).toString()), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        self.webAppAbPath = jcall!(storeThemeCategoryInterface).getWebAppPath();
    }
    

    // throws Exception
    pub fn getTransformInfoInterface(&mut self) -> Option<Rc<RefCell<dyn TransformInfoInterface>>> {
        return self.transformInfoInterface.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.themeName.clone();
    }
    

    pub fn getPreviewImageName(&mut self) -> String {
        return self.themePreviewImageName.clone();
    }
    

    pub fn getPreviewImagePath(&mut self) -> String {
        return format!("{}{}", jcall!(self.webAppAbPath).toString(), jcall!(self.categoryAbPath).toString());
    }
    

    pub fn getPath(&mut self) -> String {
        return jcall!(self.categoryAbPath).toString();
    }
    

    // throws Exception
    pub fn getCssStyleValidation(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::template::customizer::includes::style::css::template::retail::CssStyleValidation::CssStyleValidationDyn>>> {
        if self.styleValidationInterface.is_none() {
            let mut cssStyleFileAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(jcall!(self.fileAbPath).toString(), format!("{}{}", format!("{}{}", self.themeName, jcall!(AbPathData::getInstance()).EXTENSION_SEP), jcall!(CategoryData::getInstance()).UNCRYPTED_EXTENSION)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
            let mut cryptFileReader: Option<Rc<RefCell<CryptFileReader>>> = CryptFileReader::new(jcall!(CategoryData::getInstance()).UNCRYPTED_EXTENSION.clone(), jcall!(CategoryData::getInstance()).ENCRYPTED_EXTENSION.clone());
            let mut document: Option<Rc<RefCell<Document>>> = DomDocumentHelper::create1(jcall!(cryptFileReader).get(cssStyleFileAbPath.clone()));
            self.styleValidationInterface = CssStyleValidation::new1(document.clone()).map(|reference| reference as Rc<RefCell<dyn ValidationInterface>>);
        }
        return (self.styleValidationInterface).clone() /* as CssStyleValidation */;
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut isValid: Option<bool> = true;
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone());
            }
            if !jcall!(StringValidationUtil::getInstance()).isValidRequired(jcall!(self.categoryAbPath).toString(), crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MINSTRING.with(Clone::clone).clone(), crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone).clone()) {
                isValid = false;
            }
            if !jcall!(StringValidationUtil::getInstance()).isValidRequired(self.themeName.clone(), crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MINSTRING.with(Clone::clone).clone(), crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone).clone()) {
                isValid = false;
            }
            if !jcall!(self.getCssStyleValidation()).isValid().unwrap() {
                isValid = false;
            }
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("End: "), isValid), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone());
            }
            return isValid.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to validate form"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
    

    pub fn validationInfo(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("Theme Validation Error"));
            return jcall!(stringBuffer).toString();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to generate validation error info"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("validationInfo()"), crate::java_into_object(e.clone()));
            }
            return String::from("Error Validating Form");
            }
        }
    }
    

    pub fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    pub fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return None;
    }
    

    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return (self.getName()).clone() /* as Any */;
    }
    

    pub fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(vector).add(crate::java_into_object(self.getName()));
        jcall!(vector).add(crate::java_into_object(self.getPath()));
        jcall!(vector).add(crate::java_into_object(self.getPreviewImageName()));
        jcall!(vector).add(crate::java_into_object(self.getPreviewImagePath()));
        return vector.clone();
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut themeData: Option<Rc<RefCell<ThemeData>>> = ThemeData::getInstance();
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(jcall!(themeData).NAME.clone(), self.getName());
        jcall!(hashMap).put(jcall!(themeData).PATH.clone(), self.getPath());
        jcall!(hashMap).put(jcall!(themeData).PREVIEW_IMAGE_NAME.clone(), self.getPreviewImageName());
        jcall!(hashMap).put(jcall!(themeData).PREVIEW_IMAGE_PATH.clone(), self.getPreviewImagePath());
        return hashMap.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = ModDomHelper::createNameValueNodes(document.clone(), jcall!(ThemeData::getInstance()).NAME.clone(), self.toHashMap());
        let mut domNodeInterface: Option<Rc<RefCell<dyn DomNodeInterface>>> = (self.getCssStyleValidation()).clone() /* as DomNodeInterface */;
        jcall!(node).appendChild(jcall!(domNodeInterface).toXmlNode(document.clone()));
        return node.clone();
    }
}

