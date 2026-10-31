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

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformInfoProperties {
    pub name: String,
    pub label: String,
    pub description: String,
    pub objectFileName: String,
    pub objectConfigFileName: String,
    pub templateFileName: String,
}


impl TransformInfoProperties {
    

    pub fn new_struct(mut name: String, mut label: String, mut description: String, mut objectFileName: String, mut objectConfigFileName: String, mut templateFileName: String) -> Self {
        let mut this = Self {
            name: String::new(),
            label: String::new(),
            description: String::new(),
            objectFileName: String::new(),
            objectConfigFileName: String::new(),
            templateFileName: String::new(),
        };
        this.name = name.clone();
        this.label = label.clone();
        this.description = description.clone();
        this.objectFileName = objectFileName.clone();
        this.templateFileName = templateFileName.clone();
        this.objectConfigFileName = objectConfigFileName.clone();
        this
    }

    pub fn new(mut name: String, mut label: String, mut description: String, mut objectFileName: String, mut objectConfigFileName: String, mut templateFileName: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, label, description, objectFileName, objectConfigFileName, templateFileName))))
    }
    

    pub fn setName(&mut self, mut value: String) {
        self.name = value.clone();
    }
    

    pub fn setDescription(&mut self, mut value: String) {
        self.description = value.clone();
    }
    

    pub fn setViewFile(&mut self, mut value: String) {
        self.objectFileName = value.clone();
    }
    

    pub fn setTemplateFile(&mut self, mut value: String) {
        self.templateFileName = value.clone();
    }
    

    pub fn setObjectConfigFile(&mut self, mut value: String) {
        self.objectConfigFileName = value.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn getLabel(&mut self) -> String {
        return self.label.clone();
    }
    

    pub fn getDescription(&mut self) -> String {
        return self.description.clone();
    }
    

    pub fn getViewFile(&mut self) -> String {
        return self.objectFileName.clone();
    }
    

    pub fn getTemplateFile(&mut self) -> String {
        return self.templateFileName.clone();
    }
    

    pub fn getObjectConfigFile(&mut self) -> String {
        return self.objectConfigFileName.clone();
    }
}

