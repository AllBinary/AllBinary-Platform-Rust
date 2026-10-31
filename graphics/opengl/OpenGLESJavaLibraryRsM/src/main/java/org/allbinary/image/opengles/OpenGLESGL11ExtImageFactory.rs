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
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::platform::graphics::PlatformBitmapBaseFactory::PlatformBitmapBaseFactory;
use crate::org::allbinary::platform::opengles::PlatformTextureBaseFactory::PlatformTextureBaseFactory;
use super::OpenGLImageFactory::OpenGLImageFactory;
use super::OpenGLESGL11ExtImage::OpenGLESGL11ExtImage;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLESGL11ExtImageFactory {
    pub base: OpenGLImageFactory,
}


impl OpenGLESGL11ExtImageFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: OpenGLImageFactory::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut bitmapFactory: Option<Rc<RefCell<dyn crate::org::allbinary::platform::graphics::PlatformBitmapBaseFactory::PlatformBitmapBaseFactoryDyn>>>, mut textureFactory: Option<Rc<RefCell<dyn crate::org::allbinary::platform::opengles::PlatformTextureBaseFactory::PlatformTextureBaseFactoryDyn>>>) -> Option<Rc<RefCell<Image>>> {
        return OpenGLESGL11ExtImage::new(image.clone(), bitmapFactory.clone(), textureFactory.clone()).map(|reference| Rc::new(RefCell::new(Rc::try_unwrap(reference).ok().expect("new object is not shared").into_inner().base.base)));
    }
}
impl std::ops::Deref for OpenGLESGL11ExtImageFactory {
    type Target = OpenGLImageFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for OpenGLESGL11ExtImageFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::image::opengles::OpenGLImageFactory::OpenGLImageFactoryDyn for OpenGLESGL11ExtImageFactory {
    fn as_OpenGLImageFactory(&self) -> &crate::org::allbinary::image::opengles::OpenGLImageFactory::OpenGLImageFactory { &self.base }
    fn as_OpenGLImageFactory_mut(&mut self) -> &mut crate::org::allbinary::image::opengles::OpenGLImageFactory::OpenGLImageFactory { &mut self.base }
    fn getInstance(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut bitmapFactory: Option<Rc<RefCell<dyn crate::org::allbinary::platform::graphics::PlatformBitmapBaseFactory::PlatformBitmapBaseFactoryDyn>>>, mut textureFactory: Option<Rc<RefCell<dyn crate::org::allbinary::platform::opengles::PlatformTextureBaseFactory::PlatformTextureBaseFactoryDyn>>>) -> Option<Rc<RefCell<Image>>> { Self::getInstance(self, image, bitmapFactory, textureFactory) }
}


