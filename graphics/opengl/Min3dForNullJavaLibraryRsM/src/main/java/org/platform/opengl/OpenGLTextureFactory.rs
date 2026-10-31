// Generated Code Do Not Modify
// package org.platform.opengl
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::khronos::opengles::GL10::GL10;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::platform::opengles::PlatformTextureBaseFactory::PlatformTextureBaseFactory;
use super::OpenGLGLUUtils::OpenGLGLUUtils;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLTextureFactory {
    pub base: PlatformTextureBaseFactory,
    pub gluUtils: Option<Rc<RefCell<OpenGLGLUUtils>>>,
}

thread_local! { pub static OpenGLTextureFactory_instance: Option<Rc<RefCell<OpenGLTextureFactory>>> = OpenGLTextureFactory::new(); }

impl OpenGLTextureFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<OpenGLTextureFactory>>> {
        return OpenGLTextureFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: PlatformTextureBaseFactory::new_struct(),
            gluUtils: None,
        };
        this.gluUtils = OpenGLGLUUtils::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn load(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut target: i32, mut level: i32, mut image: Option<Rc<RefCell<Image>>>, mut border: i32, mut flip: bool) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}
impl std::ops::Deref for OpenGLTextureFactory {
    type Target = PlatformTextureBaseFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for OpenGLTextureFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::platform::opengles::PlatformTextureBaseFactory::PlatformTextureBaseFactoryDyn for OpenGLTextureFactory {
    fn as_PlatformTextureBaseFactory(&self) -> &crate::org::allbinary::platform::opengles::PlatformTextureBaseFactory::PlatformTextureBaseFactory { &self.base }
    fn as_PlatformTextureBaseFactory_mut(&mut self) -> &mut crate::org::allbinary::platform::opengles::PlatformTextureBaseFactory::PlatformTextureBaseFactory { &mut self.base }
    fn load(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut target: i32, mut level: i32, mut image: Option<Rc<RefCell<Image>>>, mut border: i32, mut flip: bool) { Self::load(self, gl, target, level, image, border, flip) }
}


