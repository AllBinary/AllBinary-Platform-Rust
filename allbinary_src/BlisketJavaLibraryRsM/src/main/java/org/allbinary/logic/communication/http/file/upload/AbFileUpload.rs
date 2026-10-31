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
// package org.allbinary.logic.communication.http.file.upload
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::IOException::IOException;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
use crate::org::apache::commons::fileupload::FileItemFactory::FileItemFactory;
use crate::org::apache::commons::fileupload::FileItemIterator::FileItemIterator;
use crate::org::apache::commons::fileupload::FileUpload::FileUpload;
use crate::org::apache::commons::fileupload::FileUploadException::FileUploadException;
use crate::org::apache::commons::fileupload::servlet::ServletRequestContext::ServletRequestContext;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbFileUpload {
    pub base: FileUpload,
}


impl AbFileUpload {
    

    pub fn new_struct(mut fileItemFactory: Option<Rc<RefCell<FileItemFactory>>>) -> Self {
        let mut this = Self {
            base: FileUpload::new_struct1(fileItemFactory.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut fileItemFactory: Option<Rc<RefCell<FileItemFactory>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(fileItemFactory))))
    }
    

    // throws FileUploadException, IOException
    pub fn getItemIterator(&mut self, mut request: Option<Rc<RefCell<HttpServletRequest>>>) -> Option<Rc<RefCell<FileItemIterator>>> {
        return self.base.getItemIterator(ServletRequestContext::new(request.clone()));
    }
}
impl std::ops::Deref for AbFileUpload {
    type Target = FileUpload;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AbFileUpload {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



