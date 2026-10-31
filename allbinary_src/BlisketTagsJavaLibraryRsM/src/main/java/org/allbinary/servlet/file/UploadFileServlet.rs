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
use crate::org::allbinary::logic::communication::http::file::upload::FileUploadData::FileUploadData;
use crate::org::allbinary::logic::communication::http::file::upload::HttpFileUploadUtil::HttpFileUploadUtil;
use crate::org::allbinary::logic::communication::http::request::HttpRequestUtil::HttpRequestUtil;
use crate::org::allbinary::logic::communication::http::request::MultipartRequestParams::MultipartRequestParams;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::AbFileOutputStream::AbFileOutputStream;
use crate::org::allbinary::logic::io::StreamUtil::StreamUtil;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::servlet::BlisketServletUtil::BlisketServletUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::apache::commons::fileupload::FileItem::FileItem;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UploadFileServlet {
    pub base: HttpServlet,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub requestHashMap: Option<Rc<RefCell<HashMap>>>,
    pub fileName: String,
}


impl UploadFileServlet {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: HttpServlet::new_struct(),
            logUtil: None,
            commonStrings: None,
            requestHashMap: None,
            fileName: String::new(),
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
        let mut isError: bool = false;
        let mut inputStream: Option<Rc<RefCell<InputStream>>> = None;
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(BlisketServletUtil::getInstance()).init(request.clone());
            self.requestHashMap = jcall!(MultipartRequestParams::new1(request.clone())).toHashMap();
            let mut filePath: String = ({ let java_receiver = self.requestHashMap.clone(); let java_arguments = (jcall!(FileUploadData::getInstance()).UPLOAD_TO_FILE_PATH.clone(),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as String */;
            if false {
                isError = true;
                jcall!(response).sendError(unsafe { crate::javax::servlet::http::HttpServletResponse::HttpServletResponse_SC_NOT_FOUND.clone() }.clone());
                return;
            }
            let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            let mut authenticationHelper: Option<Rc<RefCell<AuthenticationHelper>>> = (jcall!(AuthenticationHelperFactory::new()).getInstance(hashMap.clone(), request.clone())).clone() /* as AuthenticationHelper */;
            if jcall!(authenticationHelper).isAuthenticated() {
                if jcall!(AuthenticationHelperUtil::getInstance()).isAuthorized(authenticationHelper.clone(), filePath.clone()) {
                    self.saveFile(filePath.clone());
                    jcall!(response).setContentType(String::from("text/plain"));
                    jcall!(jcall!(response).getOutputStream()).write(String::from("OK!").bytes().map(|value| value as i8).collect::<Vec<i8>>());
                } else {
                    isError = true;
                    jcall!(response).sendError(unsafe { crate::javax::servlet::http::HttpServletResponse::HttpServletResponse_SC_UNAUTHORIZED.clone() }.clone(), String::from("You are not Authorized"));
                }
            } else {
                isError = true;
                jcall!(response).sendError(unsafe { crate::javax::servlet::http::HttpServletResponse::HttpServletResponse_SC_UNAUTHORIZED.clone() }.clone(), String::from("Please Login"));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("processRequest()"), crate::java_into_object(e.clone()));
            }
            isError = true;
            jcall!(response).sendError(unsafe { crate::javax::servlet::http::HttpServletResponse::HttpServletResponse_SC_NOT_FOUND.clone() }.clone());
            }
        }
        { // finally
            if !isError {
                jcall!(StreamUtil::getInstance()).close(jcall!(response).getOutputStream());
            }
            if !jcall!(StreamUtil::getInstance()).close(inputStream.clone()) {
            }
        }
    }
    

    // throws Exception
    pub fn saveFile(&mut self, mut filePath: String) {
        let mut fileOutputStream: Option<Rc<RefCell<AbFileOutputStream>>> = None;
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut fileItem: Option<Rc<RefCell<FileItem>>> = ({ let java_receiver = self.requestHashMap.clone(); let java_arguments = (jcall!(FileUploadData::getInstance()).FILE_DATA.clone(),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as FileItem */;
            if fileItem.is_some() && jcall!(fileItem).getSize() > 1 {
                self.fileName = jcall!(HttpRequestUtil::getInstance()).generateFileName(jcall!(fileItem).getName());
                if filePath.ends_with(&String::from("/")) || filePath.ends_with(&String::from("\\")) {
                    filePath = format!("{}{}", filePath, self.fileName);
                }
                let mut file: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(format!("{}{}", URLGLOBALS::getWebappPath(), filePath));
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
                    let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                    jcall!(stringBuffer).append(String::from("Uploaded File: "));
                    jcall!(stringBuffer).append(self.fileName.clone());
                    jcall!(stringBuffer).append(String::from(" New File: "));
                    jcall!(stringBuffer).append(jcall!(file).getPath());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("saveFile()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                HttpFileUploadUtil::log(fileItem.clone());
                let mut byteArray: Vec<i8> = jcall!(fileItem).get();
                if jcall!(file).isFile() {
                    jcall!(file).delete();
                    jcall!(file).createNewFile();
                }
                fileOutputStream = AbFileOutputStream::createFromAbFile(file.clone());
                jcall!(fileOutputStream).write(byteArray.clone());
            }
        })) {
            Ok(_) => {}
            Err(error) => std::panic::resume_unwind(error),
        }
        { // finally
            if fileOutputStream.is_some() {
                jcall!(fileOutputStream).close();
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
        return String::from("Upload Cloud Files");
    }
}
impl std::ops::Deref for UploadFileServlet {
    type Target = HttpServlet;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for UploadFileServlet {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



