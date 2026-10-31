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
// package org.allbinary.servlet.file
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::io::IOException::IOException;
// use crate::java::io::InputStream::InputStream;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::ServletException::ServletException;
// use crate::javax::servlet::http::HttpServlet::HttpServlet;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
// use crate::javax::servlet::http::HttpServletResponse::HttpServletResponse;
use crate::admin::taghelpers::AuthenticationHelper::AuthenticationHelper;
use crate::admin::taghelpers::AuthenticationHelperFactory::AuthenticationHelperFactory;
use crate::admin::taghelpers::AuthenticationHelperUtil::AuthenticationHelperUtil;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::CloudStreamUtil::CloudStreamUtil;
use crate::org::allbinary::logic::io::StreamUtil::StreamUtil;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::servlet::BlisketServletUtil::BlisketServletUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DownloadFileServlet {
    pub base: HttpServlet,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub DOWNLOAD: String,
    pub DEFAULT_BUFFER_SIZE: i32,
}


impl DownloadFileServlet {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: HttpServlet::new_struct(),
            logUtil: None,
            commonStrings: None,
            DOWNLOAD: String::new(),
            DEFAULT_BUFFER_SIZE: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.DOWNLOAD = String::from("download");
        this.DEFAULT_BUFFER_SIZE = 16384;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws ServletException, IOException
    pub fn processRequest(&mut self, mut request: Option<Rc<RefCell<HttpServletRequest>>>, mut response: Option<Rc<RefCell<HttpServletResponse>>>) {
        let mut inputStream: Option<Rc<RefCell<InputStream>>> = None;
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(BlisketServletUtil::getInstance()).init(request.clone());
            let mut requestPath: String = jcall!(request).getRequestURI();
            if false {
                jcall!(response).sendError(unsafe { crate::javax::servlet::http::HttpServletResponse::HttpServletResponse_SC_NOT_FOUND.clone() }.clone());
                return;
            }
            let mut beginIndex: i32 = requestPath.find(&self.DOWNLOAD.clone()).map(|index| index as i32).unwrap_or(-1);
            let mut filePath: String = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
            if beginIndex >= 0 {
                filePath = requestPath[(beginIndex + (self.DOWNLOAD.chars().count() as i32)) as usize..].to_string();
            } else {
                jcall!(response).sendError(unsafe { crate::javax::servlet::http::HttpServletResponse::HttpServletResponse_SC_NOT_FOUND.clone() }.clone());
            }
            let mut file: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(format!("{}{}", URLGLOBALS::getWebappPath(), filePath));
            if !jcall!(file).exists() {
                jcall!(response).sendError(unsafe { crate::javax::servlet::http::HttpServletResponse::HttpServletResponse_SC_NOT_FOUND.clone() }.clone());
                return;
            }
            let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            let mut authenticationHelper: Option<Rc<RefCell<AuthenticationHelper>>> = (jcall!(AuthenticationHelperFactory::new()).getInstance(hashMap.clone(), request.clone())).clone() /* as AuthenticationHelper */;
            if jcall!(authenticationHelper).isAuthenticated() {
                if jcall!(AuthenticationHelperUtil::getInstance()).isAuthorized(authenticationHelper.clone(), filePath.clone()) {
                    inputStream = jcall!(CloudStreamUtil::getInstance()).getFile(file.clone());
                    let mut contentType: String = jcall!(self.getServletContext()).getMimeType(jcall!(file).getName());
                    if false {
                        contentType = String::from("application/octet-stream");
                    }
                    jcall!(response).reset();
                    jcall!(response).setBufferSize(self.DEFAULT_BUFFER_SIZE.clone());
                    jcall!(response).setContentType(contentType.clone());
                    jcall!(response).setHeader(String::from("Content-Length"), (jcall!(file).length()).to_string());
                    let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                    jcall!(stringBuffer).append(String::from("attachment; filename=\""));
                    jcall!(stringBuffer).append(jcall!(file).getName());
                    jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).QUOTE.clone());
                    jcall!(response).setHeader(String::from("Content-Disposition"), jcall!(stringBuffer).toString());
                    jcall!(StreamUtil::getInstance()).get(inputStream.clone(), jcall!(response).getOutputStream(), vec![0; (16348) as usize]);
                } else {
                    jcall!(response).sendError(unsafe { crate::javax::servlet::http::HttpServletResponse::HttpServletResponse_SC_UNAUTHORIZED.clone() }.clone(), String::from("You are not Authorized"));
                }
            } else {
                jcall!(response).sendError(unsafe { crate::javax::servlet::http::HttpServletResponse::HttpServletResponse_SC_UNAUTHORIZED.clone() }.clone(), String::from("Please Login"));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("processRequest()"), crate::java_into_object(e.clone()));
            }
            jcall!(response).sendError(unsafe { crate::javax::servlet::http::HttpServletResponse::HttpServletResponse_SC_NOT_FOUND.clone() }.clone());
            }
        }
        { // finally
            jcall!(StreamUtil::getInstance()).close(jcall!(response).getOutputStream());
            if !jcall!(StreamUtil::getInstance()).close(inputStream.clone()) {
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
        return String::from("Serves Up Cloud Files");
    }
}
impl std::ops::Deref for DownloadFileServlet {
    type Target = HttpServlet;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for DownloadFileServlet {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



