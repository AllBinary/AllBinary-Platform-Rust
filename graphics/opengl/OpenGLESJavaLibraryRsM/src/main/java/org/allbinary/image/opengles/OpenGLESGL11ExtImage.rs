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
// use crate::javax::microedition::khronos::opengles::GL11::GL11;
// use crate::javax::microedition::khronos::opengles::GL11Ext::GL11Ext;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::graphics::displayable::event::DisplayChangeEvent::DisplayChangeEvent;
use crate::org::allbinary::graphics::opengles::OpenGLLogUtil::OpenGLLogUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::platform::graphics::PlatformBitmapBaseFactory::PlatformBitmapBaseFactory;
use crate::org::allbinary::platform::opengles::PlatformTextureBaseFactory::PlatformTextureBaseFactory;
use super::OpenGLESImage::OpenGLESImage;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//Many devices don't support this even though it is supposed to
pub struct OpenGLESGL11ExtImage {
    pub base: OpenGLESImage,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub a: i32,
    pub rectangle: Vec<i32>,
}


impl OpenGLESGL11ExtImage {
    

    pub fn new_struct(mut image: Option<Rc<RefCell<Image>>>, mut bitmapFactory: Option<Rc<RefCell<dyn crate::org::allbinary::platform::graphics::PlatformBitmapBaseFactory::PlatformBitmapBaseFactoryDyn>>>, mut textureFactory: Option<Rc<RefCell<dyn crate::org::allbinary::platform::opengles::PlatformTextureBaseFactory::PlatformTextureBaseFactoryDyn>>>) -> Self {
        let mut this = Self {
            base: OpenGLESImage::new_struct(image.clone(), bitmapFactory.clone(), textureFactory.clone()),
            logUtil: None,
            a: 0,
            rectangle: Vec::new(),
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this.onDisplayChangeEvent(None);
        this.rectangle = vec![0, this.getHeight(), this.getWidth(), -this.getHeight()];
        this
    }

    pub fn new(mut image: Option<Rc<RefCell<Image>>>, mut bitmapFactory: Option<Rc<RefCell<dyn crate::org::allbinary::platform::graphics::PlatformBitmapBaseFactory::PlatformBitmapBaseFactoryDyn>>>, mut textureFactory: Option<Rc<RefCell<dyn crate::org::allbinary::platform::opengles::PlatformTextureBaseFactory::PlatformTextureBaseFactoryDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(image, bitmapFactory, textureFactory))))
    }
    

    pub fn onDisplayChangeEvent(&mut self, mut displayChangeEvent: Option<Rc<RefCell<DisplayChangeEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onResize"));
            self.a = jcall!(DisplayInfoSingleton::getInstance()).getLastHeight() - self.getHeight();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onResize"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) {
        self.onDisplayChangeEvent(None);
        let mut gl11: Option<Rc<RefCell<dyn GL11>>> = panic!("ClassCastException: a Rust dyn GL cannot be cast to dyn GL11");
        if self.base.initTexture(gl11.clone()) {
            jcall!(self.textureFactory).load(gl11.clone().map(|reference| reference as Rc<RefCell<dyn GL10>>), unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.clone() }.clone(), 0, None /* Java this: a &mut self method has no Rc of itself */, 0, true);
            jcall!(gl11).glTexParameteriv(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.clone() }.clone(), unsafe { crate::javax::microedition::khronos::opengles::GL11Ext::GL11Ext_GL_TEXTURE_CROP_RECT_OES.clone() }.clone(), self.rectangle.clone(), 0);
            jcall!(gl11).glDisable(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.clone() }.clone());
            jcall!(OpenGLLogUtil::getInstance()).logError2(gl11.clone().map(|reference| reference as Rc<RefCell<dyn GL10>>), None /* Java this: a &mut self method has no Rc of itself */);
        }
    }
    

    pub fn draw(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut x: i32, mut y: i32, mut z: i32) {
        jcall!(gl).glEnable(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.clone() }.clone());
        jcall!(gl).glBindTexture(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.clone() }.clone(), jcall!(self.openGLESImageProperties).textureID.clone());
        jcall!((panic!("ClassCastException: a Rust dyn GL cannot be cast to dyn GL11Ext"))).glDrawTexfOES(x.clone(), self.a - y, z.clone(), self.getWidth(), self.getHeight());
        jcall!(gl).glDisable(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.clone() }.clone());
    }
}
impl std::ops::Deref for OpenGLESGL11ExtImage {
    type Target = OpenGLESImage;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for OpenGLESGL11ExtImage {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn for OpenGLESGL11ExtImage {
    fn as_OpenGLESImage(&self) -> &crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImage { &self.base }
    fn as_OpenGLESImage_mut(&mut self) -> &mut crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImage { &mut self.base }
    fn draw(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut x: i32, mut y: i32, mut z: i32) { Self::draw(self, gl, x, y, z) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for OpenGLESGL11ExtImage {
    fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) { Self::set(self, gl) }
    fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) { Self::set(self, gl) }
}


