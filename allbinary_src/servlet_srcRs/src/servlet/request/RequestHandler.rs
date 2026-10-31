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
// package servlet.request
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::*;
// use crate::java::net::*;
// use crate::javax::servlet::*;
// use crate::javax::servlet::http::*;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::workflow::WorkFlowInterface::WorkFlowInterface;
use crate::org::allbinary::logic::control::workflow::RequestWorkFlowFactory::RequestWorkFlowFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RequestHandler {
    pub base: HttpServlet,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl RequestHandler {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: HttpServlet::new_struct(),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws ServletException
    pub fn init(&mut self, mut config: Option<Rc<RefCell<ServletConfig>>>) {
        self.base.init(config.clone());
    }
    

    pub fn destroy(&mut self) {
    }
    

    // throws ServletException, IOException
    pub fn processRequest(&mut self, mut request: Option<Rc<RefCell<HttpServletRequest>>>, mut response: Option<Rc<RefCell<HttpServletResponse>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(response).setContentType(String::from("text/html"));
            let mut out: Option<Rc<RefCell<PrintWriter>>> = jcall!(response).getWriter();
            let mut workflowInterface: Option<Rc<RefCell<dyn WorkFlowInterface>>> = RequestWorkFlowFactory::getInstance((request).clone() /* as ServletRequest */, (response).clone() /* as ServletResponse */, self.getServletConfig(), self.getServletContext());
            if workflowInterface.is_some() {
                jcall!(out).print(jcall!(workflowInterface).process());
            }
            jcall!(out).flush();
            jcall!(out).close();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SERVLETERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Servlet LicensingException"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("processRequest()"), e.clone());
            }
            jcall!(response).sendRedirect(jcall!(jcall!(jcall!(self.base.abcs).globals).URLGLOBALS).LICENSEERRORPAGE.clone());
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
        return String::from("WorkFlow Request Handler");
    }
}
impl std::ops::Deref for RequestHandler {
    type Target = HttpServlet;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RequestHandler {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



