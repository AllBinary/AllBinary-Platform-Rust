// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
// package javax.microedition.lcdui
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use super::Image::Image;
use super::Graphics::Graphics;
use super::lcdui::lcdui;
use super::microedition::microedition;
use super::javax::javax;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//PlatformNullImageJavaLibrary
pub struct PlatformImage {
}


impl PlatformImage {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getImage(&mut self, mut graphicsSurface2: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return NullImage_NULL_IMAGE.with(Clone::clone).clone();
    }
    

    pub fn getSurfaceWidth(&mut self, mut graphicsSurface2: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut width: i32) -> i32 {
        return width.clone();
    }
    

    pub fn getSurfaceHeight(&mut self, mut graphicsSurface2: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut height: i32) -> i32 {
        return height.clone();
    }
    

    pub fn getWidth(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut width: i32) -> i32 {
        return width.clone();
    }
    

    pub fn getHeight(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut height: i32) -> i32 {
        return height.clone();
    }
    

    pub fn getGraphics(&mut self, mut graphicsSurface: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut width: i32, mut height: i32, mut image: Option<Rc<RefCell<Image>>>) -> Option<Rc<RefCell<Graphics>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn drawImage(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut x: i32, mut y: i32, mut anchor: i32, mut g2: Option<Rc<RefCell<Graphics>>>, mut g: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
    

    pub fn drawRegion(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut x_src: i32, mut y_src: i32, mut width: i32, mut height: i32, mut transform: i32, mut x_dst: i32, mut y_dst: i32, mut anchor: i32, mut g: Option<Rc<RefCell<Graphics>>>) {
    }
    

    pub fn getRGB(&mut self, mut argb: Vec<i32>, mut offset: i32, mut scanlength: i32, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut image: Option<Rc<RefCell<Image>>>) {
    }
    

    pub fn setRGB(&mut self, mut argb: Vec<i32>, mut offset: i32, mut scanlength: i32, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut image: Option<Rc<RefCell<Image>>>) {
    }
    

    pub fn getData(&mut self, mut image: Option<Rc<RefCell<Image>>>) -> Vec<i32> {
        return jcall!(NullUtil::getInstance()).NULL_INT_ARRAY.clone();
    }
}
pub trait PlatformImageDyn {
    fn as_PlatformImage(&self) -> &PlatformImage;
    fn as_PlatformImage_mut(&mut self) -> &mut PlatformImage;
    fn getImage(&mut self, mut graphicsSurface2: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { PlatformImage::getImage(self.as_PlatformImage_mut(), graphicsSurface2) }
    fn getSurfaceWidth(&mut self, mut graphicsSurface2: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut width: i32) -> i32 { PlatformImage::getSurfaceWidth(self.as_PlatformImage_mut(), graphicsSurface2, width) }
    fn getSurfaceHeight(&mut self, mut graphicsSurface2: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut height: i32) -> i32 { PlatformImage::getSurfaceHeight(self.as_PlatformImage_mut(), graphicsSurface2, height) }
    fn getWidth(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut width: i32) -> i32 { PlatformImage::getWidth(self.as_PlatformImage_mut(), image, width) }
    fn getHeight(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut height: i32) -> i32 { PlatformImage::getHeight(self.as_PlatformImage_mut(), image, height) }
    fn getGraphics(&mut self, mut graphicsSurface: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut width: i32, mut height: i32, mut image: Option<Rc<RefCell<Image>>>) -> Option<Rc<RefCell<Graphics>>> { PlatformImage::getGraphics(self.as_PlatformImage_mut(), graphicsSurface, width, height, image) }
    fn drawImage(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut x: i32, mut y: i32, mut anchor: i32, mut g2: Option<Rc<RefCell<Graphics>>>, mut g: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { PlatformImage::drawImage(self.as_PlatformImage_mut(), image, x, y, anchor, g2, g) }
    fn drawRegion(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut x_src: i32, mut y_src: i32, mut width: i32, mut height: i32, mut transform: i32, mut x_dst: i32, mut y_dst: i32, mut anchor: i32, mut g: Option<Rc<RefCell<Graphics>>>) { PlatformImage::drawRegion(self.as_PlatformImage_mut(), image, x_src, y_src, width, height, transform, x_dst, y_dst, anchor, g) }
    fn getRGB(&mut self, mut argb: Vec<i32>, mut offset: i32, mut scanlength: i32, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut image: Option<Rc<RefCell<Image>>>) { PlatformImage::getRGB(self.as_PlatformImage_mut(), argb, offset, scanlength, x, y, width, height, image) }
    fn setRGB(&mut self, mut argb: Vec<i32>, mut offset: i32, mut scanlength: i32, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut image: Option<Rc<RefCell<Image>>>) { PlatformImage::setRGB(self.as_PlatformImage_mut(), argb, offset, scanlength, x, y, width, height, image) }
    fn getData(&mut self, mut image: Option<Rc<RefCell<Image>>>) -> Vec<i32> { PlatformImage::getData(self.as_PlatformImage_mut(), image) }
}
impl PlatformImageDyn for PlatformImage {
    fn as_PlatformImage(&self) -> &PlatformImage { self }
    fn as_PlatformImage_mut(&mut self) -> &mut PlatformImage { self }
}


