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
// use crate::java::nio::channels::FileChannel::FileChannel;
// use crate::java::nio::channels::FileLock::FileLock;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::AbFileOutputStream::AbFileOutputStream;
use crate::org::allbinary::logic::io::StreamUtil::StreamUtil;
use super::AbFile::AbFile;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FileLockUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static FileLockUtil_instance: Option<Rc<RefCell<FileLockUtil>>> = FileLockUtil::new(); }

impl FileLockUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<FileLockUtil>>> {
        return FileLockUtil_instance.with(Clone::clone).clone();
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
    pub fn getAll(&mut self, mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut isReturnOnFailure: bool) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut fileLockVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut size: i32 = jcall!(vector).size();
        for index in (0..size) {
            let mut file: Option<Rc<RefCell<AbFile>>> = { let java_object = (jcall!(vector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<AbFile>>>>().expect("ClassCastException").clone()) };
            let mut fileLock: Option<Rc<RefCell<FileLock>>> = self.getLock(file.clone());
            if fileLock.is_some() {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("File Lock Obtained: "), jcall!(file).getAbsolutePath()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getAll"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                jcall!(fileLockVector).add(crate::java_into_object(fileLock.clone()));
            } else if isReturnOnFailure {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Total Locks Obtained: "), jcall!(fileLockVector).size()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getAll"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                return fileLockVector.clone();
            }
        }
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Total Locks Obtained: "), jcall!(fileLockVector).size()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getAll"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        return fileLockVector.clone();
    }
    

    // throws Exception
    pub fn getAllPossible(&mut self, mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.getAll(vector.clone(), false);
    }
    

    // throws Exception
    pub fn getAllOrNone(&mut self, mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut fileLockVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getAll(vector.clone(), true);
        if jcall!(vector).size() != jcall!(fileLockVector).size() {
            return BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        } else {
            return fileLockVector.clone();
        }
    }
    

    // throws Exception
    pub fn getLock(&mut self, mut file: Option<Rc<RefCell<AbFile>>>) -> Option<Rc<RefCell<FileLock>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return { let java_arguments = (AbFileOutputStream::createFromAbFileAppend(file.clone(), true),); self.getLock(java_arguments.0) };
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(String::from("Exception returns null"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getLock"), crate::java_into_object(e.clone()));
            return None;
            }
        }
        { // finally
        }
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getLock(&mut self, mut fileOutputStream: Option<Rc<RefCell<AbFileOutputStream>>>) -> Option<Rc<RefCell<FileLock>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut fileLock: Option<Rc<RefCell<FileLock>>> = { let java_arguments = (jcall!(fileOutputStream).getChannel(),); self.getLock(java_arguments.0) };
            return fileLock.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(String::from("Exception returns null"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getLock"), crate::java_into_object(e.clone()));
            return None;
            }
        }
        { // finally
            jcall!(self.logUtil).putF(String::from("Finally - Closing FileOutputStream"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getLock"));
            jcall!(StreamUtil::getInstance()).close(fileOutputStream.clone());
        }
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getLock(&mut self, mut fileChannel: Option<Rc<RefCell<FileChannel>>>) -> Option<Rc<RefCell<FileLock>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut fileLock: Option<Rc<RefCell<FileLock>>> = jcall!(fileChannel).tryLock();
            return fileLock.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(String::from("Exception returns null"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getLock"), crate::java_into_object(e.clone()));
            return None;
            }
        }
        { // finally
            jcall!(self.logUtil).putF(String::from("Finally - Closing FileChannel"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getLock"));
            jcall!(fileChannel).close();
        }
    }
}

