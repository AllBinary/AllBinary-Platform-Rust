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
// package org.allbinary.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::displayable::CanvasStrings::CanvasStrings;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Layer {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub width: i32,
    pub height: i32,
    pub visible: bool,
}

thread_local! { pub static Layer_ID: Option<i32> = jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(0); }

impl Layer {
    

    pub fn new_struct(mut width: i32, mut height: i32) -> Self {
        let mut this = Self {
            logUtil: None,
            x: 0,
            y: 0,
            z: 0,
            width: 0,
            height: 0,
            visible: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.z = 3;
        this.visible = true;
        this.setLayerWidth(width.clone());
        this.setLayerHeight(height.clone());
        this
    }

    pub fn new(mut width: i32, mut height: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(width, height))))
    }
    

    pub fn setPosition(&mut self, mut x: i32, mut y: i32, mut z: i32) {
        self.x = x.clone();
        self.y = y.clone();
        self.z = z.clone();
    }
    

    pub fn moveDXY(&mut self, mut dx: i32, mut dy: i32) {
        self.x += dx.clone();
        self.y += dy.clone();
    }
    

    pub fn moveDXYZ(&mut self, mut dx: i32, mut dy: i32, mut dz: i32) {
        self.x += dx.clone();
        self.y += dy.clone();
        self.z += dz.clone();
    }
    

    pub fn getXP(&mut self) -> i32 {
        return self.x.clone();
    }
    

    pub fn getYP(&mut self) -> i32 {
        return self.y.clone();
    }
    

    pub fn getZP(&mut self) -> i32 {
        return self.z.clone();
    }
    

    pub fn getWidth(&mut self) -> i32 {
        return self.width.clone();
    }
    

    pub fn getHeight(&mut self) -> i32 {
        return self.height.clone();
    }
    

    pub fn getDepth(&mut self) -> i64 {
        return 0_i64;
    }
    

    pub fn setVisible(&mut self, mut visible: bool) {
        self.visible = visible.clone();
    }
    

    pub fn isVisible(&mut self) -> bool {
        return self.visible.clone();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        let mut canvasStrings: Option<Rc<RefCell<CanvasStrings>>> = CanvasStrings::getInstance();
        jcall!(self.logUtil).putF(jcall!(commonStrings).NOT_IMPLEMENTED.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(canvasStrings).PAINT.clone());
    }
    

    pub fn setLayerWidth(&mut self, mut width: i32) {
        if width < 0 {
            panic!("{:?}", format!("{}: {:?}", "IllegalArgumentException", ()));
        }
        self.width = width.clone();
    }
    

    pub fn setLayerHeight(&mut self, mut height: i32) {
        if height < 0 {
            panic!("{:?}", format!("{}: {:?}", "IllegalArgumentException", ()));
        }
        self.height = height.clone();
    }
    

    pub fn toStringAppend(&mut self, mut stringBuffer: Option<Rc<RefCell<StringMaker>>>) {
    }
}
pub trait LayerDyn {
    fn as_Layer(&self) -> &Layer;
    fn as_Layer_mut(&mut self) -> &mut Layer;
    fn setPosition(&mut self, mut x: i32, mut y: i32, mut z: i32) { Layer::setPosition(self.as_Layer_mut(), x, y, z) }
    fn moveDXY(&mut self, mut dx: i32, mut dy: i32) { Layer::moveDXY(self.as_Layer_mut(), dx, dy) }
    fn moveDXYZ(&mut self, mut dx: i32, mut dy: i32, mut dz: i32) { Layer::moveDXYZ(self.as_Layer_mut(), dx, dy, dz) }
    fn getXP(&mut self) -> i32 { Layer::getXP(self.as_Layer_mut()) }
    fn getYP(&mut self) -> i32 { Layer::getYP(self.as_Layer_mut()) }
    fn getZP(&mut self) -> i32 { Layer::getZP(self.as_Layer_mut()) }
    fn getWidth(&mut self) -> i32 { Layer::getWidth(self.as_Layer_mut()) }
    fn getHeight(&mut self) -> i32 { Layer::getHeight(self.as_Layer_mut()) }
    fn getDepth(&mut self) -> i64 { Layer::getDepth(self.as_Layer_mut()) }
    fn setVisible(&mut self, mut visible: bool) { Layer::setVisible(self.as_Layer_mut(), visible) }
    fn isVisible(&mut self) -> bool { Layer::isVisible(self.as_Layer_mut()) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Layer::paint(self.as_Layer_mut(), graphics) }
    fn setLayerWidth(&mut self, mut width: i32) { Layer::setLayerWidth(self.as_Layer_mut(), width) }
    fn setLayerHeight(&mut self, mut height: i32) { Layer::setLayerHeight(self.as_Layer_mut(), height) }
    fn toStringAppend(&mut self, mut stringBuffer: Option<Rc<RefCell<StringMaker>>>) { Layer::toStringAppend(self.as_Layer_mut(), stringBuffer) }
}
impl LayerDyn for Layer {
    fn as_Layer(&self) -> &Layer { self }
    fn as_Layer_mut(&mut self) -> &mut Layer { self }
}


