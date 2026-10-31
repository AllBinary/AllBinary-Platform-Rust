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
// package org.allbinary.log
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::BufferedReader::BufferedReader;
// use crate::java::io::BufferedWriter::BufferedWriter;
// use crate::java::io::File::File;
// use crate::java::io::FileReader::FileReader;
// use crate::java::io::FileWriter::FileWriter;
// use crate::java::io::RandomAccessFile::RandomAccessFile;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::communication::log::LogFormatUtil::LogFormatUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FileLog {
}

thread_local! { pub static FileLog_stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance(); }
thread_local! { pub static FileLog_logLength: i64 = 33000000_i64; }
thread_local! { pub static FileLog_logPath: String = String::from("g:\\log\\"); }
thread_local! { pub static FileLog_extension: String = String::from("dat").clone(); }
thread_local! { pub static FileLog_fileName: String = format!("{}{}", String::from("log."), FileLog_extension.with(Clone::clone)).clone(); }
thread_local! { pub static FileLog_backupFileName: String = jcall!(FileLog_fileName.with(Clone::clone)).concat(String::from(".bak")); }
thread_local! { pub static FileLog_ORG_ALLBINARY: String = String::from("org.allbinary: "); }
pub static mut FileLog_firstTime: std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| true);
pub static mut FileLog_logFile: std::cell::LazyCell<Option<Rc<RefCell<File>>>> = std::cell::LazyCell::new(|| File::new(FileLog_logPath.with(Clone::clone).clone(), FileLog_fileName.with(Clone::clone).clone()));
pub static mut FileLog_logFileBak: std::cell::LazyCell<Option<Rc<RefCell<File>>>> = std::cell::LazyCell::new(|| None);
pub static mut FileLog_fileOut: std::cell::LazyCell<Option<Rc<RefCell<BufferedWriter>>>> = std::cell::LazyCell::new(|| None);
pub static mut FileLog_backupIndex: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| 0);

impl FileLog {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // synchronized
    pub fn createLogFile() -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            unsafe { *FileLog_logFile = File::new(FileLog_logPath.with(Clone::clone).clone(), FileLog_fileName.with(Clone::clone).clone()) };
            if !unsafe { FileLog_firstTime.clone() } {
                unsafe { *FileLog_fileOut = BufferedWriter::new(FileWriter::new(unsafe { FileLog_logFile.clone() }.clone())) };
            } else {
                unsafe { *FileLog_firstTime = false };
                let mut raFile: Option<Rc<RefCell<RandomAccessFile>>> = RandomAccessFile::new(unsafe { FileLog_logFile.clone() }.clone(), String::from("rw"));
                jcall!(raFile).seek(jcall!(raFile).length());
                unsafe { *FileLog_fileOut = BufferedWriter::new(FileWriter::new(jcall!(raFile).getFD())) };
            }
            let mut canWrite: bool = jcall!(unsafe { FileLog_logFile.clone() }).canWrite();
            return canWrite.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            println!("{}", format!("{}{}", String::from("Error Creating Log: "), e));
            return false;
            }
        }
    }
    

    // synchronized
    pub fn createLogFileBackup() -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringMaker: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            unsafe { *FileLog_logFileBak = File::new(FileLog_logPath.with(Clone::clone).clone(), jcall!(stringMaker).append(FileLog_backupFileName.with(Clone::clone).clone()).append(jcall!(CommonSeps::getInstance()).PERIOD.clone()).appendint(unsafe { FileLog_backupIndex.clone() }.clone()).toString()) };
            while jcall!(unsafe { FileLog_logFileBak.clone() }).isFile() {
                unsafe { *FileLog_backupIndex += 1 };
                jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
                unsafe { *FileLog_logFileBak = File::new(FileLog_logPath.with(Clone::clone).clone(), jcall!(stringMaker).append(FileLog_backupFileName.with(Clone::clone).clone()).append(jcall!(CommonSeps::getInstance()).PERIOD.clone()).appendint(unsafe { FileLog_backupIndex.clone() }.clone()).toString()) };
            }
            let mut line: String = String::new();
            let mut tmpOut: Option<Rc<RefCell<BufferedWriter>>> = BufferedWriter::new(FileWriter::new(unsafe { FileLog_logFileBak.clone() }.clone()));
            let mut tmpIn: Option<Rc<RefCell<BufferedReader>>> = BufferedReader::new(FileReader::new(unsafe { FileLog_logFile.clone() }.clone()));
            while ({ line = jcall!(tmpIn).readLine(); line.clone() }).is_some() {
                jcall!(tmpOut).write(line.clone(), 0, (line.chars().count() as i32));
                jcall!(tmpOut).newLine();
            }
            jcall!(tmpOut).flush();
            jcall!(tmpOut).close();
            jcall!(tmpIn).close();
            jcall!(unsafe { FileLog_logFile.clone() }).delete();
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            println!("{}", format!("{}{}", String::from("Error Creating Backup: "), e));
            return false;
            }
        }
    }
    

    // synchronized
    pub fn put(mut specialMessage: String, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut functionName: String) -> String {
        return FileLog::put4(specialMessage.clone(), anyType.clone(), functionName.clone(), None);
    }
    

    // Java overloaded method: Rust needs a different name
    // synchronized
    pub fn put4(mut specialMessage: String, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut functionName: String, mut exception: String) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if unsafe { FileLog_firstTime.clone() } == true {
                FileLog::createLogFileBackup();
                if Self::createLogFile() == false {
                    return String::from("Log File Creation Error").clone();
                }
            }
            let mut length: i64 = ((jcall!(unsafe { FileLog_logFile.clone() }).length()) as i64);
            if length > FileLog_logLength.with(Clone::clone) {
                if !(Self::createLogFileBackup()) || Self::createLogFile() == false {
                    return String::from("Couldn't Create a Backup file").clone();
                }
            }
            let mut className: String = jcall!(FileLog_stringUtil.with(Clone::clone)).NULL_STRING.clone().clone();
            if false {
                functionName = jcall!(FileLog_stringUtil.with(Clone::clone)).NULL_STRING.clone().clone();
            }
            if false {
                specialMessage = jcall!(FileLog_stringUtil.with(Clone::clone)).NULL_STRING.clone().clone();
            }
            if true {
                className = std::any::type_name_of_val(&anyType).to_string().clone();
            }
            let mut message: String = jcall!(LogFormatUtil::getInstance()).get(className.clone(), functionName.clone(), specialMessage.clone(), crate::java_into_object(exception.clone()));
            jcall!(unsafe { FileLog_fileOut.clone() }).write(message.clone(), 0, (message.chars().count() as i32));
            jcall!(unsafe { FileLog_fileOut.clone() }).newLine();
            jcall!(unsafe { FileLog_fileOut.clone() }).flush();
            return format!("{}{}", FileLog_ORG_ALLBINARY.with(Clone::clone), message).clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            return format!("{}{}", String::from("Error: "), e).clone();
            }
        }
    }
    

    // Java overloaded method: Rust needs a different name
    // synchronized
    pub fn put(mut specialMessage: String, mut className: String, mut functionName: String) -> String {
        return FileLog::put4(specialMessage.clone(), className.clone(), functionName.clone(), None);
    }
    

    // Java overloaded method: Rust needs a different name
    // synchronized
    pub fn put4(mut specialMessage: String, mut className: String, mut functionName: String, mut exception: String) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if unsafe { FileLog_firstTime.clone() } == true {
                FileLog::createLogFileBackup();
                if Self::createLogFile() == false {
                    return String::from("Log File Creation Error").clone();
                }
            }
            let mut length: i64 = ((jcall!(unsafe { FileLog_logFile.clone() }).length()) as i64);
            if length > FileLog_logLength.with(Clone::clone) {
                if !(Self::createLogFileBackup()) || Self::createLogFile() == false {
                    return String::from("Couldn't Create a Backup file").clone();
                }
            }
            if false {
                functionName = jcall!(FileLog_stringUtil.with(Clone::clone)).NULL_STRING.clone().clone();
            }
            if false {
                specialMessage = jcall!(FileLog_stringUtil.with(Clone::clone)).NULL_STRING.clone().clone();
            }
            let mut message: String = jcall!(LogFormatUtil::getInstance()).get(className.clone(), functionName.clone(), specialMessage.clone(), crate::java_into_object(exception.clone()));
            jcall!(unsafe { FileLog_fileOut.clone() }).write(message.clone(), 0, (message.chars().count() as i32));
            jcall!(unsafe { FileLog_fileOut.clone() }).newLine();
            jcall!(unsafe { FileLog_fileOut.clone() }).flush();
            return format!("{}{}", FileLog_ORG_ALLBINARY.with(Clone::clone), message).clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            return format!("{}{}", String::from("Error: "), e).clone();
            }
        }
    }
    

    pub fn getFilePath() -> String {
        return format!("{}{}", FileLog_logPath.with(Clone::clone), FileLog_fileName.with(Clone::clone));
    }
}

