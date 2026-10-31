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
// package org.allbinary.logic.visual.transform.data
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSession::WeblisketSession;
use super::TransformHttpRequestDocument::TransformHttpRequestDocument;
use super::TransformDocumentInterface::TransformDocumentInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformHttpRequestDocumentFactory {
}


impl TransformHttpRequestDocumentFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getInstance(mut pageContext: Option<Rc<RefCell<PageContext>>>, mut weblisketSession: Option<Rc<RefCell<WeblisketSession>>>) -> Option<Rc<RefCell<dyn TransformDocumentInterface>>> {
        return (TransformHttpRequestDocument::new2(pageContext.clone(), weblisketSession.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::data::TransformHttpRequestDocument::TransformHttpRequestDocumentDyn>>)).clone() /* as TransformDocumentInterface */;
    }
}

