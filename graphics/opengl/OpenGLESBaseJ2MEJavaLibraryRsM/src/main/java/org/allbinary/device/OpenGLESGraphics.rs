// Generated Code Do Not Modify
// package org.allbinary.device
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::khronos::opengles::GL::GL;
// use crate::javax::microedition::khronos::opengles::GL10::GL10;
// use crate::javax::microedition::lcdui::Font::Font;
use crate::org::allbinary::graphics::font::FontDebugBase::FontDebugBase;
use crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::GraphicsInterface::GraphicsInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLESGraphics {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub gl: Option<Rc<RefCell<dyn GL10>>>,
}

impl OpenGLSurfaceChangedInterface for OpenGLESGraphics {
    fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) { Self::set(self, gl) }
}
impl GraphicsInterface for OpenGLESGraphics {
    fn setFontDebug(&mut self, mut font: Option<Rc<RefCell<Font>>>, mut fontDebug: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::FontDebugBase::FontDebugBaseDyn>>>) { Self::setFontDebug(self, font, fontDebug) }
}

impl OpenGLESGraphics {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            commonStrings: None,
            gl: None,
        };
        this.commonStrings = CommonStrings::getInstance();
        this.gl = NullGL10_NULL_GL10.with(Clone::clone).clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn updateGL(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) {
    }
    

    pub fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) {
    }
    

    // throws Exception
    pub fn init(&mut self) {
    }
    

    // throws Exception
    pub fn update(&mut self) {
    }
    

    pub fn setOrthoginalMode(&mut self) {
    }
    

    pub fn setCameraMode(&mut self) {
    }
    

    pub fn getGlP(&mut self) -> Option<Rc<RefCell<dyn GL10>>> {
        return self.gl.clone();
    }
    

    pub fn setFontDebug(&mut self, mut font: Option<Rc<RefCell<Font>>>, mut fontDebug: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::FontDebugBase::FontDebugBaseDyn>>>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}
pub trait OpenGLESGraphicsDyn: crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface {
    fn as_OpenGLESGraphics(&self) -> &OpenGLESGraphics;
    fn as_OpenGLESGraphics_mut(&mut self) -> &mut OpenGLESGraphics;
    fn updateGL(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) { OpenGLESGraphics::updateGL(self.as_OpenGLESGraphics_mut(), gl) }
    fn init(&mut self) { OpenGLESGraphics::init(self.as_OpenGLESGraphics_mut()) }
    fn update(&mut self) { OpenGLESGraphics::update(self.as_OpenGLESGraphics_mut()) }
    fn setOrthoginalMode(&mut self) { OpenGLESGraphics::setOrthoginalMode(self.as_OpenGLESGraphics_mut()) }
    fn setCameraMode(&mut self) { OpenGLESGraphics::setCameraMode(self.as_OpenGLESGraphics_mut()) }
    fn getGlP(&mut self) -> Option<Rc<RefCell<dyn GL10>>> { OpenGLESGraphics::getGlP(self.as_OpenGLESGraphics_mut()) }
    fn setFontDebug(&mut self, mut font: Option<Rc<RefCell<Font>>>, mut fontDebug: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::FontDebugBase::FontDebugBaseDyn>>>) { OpenGLESGraphics::setFontDebug(self.as_OpenGLESGraphics_mut(), font, fontDebug) }
}
impl OpenGLESGraphicsDyn for OpenGLESGraphics {
    fn as_OpenGLESGraphics(&self) -> &OpenGLESGraphics { self }
    fn as_OpenGLESGraphics_mut(&mut self) -> &mut OpenGLESGraphics { self }
}


