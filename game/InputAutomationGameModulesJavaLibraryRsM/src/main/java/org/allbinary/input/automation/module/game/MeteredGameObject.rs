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
use super::MeteredGameObjectInterface::MeteredGameObjectInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MeteredGameObject {
    pub ratios: Vec<Option<f64>>,
    pub good: bool,
    pub dropping: bool,
    pub minX: Option<i32>,
    pub maxX: Option<i32>,
    pub y: Option<i32>,
    pub size: Option<i32>,
    pub RATIOS_LABEL: String,
    pub IS_GOOD_LABEL: String,
    pub IS_DROPPPING_LABEL: String,
}

impl MeteredGameObjectInterface for MeteredGameObject {
    fn getRatios(&mut self) -> Vec<Option<f64>> { Self::getRatios(self) }
    fn isGood(&mut self) -> bool { Self::isGood(self) }
    fn isDropping(&mut self) -> bool { Self::isDropping(self) }
}

impl MeteredGameObject {
    

    pub fn new_struct(mut bufferedImage: Vec<Option<Rc<RefCell<BufferedImage>>>>, mut minX: Option<i32>, mut maxX: Option<i32>, mut y: Option<i32>) -> Self {
        let mut this = Self {
            ratios: Vec::new(),
            good: false,
            dropping: false,
            minX: None,
            maxX: None,
            y: None,
            size: None,
            RATIOS_LABEL: String::new(),
            IS_GOOD_LABEL: String::new(),
            IS_DROPPPING_LABEL: String::new(),
        };
        this.good = false;
        this.dropping = false;
        this.RATIOS_LABEL = String::from("Ratios: ");
        this.IS_GOOD_LABEL = String::from("isGood: ");
        this.IS_DROPPPING_LABEL = String::from("isDropping: ");
        this.setRatios(vec![None; ((bufferedImage.len() as i32)) as usize]);
        this.setMinX(minX.clone());
        this.setMaxX(maxX.clone());
        this.setY(y.clone());
        this.setSize(maxX - minX);
        for index in (0..(bufferedImage.len() as i32)) {
            let mut nominator: f64 = GraphicsAnalysisUtil::GraphicsAnalysisUtil::getNominator(bufferedImage[(index) as usize].clone(), this.getMinX(), this.getMaxX(), this.getY());
            this.getRatios()[(index) as usize] = Some((nominator / this.getSize()) as f64);
        }
        if ((this.getRatios()[((bufferedImage.len() as i32) - 1) as usize]).unwrap() as f64) > ((0.93_f32) as f64) {
            this.setGood(true);
        } else {
            this.setGood(false);
        }
        if ((this.getRatios()[((bufferedImage.len() as i32) - 1) as usize]).unwrap() as f64) < ((this.getRatios()[(0) as usize]).unwrap() as f64) {
            this.setDropping(true);
        } else {
            this.setDropping(false);
        }
        this
    }

    pub fn new(mut bufferedImage: Vec<Option<Rc<RefCell<BufferedImage>>>>, mut minX: Option<i32>, mut maxX: Option<i32>, mut y: Option<i32>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(bufferedImage, minX, maxX, y))))
    }
    

    pub fn isGood(&mut self) -> bool {
        return self.good.clone();
    }
    

    pub fn setGood(&mut self, mut good: bool) {
        self.good = good.clone();
    }
    

    pub fn isDropping(&mut self) -> bool {
        return self.dropping.clone();
    }
    

    pub fn setDropping(&mut self, mut dropping: bool) {
        self.dropping = dropping.clone();
    }
    

    pub fn getRatios(&mut self) -> Vec<Option<f64>> {
        return self.ratios.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        jcall!(stringBuffer).append(jcall!(commonSeps).NEW_LINE.clone());
        jcall!(stringBuffer).append(self.RATIOS_LABEL.clone());
        for index in (0..(self.getRatios().len() as i32)) {
            jcall!(stringBuffer).append(jcall!(self.getRatios()[(index) as usize]).toString());
            if index < (self.getRatios().len() as i32) - 1 {
                jcall!(stringBuffer).append(jcall!(commonSeps).COMMA_SEP.clone());
            }
        }
        jcall!(stringBuffer).append(jcall!(commonSeps).NEW_LINE.clone());
        jcall!(stringBuffer).append(self.IS_GOOD_LABEL.clone());
        jcall!(stringBuffer).appendboolean(self.isGood());
        jcall!(stringBuffer).append(jcall!(commonSeps).NEW_LINE.clone());
        jcall!(stringBuffer).append(self.IS_DROPPPING_LABEL.clone());
        jcall!(stringBuffer).appendboolean(self.isDropping());
        return jcall!(stringBuffer).toString();
    }
    

    pub fn setRatios(&mut self, mut ratios: Vec<Option<f64>>) {
        self.ratios = ratios.clone();
    }
    

    pub fn getMinX(&mut self) -> Option<i32> {
        return self.minX.clone();
    }
    

    pub fn setMinX(&mut self, mut minX: Option<i32>) {
        self.minX = minX.clone();
    }
    

    pub fn getMaxX(&mut self) -> Option<i32> {
        return self.maxX.clone();
    }
    

    pub fn setMaxX(&mut self, mut maxX: Option<i32>) {
        self.maxX = maxX.clone();
    }
    

    pub fn getY(&mut self) -> Option<i32> {
        return self.y.clone();
    }
    

    pub fn setY(&mut self, mut y: Option<i32>) {
        self.y = y.clone();
    }
    

    pub fn getSize(&mut self) -> Option<i32> {
        return self.size.clone();
    }
    

    pub fn setSize(&mut self, mut size: Option<i32>) {
        self.size = size.clone();
    }
}
pub trait MeteredGameObjectDyn {
    fn as_MeteredGameObject(&self) -> &MeteredGameObject;
    fn as_MeteredGameObject_mut(&mut self) -> &mut MeteredGameObject;
    fn isGood(&mut self) -> bool { MeteredGameObject::isGood(self.as_MeteredGameObject_mut()) }
    fn setGood(&mut self, mut good: bool) { MeteredGameObject::setGood(self.as_MeteredGameObject_mut(), good) }
    fn isDropping(&mut self) -> bool { MeteredGameObject::isDropping(self.as_MeteredGameObject_mut()) }
    fn setDropping(&mut self, mut dropping: bool) { MeteredGameObject::setDropping(self.as_MeteredGameObject_mut(), dropping) }
    fn getRatios(&mut self) -> Vec<Option<f64>> { MeteredGameObject::getRatios(self.as_MeteredGameObject_mut()) }
    fn toString(&mut self) -> String { MeteredGameObject::toString(self.as_MeteredGameObject_mut()) }
    fn setRatios(&mut self, mut ratios: Vec<Option<f64>>) { MeteredGameObject::setRatios(self.as_MeteredGameObject_mut(), ratios) }
    fn getMinX(&mut self) -> Option<i32> { MeteredGameObject::getMinX(self.as_MeteredGameObject_mut()) }
    fn setMinX(&mut self, mut minX: Option<i32>) { MeteredGameObject::setMinX(self.as_MeteredGameObject_mut(), minX) }
    fn getMaxX(&mut self) -> Option<i32> { MeteredGameObject::getMaxX(self.as_MeteredGameObject_mut()) }
    fn setMaxX(&mut self, mut maxX: Option<i32>) { MeteredGameObject::setMaxX(self.as_MeteredGameObject_mut(), maxX) }
    fn getY(&mut self) -> Option<i32> { MeteredGameObject::getY(self.as_MeteredGameObject_mut()) }
    fn setY(&mut self, mut y: Option<i32>) { MeteredGameObject::setY(self.as_MeteredGameObject_mut(), y) }
    fn getSize(&mut self) -> Option<i32> { MeteredGameObject::getSize(self.as_MeteredGameObject_mut()) }
    fn setSize(&mut self, mut size: Option<i32>) { MeteredGameObject::setSize(self.as_MeteredGameObject_mut(), size) }
}
impl MeteredGameObjectDyn for MeteredGameObject {
    fn as_MeteredGameObject(&self) -> &MeteredGameObject { self }
    fn as_MeteredGameObject_mut(&mut self) -> &mut MeteredGameObject { self }
}


