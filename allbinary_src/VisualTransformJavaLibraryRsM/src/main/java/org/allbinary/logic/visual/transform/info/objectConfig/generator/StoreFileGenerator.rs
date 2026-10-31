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
// package org.allbinary.logic.visual.transform.info.objectConfig.generator
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::io::ByteArrayInputStream::ByteArrayInputStream;
// use crate::java::io::InputStream::InputStream;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSession::WeblisketSession;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::InputOutputTypeData::InputOutputTypeData;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::FileUtil::FileUtil;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::string::regex::replace::Replace::Replace;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpInterface::TransformInfoHttpInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpStoreInterface::TransformInfoHttpStoreInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfosData::TransformInfosData;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::TransformInfoObjectConfigGenerator::TransformInfoObjectConfigGenerator;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StoreFileGenerator {
    pub base: TransformInfoObjectConfigGenerator,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub output: String,
    pub fileAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>,
    pub file: Option<Rc<RefCell<AbFile>>>,
}


impl StoreFileGenerator {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: TransformInfoObjectConfigGenerator::new_struct(),
            logUtil: None,
            commonStrings: None,
            output: String::new(),
            fileAbPath: None,
            file: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        let mut httpTransformInfoInterface: Option<Rc<RefCell<dyn TransformInfoHttpInterface>>> = panic!("ClassCastException: a Rust dyn TransformInfoInterface cannot be cast to dyn TransformInfoHttpInterface");
        let mut weblisketSession: Option<Rc<RefCell<WeblisketSession>>> = WeblisketSession::new2(jcall!(httpTransformInfoInterface).getPropertiesHashMap(), jcall!(httpTransformInfoInterface).getPageContext());
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(URLGLOBALS::getWebappPath());
        jcall!(stringBuffer).append(jcall!(weblisketSession).getStoreName());
        jcall!(stringBuffer).append(jcall!(AbPathData::getInstance()).SEPARATOR.clone());
        let mut abPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(jcall!(stringBuffer).toString(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        let mut fileName: String = (jcall!(jcall!(httpTransformInfoInterface).getPropertiesHashMap()).get(jcall!(InputOutputTypeData::getInstance()).FILE.clone())).clone() /* as String */;
        let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
        if jcall!(stringValidationUtil).isEmpty(fileName.clone()) {
            let mut transformInfoHttpStoreInterface: Option<Rc<RefCell<dyn TransformInfoHttpStoreInterface>>> = panic!("ClassCastException: a Rust dyn TransformInfoInterface cannot be cast to dyn TransformInfoHttpStoreInterface");
            fileName = jcall!(jcall!(transformInfoInterface).getName()).substring(jcall!(jcall!(transformInfoHttpStoreInterface).getStoreName()).length() + 1);
            let mut outputMappingHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            jcall!(outputMappingHashMap).put(jcall!(TransformInfosData::getInstance()).PREVIEW.clone(), String::from("Preview"));
            jcall!(outputMappingHashMap).put(jcall!(TransformInfosData::getInstance()).SMALL_PREVIEW.clone(), String::from("SmallPreview"));
            jcall!(outputMappingHashMap).put(jcall!(CommonSeps::getInstance()).SPACE.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone());
            let mut replace: Option<Rc<RefCell<Replace>>> = Replace::new1(outputMappingHashMap.clone());
            fileName = jcall!(replace).all(fileName.clone());
        }
        if jcall!(stringValidationUtil).isEmpty(fileName.clone()) {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("TransformInfoObjectConfigGenerator FileName Not Specified"))));
        }
        let mut tempOutput: String = (jcall!(jcall!(httpTransformInfoInterface).getPropertiesHashMap()).get(jcall!(InputOutputTypeData::getInstance()).NAME.clone())).clone() /* as String */;
        if jcall!(stringValidationUtil).isEmpty(tempOutput.clone()) {
            tempOutput = jcall!(InputOutputTypeData::getInstance()).DEFAULT.clone();
        }
        this.output = tempOutput.clone();
        if jcall!(stringValidationUtil).isEmpty(this.output.clone()) {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("TransformInfoObjectConfigGenerator Output=extension Not Specified"))));
        }
        stringBuffer = StringMaker::new();
        jcall!(stringBuffer).append(fileName.clone());
        jcall!(stringBuffer).append(jcall!(AbPathData::getInstance()).EXTENSION_SEP.clone());
        jcall!(stringBuffer).append(this.output.clone());
        abPath = AbPath::new(jcall!(abPath).toString(), jcall!(stringBuffer).toString()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        this.fileAbPath = abPath.clone();
        this.file = AbFile::createAbFileFromAbPath(this.fileAbPath.clone());
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("File: "), jcall!(this.fileAbPath).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    // throws Exception
    pub fn process(&mut self, mut input: String) -> String {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            jcall!(self.logUtil).putF(String::from("Processing"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).PROCESS.clone());
        }
        if !jcall!(self.file).exists() {
            jcall!(self.file).createNewFile();
        }
        if jcall!(self.file).exists() {
            let mut inputStream: Option<Rc<RefCell<InputStream>>> = ByteArrayInputStream::new(input.bytes().map(|value| value as i8).collect::<Vec<i8>>());
            jcall!(FileUtil::getInstance()).write(inputStream.clone(), self.file.clone());
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("Could Not Create: "), jcall!(self.fileAbPath).toString()))));
        }
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            jcall!(self.logUtil).putF(String::from("Processed"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).PROCESS.clone());
        }
        return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
    }
}
impl std::ops::Deref for StoreFileGenerator {
    type Target = TransformInfoObjectConfigGenerator;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for StoreFileGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::visual::transform::info::objectConfig::generator::TransformInfoObjectConfigGenerator::TransformInfoObjectConfigGeneratorDyn for StoreFileGenerator {
    fn as_TransformInfoObjectConfigGenerator(&self) -> &crate::org::allbinary::logic::visual::transform::info::objectConfig::generator::TransformInfoObjectConfigGenerator::TransformInfoObjectConfigGenerator { &self.base }
    fn as_TransformInfoObjectConfigGenerator_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::objectConfig::generator::TransformInfoObjectConfigGenerator::TransformInfoObjectConfigGenerator { &mut self.base }
}
impl crate::org::allbinary::logic::visual::transform::info::objectConfig::generator::TransformInfoObjectConfigGeneratorInterface::TransformInfoObjectConfigGeneratorInterface for StoreFileGenerator {
    fn process(&mut self, mut input: String) -> String { Self::process(self, input) }
}


