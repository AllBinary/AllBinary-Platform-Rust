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
// package org.allbinary.servlet
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::ByteArrayOutputStream::ByteArrayOutputStream;
use crate::java::io::IOException::IOException;
use crate::java::io::InputStream::InputStream;
// use crate::javax::servlet::ServletException::ServletException;
// use crate::javax::servlet::http::HttpServlet::HttpServlet;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
// use crate::javax::servlet::http::HttpServletResponse::HttpServletResponse;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::CloudStreamUtil::CloudStreamUtil;
use crate::org::allbinary::logic::io::StreamUtil::StreamUtil;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CloudImageServlet {
    pub base: HttpServlet,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}


impl CloudImageServlet {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: HttpServlet::new_struct(),
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
    

    // throws ServletException, IOException
    pub fn processRequest(&mut self, mut request: Option<Rc<RefCell<HttpServletRequest>>>, mut response: Option<Rc<RefCell<HttpServletResponse>>>) {
        let mut inputStream: Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>> = None;
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(BlisketServletUtil::BlisketServletUtil::getInstance()).init(request.clone());
            let mut requestURI: String = jcall!(request).getRequestURI();
            let mut file: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(format!("{}{}", URLGLOBALS::getWebappPath(), requestURI));
            inputStream = { let java_value = (jcall!(CloudStreamUtil::getInstance()).getFile(file.clone())).clone(); java_value };
            let mut outputStream: Option<Rc<RefCell<crate::java::io::OutputStream::OutputStream>>> = crate::java::io::OutputStream::OutputStream::new();
            let mut byteArray: Vec<i8> = jcall!(StreamUtil::getInstance()).getByteArray(inputStream.clone(), outputStream.clone(), vec![0; (16384) as usize]);
            jcall!(response).setContentType(String::from("image/jpeg;charset=utf-8"));
            jcall!(jcall!(response).getOutputStream()).write(byteArray.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEWERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("processRequest()"), crate::java_into_object(e.clone()));
            }
            }
        }
        { // finally
            jcall!(StreamUtil::getInstance()).close(jcall!(response).getOutputStream());
            if !jcall!(StreamUtil::getInstance()).close(inputStream.clone().map(|_java_reference| -> Rc<RefCell<dyn crate::java::io::Closeable::Closeable>> { panic!("ClassCastException: the Rust trait of InputStream does not have Closeable as a base trait") })) {
                jcall!(response).sendError(unsafe { crate::javax::servlet::http::HttpServletResponse::HttpServletResponse_SC_NOT_FOUND.clone() }.clone());
            }
        }
    }
    

    // throws ServletException, IOException
    pub fn doGet(&mut self, mut request: Option<Rc<RefCell<HttpServletRequest>>>, mut response: Option<Rc<RefCell<HttpServletResponse>>>) {
        self.processRequest(request.clone(), response.clone());
    }
    

    // throws ServletException, IOException
    pub fn doPost(&mut self, mut request: Option<Rc<RefCell<HttpServletRequest>>>, mut response: Option<Rc<RefCell<HttpServletResponse>>>) {
        self.processRequest(request.clone(), response.clone());
    }
    

    pub fn getServletInfo(&mut self) -> String {
        return String::from("Serves Up Cloud Image");
    }
}
impl std::ops::Deref for CloudImageServlet {
    type Target = HttpServlet;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CloudImageServlet {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



