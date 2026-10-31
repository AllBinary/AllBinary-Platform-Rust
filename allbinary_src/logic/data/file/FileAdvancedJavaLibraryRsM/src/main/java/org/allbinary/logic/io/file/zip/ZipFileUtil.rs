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
// package org.allbinary.logic.io.file.zip
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::util::zip::ZipEntry::ZipEntry;
// use crate::java::util::zip::ZipInputStream::ZipInputStream;
// use crate::java::util::zip::ZipOutputStream::ZipOutputStream;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::AbDataOutputStream::AbDataOutputStream;
use crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStream;
use crate::org::allbinary::logic::io::AbFileOutputStream::AbFileOutputStream;
use crate::org::allbinary::logic::io::DataOutputStreamFactory::DataOutputStreamFactory;
use crate::org::allbinary::logic::io::StreamUtil::StreamUtil;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::FileUtil::FileUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ZipFileUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

thread_local! { pub static ZipFileUtil_instance: Option<Rc<RefCell<ZipFileUtil>>> = ZipFileUtil::new(); }

impl ZipFileUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ZipFileUtil>>> {
        return ZipFileUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn create(&mut self, mut outFilename: String, mut fileBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut zipFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(outFilename.clone());
            if jcall!(zipFile).exists() {
                jcall!(zipFile).delete();
            }
            jcall!(zipFile).createNewFile();
            let mut outputStream: Option<Rc<RefCell<ZipOutputStream>>> = ZipOutputStream::new(AbFileOutputStream::createFromAbFile(zipFile.clone()));
            let mut streamUtil: Option<Rc<RefCell<StreamUtil>>> = StreamUtil::getInstance();
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            let mut byteArray: Vec<i8> = vec![0; (16384) as usize];
            let mut fileInputStream: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStreamDyn>>> = None;
            let mut size: i32 = jcall!(fileBasicArrayList).size();
            let mut current: i32 = 0;
            for i in (0..size) {
                let mut file: Option<Rc<RefCell<AbFile>>> = { let java_object = (jcall!(fileBasicArrayList).get(i.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<AbFile>>>>().expect("ClassCastException").clone()) };
                if jcall!(file).isDirectory() {
                } else if jcall!(file).isFile() {
                    jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                    jcall!(stringBuffer).appendint(current.clone());
                    jcall!(stringBuffer).append(String::from(" Creating Zip File Entry: "));
                    jcall!(stringBuffer).append(jcall!(file).getPath());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("create()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        fileInputStream = AbFileInputStream::new1(file.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStreamDyn>>);
                        jcall!(outputStream).putNextEntry(ZipEntry::new(jcall!(file).getPath()));
                        jcall!(streamUtil).get(fileInputStream.clone(), outputStream.clone(), byteArray.clone());
                        jcall!(outputStream).closeEntry();
                        jcall!(streamUtil).close(fileInputStream.clone());
                    })) {
                        Ok(_) => {}
                        Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Skipping File (Probably Local): "), jcall!(file).getPath()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("create()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                        }
                    }
                }
                current += 1;
            }
            jcall!(streamUtil).close(outputStream.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("create"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn unzip(&mut self, mut path: String, mut zipFile: Option<Rc<RefCell<AbFile>>>, mut fileName: String) {
        let mut buffer: Vec<i8> = vec![0; (16384) as usize];
        let mut inputStream: Option<Rc<RefCell<ZipInputStream>>> = None;
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            inputStream = ZipInputStream::new(AbFileInputStream::new1(zipFile.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStreamDyn>>));
            let mut zipEntry: Option<Rc<RefCell<ZipEntry>>> = None;
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            let mut fileUtil: Option<Rc<RefCell<FileUtil>>> = FileUtil::getInstance();
            while ({ zipEntry = jcall!(inputStream).getNextEntry(); zipEntry.clone() }).is_some() {
                let mut entryName: String = jcall!(zipEntry).getName();
                if jcall!(zipEntry).isDirectory() {
                } else {
                    let mut entryFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(format!("{}{}", path, entryName));
                    jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                    jcall!(stringBuffer).append(String::from("ZipEntry Name: "));
                    jcall!(stringBuffer).append(entryName.clone());
                    jcall!(stringBuffer).append(String::from(" isFile: "));
                    jcall!(stringBuffer).appendboolean(jcall!(entryFile).isFile());
                    jcall!(stringBuffer).append(String::from(" isDirectory: "));
                    jcall!(stringBuffer).appendboolean(jcall!(entryFile).isDirectory());
                    jcall!(stringBuffer).append(String::from(" getParent: "));
                    jcall!(stringBuffer).append(jcall!(entryFile).getParent());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("unzip"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    let mut dataOutputStream: Option<Rc<RefCell<AbDataOutputStream>>> = jcall!(DataOutputStreamFactory::getInstance()).getInstanceForAbFile(entryFile.clone());
                    jcall!(fileUtil).write3(inputStream.clone(), dataOutputStream.clone(), buffer.clone());
                }
                jcall!(inputStream).closeEntry();
            }
        })) {
            Ok(_) => {}
            Err(error) => std::panic::resume_unwind(error),
        }
        { // finally
            jcall!(StreamUtil::getInstance()).close(inputStream.clone());
        }
    }
}

