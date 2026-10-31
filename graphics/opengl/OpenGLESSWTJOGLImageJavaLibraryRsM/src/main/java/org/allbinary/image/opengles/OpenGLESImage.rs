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
// use crate::javax::microedition::lcdui::OpenGLESPostLoadPlatformImage::OpenGLESPostLoadPlatformImage;
// use crate::javax::microedition::lcdui::PostLoadImageProcessor::PostLoadImageProcessor;
use crate::org::allbinary::graphics::GraphicsStrings::GraphicsStrings;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::platform::graphics::PlatformBitmapBase::PlatformBitmapBase;
use crate::org::allbinary::platform::graphics::PlatformBitmapBaseFactory::PlatformBitmapBaseFactory;
use crate::org::allbinary::platform::opengles::PlatformTextureBaseFactory::PlatformTextureBaseFactory;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface;
use super::OpenGLESImageProcessor::OpenGLESImageProcessor;
use super::OpenGLESImageTranslate::OpenGLESImageTranslate;
use super::OpenGLESImageProperties::OpenGLESImageProperties;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//OpenGLESSWTJOGLImageJavaLibrary
pub struct OpenGLESImage {
    pub base: Image,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub textureFactory: Option<Rc<RefCell<dyn crate::org::allbinary::platform::opengles::PlatformTextureBaseFactory::PlatformTextureBaseFactoryDyn>>>,
    pub openGLBitmap: Option<Rc<RefCell<dyn crate::org::allbinary::platform::graphics::PlatformBitmapBase::PlatformBitmapBaseDyn>>>,
    pub imageProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImageProcessor::OpenGLESImageProcessorDyn>>>,
    pub openGLESImageTranslate: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImageTranslate::OpenGLESImageTranslateDyn>>>,
    pub openGLESImageProperties: Option<Rc<RefCell<OpenGLESImageProperties>>>,
}

thread_local! { pub static OpenGLESImage_NULL_OPENGL_IMAGE: Option<Rc<RefCell<dyn OpenGLESImageDyn>>> = OpenGLESImage::new(crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone).clone(), crate::org::allbinary::platform::graphics::PlatformBitmapBaseFactory::PlatformBitmapBaseFactory_NULL_PLATFORM_BITMAP_BASE_FACTORY.with(Clone::clone).clone(), crate::org::allbinary::platform::opengles::PlatformTextureBaseFactory::PlatformTextureBaseFactory_NULL_PLATFORM_TEXTURE_BASE_FACTORY.with(Clone::clone).clone()).map(|reference| reference as Rc<RefCell<dyn OpenGLESImageDyn>>); }
thread_local! { pub static OpenGLESImage_texture2dList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>); }
thread_local! { pub static OpenGLESImage_TYPE: i32 = 4; }
impl OpenGLSurfaceChangedInterface for OpenGLESImage {
    fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) { Self::set(self, gl) }
    fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) { Self::set(self, gl) }
}

impl OpenGLESImage {
    

    pub fn new_struct(mut image: Option<Rc<RefCell<Image>>>, mut bitmapFactory: Option<Rc<RefCell<dyn crate::org::allbinary::platform::graphics::PlatformBitmapBaseFactory::PlatformBitmapBaseFactoryDyn>>>, mut textureFactory: Option<Rc<RefCell<dyn crate::org::allbinary::platform::opengles::PlatformTextureBaseFactory::PlatformTextureBaseFactoryDyn>>>) -> Self {
        let mut this = Self {
            base: Image::new_struct2(jcall!(image).getName(), unsafe { crate::javax::microedition::lcdui::PostLoadImageProcessor::PostLoadImageProcessor_NULL_POST_LOAD_IMAGE_PROCESSOR.clone() }.clone()),
            logUtil: None,
            commonStrings: None,
            textureFactory: None,
            openGLBitmap: None,
            imageProcessor: None,
            openGLESImageTranslate: None,
            openGLESImageProperties: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.imageProcessor = OpenGLESImageProcessor::getInstance();
        this.openGLESImageTranslate = OpenGLESImageTranslate::getInstance();
        this.openGLESImageProperties = OpenGLESImageProperties::new();
        // super(...) is in the struct literal above
        this.openGLBitmap = jcall!(bitmapFactory).createBitmap(image.clone());
        this.textureFactory = textureFactory.clone();
        this.platformImage = OpenGLESPostLoadPlatformImage::getInstance();
        this
    }

    pub fn new(mut image: Option<Rc<RefCell<Image>>>, mut bitmapFactory: Option<Rc<RefCell<dyn crate::org::allbinary::platform::graphics::PlatformBitmapBaseFactory::PlatformBitmapBaseFactoryDyn>>>, mut textureFactory: Option<Rc<RefCell<dyn crate::org::allbinary::platform::opengles::PlatformTextureBaseFactory::PlatformTextureBaseFactoryDyn>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(image, bitmapFactory, textureFactory))));
            jcall!(OpenGLImageCacheFactory::OpenGLImageCacheFactory::getInstance()).init((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    pub fn getHeight(&mut self) -> i32 {
        return (((((jcall!(self.openGLBitmap).getHeight()) as f32) * jcall!(self.openGLESImageProperties).scaleY2)) as i32);
    }
    

    pub fn getWidth(&mut self) -> i32 {
        return (((((jcall!(self.openGLBitmap).getWidth()) as f32) * jcall!(self.openGLESImageProperties).scaleX2)) as i32);
    }
    

    // throws Exception
    pub fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(self.commonStrings).NOT_IMPLEMENTED.clone())));
    }
    

    pub fn initTexture(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) -> bool {
        if !jcall!(OpenGLESImage_texture2dList.with(Clone::clone)).contains(None /* Java this: a &mut self method has no Rc of itself */) {
            jcall!(OpenGLESImage_texture2dList.with(Clone::clone)).add(None /* Java this: a &mut self method has no Rc of itself */);
            let mut textures: Vec<i32> = vec![0; (1) as usize];
            jcall!(gl).glEnable(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.clone() }.clone());
            if jcall!(self.openGLESImageProperties).textureID != -1 {
                textures[(0) as usize] = jcall!(self.openGLESImageProperties).textureID.clone();
                jcall!(gl).glDeleteTextures(1, textures.clone(), 0);
            }
            jcall!(gl).glGenTextures(1, textures.clone(), 0);
            jcall!(self.openGLESImageProperties).textureID = textures[(0) as usize].clone();
            jcall!(gl).glBindTexture(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.clone() }.clone(), jcall!(self.openGLESImageProperties).textureID.clone());
            jcall!(gl).glTexParameterf(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.clone() }.clone(), unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_MIN_FILTER.clone() }.clone(), unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_NEAREST.clone() }.clone());
            jcall!(gl).glTexParameterf(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.clone() }.clone(), unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_WRAP_S.clone() }.clone(), unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_CLAMP_TO_EDGE.clone() }.clone());
            jcall!(gl).glTexParameterf(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.clone() }.clone(), unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_WRAP_T.clone() }.clone(), unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_CLAMP_TO_EDGE.clone() }.clone());
            return true;
        }
        return false;
    }
    

    pub fn drawRegion(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut viewHeight: i32, mut x_src: f32, mut y_src: f32, mut width: f32, mut height: f32, mut x: i32, mut y: i32, mut z: i32) {
    }
    

    pub fn draw(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut x: i32, mut y: i32, mut z: i32) {
    }
    

    pub fn getType(&mut self) -> i32 {
        return OpenGLESImage_TYPE.with(Clone::clone).clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut graphicsStrings: Option<Rc<RefCell<GraphicsStrings>>> = GraphicsStrings::getInstance();
        return jcall!(StringMaker::new()).append(self.getName()).append(self.base.toString()).append(jcall!(graphicsStrings).OPACITY.clone()).appendfloat(jcall!(self.openGLESImageProperties).alphaf.clone()).toString();
    }
}
impl std::ops::Deref for OpenGLESImage {
    type Target = Image;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for OpenGLESImage {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait OpenGLESImageDyn: crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface {
    fn as_OpenGLESImage(&self) -> &OpenGLESImage;
    fn as_OpenGLESImage_mut(&mut self) -> &mut OpenGLESImage;
    fn getHeight(&mut self) -> i32 { OpenGLESImage::getHeight(self.as_OpenGLESImage_mut()) }
    fn getWidth(&mut self) -> i32 { OpenGLESImage::getWidth(self.as_OpenGLESImage_mut()) }
    fn initTexture(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) -> bool { OpenGLESImage::initTexture(self.as_OpenGLESImage_mut(), gl) }
    fn drawRegion(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut viewHeight: i32, mut x_src: f32, mut y_src: f32, mut width: f32, mut height: f32, mut x: i32, mut y: i32, mut z: i32) { OpenGLESImage::drawRegion(self.as_OpenGLESImage_mut(), gl, viewHeight, x_src, y_src, width, height, x, y, z) }
    fn draw(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut x: i32, mut y: i32, mut z: i32) { OpenGLESImage::draw(self.as_OpenGLESImage_mut(), gl, x, y, z) }
    fn getType(&mut self) -> i32 { OpenGLESImage::getType(self.as_OpenGLESImage_mut()) }
    fn initTexture(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) -> bool { OpenGLESImage::initTexture(self.as_OpenGLESImage_mut(), gl) }
    fn drawRegion(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut viewHeight: i32, mut x_src: f32, mut y_src: f32, mut width: f32, mut height: f32, mut x: i32, mut y: i32, mut z: i32) { OpenGLESImage::drawRegion(self.as_OpenGLESImage_mut(), gl, viewHeight, x_src, y_src, width, height, x, y, z) }
    fn draw(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut x: i32, mut y: i32, mut z: i32) { OpenGLESImage::draw(self.as_OpenGLESImage_mut(), gl, x, y, z) }
    fn getType(&mut self) -> i32 { OpenGLESImage::getType(self.as_OpenGLESImage_mut()) }
}
impl OpenGLESImageDyn for OpenGLESImage {
    fn as_OpenGLESImage(&self) -> &OpenGLESImage { self }
    fn as_OpenGLESImage_mut(&mut self) -> &mut OpenGLESImage { self }
}



