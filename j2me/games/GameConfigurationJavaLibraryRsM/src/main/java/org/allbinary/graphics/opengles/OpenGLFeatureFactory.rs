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
// package org.allbinary.graphics.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::OpenGLFeature::OpenGLFeature;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLFeatureFactory {
    pub OPENGL_OPTIONS: Option<Rc<RefCell<OpenGLFeature>>>,
    pub OPENGL: Option<Rc<RefCell<OpenGLFeature>>>,
    pub OPENGL_SURFACE_VIEW: Option<Rc<RefCell<OpenGLFeature>>>,
    pub OPENGL_2D: Option<Rc<RefCell<OpenGLFeature>>>,
    pub OPENGL_3D: Option<Rc<RefCell<OpenGLFeature>>>,
    pub OPENGL_2D_AND_3D: Option<Rc<RefCell<OpenGLFeature>>>,
    pub OPENGL_AUTO_SELECT: Option<Rc<RefCell<OpenGLFeature>>>,
    pub OPENGL_MINIMUM: Option<Rc<RefCell<OpenGLFeature>>>,
    pub OPENGL_AS_GAME_THREAD: Option<Rc<RefCell<OpenGLFeature>>>,
    pub OPENGL_AND_GAME_HAVE_DIFFERENT_THREADS: Option<Rc<RefCell<OpenGLFeature>>>,
    pub IMAGE_COLOR_DEPTH_4444: Option<Rc<RefCell<OpenGLFeature>>>,
    pub IMAGE_COLOR_DEPTH_8888: Option<Rc<RefCell<OpenGLFeature>>>,
    pub IMAGE_COLOR_DEPTH_565: Option<Rc<RefCell<OpenGLFeature>>>,
    pub OPENGL_COLOR_DEPTH_4444: Option<Rc<RefCell<OpenGLFeature>>>,
    pub OPENGL_COLOR_DEPTH_8888: Option<Rc<RefCell<OpenGLFeature>>>,
    pub OPENGL_COLOR_DEPTH_565: Option<Rc<RefCell<OpenGLFeature>>>,
    pub OPENGL_DRAW_TEXTURE: Option<Rc<RefCell<OpenGLFeature>>>,
    pub OPENGL_VERTEX_BUFFER_OBJECT: Option<Rc<RefCell<OpenGLFeature>>>,
    pub OPENGL_SIMPLE_OBJECT3D_PROCESSOR: Option<Rc<RefCell<OpenGLFeature>>>,
    pub OPENGL_COMPLEX_OBJECT3D_PROCESSOR: Option<Rc<RefCell<OpenGLFeature>>>,
    pub OPENGL_SIMPLE_TEXTURE_PROCESSOR: Option<Rc<RefCell<OpenGLFeature>>>,
    pub OPENGL_COMPLEX_TEXTURE_PROCESSOR: Option<Rc<RefCell<OpenGLFeature>>>,
}

thread_local! { pub static OpenGLFeatureFactory_instance: Option<Rc<RefCell<OpenGLFeatureFactory>>> = OpenGLFeatureFactory::new(); }

impl OpenGLFeatureFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<OpenGLFeatureFactory>>> {
        return OpenGLFeatureFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            OPENGL_OPTIONS: None,
            OPENGL: None,
            OPENGL_SURFACE_VIEW: None,
            OPENGL_2D: None,
            OPENGL_3D: None,
            OPENGL_2D_AND_3D: None,
            OPENGL_AUTO_SELECT: None,
            OPENGL_MINIMUM: None,
            OPENGL_AS_GAME_THREAD: None,
            OPENGL_AND_GAME_HAVE_DIFFERENT_THREADS: None,
            IMAGE_COLOR_DEPTH_4444: None,
            IMAGE_COLOR_DEPTH_8888: None,
            IMAGE_COLOR_DEPTH_565: None,
            OPENGL_COLOR_DEPTH_4444: None,
            OPENGL_COLOR_DEPTH_8888: None,
            OPENGL_COLOR_DEPTH_565: None,
            OPENGL_DRAW_TEXTURE: None,
            OPENGL_VERTEX_BUFFER_OBJECT: None,
            OPENGL_SIMPLE_OBJECT3D_PROCESSOR: None,
            OPENGL_COMPLEX_OBJECT3D_PROCESSOR: None,
            OPENGL_SIMPLE_TEXTURE_PROCESSOR: None,
            OPENGL_COMPLEX_TEXTURE_PROCESSOR: None,
        };
        this.OPENGL_OPTIONS = OpenGLFeature::new(String::from("OpenGL Options"));
        this.OPENGL = OpenGLFeature::new(String::from("OpenGL"));
        this.OPENGL_SURFACE_VIEW = OpenGLFeature::new(String::from("GLSurfaceView"));
        this.OPENGL_2D = OpenGLFeature::new(String::from("OpenGL 2D"));
        this.OPENGL_3D = OpenGLFeature::new(String::from("OpenGL 3D"));
        this.OPENGL_2D_AND_3D = OpenGLFeature::new(String::from("OpenGL 2D/3D"));
        this.OPENGL_AUTO_SELECT = OpenGLFeature::new(String::from("Auto Select"));
        this.OPENGL_MINIMUM = OpenGLFeature::new(String::from("Minimum"));
        this.OPENGL_AS_GAME_THREAD = OpenGLFeature::new(String::from("As game thread"));
        this.OPENGL_AND_GAME_HAVE_DIFFERENT_THREADS = OpenGLFeature::new(String::from("As different threads"));
        this.IMAGE_COLOR_DEPTH_4444 = OpenGLFeature::new(String::from("4444"));
        this.IMAGE_COLOR_DEPTH_8888 = OpenGLFeature::new(String::from("8888"));
        this.IMAGE_COLOR_DEPTH_565 = OpenGLFeature::new(String::from("565"));
        this.OPENGL_COLOR_DEPTH_4444 = OpenGLFeature::new(String::from("4444"));
        this.OPENGL_COLOR_DEPTH_8888 = OpenGLFeature::new(String::from("8888"));
        this.OPENGL_COLOR_DEPTH_565 = OpenGLFeature::new(String::from("565"));
        this.OPENGL_DRAW_TEXTURE = OpenGLFeature::new(String::from("GL_OES_draw_texture"));
        this.OPENGL_VERTEX_BUFFER_OBJECT = OpenGLFeature::new(String::from("vertex_buffer_object"));
        this.OPENGL_SIMPLE_OBJECT3D_PROCESSOR = OpenGLFeature::new(String::from("Simple Object3d Processor"));
        this.OPENGL_COMPLEX_OBJECT3D_PROCESSOR = OpenGLFeature::new(String::from("Complex Object3d Processor"));
        this.OPENGL_SIMPLE_TEXTURE_PROCESSOR = OpenGLFeature::new(String::from("Simple Texture Processor"));
        this.OPENGL_COMPLEX_TEXTURE_PROCESSOR = OpenGLFeature::new(String::from("Complex Texture Processor"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

