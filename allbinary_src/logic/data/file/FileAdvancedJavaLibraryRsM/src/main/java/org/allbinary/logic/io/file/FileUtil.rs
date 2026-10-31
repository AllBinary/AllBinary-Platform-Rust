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
// package org.allbinary.logic.io.file
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::InputStream::InputStream;
// use crate::java::io::OutputStream::OutputStream;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypeFactory::LogConfigTypeFactory;
use crate::org::allbinary::logic::io::AbDataOutputStream::AbDataOutputStream;
use crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStream;
use crate::org::allbinary::logic::io::AbFileLocalInputStream::AbFileLocalInputStream;
use crate::org::allbinary::logic::io::AbIOSystem::AbIOSystem;
use crate::org::allbinary::logic::io::DataOutputStreamFactory::DataOutputStreamFactory;
use crate::org::allbinary::logic::io::FileStreamFactory::FileStreamFactory;
use crate::org::allbinary::logic::io::StreamUtil::StreamUtil;
use crate::org::allbinary::logic::io::file::directory::Directory::Directory;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::AbFile::AbFile;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//data/init/views/TestStore/template/type/genericTemplateObjectConfig.xml
pub struct FileUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub directory: Option<Rc<RefCell<Directory>>>,
    pub logConfigTypeFactory: Option<Rc<RefCell<LogConfigTypeFactory>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub streamUtil: Option<Rc<RefCell<StreamUtil>>>,
    pub WRITE_LABEL: String,
    pub DATA_LABEL: String,
    pub WRITE_METHOD: String,
}

thread_local! { pub static FileUtil_instance: Option<Rc<RefCell<FileUtil>>> = FileUtil::new(); }

impl FileUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<FileUtil>>> {
        return FileUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            directory: None,
            logConfigTypeFactory: None,
            commonStrings: None,
            streamUtil: None,
            WRITE_LABEL: String::new(),
            DATA_LABEL: String::new(),
            WRITE_METHOD: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.directory = Directory::getInstance();
        this.logConfigTypeFactory = LogConfigTypeFactory::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.streamUtil = StreamUtil::getInstance();
        this.WRITE_LABEL = String::from("Write file: ");
        this.DATA_LABEL = String::from(" data: ");
        this.WRITE_METHOD = String::from("write");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn write(&mut self, mut inputStream: Option<Rc<RefCell<InputStream>>>, mut file: Option<Rc<RefCell<AbFile>>>) {
        let mut dataOutputStream: Option<Rc<RefCell<AbDataOutputStream>>> = jcall!(DataOutputStreamFactory::getInstance()).getInstanceForAbFile(file.clone());
        self.write(inputStream.clone(), dataOutputStream.clone());
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn write(&mut self, mut inputStream: Option<Rc<RefCell<InputStream>>>, mut dataOutputStream: Option<Rc<RefCell<AbDataOutputStream>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            dataOutputStream = (jcall!(StreamUtil::getInstance()).get(inputStream.clone(), dataOutputStream.clone(), vec![0; (16384) as usize])).clone() /* as AbDataOutputStream */;
            jcall!(dataOutputStream).flush();
        })) {
            Ok(_) => {}
            Err(error) => std::panic::resume_unwind(error),
        }
        { // finally
            jcall!(StreamUtil::getInstance()).close(dataOutputStream.clone());
        }
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn write3(&mut self, mut inputStream: Option<Rc<RefCell<InputStream>>>, mut dataOutputStream: Option<Rc<RefCell<AbDataOutputStream>>>, mut buffer: Vec<i8>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            dataOutputStream = (jcall!(StreamUtil::getInstance()).get(inputStream.clone(), dataOutputStream.clone(), buffer.clone())).clone() /* as AbDataOutputStream */;
            jcall!(dataOutputStream).flush();
        })) {
            Ok(_) => {}
            Err(error) => std::panic::resume_unwind(error),
        }
        { // finally
            jcall!(StreamUtil::getInstance()).close(dataOutputStream.clone());
        }
    }
    

    // throws Exception
    pub fn copy(&mut self, mut dataInputStream: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStreamDyn>>>, mut dataOutputStream: Option<Rc<RefCell<AbDataOutputStream>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            dataOutputStream = (jcall!(StreamUtil::getInstance()).get(dataInputStream.clone(), dataOutputStream.clone(), vec![0; (16384) as usize])).clone() /* as AbDataOutputStream */;
            jcall!(dataOutputStream).flush();
        })) {
            Ok(_) => {}
            Err(error) => std::panic::resume_unwind(error),
        }
        { // finally
            jcall!(StreamUtil::getInstance()).close(dataOutputStream.clone());
            jcall!(StreamUtil::getInstance()).close(dataInputStream.clone());
        }
    }
    

    // throws Exception
    pub fn fixPath(&mut self, mut file: Option<Rc<RefCell<AbFile>>>, mut path: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>, mut realPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>, mut cloud: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> {
        let mut beginIndex: i32 = jcall!(jcall!(file).getPath()).indexOf(cloud.clone()) + (cloud.chars().count() as i32);
        let mut endIndex: i32 = jcall!(jcall!(file).getPath()).indexOf(jcall!(file).getName());
        let mut fixedPathString: String = format!("{}{}", jcall!(realPath).toFileSystemString(), jcall!(jcall!(file).getPath()).substring(beginIndex.clone(), endIndex.clone()));
        let mut fixedPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(fixedPathString.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        jcall!(self.directory).create(fixedPath.clone());
        let mut string: String = jcall!(fixedPath).toFileSystemString();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(self.logConfigTypeFactory).FILE.clone()) {
            jcall!(self.logUtil).putF(string.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("fixPath"));
        }
        let mut outPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(string.clone(), jcall!(file).getName()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        return outPath.clone();
    }
    

    // throws Exception
    pub fn copyToCloud4(&mut self, mut file: Option<Rc<RefCell<AbFile>>>, mut path: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>, mut realPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>, mut cloud: String) {
        self.copyToCloud6(file.clone(), path.clone(), realPath.clone(), cloud.clone(), false, false);
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn copyToCloud6(&mut self, mut file: Option<Rc<RefCell<AbFile>>>, mut path: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>, mut realPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>, mut cloud: String, mut overwriteNewer: bool, mut overwriteAll: bool) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut outPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = self.fixPath(file.clone(), path.clone(), realPath.clone(), cloud.clone());
            let mut outFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFileFromAbPath(outPath.clone());
            if !self.copyPrepare(file.clone(), outFile.clone(), overwriteNewer.clone(), overwriteAll.clone()) {
                return;
            } else {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(self.logConfigTypeFactory).FILE.clone()) {
                    let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                    jcall!(stringBuffer).append(String::from("Out File: "));
                    jcall!(stringBuffer).append(jcall!(outFile).getPath());
                    jcall!(stringBuffer).append(String::from(" In File: "));
                    jcall!(stringBuffer).append(jcall!(file).getPath());
                    jcall!(stringBuffer).append(String::from(" to cloud."));
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(Self::getInstance()), String::from("copyToCloud"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
            }
            let mut dataOutputStream: Option<Rc<RefCell<AbDataOutputStream>>> = jcall!(DataOutputStreamFactory::getInstance()).getInstanceForAbFile(outFile.clone());
            let mut fileInputStream: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStreamDyn>>> = AbFileLocalInputStream::new(file.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStreamDyn>>);
            self.copy(fileInputStream.clone(), dataOutputStream.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(self.logConfigTypeFactory).FILEERROR.clone()) {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("Error Copying File File: "));
                jcall!(stringBuffer).append(jcall!(file).toString());
                jcall!(stringBuffer).append(String::from(" to cloud."));
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(Self::getInstance()), String::from("copyToCloud"), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn copyToCloud(&mut self, mut file: Option<Rc<RefCell<AbFile>>>, mut outFile: Option<Rc<RefCell<AbFile>>>) {
        self.copyToCloud4(file.clone(), outFile.clone(), false, false);
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn copyToCloud4(&mut self, mut file: Option<Rc<RefCell<AbFile>>>, mut outFile: Option<Rc<RefCell<AbFile>>>, mut overwriteNewer: bool, mut overwriteAll: bool) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            if !self.copyPrepare(file.clone(), outFile.clone(), overwriteNewer.clone(), overwriteAll.clone()) {
                return;
            } else {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(self.logConfigTypeFactory).FILE.clone()) {
                    jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                    jcall!(stringBuffer).append(String::from("Out File: "));
                    jcall!(stringBuffer).append(jcall!(outFile).getPath());
                    jcall!(stringBuffer).append(String::from(" In File: "));
                    jcall!(stringBuffer).append(jcall!(file).getPath());
                    jcall!(stringBuffer).append(String::from(" to cloud."));
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(Self::getInstance()), String::from("copyToCloud"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
            }
            let mut dataOutputStream: Option<Rc<RefCell<AbDataOutputStream>>> = jcall!(DataOutputStreamFactory::getInstance()).getInstanceForAbFile(outFile.clone());
            let mut fileInputStream: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStreamDyn>>> = AbFileLocalInputStream::new(file.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStreamDyn>>);
            self.copy(fileInputStream.clone(), dataOutputStream.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(self.logConfigTypeFactory).FILEERROR.clone()) {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("Error Copying File File: "));
                jcall!(stringBuffer).append(jcall!(file).toString());
                jcall!(stringBuffer).append(String::from(" to cloud."));
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(Self::getInstance()), String::from("copyToCloud"), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    pub fn copyPrepare(&mut self, mut fromFile: Option<Rc<RefCell<AbFile>>>, mut toFile: Option<Rc<RefCell<AbFile>>>, mut overwriteNewer: bool, mut overwriteAll: bool) -> bool {
        if jcall!(toFile).exists() {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            if overwriteAll {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(self.logConfigTypeFactory).FILE.clone()) {
                    jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                    jcall!(stringBuffer).append(String::from("Overwriting File: "));
                    jcall!(stringBuffer).append(jcall!(toFile).getPath());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(Self::getInstance()), String::from("copyFile"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                jcall!(toFile).delete();
                jcall!(toFile).createNewFile();
            } else if jcall!(toFile).lastModified() < jcall!(fromFile).lastModified() && overwriteNewer {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(self.logConfigTypeFactory).FILE.clone()) {
                    jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                    jcall!(stringBuffer).append(String::from("Newer by: "));
                    jcall!(stringBuffer).appendlong(jcall!(fromFile).lastModified() - jcall!(toFile).lastModified());
                    jcall!(stringBuffer).append(String::from("ms "));
                    jcall!(stringBuffer).append(String::from("Copying File: "));
                    jcall!(stringBuffer).append(jcall!(toFile).getPath());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(Self::getInstance()), String::from("copyFile"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                jcall!(toFile).delete();
                jcall!(toFile).createNewFile();
            } else {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(self.logConfigTypeFactory).FILE.clone()) {
                    jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                    jcall!(stringBuffer).append(String::from("File Already Exists"));
                    if overwriteNewer {
                        jcall!(stringBuffer).append(String::from(" And Is Not Older"));
                    } else {
                        jcall!(stringBuffer).append(String::from(" And Not A In Overwrite Mode"));
                    }
                    jcall!(stringBuffer).append(jcall!(CommonLabels::getInstance()).COLON_SEP.clone());
                    jcall!(stringBuffer).append(jcall!(toFile).getPath());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(Self::getInstance()), String::from("copyFile"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                return false;
            }
        } else {
            jcall!(toFile).createNewFile();
        }
        return true;
    }
    

    // throws Exception
    pub fn copyFile(&mut self, mut fromFile: Option<Rc<RefCell<AbFile>>>, mut toFile: Option<Rc<RefCell<AbFile>>>) {
        self.copyFile4(fromFile.clone(), toFile.clone(), false, false);
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn copyFile4(&mut self, mut fromFile: Option<Rc<RefCell<AbFile>>>, mut toFile: Option<Rc<RefCell<AbFile>>>, mut overwriteNewer: bool, mut overwriteAll: bool) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(self.logConfigTypeFactory).FILE.clone()) {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("Copying "));
                jcall!(stringBuffer).appendlong(((jcall!(fromFile).length()) as i64));
                jcall!(stringBuffer).append(String::from(" bytes from File: "));
                jcall!(stringBuffer).append(jcall!(fromFile).getPath());
                jcall!(stringBuffer).append(String::from(" to: "));
                jcall!(stringBuffer).append(jcall!(toFile).getPath());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(Self::getInstance()), String::from("copyFile"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            if !self.copyPrepare(fromFile.clone(), toFile.clone(), overwriteNewer.clone(), overwriteAll.clone()) {
                return;
            }
            let mut dataOutputStream: Option<Rc<RefCell<AbDataOutputStream>>> = jcall!(DataOutputStreamFactory::getInstance()).getInstanceForAbFile(toFile.clone());
            let mut fileInputStream: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStreamDyn>>> = AbFileInputStream::new1(fromFile.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStreamDyn>>);
            self.copy(fileInputStream.clone(), dataOutputStream.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(self.logConfigTypeFactory).FILEERROR.clone()) {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("Error Copying File fromFile: "));
                jcall!(stringBuffer).append(jcall!(fromFile).toString());
                jcall!(stringBuffer).append(String::from(" to: "));
                jcall!(stringBuffer).append(jcall!(toFile).toString());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(Self::getInstance()), String::from("copyFile"), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
            panic!("{:?}", e);
            }
        }
    }
    

    pub fn getNewDirectory(mut fromFile: Option<Rc<RefCell<AbFile>>>) -> String {
        let mut newDirectory: String = jcall!(fromFile).getPath();
        let mut separatorChar: String = jcall!(FilePathData::FilePathData::getInstance()).PATH_START.clone();
        if jcall!(AbIOSystem::getInstance()).isType(String::from("com.vobject.appengine.java.io")) {
            separatorChar = jcall!(AbPathData::getInstance()).SEPARATOR.clone();
        }
        let mut lastIndex: i32 = newDirectory.rfind(&separatorChar.clone()).map(|index| index as i32).unwrap_or(-1);
        if lastIndex >= (newDirectory.chars().count() as i32) {
            newDirectory = newDirectory[(0) as usize..((newDirectory.chars().count() as i32) - 1) as usize].to_string();
            lastIndex = newDirectory.rfind(&separatorChar.clone()).map(|index| index as i32).unwrap_or(-1);
        }
        return newDirectory[(lastIndex) as usize..((newDirectory.chars().count() as i32)) as usize].to_string();
    }
    

    // throws Exception
    pub fn copyDirectoryPortion(&mut self, mut fromDirectoryAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>, mut toDirectoryAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>, mut overwriteNewer: bool, mut overwriteAll: bool, mut current: i32, mut total: i32) {
        let mut file: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFileFromAbPath(fromDirectoryAbPath.clone());
        if !jcall!(file).isDirectory() {
            panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("Not a directory: "), jcall!(file).getPath()))));
        }
        let mut fileList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(self.directory).search2(file.clone(), true);
        let mut size: i32 = jcall!(fileList).size();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("Searched: "));
        jcall!(stringBuffer).append(jcall!(file).getPath());
        jcall!(stringBuffer).append(String::from(" Total: "));
        jcall!(stringBuffer).appendint(size.clone());
        let mut portion: i32 = size / total + 1;
        let mut start: i32 = portion * current;
        let mut end: i32 = start + portion;
        if end > size {
            end = size.clone();
        }
        jcall!(stringBuffer).append(String::from(" Section: "));
        jcall!(stringBuffer).appendint(start.clone());
        jcall!(stringBuffer).append(String::from(" - "));
        jcall!(stringBuffer).appendint(end.clone());
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(self.logConfigTypeFactory).FILE.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(Self::getInstance()), String::from("copySomeFilesToDirectory()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut nextFile: Option<Rc<RefCell<AbFile>>> = None;
        for index in (start..end) {
            nextFile = { let java_object = (jcall!(fileList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<AbFile>>>>().expect("ClassCastException").clone()) };
            if jcall!(nextFile).isDirectory() {
            } else {
                let mut path: String = jcall!(AbPath::new(jcall!(nextFile).getPath(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>)).getPath();
                let mut beginIndex: i32 = jcall!(jcall!(fromDirectoryAbPath).getPath()).length();
                if jcall!(path).as_AbPath_mut().indexOf(URLGLOBALS::getWebappPath()) < 0 {
                    beginIndex -= jcall!(jcall!(AbPath::new(URLGLOBALS::getWebappPath(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone())).getPath()).length();
                }
                let mut newPath: String = format!("{}{}", jcall!(toDirectoryAbPath).toFileSystemString(), jcall!(path).as_AbPath_mut().substring(beginIndex.clone()));
                let mut toFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFileFromAbPath(AbPath::new(newPath.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>));
                self.copyFile4(nextFile.clone(), toFile.clone(), overwriteNewer.clone(), overwriteAll.clone());
            }
        }
    }
    

    // throws Exception
    pub fn copyDirectory(&mut self, mut fromFile: Option<Rc<RefCell<AbFile>>>, mut to: Option<Rc<RefCell<AbFile>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(self.logConfigTypeFactory).FILE.clone()) {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("Copying Directory from: "));
                jcall!(stringBuffer).append(jcall!(fromFile).getPath());
                jcall!(stringBuffer).append(String::from(" to: "));
                jcall!(stringBuffer).append(jcall!(to).getPath());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(Self::getInstance()), String::from("copyDirectory"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            let mut newDirectory: String = FileUtil::getNewDirectory(fromFile.clone());
            let mut newDirectoryAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(format!("{}{}", format!("{}{}", jcall!(to).getPath(), jcall!(AbPathData::getInstance()).SEPARATOR), newDirectory), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
            if !jcall!(self.directory).create(newDirectoryAbPath.clone()) {
                panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("Failed to create directory: "), newDirectoryAbPath))));
            }
            let mut fileArray: Vec<Option<Rc<RefCell<AbFile>>>> = FileWrapperUtil::FileWrapperUtil::wrapFiles(jcall!(fromFile).listFiles());
            let mut size: i32 = (fileArray.len() as i32).clone();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(self.logConfigTypeFactory).FILE.clone()) {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("Copying "));
                jcall!(stringBuffer).appendint(size.clone());
                jcall!(stringBuffer).append(String::from(" files from: "));
                jcall!(stringBuffer).append(jcall!(fromFile).getPath());
                jcall!(stringBuffer).append(String::from(" to: "));
                jcall!(stringBuffer).append(jcall!(to).getPath());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(Self::getInstance()), String::from("copyDirectory"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            let mut file: Option<Rc<RefCell<AbFile>>> = None;
            for index in (0..size) {
                file = fileArray[(index) as usize].clone();
                if jcall!(file).isFile() {
                    { let java_arguments = (file.clone(), AbFile::createAbFilePathAndName(jcall!(newDirectoryAbPath).toString(), jcall!(file).getName()),); self.copyFile(java_arguments.0, java_arguments.1) };
                } else if jcall!(file).isDirectory() {
                    { let java_arguments = (file.clone(), AbFile::createAbFileFromAbPath(newDirectoryAbPath.clone()),); self.copyDirectory(java_arguments.0, java_arguments.1) };
                } else {
                    panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("File Copy Error"))));
                }
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(self.logConfigTypeFactory).FILEERROR.clone()) {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("Error Copying Directory fromFile: "));
                jcall!(stringBuffer).append(jcall!(fromFile).toString());
                jcall!(stringBuffer).append(String::from(" to: "));
                jcall!(stringBuffer).append(jcall!(to).toString());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(Self::getInstance()), String::from("copyDirectory"), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    // synchronized
    pub fn copy(&mut self, mut fromAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>, mut to: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> bool {
        let mut COPY: String = String::from("copy");
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(self.logConfigTypeFactory).FILE.clone()) {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("Copying AbPaths from: "));
                jcall!(stringBuffer).append(jcall!(fromAbPath).toString());
                jcall!(stringBuffer).append(String::from(" to: "));
                jcall!(stringBuffer).append(jcall!(to).toString());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(Self::getInstance()), COPY.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            if fromAbPath.is_none() {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Cannot Copy From A Null Location"))));
            }
            if to.is_none() {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Cannot Copy To A Null Location"))));
            }
            let mut fromLocationFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFileFromAbPath(fromAbPath.clone());
            let mut toLocationFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFileFromAbPath(to.clone());
            if jcall!(fromLocationFile).isFile() {
                if jcall!(toLocationFile).isDirectory() {
                    let mut file: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFileWithChild(toLocationFile.clone(), jcall!(fromLocationFile).getName());
                    self.copyFile(fromLocationFile.clone(), file.clone());
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(self.logConfigTypeFactory).FILE.clone()) {
                        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                        jcall!(stringBuffer).append(String::from("Copied file="));
                        jcall!(stringBuffer).append(jcall!(fromLocationFile).getName());
                        jcall!(stringBuffer).append(String::from(" from: "));
                        jcall!(stringBuffer).append(jcall!(fromLocationFile).getPath());
                        jcall!(stringBuffer).append(String::from(" to: "));
                        jcall!(stringBuffer).append(jcall!(toLocationFile).getPath());
                        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(Self::getInstance()), COPY.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    }
                } else {
                    self.copyFile(fromLocationFile.clone(), toLocationFile.clone());
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(self.logConfigTypeFactory).FILE.clone()) {
                        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                        jcall!(stringBuffer).append(String::from("Copied file with new name "));
                        jcall!(stringBuffer).append(String::from(" from: "));
                        jcall!(stringBuffer).append(jcall!(fromLocationFile).getPath());
                        jcall!(stringBuffer).append(String::from(" to: "));
                        jcall!(stringBuffer).append(jcall!(toLocationFile).getPath());
                        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(Self::getInstance()), COPY.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    }
                }
            } else if jcall!(fromLocationFile).isDirectory() && jcall!(toLocationFile).isDirectory() {
                if match (&jcall!(jcall!(fromAbPath).toFileSystemString()).charAt(jcall!(jcall!(fromAbPath).toFileSystemString()).length() - 1), &std::path::MAIN_SEPARATOR) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(self.logConfigTypeFactory).FILE.clone()) {
                        { let java_receiver = self.logUtil.clone(); let java_arguments = (String::from("Copying subdirectories"), crate::java_into_object(Self::getInstance()), COPY.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    }
                    let mut fileArray: Vec<Option<Rc<RefCell<AbFile>>>> = FileWrapperUtil::FileWrapperUtil::wrapFiles(jcall!(fromLocationFile).listFiles());
                    for index in (0..(fileArray.len() as i32)) {
                        let mut file: Option<Rc<RefCell<AbFile>>> = fileArray[(index) as usize].clone();
                        if jcall!(file).isFile() {
                            let mut aFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFileWithChild(toLocationFile.clone(), jcall!(file).getName());
                            self.copyFile(file.clone(), aFile.clone());
                        } else if jcall!(file).isDirectory() {
                            self.copyDirectory(file.clone(), toLocationFile.clone());
                        } else {
                            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("File Copy Error"))));
                        }
                    }
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(self.logConfigTypeFactory).FILE.clone()) {
                        { let java_receiver = self.logUtil.clone(); let java_arguments = (String::from("Copied subdirectories"), crate::java_into_object(Self::getInstance()), COPY.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    }
                } else {
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(self.logConfigTypeFactory).FILE.clone()) {
                        { let java_receiver = self.logUtil.clone(); let java_arguments = (String::from("Copyinhg directory"), crate::java_into_object(Self::getInstance()), COPY.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    }
                    self.copyDirectory(fromLocationFile.clone(), toLocationFile.clone());
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(self.logConfigTypeFactory).FILE.clone()) {
                        { let java_receiver = self.logUtil.clone(); let java_arguments = (String::from("Copied directory"), crate::java_into_object(Self::getInstance()), COPY.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    }
                }
            }
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(self.logConfigTypeFactory).FILEERROR.clone()) {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("Error Copying fromAbPath: "));
                jcall!(stringBuffer).append(jcall!(fromAbPath).toString());
                jcall!(stringBuffer).append(String::from(" to: "));
                jcall!(stringBuffer).append(jcall!(to).toString());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(Self::getInstance()), COPY.clone(), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
            panic!("{:?}", e);
            }
        }
    }
    

    pub fn shouldSkip(&mut self, mut file: Option<Rc<RefCell<AbFile>>>, mut skipFiles: Vec<String>) -> bool {
        let mut size: i32 = (skipFiles.len() as i32).clone();
        let mut skipFile: String = String::new();
        for index in (0..size) {
            skipFile = skipFiles[(index) as usize].clone();
            if jcall!(jcall!(file).getPath()).compareTo(skipFile.clone()) == 0 {
                return true;
            }
        }
        return false;
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn write(&mut self, mut filePath: String, mut string: String) {
        let mut dataOutputStream: Option<Rc<RefCell<AbDataOutputStream>>> = None;
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            PreLogUtil::put(jcall!(StringMaker::new()).append(self.WRITE_LABEL.clone()).append(filePath.clone()).append(self.DATA_LABEL.clone()).append(string.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.WRITE_METHOD.clone());
            let mut fileInputStreamFactory: Option<Rc<RefCell<FileStreamFactory>>> = FileStreamFactory::getInstance();
            let mut fileOutputStream: Option<Rc<RefCell<OutputStream>>> = jcall!(fileInputStreamFactory).getFileOutputStreamInstance(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), filePath.clone());
            dataOutputStream = AbDataOutputStream::new(fileOutputStream.clone());
            jcall!(dataOutputStream).write(string.bytes().map(|value| value as i8).collect::<Vec<i8>>(), 0, (string.chars().count() as i32));
            jcall!(dataOutputStream).flush();
        })) {
            Ok(_) => {}
            Err(error) => std::panic::resume_unwind(error),
        }
        { // finally
            jcall!(StreamUtil::getInstance()).close(dataOutputStream.clone());
        }
    }
    

    pub fn loadFileAsList(&mut self, mut file: Option<Rc<RefCell<AbFile>>>, mut max: i32, mut byteArray1: Vec<i8>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return jcall!(FileUtil2::FileUtil2::getInstance()).loadFileAsList(file.clone(), max.clone(), byteArray1.clone());
    }
}

