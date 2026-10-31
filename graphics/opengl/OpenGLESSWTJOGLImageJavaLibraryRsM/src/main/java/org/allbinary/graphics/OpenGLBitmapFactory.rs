// Generated Code Do Not Modify
// package org.allbinary.graphics
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::IOException::IOException;
// use crate::java::io::InputStream::InputStream;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::image::ImageFactory::ImageFactory;
use crate::org::allbinary::platform::graphics::PlatformBitmapBaseFactory::PlatformBitmapBaseFactory;
use crate::org::allbinary::platform::graphics::PlatformBitmapBase::PlatformBitmapBase;
use super::Config::Config;
use super::OpenGLBitmap::OpenGLBitmap;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLBitmapFactory {
    pub base: PlatformBitmapBaseFactory,
    pub ARGB_8888: Option<Rc<RefCell<Config>>>,
}

thread_local! { pub static OpenGLBitmapFactory_instance: Option<Rc<RefCell<OpenGLBitmapFactory>>> = OpenGLBitmapFactory::new(); }

impl OpenGLBitmapFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: PlatformBitmapBaseFactory::new_struct(),
            ARGB_8888: None,
        };
        this.ARGB_8888 = Config::new(None);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<OpenGLBitmapFactory>>> {
        return OpenGLBitmapFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn createBitmap3(&mut self, mut width: i32, mut height: i32, mut config: Option<Rc<RefCell<Config>>>) -> Option<Rc<RefCell<OpenGLBitmap>>> {
        return OpenGLBitmap::new(Image::createImage(width.clone(), height.clone()));
    }
    

    // throws Exception
    pub fn decodeStream(&mut self, mut inputStream: Option<Rc<RefCell<InputStream>>>) -> Option<Rc<RefCell<OpenGLBitmap>>> {
        return OpenGLBitmap::new(jcall!(ImageFactory::getInstance()).createImageFromInputStream(inputStream.clone()));
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn createBitmap(&mut self, mut image: Option<Rc<RefCell<Image>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::platform::graphics::PlatformBitmapBase::PlatformBitmapBaseDyn>>> {
        return OpenGLBitmap::new(image.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::platform::graphics::PlatformBitmapBase::PlatformBitmapBaseDyn>>);
    }
}
impl std::ops::Deref for OpenGLBitmapFactory {
    type Target = PlatformBitmapBaseFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for OpenGLBitmapFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::platform::graphics::PlatformBitmapBaseFactory::PlatformBitmapBaseFactoryDyn for OpenGLBitmapFactory {
    fn as_PlatformBitmapBaseFactory(&self) -> &crate::org::allbinary::platform::graphics::PlatformBitmapBaseFactory::PlatformBitmapBaseFactory { &self.base }
    fn as_PlatformBitmapBaseFactory_mut(&mut self) -> &mut crate::org::allbinary::platform::graphics::PlatformBitmapBaseFactory::PlatformBitmapBaseFactory { &mut self.base }
    fn createBitmap(&mut self, mut image: Option<Rc<RefCell<Image>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::platform::graphics::PlatformBitmapBase::PlatformBitmapBaseDyn>>> { Self::createBitmap(self, image) }
}


pub struct Config {
    pub bitmapConfig: Option<Rc<RefCell<Box<dyn std::any::Any>>>>,
}


impl Config {
        

        pub fn new_struct(mut bitmapConfig: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Self {
            let mut this = Self {
            bitmapConfig: None,
            };
            this.bitmapConfig = bitmapConfig.clone();
            this
        }

        pub fn new(mut bitmapConfig: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(bitmapConfig))))
        }
}

