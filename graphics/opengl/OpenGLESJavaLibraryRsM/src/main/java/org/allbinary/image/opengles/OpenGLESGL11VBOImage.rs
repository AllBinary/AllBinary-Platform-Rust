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
// use crate::java::nio::ByteBuffer::ByteBuffer;
// use crate::java::nio::ByteOrder::ByteOrder;
// use crate::java::nio::FloatBuffer::FloatBuffer;
// use crate::javax::microedition::khronos::opengles::GL::GL;
// use crate::javax::microedition::khronos::opengles::GL10::GL10;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::graphics::opengles::OpenGLLogUtil::OpenGLLogUtil;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::opengles::GLUtil::GLUtil;
use crate::org::allbinary::platform::graphics::PlatformBitmapBaseFactory::PlatformBitmapBaseFactory;
use crate::org::allbinary::platform::opengles::PlatformTextureBaseFactory::PlatformTextureBaseFactory;
use super::OpenGLESImage::OpenGLESImage;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//This is not actually VBO impl
pub struct OpenGLESGL11VBOImage {
    pub base: OpenGLESImage,
    pub glUtil: Option<Rc<RefCell<GLUtil>>>,
    pub textureVertexFloatBuffer: Option<Rc<RefCell<FloatBuffer>>>,
    pub regionRectangleFloatArray: Vec<f32>,
    pub regionRectangleVertexFloatBuffer: Option<Rc<RefCell<FloatBuffer>>>,
    pub regionTextureRectangleFloatArray: Vec<f32>,
    pub regionTextureVertexFloatBuffer: Option<Rc<RefCell<FloatBuffer>>>,
}


impl OpenGLESGL11VBOImage {
    

    pub fn new_struct(mut image: Option<Rc<RefCell<Image>>>, mut bitmapFactory: Option<Rc<RefCell<dyn crate::org::allbinary::platform::graphics::PlatformBitmapBaseFactory::PlatformBitmapBaseFactoryDyn>>>, mut textureFactory: Option<Rc<RefCell<dyn crate::org::allbinary::platform::opengles::PlatformTextureBaseFactory::PlatformTextureBaseFactoryDyn>>>) -> Self {
        let mut this = Self {
            base: OpenGLESImage::new_struct(image.clone(), bitmapFactory.clone(), textureFactory.clone()),
            glUtil: None,
            textureVertexFloatBuffer: None,
            regionRectangleFloatArray: Vec::new(),
            regionRectangleVertexFloatBuffer: None,
            regionTextureRectangleFloatArray: Vec::new(),
            regionTextureVertexFloatBuffer: None,
        };
        this.glUtil = GLUtil::getInstance();
        this.textureVertexFloatBuffer = jcall!(jcall!(ByteBuffer::allocateDirect(4 * 4 * 2)).order(ByteOrder::nativeOrder())).asFloatBuffer();
        this.regionRectangleFloatArray = vec![0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32];
        this.regionRectangleVertexFloatBuffer = jcall!(jcall!(ByteBuffer::allocateDirect(4 * 4 * 3)).order(ByteOrder::nativeOrder())).asFloatBuffer();
        this.regionTextureRectangleFloatArray = vec![0.0_f32; (8) as usize];
        this.regionTextureVertexFloatBuffer = jcall!(jcall!(ByteBuffer::allocateDirect(4 * 4 * 2)).order(ByteOrder::nativeOrder())).asFloatBuffer();
        // super(...) is in the struct literal above
        this.initVertices();
        { let java_receiver = this.regionRectangleVertexFloatBuffer.clone(); let java_arguments = (FloatBuffer::wrap(this.regionRectangleFloatArray.clone()),); jcall!(java_receiver).put(java_arguments.0); };
        { let java_receiver = this.regionTextureVertexFloatBuffer.clone(); let java_arguments = (FloatBuffer::wrap(this.regionTextureRectangleFloatArray.clone()),); jcall!(java_receiver).put(java_arguments.0); };
        this
    }

    pub fn new(mut image: Option<Rc<RefCell<Image>>>, mut bitmapFactory: Option<Rc<RefCell<dyn crate::org::allbinary::platform::graphics::PlatformBitmapBaseFactory::PlatformBitmapBaseFactoryDyn>>>, mut textureFactory: Option<Rc<RefCell<dyn crate::org::allbinary::platform::opengles::PlatformTextureBaseFactory::PlatformTextureBaseFactoryDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(image, bitmapFactory, textureFactory))))
    }
    

    pub fn initVertices(&mut self) {
        jcall!(self.textureVertexFloatBuffer).put(0);
        jcall!(self.textureVertexFloatBuffer).put(1);
        jcall!(self.textureVertexFloatBuffer).put(1);
        jcall!(self.textureVertexFloatBuffer).put(1);
        jcall!(self.textureVertexFloatBuffer).put(0);
        jcall!(self.textureVertexFloatBuffer).put(0);
        jcall!(self.textureVertexFloatBuffer).put(1);
        jcall!(self.textureVertexFloatBuffer).put(0);
        jcall!(self.textureVertexFloatBuffer).rewind();
    }
    

    pub fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) {
        let mut gl10: Option<Rc<RefCell<dyn GL10>>> = panic!("ClassCastException: a Rust dyn GL cannot be cast to dyn GL10");
        if self.base.initTexture(gl10.clone()) {
            jcall!(self.textureFactory).load(gl10.clone(), unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.clone() }.clone(), 0, None /* Java this: a &mut self method has no Rc of itself */, 0, true);
            jcall!(gl10).glDisable(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.clone() }.clone());
            jcall!(OpenGLLogUtil::getInstance()).logError2(gl10.clone(), None /* Java this: a &mut self method has no Rc of itself */);
        }
    }
    

    pub fn drawRegion(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut viewHeight: i32, mut x_src: f32, mut y_src: f32, mut width: f32, mut height: f32, mut x: i32, mut y: i32, mut z: i32) {
        self.regionRectangleFloatArray[(0) as usize] = ((x) as f32).clone();
        self.regionRectangleFloatArray[(7) as usize] = ((viewHeight - y) as f32);
        self.regionRectangleFloatArray[(1) as usize] = self.regionRectangleFloatArray[(7) as usize] - height;
        self.regionRectangleFloatArray[(3) as usize] = ((x) as f32) + width;
        self.regionRectangleFloatArray[(4) as usize] = self.regionRectangleFloatArray[(1) as usize].clone();
        self.regionRectangleFloatArray[(6) as usize] = ((x) as f32).clone();
        self.regionRectangleFloatArray[(9) as usize] = self.regionRectangleFloatArray[(3) as usize].clone();
        self.regionRectangleFloatArray[(10) as usize] = self.regionRectangleFloatArray[(7) as usize].clone();
        self.regionTextureRectangleFloatArray[(0) as usize] = x_src / self.getWidth();
        self.regionTextureRectangleFloatArray[(1) as usize] = ((y_src + height)) / self.getHeight();
        self.regionTextureRectangleFloatArray[(2) as usize] = ((x_src + width)) / self.getWidth();
        self.regionTextureRectangleFloatArray[(3) as usize] = self.regionTextureRectangleFloatArray[(1) as usize].clone();
        self.regionTextureRectangleFloatArray[(4) as usize] = self.regionTextureRectangleFloatArray[(0) as usize].clone();
        self.regionTextureRectangleFloatArray[(5) as usize] = y_src / self.getHeight();
        self.regionTextureRectangleFloatArray[(6) as usize] = self.regionTextureRectangleFloatArray[(2) as usize].clone();
        self.regionTextureRectangleFloatArray[(7) as usize] = self.regionTextureRectangleFloatArray[(5) as usize].clone();
        jcall!(self.glUtil).position(self.regionRectangleVertexFloatBuffer.clone(), 0);
        jcall!(gl).glVertexPointer(3, unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_FLOAT.clone() }.clone(), 0, self.regionRectangleVertexFloatBuffer.clone());
        jcall!(gl).glEnableClientState(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_VERTEX_ARRAY.clone() }.clone());
        jcall!(gl).glEnable(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.clone() }.clone());
        jcall!(gl).glBindTexture(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.clone() }.clone(), jcall!(self.openGLESImageProperties).textureID.clone());
        jcall!(gl).glEnableClientState(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_COORD_ARRAY.clone() }.clone());
        jcall!(self.glUtil).position(self.regionTextureVertexFloatBuffer.clone(), 0);
        jcall!(gl).glTexCoordPointer(2, unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_FLOAT.clone() }.clone(), 0, self.regionTextureVertexFloatBuffer.clone());
        jcall!(gl).glDrawArrays(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TRIANGLE_STRIP.clone() }.clone(), 0, 4);
        jcall!(gl).glDisableClientState(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_VERTEX_ARRAY.clone() }.clone());
        jcall!(gl).glDisableClientState(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_COORD_ARRAY.clone() }.clone());
        jcall!(gl).glDisable(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.clone() }.clone());
    }
    

    pub fn draw(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut x: i32, mut y: i32, mut z: i32) {
        self.regionRectangleFloatArray[(0) as usize] = ((x) as f32).clone();
        self.regionRectangleFloatArray[(7) as usize] = ((jcall!(DisplayInfoSingleton::getInstance()).getLastHeight() - y) as f32);
        self.regionRectangleFloatArray[(1) as usize] = self.regionRectangleFloatArray[(7) as usize] - self.getHeight();
        self.regionRectangleFloatArray[(3) as usize] = x + self.getWidth();
        self.regionRectangleFloatArray[(4) as usize] = self.regionRectangleFloatArray[(1) as usize].clone();
        self.regionRectangleFloatArray[(6) as usize] = ((x) as f32).clone();
        self.regionRectangleFloatArray[(9) as usize] = self.regionRectangleFloatArray[(3) as usize].clone();
        self.regionRectangleFloatArray[(10) as usize] = self.regionRectangleFloatArray[(7) as usize].clone();
        jcall!(self.glUtil).position(self.regionRectangleVertexFloatBuffer.clone(), 0);
        jcall!(gl).glVertexPointer(3, unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_FLOAT.clone() }.clone(), 0, self.regionRectangleVertexFloatBuffer.clone());
        jcall!(gl).glEnableClientState(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_VERTEX_ARRAY.clone() }.clone());
        jcall!(gl).glEnable(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.clone() }.clone());
        jcall!(gl).glBindTexture(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.clone() }.clone(), jcall!(self.openGLESImageProperties).textureID.clone());
        jcall!(gl).glEnableClientState(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_COORD_ARRAY.clone() }.clone());
        jcall!(self.glUtil).position(self.textureVertexFloatBuffer.clone(), 0);
        jcall!(gl).glTexCoordPointer(2, unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_FLOAT.clone() }.clone(), 0, self.textureVertexFloatBuffer.clone());
        jcall!(gl).glDrawArrays(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TRIANGLE_STRIP.clone() }.clone(), 0, 4);
        jcall!(gl).glDisableClientState(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_VERTEX_ARRAY.clone() }.clone());
        jcall!(gl).glDisableClientState(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_COORD_ARRAY.clone() }.clone());
        jcall!(gl).glDisable(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.clone() }.clone());
    }
}
impl std::ops::Deref for OpenGLESGL11VBOImage {
    type Target = OpenGLESImage;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for OpenGLESGL11VBOImage {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn for OpenGLESGL11VBOImage {
    fn as_OpenGLESImage(&self) -> &crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImage { &self.base }
    fn as_OpenGLESImage_mut(&mut self) -> &mut crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImage { &mut self.base }
    fn drawRegion(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut viewHeight: i32, mut x_src: f32, mut y_src: f32, mut width: f32, mut height: f32, mut x: i32, mut y: i32, mut z: i32) { Self::drawRegion(self, gl, viewHeight, x_src, y_src, width, height, x, y, z) }
    fn draw(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut x: i32, mut y: i32, mut z: i32) { Self::draw(self, gl, x, y, z) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for OpenGLESGL11VBOImage {
    fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) { Self::set(self, gl) }
    fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) { Self::set(self, gl) }
}


