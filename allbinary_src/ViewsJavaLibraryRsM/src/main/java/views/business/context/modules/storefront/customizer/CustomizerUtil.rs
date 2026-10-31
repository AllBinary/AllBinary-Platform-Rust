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
// package views.business.context.modules.storefront.customizer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::data::tables::transform::info::TransformInfoEntity::TransformInfoEntity;
use crate::org::allbinary::data::tables::transform::info::TransformInfoEntityBuilder::TransformInfoEntityBuilder;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::data::tree::dom::document::XmlDocumentHelper::XmlDocumentHelper;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfo;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpInterface::TransformInfoHttpInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfigInterface::TransformInfoObjectConfigInterface;
use crate::org::w3c::dom::Document::Document;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CustomizerUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static CustomizerUtil_instance: Option<Rc<RefCell<CustomizerUtil>>> = CustomizerUtil::new(); }

impl CustomizerUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<CustomizerUtil>>> {
        return CustomizerUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getTransformInfoInterfaceToCustomize(&mut self, mut customizerTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<dyn TransformInfoInterface>>> {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("View Name: "), jcall!(customizerTransformInfoInterface).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getTransformInfoInterfaceToCustomize()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut transformInfoEntityInterface: Option<Rc<RefCell<TransformInfoEntity>>> = TransformInfoEntityBuilder::getInstance();
        let mut viewName: String = jcall!(transformInfoInterface).getName();
        let mut httpTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoHttpInterface>>> = panic!("ClassCastException: a Rust dyn TransformInfoInterface cannot be cast to dyn TransformInfoHttpInterface");
        let mut customizedTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>> = jcall!(transformInfoEntityInterface).get(viewName.clone(), jcall!(httpTransformInfoInterface).getPropertiesHashMap(), jcall!(httpTransformInfoInterface).getPageContext());
        return customizedTransformInfoInterface.clone();
    }
    

    // throws Exception
    pub fn getTransformInfoObjectConfigComponentNodesToCustomize(&mut self, mut customizerTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("View Name: "), jcall!(customizerTransformInfoInterface).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getTransformInfoObjectConfigComponentNodesToCustomize()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut transformInfoEntityInterface: Option<Rc<RefCell<TransformInfoEntity>>> = TransformInfoEntityBuilder::getInstance();
        let mut viewName: String = jcall!(transformInfoInterface).getName();
        let mut httpTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoHttpInterface>>> = panic!("ClassCastException: a Rust dyn TransformInfoInterface cannot be cast to dyn TransformInfoHttpInterface");
        let mut customizedTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>> = jcall!(transformInfoEntityInterface).get(viewName.clone(), jcall!(httpTransformInfoInterface).getPropertiesHashMap(), jcall!(httpTransformInfoInterface).getPageContext());
        return self.getTransformInfoObjectConfigComponentNodes(customizedTransformInfoInterface.clone());
    }
    

    // throws Exception
    pub fn getTransformInfoObjectConfigComponentNodes(&mut self, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("View Name: "), jcall!(transformInfoInterface).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getTransformInfoObjectConfigComponentNodes()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut transformInfoObjectConfigInterface: Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>> = jcall!(transformInfoInterface).getObjectConfigInterface();
        return jcall!(transformInfoObjectConfigInterface).getTransforms();
    }
    

    // throws Exception
    pub fn getTransformInfoObjectConfigGroupComponentNodes(&mut self, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("View Name: "), jcall!(transformInfoInterface).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getTransformInfoObjectConfigComponentNodes()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut transformInfoObjectConfigInterface: Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>> = jcall!(transformInfoInterface).getObjectConfigInterface();
        return jcall!(transformInfoObjectConfigInterface).getGroupTransforms();
    }
    

    // throws Exception
    pub fn insert(&mut self, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut domNodeInterface: Option<Rc<RefCell<dyn DomNodeInterface>>>) {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("View Name: "), jcall!(transformInfoInterface).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("insert()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut transformInfoEntityInterface: Option<Rc<RefCell<TransformInfoEntity>>> = TransformInfoEntityBuilder::getInstance();
        let mut allViewsToBeModified: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getTransformInfoObjectConfigGroupComponentNodes(transformInfoInterface.clone());
        let mut document: Option<Rc<RefCell<Document>>> = DomDocumentHelper::create();
        jcall!(document).appendChild(jcall!(domNodeInterface).toXmlNode(document.clone()));
        let mut documentString: String = XmlDocumentHelper::toString(document.clone());
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            jcall!(stringBuffer).append(String::from("Total Views For Modification: "));
            jcall!(stringBuffer).appendint(jcall!(allViewsToBeModified).size());
            jcall!(stringBuffer).append(String::from(" New Xml/Data: "));
            jcall!(stringBuffer).append(documentString.clone());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("insert()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        jcall!(transformInfoInterface).setData(documentString.clone());
        let mut size: i32 = jcall!(allViewsToBeModified).size();
        for index in (0..size) {
            let mut transformInfo: Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfoDyn>>> = { let java_object = (jcall!(allViewsToBeModified).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfoDyn>>>>().expect("ClassCastException").clone()) };
            let mut viewNameOfViewToBeModified: String = jcall!(transformInfo).getName();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                jcall!(stringBuffer).append(jcall!(transformInfoInterface).getName());
                jcall!(stringBuffer).append(String::from(" is modifying view: "));
                jcall!(stringBuffer).append(viewNameOfViewToBeModified.clone());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("insert()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            let mut httpTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoHttpInterface>>> = panic!("ClassCastException: a Rust dyn TransformInfoInterface cannot be cast to dyn TransformInfoHttpInterface");
            let mut specifiedTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>> = jcall!(transformInfoEntityInterface).get(viewNameOfViewToBeModified.clone(), jcall!(httpTransformInfoInterface).getPropertiesHashMap(), jcall!(httpTransformInfoInterface).getPageContext());
            if specifiedTransformInfoInterface.is_none() {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                    jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                    jcall!(stringBuffer).append(jcall!(transformInfoInterface).getName());
                    jcall!(stringBuffer).append(String::from(" is unable to add data to view: "));
                    jcall!(stringBuffer).append(viewNameOfViewToBeModified.clone());
                    jcall!(stringBuffer).append(String::from(" since it does not exist"));
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("insert()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
            } else {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", format!("{}{}", jcall!(transformInfoInterface).getName(), String::from(" is adding data to view: ")), viewNameOfViewToBeModified), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("insert()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                    jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                    jcall!(stringBuffer).append(viewNameOfViewToBeModified.clone());
                    jcall!(stringBuffer).append(String::from(" is changing data in "));
                    jcall!(stringBuffer).append(jcall!(jcall!(specifiedTransformInfoInterface).getDataFilePath()).toString());
                    jcall!(stringBuffer).append(String::from(" to the following data:\n"));
                    jcall!(stringBuffer).append(documentString.clone());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("insert()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                self.write(specifiedTransformInfoInterface.clone(), documentString.clone());
            }
        }
    }
    

    // throws Exception
    pub fn write(&mut self, mut specifiedTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut documentString: String) {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("View Name: "), jcall!(specifiedTransformInfoInterface).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("write()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut transformInfoEntityInterface: Option<Rc<RefCell<TransformInfoEntity>>> = TransformInfoEntityBuilder::getInstance();
        jcall!(specifiedTransformInfoInterface).setData(documentString.clone());
        jcall!(transformInfoEntityInterface).update(jcall!(specifiedTransformInfoInterface).toHashMap());
    }
    

    // throws Exception
    pub fn getViewDataForComponentsInObjectConfig(&mut self, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Document>>> {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("View Name: "), jcall!(transformInfoInterface).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getViewDataForComponentsInObjectConfig()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut transformInfoObjectConfigInterface: Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>> = jcall!(transformInfoInterface).getObjectConfigInterface();
        let mut allViewsToBeModified: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(transformInfoObjectConfigInterface).getGroupTransforms();
        let mut transformInfoEntityInterface: Option<Rc<RefCell<TransformInfoEntity>>> = TransformInfoEntityBuilder::getInstance();
        let mut size: i32 = jcall!(allViewsToBeModified).size();
        for index in (0..size) {
            let mut nextTransformInfoInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfoDyn>>> = { let java_object = (jcall!(allViewsToBeModified).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfoDyn>>>>().expect("ClassCastException").clone()) };
            let mut viewNameOfViewToBeModified: String = jcall!(nextTransformInfoInterface).getName();
            let mut httpTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoHttpInterface>>> = panic!("ClassCastException: a Rust dyn TransformInfoInterface cannot be cast to dyn TransformInfoHttpInterface");
            let mut specifiedTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>> = jcall!(transformInfoEntityInterface).get(viewNameOfViewToBeModified.clone(), jcall!(httpTransformInfoInterface).getPropertiesHashMap(), jcall!(httpTransformInfoInterface).getPageContext());
            return jcall!(specifiedTransformInfoInterface).getDataDocument();
        }
        panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No Components Specified in ObjectConfig"))));
    }
}

