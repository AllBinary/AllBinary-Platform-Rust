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
use super::*;
use crate::java::util::HashMap::HashMap;
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
use crate::java::lang::Exception::*;
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
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> { Self::toHashMap(self) }
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

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

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
        this.categoryAbPath = { let java_value = (jcall!(pathUtil).removeNameFromPath(categoryThemePath.clone())).clone(); java_value };
        this.themeName = { let java_value = (jcall!(pathData).getNameFromPath(categoryThemePath.clone())).clone(); java_value };
        this.styleValidationInterface = cssStyleValidation.clone().map(|_java_reference| -> Rc<RefCell<dyn ValidationInterface>> { panic!("ClassCastException: the Rust trait of CssStyleValidation does not have ValidationInterface as a base trait") });
        this.init();
        this
    }

    pub fn new3(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut cssStyleValidation: Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::template::customizer::includes::style::css::template::retail::CssStyleValidation::CssStyleValidationDyn>>>, mut categoryThemePath: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct3(transformInfoInterface, cssStyleValidation, categoryThemePath))))
    }
    

    pub fn new_struct2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Self {
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
        let mut categoryThemeAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new({ let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(ThemeData::getInstance()).PATH.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }, { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        let mut pathData: Option<Rc<RefCell<AbPathData>>> = AbPathData::getInstance();
        let mut pathUtil: Option<Rc<RefCell<PathUtil>>> = PathUtil::getInstance();
        this.categoryAbPath = { let java_value = (jcall!(pathUtil).removeNameFromPath(jcall!(categoryThemeAbPath).toString())).clone(); java_value };
        this.themeName = { let java_value = (jcall!(pathData).getNameFromPath(jcall!(categoryThemeAbPath).toString())).clone(); java_value };
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
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

    pub fn new2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(transformInfoInterface, hashMap))))
    }
    

    pub fn new_struct2_StoreThemeCategoryInterface_Node(mut storeThemeCategoryInterface: Option<Rc<RefCell<dyn StoreThemeCategoryInterface>>>, mut node: Option<Rc<RefCell<Node>>>) -> Self {
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
        this.transformInfoInterface = { let java_value = (jcall!(storeThemeCategoryInterface).getTransformInfoInterface()).clone(); java_value };
        this.categoryAbPath = { let java_value = (jcall!(storeThemeCategoryInterface).getPath()).clone(); java_value };
        let mut themeNameNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode({ let java_field = jcall!(ThemeData::getInstance()).NAME.clone(); java_field }.clone(), jcall!(node).getChildNodes());
        let mut valueNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(crate::org::allbinary::data::tree::dom::DomData::DomData_VALUE.with(Clone::clone).clone(), jcall!(themeNameNode).getChildNodes());
        this.themeName = { let java_value = (DomNodeHelper::getTextNodeValue(valueNode.clone())).clone(); java_value };
        let mut previewImageNameNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode({ let java_field = jcall!(ThemeData::getInstance()).PREVIEW_IMAGE_NAME.clone(); java_field }.clone(), jcall!(node).getChildNodes());
        let mut previewImageNameValueNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(crate::org::allbinary::data::tree::dom::DomData::DomData_VALUE.with(Clone::clone).clone(), jcall!(previewImageNameNode).getChildNodes());
        this.themePreviewImageName = { let java_value = (DomNodeHelper::getTextNodeValue(previewImageNameValueNode.clone())).clone(); java_value };
        this.webAppAbPath = { let java_value = (jcall!(storeThemeCategoryInterface).getWebAppPath()).clone(); java_value };
        this.init1(storeThemeCategoryInterface.clone());
        this
    }

    pub fn new2_StoreThemeCategoryInterface_Node(mut storeThemeCategoryInterface: Option<Rc<RefCell<dyn StoreThemeCategoryInterface>>>, mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2_StoreThemeCategoryInterface_Node(storeThemeCategoryInterface, node))))
    }
    

    // throws Exception
    pub fn init(&mut self) {
        let mut categoryFactoryInterface: Option<Rc<RefCell<dyn CategoryFactoryInterface>>> = StoreThemeCategoryFactory::new(self.getTransformInfoInterface()).map(|reference| reference as Rc<RefCell<dyn CategoryFactoryInterface>>).map(|reference| reference as Rc<RefCell<dyn CategoryFactoryInterface>>);
        let mut categoryLoaderInterface: Option<Rc<RefCell<dyn CategoryLoaderInterface>>> = CategoryLoaderFactory::getInstance(categoryFactoryInterface.clone());
        let mut rootStoreThemeCategoryInterface: Option<Rc<RefCell<dyn StoreThemeCategoryInterface>>> = (|| -> Option<Rc<RefCell<dyn StoreThemeCategoryInterface>>> { panic!("ClassCastException: a Rust dyn CategoryInterface cannot be cast to dyn StoreThemeCategoryInterface") })();
        self.init1(rootStoreThemeCategoryInterface.clone());
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn init1(&mut self, mut storeThemeCategoryInterface: Option<Rc<RefCell<dyn StoreThemeCategoryInterface>>>) {
        self.fileAbPath = { let java_value = (AbPath::new(format!("{}{}", jcall!(jcall!(storeThemeCategoryInterface).getRootFilePath()).toString(), jcall!(self.categoryAbPath).toString()), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>)).clone(); java_value };
        self.webAppAbPath = { let java_value = (jcall!(storeThemeCategoryInterface).getWebAppPath()).clone(); java_value };
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
            let mut cssStyleFileAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(jcall!(self.fileAbPath).toString(), format!("{}{}", format!("{}{}", self.themeName, { let java_field = jcall!(AbPathData::getInstance()).EXTENSION_SEP.clone(); java_field }), { let java_field = jcall!(CategoryData::getInstance()).UNCRYPTED_EXTENSION.clone(); java_field })).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
            let mut cryptFileReader: Option<Rc<RefCell<CryptFileReader>>> = CryptFileReader::new({ let java_field = jcall!(CategoryData::getInstance()).UNCRYPTED_EXTENSION.clone(); java_field }.clone(), { let java_field = jcall!(CategoryData::getInstance()).ENCRYPTED_EXTENSION.clone(); java_field }.clone());
            let mut document: Option<Rc<RefCell<Document>>> = DomDocumentHelper::create1(jcall!(cryptFileReader).get(cssStyleFileAbPath.clone()));
            self.styleValidationInterface = CssStyleValidation::new1(document.clone()).map(|reference| reference as Rc<RefCell<dyn ValidationInterface>>);
        }
        return (|| -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::template::customizer::includes::style::css::template::retail::CssStyleValidation::CssStyleValidationDyn>>> { panic!("ClassCastException: a Rust dyn ValidationInterface cannot be cast to CssStyleValidation") })();
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut isValid: Option<bool> = true;
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).IS_VALID.clone(); java_field }.clone());
            }
            if !jcall!(StringValidationUtil::getInstance()).isValidRequired(jcall!(self.categoryAbPath).toString(), crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MINSTRING.with(Clone::clone).clone(), crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone).clone()) {
                isValid = { let java_value = (false).clone(); java_value }.clone();
            }
            if !jcall!(StringValidationUtil::getInstance()).isValidRequired(self.themeName.clone(), crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MINSTRING.with(Clone::clone).clone(), crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone).clone()) {
                isValid = { let java_value = (false).clone(); java_value }.clone();
            }
            if !jcall!(self.getCssStyleValidation()).isValid().unwrap() {
                isValid = { let java_value = (false).clone(); java_value }.clone();
            }
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("End: "), isValid), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).IS_VALID.clone(); java_field }.clone());
            }
            return Some(isValid.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLTAGSERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to validate form"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).IS_VALID.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
            return false;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn validationInfo(&mut self) -> String {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("Theme Validation Error"));
            return Some(jcall!(stringBuffer).toString());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEWERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to generate validation error info"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("validationInfo()"), crate::java_into_object(e.clone()));
            }
            return String::from("Error Validating Form");
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    pub fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return None;
    }
    

    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return crate::java_into_object(crate::java_into_object((self.getName()).clone()));
    }
    

    pub fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(vector).add(crate::java_into_object(self.getName()));
        jcall!(vector).add(crate::java_into_object(self.getPath()));
        jcall!(vector).add(crate::java_into_object(self.getPreviewImageName()));
        jcall!(vector).add(crate::java_into_object(self.getPreviewImagePath()));
        return vector.clone();
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> {
        let mut themeData: Option<Rc<RefCell<ThemeData>>> = ThemeData::getInstance();
        let mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(crate::java_into_object({ let java_field = jcall!(themeData).NAME.clone(); java_field }.clone()), crate::java_into_object(self.getName()));
        jcall!(hashMap).put(crate::java_into_object({ let java_field = jcall!(themeData).PATH.clone(); java_field }.clone()), crate::java_into_object(self.getPath()));
        jcall!(hashMap).put(crate::java_into_object({ let java_field = jcall!(themeData).PREVIEW_IMAGE_NAME.clone(); java_field }.clone()), crate::java_into_object(self.getPreviewImageName()));
        jcall!(hashMap).put(crate::java_into_object({ let java_field = jcall!(themeData).PREVIEW_IMAGE_PATH.clone(); java_field }.clone()), crate::java_into_object(self.getPreviewImagePath()));
        return hashMap.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = ModDomHelper::createNameValueNodes_Document_String_HashMap(document.clone(), { let java_field = jcall!(ThemeData::getInstance()).NAME.clone(); java_field }.clone(), self.toHashMap());
        let mut domNodeInterface: Option<Rc<RefCell<dyn DomNodeInterface>>> = (|| -> Option<Rc<RefCell<dyn DomNodeInterface>>> { panic!("ClassCastException: a Rust dyn CssStyleValidation cannot be cast to DomNodeInterface") })();
        jcall!(node).appendChild(jcall!(domNodeInterface).toXmlNode(document.clone()));
        return node.clone();
    }
}

