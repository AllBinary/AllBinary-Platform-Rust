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
// package org.allbinary.logic.visual.transform
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::InputStream::InputStream;
// use crate::javax::xml::transform::URIResolver::URIResolver;
// use crate::javax::xml::transform::stream::StreamSource::StreamSource;
use crate::org::allbinary::logic::visual::transform::info::CompositeTransformInfoInterface::CompositeTransformInfoInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait BasicTransformerInterface: CompositeTransformInfoInterface {
    

    fn setInputStream(&mut self, inputStream: Option<Rc<RefCell<InputStream>>>);
    

    fn setURIResolver(&mut self, uriResolver: Option<Rc<RefCell<URIResolver>>>);
    

    fn getStreamSource(&mut self) -> Option<Rc<RefCell<StreamSource>>>;
    

    fn getInputStream(&mut self) -> Option<Rc<RefCell<InputStream>>>;
    

    fn getURIResolver(&mut self) -> Option<Rc<RefCell<URIResolver>>>;
    

    // throws Exception
    fn translate(&mut self, xmlDocumentStr: String) -> String;
}


