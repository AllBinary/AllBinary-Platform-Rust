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
// package org.allbinary.image.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::khronos::opengles::GL::GL;
// use crate::javax::microedition::khronos::opengles::GL10::GL10;
// use crate::javax::microedition::lcdui::Image::Image;
// use crate::javax::microedition::lcdui::NullImage::NullImage;
use crate::org::allbinary::platform::graphics::PlatformBitmapBase::PlatformBitmapBase;
use crate::org::allbinary::platform::graphics::PlatformBitmapBaseFactory::PlatformBitmapBaseFactory;
use crate::org::allbinary::platform::opengles::PlatformTextureBaseFactory::PlatformTextureBaseFactory;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface;
use super::OpenGLESImageProperties::OpenGLESImageProperties;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//OpenGLESNullImageJ2MEJavaLibrary
pub struct OpenGLESImage {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub textureFactory: Option<Rc<RefCell<dyn crate::org::allbinary::platform::opengles::PlatformTextureBaseFactory::PlatformTextureBaseFactoryDyn>>>,
    pub openGLBitmap: Option<Rc<RefCell<dyn crate::org::allbinary::platform::graphics::PlatformBitmapBase::PlatformBitmapBaseDyn>>>,
    pub openGLESImageProperties: Option<Rc<RefCell<OpenGLESImageProperties>>>,
}

thread_local! { pub static OpenGLESImage_NULL_OPENGL_IMAGE: Option<Rc<RefCell<dyn OpenGLESImageDyn>>> = OpenGLESImage::new(crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone).clone(), crate::org::allbinary::platform::graphics::PlatformBitmapBaseFactory::PlatformBitmapBaseFactory_NULL_PLATFORM_BITMAP_BASE_FACTORY.with(Clone::clone).clone(), crate::org::allbinary::platform::opengles::PlatformTextureBaseFactory::PlatformTextureBaseFactory_NULL_PLATFORM_TEXTURE_BASE_FACTORY.with(Clone::clone).clone()).map(|reference| reference as Rc<RefCell<dyn OpenGLESImageDyn>>); }
thread_local! { pub static OpenGLESImage_texture2dList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>); }
thread_local! { pub static OpenGLESImage_TYPE: i32 = 4; }
impl OpenGLSurfaceChangedInterface for OpenGLESImage {
    fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) { Self::set(self, gl) }
}

impl OpenGLESImage {
    

    pub fn new_struct(mut image: Option<Rc<RefCell<Image>>>, mut bitmapFactory: Option<Rc<RefCell<dyn crate::org::allbinary::platform::graphics::PlatformBitmapBaseFactory::PlatformBitmapBaseFactoryDyn>>>, mut textureFactory: Option<Rc<RefCell<dyn crate::org::allbinary::platform::opengles::PlatformTextureBaseFactory::PlatformTextureBaseFactoryDyn>>>) -> Self {
        let mut this = Self {
            commonStrings: None,
            textureFactory: None,
            openGLBitmap: None,
            openGLESImageProperties: None,
        };
        this.commonStrings = CommonStrings::getInstance();
        this.openGLESImageProperties = OpenGLESImageProperties::new();
        this.openGLBitmap = jcall!(bitmapFactory).createBitmap(image.clone());
        this.textureFactory = textureFactory.clone();
        this
    }

    pub fn new(mut image: Option<Rc<RefCell<Image>>>, mut bitmapFactory: Option<Rc<RefCell<dyn crate::org::allbinary::platform::graphics::PlatformBitmapBaseFactory::PlatformBitmapBaseFactoryDyn>>>, mut textureFactory: Option<Rc<RefCell<dyn crate::org::allbinary::platform::opengles::PlatformTextureBaseFactory::PlatformTextureBaseFactoryDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(image, bitmapFactory, textureFactory))))
    }
    

    // throws Exception
    pub fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(self.commonStrings).NOT_IMPLEMENTED.clone())));
    }
    

    pub fn initTexture(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) -> bool {
        return false;
    }
    

    pub fn drawRegion(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut viewHeight: i32, mut x_src: f32, mut y_src: f32, mut width: f32, mut height: f32, mut x: i32, mut y: i32, mut z: i32) {
    }
    

    pub fn draw(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut x: i32, mut y: i32, mut z: i32) {
    }
    

    pub fn getType(&mut self) -> i32 {
        return OpenGLESImage_TYPE.with(Clone::clone).clone();
    }
}
pub trait OpenGLESImageDyn: crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface {
    fn as_OpenGLESImage(&self) -> &OpenGLESImage;
    fn as_OpenGLESImage_mut(&mut self) -> &mut OpenGLESImage;
    fn initTexture(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) -> bool { OpenGLESImage::initTexture(self.as_OpenGLESImage_mut(), gl) }
    fn drawRegion(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut viewHeight: i32, mut x_src: f32, mut y_src: f32, mut width: f32, mut height: f32, mut x: i32, mut y: i32, mut z: i32) { OpenGLESImage::drawRegion(self.as_OpenGLESImage_mut(), gl, viewHeight, x_src, y_src, width, height, x, y, z) }
    fn draw(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut x: i32, mut y: i32, mut z: i32) { OpenGLESImage::draw(self.as_OpenGLESImage_mut(), gl, x, y, z) }
    fn getType(&mut self) -> i32 { OpenGLESImage::getType(self.as_OpenGLESImage_mut()) }
}
impl OpenGLESImageDyn for OpenGLESImage {
    fn as_OpenGLESImage(&self) -> &OpenGLESImage { self }
    fn as_OpenGLESImage_mut(&mut self) -> &mut OpenGLESImage { self }
}


