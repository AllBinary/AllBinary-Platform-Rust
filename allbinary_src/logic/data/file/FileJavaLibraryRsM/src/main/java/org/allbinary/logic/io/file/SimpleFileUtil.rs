// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
// use crate::java::io::ByteArrayOutputStream::ByteArrayOutputStream;
// use crate::java::io::IOException::IOException;
// use crate::java::io::InputStream::InputStream;
// use crate::java::io::OutputStream::OutputStream;
// use crate::java::io::Reader::Reader;
// use crate::java::io::Writer::Writer;
// use crate::java::nio::CharBuffer::CharBuffer;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::StreamUtil::StreamUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::CharSequence::CharSequence;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SimpleFileUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub NULL_WRITER: Option<Rc<RefCell<Writer>>>,
    pub NULL_READER: Option<Rc<RefCell<Reader>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub nullUtil: Option<Rc<RefCell<NullUtil>>>,
    pub streamUtil: Option<Rc<RefCell<StreamUtil>>>,
}

thread_local! { pub static SimpleFileUtil_instance: Option<Rc<RefCell<SimpleFileUtil>>> = SimpleFileUtil::new(); }

impl SimpleFileUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            NULL_WRITER: None,
            NULL_READER: None,
            commonStrings: None,
            nullUtil: None,
            streamUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.NULL_WRITER = SimpleFileUtil::nullWriter();
        this.NULL_READER = SimpleFileUtil::nullReader();
        this.commonStrings = CommonStrings::getInstance();
        this.nullUtil = NullUtil::getInstance();
        this.streamUtil = StreamUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<SimpleFileUtil>>> {
        return SimpleFileUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn nullOutputStream() -> Option<Rc<RefCell<OutputStream>>> {
        return OutputStream::new() /* Java anonymous class body: implement it as a named type
            

            // throws IOException
            pub fn write(&mut self, mut b: i32) {
            }
            

            // throws IOException
            pub fn write(&mut self, mut b: Vec<i8>, mut off: i32, mut len: i32) {
            }
            

            pub fn close(&mut self) {
            }
*/;
    }
    

    pub fn nullWriter() -> Option<Rc<RefCell<Writer>>> {
        return Writer::new() /* Java anonymous class body: implement it as a named type
            closed: false,
            

            // throws IOException
            pub fn append(&mut self, mut c: char) -> Option<Rc<RefCell<Writer>>> {
                return self;
            }
            

            // throws IOException
            pub fn append(&mut self, mut csq: Option<Rc<RefCell<dyn CharSequence>>>) -> Option<Rc<RefCell<Writer>>> {
                return self;
            }
            

            // throws IOException
            pub fn append(&mut self, mut csq: Option<Rc<RefCell<dyn CharSequence>>>, mut start: i32, mut end: i32) -> Option<Rc<RefCell<Writer>>> {
                return self;
            }
            

            // throws IOException
            pub fn write(&mut self, mut c: i32) {
            }
            

            // throws IOException
            pub fn write(&mut self, mut cbuf: Vec<char>, mut off: i32, mut len: i32) {
            }
            

            // throws IOException
            pub fn write(&mut self, mut str: String) {
            }
            

            // throws IOException
            pub fn write(&mut self, mut str: String, mut off: i32, mut len: i32) {
            }
            

            // throws IOException
            pub fn flush(&mut self) {
            }
            

            // throws IOException
            pub fn close(&mut self) {
            }
*/;
    }
    

    pub fn nullReader() -> Option<Rc<RefCell<Reader>>> {
        return Reader::new() /* Java anonymous class body: implement it as a named type
            

            // throws IOException
            pub fn read(&mut self) -> i32 {
                return -1;
            }
            

            // throws IOException
            pub fn read(&mut self, mut cbuf: Vec<char>, mut off: i32, mut len: i32) -> i32 {
                return -1;
            }
            

            // throws IOException
            pub fn read(&mut self, mut target: Option<Rc<RefCell<CharBuffer>>>) -> i32 {
                return -1;
            }
            

            // throws IOException
            pub fn ready(&mut self) -> bool {
                return false;
            }
            

            // throws IOException
            pub fn skip(&mut self, mut n: i64) -> i64 {
                return 0_i64;
            }
            

            pub fn close(&mut self) {
            }
*/;
    }
    

    pub fn loadFileAsList(&mut self, mut inputStream: Option<Rc<RefCell<InputStream>>>, mut max: i32, mut byteArray1: Vec<i8>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.loadFileAsListReturnLine(inputStream.clone(), max.clone(), byteArray1.clone(), 0);
    }
    

    pub fn loadFileAsListReturnLine(&mut self, mut inputStream: Option<Rc<RefCell<InputStream>>>, mut max: i32, mut byteArray1: Vec<i8>, mut includeReturnLine: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut stringList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        if inputStream.is_none() {
            return stringList.clone();
        }
        let mut byteArray: Vec<i8> = jcall!(self.nullUtil).NULL_BYTE_ARRAY.clone();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut outputStream2: Vec<u8> = Vec::new();
            byteArray = jcall!(self.streamUtil).getByteArray(inputStream.clone(), outputStream2.clone(), byteArray1.clone());
            jcall!(self.streamUtil).close(inputStream.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.streamUtil).close(inputStream.clone());
            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).ADD.clone(), crate::java_into_object(e.clone()));
            }
        }
        if byteArray.is_none() {
            return stringList.clone();
        }
        let mut size: i32 = (byteArray.len() as i32).clone();
        let mut index: i32 = 0;
        let mut startIndex: i32 = 0;
        let mut returnLine: i32 = 0;
        while index < size {
            startIndex = index.clone();
            while index < size - 1 && ((byteArray[(index) as usize]) as i32) != (((('\n') as i8)) as i32) {
                index += 1;
            }
            if includeReturnLine == 0 {
                if index > 0 && ((byteArray[(index - 1) as usize]) as i32) == (((('\r') as i8)) as i32) {
                    returnLine = 1;
                } else {
                    returnLine = 0;
                }
            }
            let mut s: String = byteArray[(startIndex) as usize..(startIndex + (index + includeReturnLine - returnLine - startIndex)) as usize].iter().collect::<String>();
            jcall!(stringList).add(crate::java_into_object(s.clone()));
            index += 1;
        }
        return stringList.clone();
    }
    

    pub fn createStringFromArrayOfStrings(&mut self, mut stringArray: Vec<String>) -> String {
        let mut stringMaker: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut size: i32 = (stringArray.len() as i32).clone();
        for index in (0..size) {
            jcall!(stringMaker).append(stringArray[(index) as usize].clone());
        }
        return jcall!(stringMaker).toString();
    }
}

