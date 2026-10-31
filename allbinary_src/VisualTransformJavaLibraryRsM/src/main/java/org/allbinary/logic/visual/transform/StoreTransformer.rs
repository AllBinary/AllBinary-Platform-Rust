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
// use crate::javax::xml::transform::URIResolver::URIResolver;
use crate::org::allbinary::data::tree::dom::BasicUriResolver::BasicUriResolver;
use crate::org::allbinary::data::tree::dom::StoreUriResolver::StoreUriResolver;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use super::BasicTransformer::BasicTransformer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StoreTransformer {
    pub base: BasicTransformer,
}


impl StoreTransformer {
    

    pub fn new_struct(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: BasicTransformer::new_struct(abeClientInformation.clone(), transformInfoInterface.clone()),
        };
        // super(...) is in the struct literal above
        { let java_arguments = ((StoreUriResolver::new(this.getTransformInfoInterface(), (this.getURIResolver()).clone() /* as BasicUriResolver */)).clone() /* as URIResolver */,); this.setURIResolver(java_arguments.0) };
        this
    }

    pub fn new(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(abeClientInformation, transformInfoInterface))))
    }
}
impl std::ops::Deref for StoreTransformer {
    type Target = BasicTransformer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for StoreTransformer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::visual::transform::AbTransformer::AbTransformerDyn for StoreTransformer {
    fn as_AbTransformer(&self) -> &crate::org::allbinary::logic::visual::transform::AbTransformer::AbTransformer { &self.base.base }
    fn as_AbTransformer_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::AbTransformer::AbTransformer { &mut self.base.base }
}
impl crate::org::allbinary::logic::visual::transform::BasicTransformer::BasicTransformerDyn for StoreTransformer {
    fn as_BasicTransformer(&self) -> &crate::org::allbinary::logic::visual::transform::BasicTransformer::BasicTransformer { &self.base }
    fn as_BasicTransformer_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::BasicTransformer::BasicTransformer { &mut self.base }
}


