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
// package org.allbinary.input.automation.module.game
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::image::BufferedImage::BufferedImage;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use super::MeteredGameObject::MeteredGameObject;
use super::MagicGameObjectInterface::MagicGameObjectInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MagicGameObject {
    pub base: MeteredGameObject,
    pub MP_LABEL: String,
}

// impl MagicGameObjectInterface for MagicGameObject with the methods of the trait

impl MagicGameObject {
    

    pub fn new_struct(mut bufferedImage: Vec<Option<Rc<RefCell<BufferedImage>>>>, mut minX: Option<i32>, mut maxX: Option<i32>, mut y: Option<i32>) -> Self {
        let mut this = Self {
            base: MeteredGameObject::new_struct(bufferedImage.clone(), minX.clone(), maxX.clone(), y.clone()),
            MP_LABEL: String::new(),
        };
        this.MP_LABEL = String::from("MP:");
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut bufferedImage: Vec<Option<Rc<RefCell<BufferedImage>>>>, mut minX: Option<i32>, mut maxX: Option<i32>, mut y: Option<i32>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(bufferedImage, minX, maxX, y))))
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).NEW_LINE.clone());
        jcall!(stringBuffer).append(self.MP_LABEL.clone());
        jcall!(stringBuffer).append(self.base.toString());
        return jcall!(stringBuffer).toString();
    }
}
impl std::ops::Deref for MagicGameObject {
    type Target = MeteredGameObject;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MagicGameObject {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::input::automation::module::game::MeteredGameObject::MeteredGameObjectDyn for MagicGameObject {
    fn as_MeteredGameObject(&self) -> &crate::org::allbinary::input::automation::module::game::MeteredGameObject::MeteredGameObject { &self.base }
    fn as_MeteredGameObject_mut(&mut self) -> &mut crate::org::allbinary::input::automation::module::game::MeteredGameObject::MeteredGameObject { &mut self.base }
    fn toString(&mut self) -> String { Self::toString(self) }
}


