// Generated Code Do Not Modify
// package org.apache.commons.fileupload
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::ByteArrayOutputStream::ByteArrayOutputStream;
use crate::java::io::InputStream::InputStream;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::StreamUtil::StreamUtil;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::AbFileNativeUtil::AbFileNativeUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::java::lang::Exception::*;
use super::FileItemStream::FileItemStream;
use super::FileItem::FileItem;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FileItemUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub DEFAULT_CHARSET: String,
}

thread_local! { pub static FileItemUtil_instance: Option<Rc<RefCell<FileItemUtil>>> = FileItemUtil::new(); }

impl FileItemUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            DEFAULT_CHARSET: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.DEFAULT_CHARSET = String::from("ISO-8859-1");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<FileItemUtil>>> {
        return FileItemUtil_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn getString(&mut self, mut byteArray: Vec<i8>) -> String {
        return String::new(byteArray.clone(), self.DEFAULT_CHARSET.clone());
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getString_FileItemStream(&mut self, mut fileItem: Option<Rc<RefCell<FileItemStream>>>) -> String {
        return String::new(self.getBytes(fileItem.clone()), self.DEFAULT_CHARSET.clone());
    }
    

    // throws Exception
    pub fn getBytes(&mut self, mut fileItem: Option<Rc<RefCell<FileItemStream>>>) -> Vec<i8> {
        let mut streamUtil: Option<Rc<RefCell<StreamUtil>>> = StreamUtil::getInstance();
        let mut inputStream: Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>> = None;
        let mut outputStream: Option<Rc<RefCell<crate::java::io::OutputStream::OutputStream>>> = None;
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            inputStream = { let java_value = (jcall!(fileItem).openStream()).clone(); java_value };
            outputStream = crate::java::io::OutputStream::OutputStream::new();
            if jcall!(fileItem).isFormField() {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("FileItemStream FieldName: "), jcall!(fileItem).getFieldName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("write()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            } else {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("Uploaded File FieldName: "));
                jcall!(stringBuffer).append(jcall!(fileItem).getFieldName());
                jcall!(stringBuffer).append(String::from(" name = "));
                jcall!(stringBuffer).append(jcall!(fileItem).getName());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("write()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            outputStream = { let java_value = ((jcall!(streamUtil).get(inputStream.clone(), outputStream.clone().map(|reference| reference as Rc<RefCell<dyn crate::java::io::OutputStream::OutputStreamDyn>>), vec![0; (16384) as usize])).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<ByteArrayOutputStream>() }, "ClassCastException: not a ByteArrayOutputStream"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<ByteArrayOutputStream>) } })).clone(); java_value };
            return Some(jcall!(outputStream).toByteArray());
        })) {
            Ok(java_return) => java_return,
            Err(error) => std::panic::resume_unwind(error),
        };
        { // finally
            jcall!(streamUtil).close(outputStream.clone().map(|reference| reference as Rc<RefCell<dyn crate::java::io::Closeable::Closeable>>));
            jcall!(streamUtil).close(inputStream.clone().map(|_java_reference| -> Rc<RefCell<dyn crate::java::io::Closeable::Closeable>> { panic!("ClassCastException: the Rust trait of InputStream does not have Closeable as a base trait") }));
        }if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // throws Exception
    pub fn write(&mut self, mut fileItem: Option<Rc<RefCell<FileItem>>>, mut file: Option<Rc<RefCell<AbFile>>>) {
        jcall!(fileItem).write(AbFileNativeUtil::get(file.clone()));
    }
}

