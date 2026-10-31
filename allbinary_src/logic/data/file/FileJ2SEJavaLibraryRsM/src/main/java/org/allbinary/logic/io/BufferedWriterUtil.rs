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
// package org.allbinary.logic.io
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::BufferedWriter::BufferedWriter;
// use crate::java::io::FileWriter::FileWriter;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::AbFileNativeUtil::AbFileNativeUtil;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BufferedWriterUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub commonLabels: Option<Rc<RefCell<CommonLabels>>>,
    pub REMOVING_OLD: String,
}

thread_local! { pub static BufferedWriterUtil_instance: Option<Rc<RefCell<BufferedWriterUtil>>> = BufferedWriterUtil::new(); }

impl BufferedWriterUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            commonLabels: None,
            REMOVING_OLD: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.commonLabels = CommonLabels::getInstance();
        this.REMOVING_OLD = String::from("Remove old for overwriting: ");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<BufferedWriterUtil>>> {
        return BufferedWriterUtil_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn overwrite(&mut self, mut path: String, mut data: String) {
        let mut abFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(path.clone());
        if jcall!(abFile).exists() {
            jcall!(self.logUtil).putF(format!("{}{}", self.REMOVING_OLD, path), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).CREATE.clone());
            jcall!(abFile).delete();
        } else {
            let mut name: String = jcall!(AbPathData::getInstance()).removeNameFromPath(path.clone());
            let mut abFileDirectory: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(name.clone());
            if jcall!(abFileDirectory).exists() {
            } else {
                let mut stringBuilder: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuilder).append(jcall!(self.commonStrings).CREATE.clone()).append(jcall!(self.commonLabels).COLON_SEP.clone()).append(name.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).CREATE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                let mut result: bool = jcall!(abFileDirectory).mkdirs();
                jcall!(stringBuilder).delete(0, jcall!(stringBuilder).length());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuilder).append(jcall!(self.commonLabels).RESULT_.clone()).appendboolean(result.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).CREATE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
        }
        self.write(abFile.clone(), data.clone());
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn overwrite(&mut self, mut abFile: Option<Rc<RefCell<AbFile>>>, mut data: String) {
        if jcall!(abFile).exists() {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", self.REMOVING_OLD, jcall!(abFile).getPath()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).CREATE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            jcall!(abFile).delete();
        } else {
            let mut name: String = jcall!(AbPathData::getInstance()).removeNameFromPath(jcall!(abFile).getAbsolutePath());
            let mut abFileDirectory: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(name.clone());
            if jcall!(abFileDirectory).exists() {
            } else {
                let mut stringBuilder: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuilder).append(jcall!(self.commonStrings).CREATE.clone()).append(jcall!(self.commonLabels).COLON_SEP.clone()).append(name.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).CREATE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                let mut result: bool = jcall!(abFileDirectory).mkdirs();
                jcall!(stringBuilder).delete(0, jcall!(stringBuilder).length());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuilder).append(jcall!(self.commonLabels).RESULT_.clone()).appendboolean(result.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).CREATE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
        }
        self.write(abFile.clone(), data.clone());
    }
    

    // throws Exception
    pub fn write(&mut self, mut abFile: Option<Rc<RefCell<AbFile>>>, mut data: String) {
        let mut fileOut: Option<Rc<RefCell<BufferedWriter>>> = BufferedWriter::new(FileWriter::new(AbFileNativeUtil::get(abFile.clone())));
        jcall!(fileOut).write(data.clone(), 0, (data.chars().count() as i32));
        jcall!(fileOut).newLine();
        jcall!(fileOut).flush();
    }
}

