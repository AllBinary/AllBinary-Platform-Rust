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
use super::HitPointGameObjectInterface::HitPointGameObjectInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HitPointGameObject {
    pub base: MeteredGameObject,
    pub damaged: bool,
    pub closeDeath: bool,
    pub HP_LABEL: String,
    pub IS_DAMAGED_LABEL: String,
    pub IS_CLOSE_DEATH_LABEL: String,
}

// impl HitPointGameObjectInterface for HitPointGameObject with the methods of the trait

impl HitPointGameObject {
    

    pub fn new_struct(mut bufferedImage: Vec<Option<Rc<RefCell<BufferedImage>>>>, mut minX: Option<i32>, mut maxX: Option<i32>, mut y: Option<i32>) -> Self {
        let mut this = Self {
            base: MeteredGameObject::new_struct(bufferedImage.clone(), minX.clone(), maxX.clone(), y.clone()),
            damaged: false,
            closeDeath: false,
            HP_LABEL: String::new(),
            IS_DAMAGED_LABEL: String::new(),
            IS_CLOSE_DEATH_LABEL: String::new(),
        };
        this.damaged = false;
        this.closeDeath = false;
        this.HP_LABEL = String::from("HP: ");
        this.IS_DAMAGED_LABEL = String::from("isDamaged: ");
        this.IS_CLOSE_DEATH_LABEL = String::from("isCloseDeath: ");
        // super(...) is in the struct literal above
        if ((this.getRatios()[((bufferedImage.len() as i32) - 1) as usize]).unwrap() as f64) < ((1.0_f32) as f64) {
            this.setDamaged(true);
        } else {
            this.setDamaged(false);
        }
        if ((this.getRatios()[((bufferedImage.len() as i32) - 1) as usize]).unwrap() as f64) > ((0.25_f32) as f64) {
            this.closeDeath = false;
        } else {
            this.closeDeath = true;
        }
        this
    }

    pub fn new(mut bufferedImage: Vec<Option<Rc<RefCell<BufferedImage>>>>, mut minX: Option<i32>, mut maxX: Option<i32>, mut y: Option<i32>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(bufferedImage, minX, maxX, y))))
    }
    

    pub fn isMoreThan(&mut self, mut aFloat: f32) -> bool {
        if ((self.getRatios()[((self.getRatios().len() as i32) - 1) as usize]).unwrap() as f64) > ((aFloat) as f64) {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn isCloseDeath(&mut self) -> bool {
        return self.closeDeath.clone();
    }
    

    pub fn setCloseDeath(&mut self, mut closeDeath: bool) {
        self.closeDeath = closeDeath.clone();
    }
    

    pub fn isDamaged(&mut self) -> bool {
        return self.damaged.clone();
    }
    

    pub fn setDamaged(&mut self, mut damaged: bool) {
        self.damaged = damaged.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(self.HP_LABEL.clone());
        jcall!(stringBuffer).append(self.base.toString());
        jcall!(stringBuffer).append(jcall!(commonSeps).NEW_LINE.clone());
        jcall!(stringBuffer).append(self.IS_DAMAGED_LABEL.clone());
        jcall!(stringBuffer).appendboolean(self.isDamaged());
        jcall!(stringBuffer).append(jcall!(commonSeps).NEW_LINE.clone());
        jcall!(stringBuffer).append(self.IS_CLOSE_DEATH_LABEL.clone());
        jcall!(stringBuffer).appendboolean(self.isCloseDeath());
        return jcall!(stringBuffer).toString();
    }
}
impl std::ops::Deref for HitPointGameObject {
    type Target = MeteredGameObject;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HitPointGameObject {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::input::automation::module::game::MeteredGameObject::MeteredGameObjectDyn for HitPointGameObject {
    fn as_MeteredGameObject(&self) -> &crate::org::allbinary::input::automation::module::game::MeteredGameObject::MeteredGameObject { &self.base }
    fn as_MeteredGameObject_mut(&mut self) -> &mut crate::org::allbinary::input::automation::module::game::MeteredGameObject::MeteredGameObject { &mut self.base }
    fn toString(&mut self) -> String { Self::toString(self) }
}


