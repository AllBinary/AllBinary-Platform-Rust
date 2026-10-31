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
// package org.allbinary.logic.visual.transform.info
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::TransformInfo::TransformInfo;
use super::TransformInfoInterface::TransformInfoInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CustomTransformInfo {
    pub base: TransformInfo,
}

// impl TransformInfoInterface for CustomTransformInfo with the methods of the trait

impl CustomTransformInfo {
    

    pub fn new_struct(mut outputTypeName: String, mut inputOutputTypeName: String, mut inputOutputTypeFile: String, mut name: String, mut objectFileName: String, mut objectConfigFileName: String, mut templateFileName: String, mut dataFileName: String) -> Self {
        let mut this = Self {
            base: TransformInfo::new_struct(),
        };
        this
    }

    pub fn new(mut outputTypeName: String, mut inputOutputTypeName: String, mut inputOutputTypeFile: String, mut name: String, mut objectFileName: String, mut objectConfigFileName: String, mut templateFileName: String, mut dataFileName: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(outputTypeName, inputOutputTypeName, inputOutputTypeFile, name, objectFileName, objectConfigFileName, templateFileName, dataFileName))))
    }
}
impl std::ops::Deref for CustomTransformInfo {
    type Target = TransformInfo;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CustomTransformInfo {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfoDyn for CustomTransformInfo {
    fn as_TransformInfo(&self) -> &crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfo { &self.base }
    fn as_TransformInfo_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::TransformInfo::TransformInfo { &mut self.base }
}


