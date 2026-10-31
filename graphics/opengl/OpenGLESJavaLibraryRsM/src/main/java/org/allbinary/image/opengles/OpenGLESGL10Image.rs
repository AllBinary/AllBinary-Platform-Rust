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
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::graphics::opengles::OpenGLLogUtil::OpenGLLogUtil;
use crate::org::allbinary::opengles::GLUtil::GLUtil;
use crate::org::allbinary::platform::graphics::PlatformBitmapBaseFactory::PlatformBitmapBaseFactory;
use crate::org::allbinary::platform::opengles::PlatformTextureBaseFactory::PlatformTextureBaseFactory;
use super::OpenGLESImage::OpenGLESImage;
use super::OpenGLESImageDraw::OpenGLESImageDraw;
use super::OpenGLESImageProperties::OpenGLESImageProperties;
use super::OpenGLESImageProcessor::OpenGLESImageProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLESGL10Image {
    pub base: OpenGLESImage,
    pub textureVertexFloatBuffer: Option<Rc<RefCell<FloatBuffer>>>,
    pub realOpenGLESImageDraw: Option<Rc<RefCell<OpenGLESImageDraw>>>,
    pub openGLESImageDraw: Option<Rc<RefCell<OpenGLESImageDraw>>>,
}


impl OpenGLESGL10Image {
    

    pub fn new_struct(mut image: Option<Rc<RefCell<Image>>>, mut bitmapFactory: Option<Rc<RefCell<dyn crate::org::allbinary::platform::graphics::PlatformBitmapBaseFactory::PlatformBitmapBaseFactoryDyn>>>, mut textureFactory: Option<Rc<RefCell<dyn crate::org::allbinary::platform::opengles::PlatformTextureBaseFactory::PlatformTextureBaseFactoryDyn>>>) -> Self {
        let mut this = Self {
            base: OpenGLESImage::new_struct(image.clone(), bitmapFactory.clone(), textureFactory.clone()),
            textureVertexFloatBuffer: None,
            realOpenGLESImageDraw: None,
            openGLESImageDraw: None,
        };
        this.textureVertexFloatBuffer = jcall!(jcall!(ByteBuffer::allocateDirect(4 * 4 * 2)).order(ByteOrder::nativeOrder())).asFloatBuffer();
        this.realOpenGLESImageDraw = OpenGLESImageDraw::new() /* Java anonymous class body: implement it as a named type
        glUtil: GLUtil::getInstance(),
        displayInfoSingleton: DisplayInfoSingleton::getInstance(),
        textureVertexFloatBuffer: this.textureVertexFloatBuffer.clone(),
        regionTextureRectangleFloatArray: vec![0.0_f32; (8) as usize],
        regionTextureVertexFloatBuffer: jcall!(jcall!(ByteBuffer::allocateDirect(4 * 4 * 2)).order(ByteOrder::nativeOrder())).asFloatBuffer(),
        regionRectangleFloatArray: vec![0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32],
        regionRectangleVertexFloatBuffer: jcall!(jcall!(ByteBuffer::allocateDirect(4 * 4 * 3)).order(ByteOrder::nativeOrder())).asFloatBuffer(),
        openGLESImageProperties: this.openGLESImageProperties.clone(),
        

        pub fn drawRegion(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut viewHeight: i32, mut x_src: f32, mut y_src: f32, mut width: f32, mut height: f32, mut x: i32, mut y: i32, mut z: i32) {
            let mut imageWidth: i32 = this.getWidth();
            let mut imageHeight: i32 = this.getHeight();
            this.regionRectangleFloatArray[(7) as usize] = viewHeight.clone();
            this.regionRectangleFloatArray[(1) as usize] = this.regionRectangleFloatArray[(7) as usize] - height;
            this.regionRectangleFloatArray[(3) as usize] = width.clone();
            this.regionRectangleFloatArray[(4) as usize] = this.regionRectangleFloatArray[(1) as usize].clone();
            this.regionRectangleFloatArray[(9) as usize] = this.regionRectangleFloatArray[(3) as usize].clone();
            this.regionRectangleFloatArray[(10) as usize] = this.regionRectangleFloatArray[(7) as usize].clone();
            this.regionTextureRectangleFloatArray[(0) as usize] = x_src / imageWidth;
            this.regionTextureRectangleFloatArray[(1) as usize] = ((y_src + height)) / imageHeight;
            this.regionTextureRectangleFloatArray[(2) as usize] = ((x_src + width)) / imageWidth;
            this.regionTextureRectangleFloatArray[(3) as usize] = this.regionTextureRectangleFloatArray[(1) as usize].clone();
            this.regionTextureRectangleFloatArray[(4) as usize] = this.regionTextureRectangleFloatArray[(0) as usize].clone();
            this.regionTextureRectangleFloatArray[(5) as usize] = y_src / imageHeight;
            this.regionTextureRectangleFloatArray[(6) as usize] = this.regionTextureRectangleFloatArray[(2) as usize].clone();
            this.regionTextureRectangleFloatArray[(7) as usize] = this.regionTextureRectangleFloatArray[(5) as usize].clone();
            let mut u_center: f32 = (this.regionTextureRectangleFloatArray[(0) as usize] + this.regionTextureRectangleFloatArray[(2) as usize]) / 2.0_f32;
            let mut v_center: f32 = (this.regionTextureRectangleFloatArray[(5) as usize] + this.regionTextureRectangleFloatArray[(1) as usize]) / 2.0_f32;
            jcall!(this.glUtil).rotateUVs(this.regionTextureRectangleFloatArray.clone(), -jcall!(this.openGLESImageProperties).angle, u_center.clone(), v_center.clone());
            jcall!(gl).glPushMatrix();
            let mut imageProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImageProcessor::OpenGLESImageProcessorDyn>>> = this.imageProcessor.clone();
            jcall!(this.openGLESImageTranslate).translate(gl.clone(), this, x.clone(), -y);
            jcall!(imageProcessor).scale(gl.clone(), jcall!(this.openGLESImageProperties).scaleX.clone(), jcall!(this.openGLESImageProperties).scaleY.clone());
            jcall!(imageProcessor).colorMask(gl.clone(), jcall!(this.openGLESImageProperties).redf.clone(), jcall!(this.openGLESImageProperties).greenf.clone(), jcall!(this.openGLESImageProperties).bluef.clone(), jcall!(this.openGLESImageProperties).alphaf.clone());
            jcall!(this.openGLESImageTranslate).translate2(gl.clone(), this);
            jcall!(this.regionRectangleVertexFloatBuffer).put(this.regionRectangleFloatArray.clone());
            jcall!(this.glUtil).position(this.regionRectangleVertexFloatBuffer.clone(), 0);
            jcall!(gl).glVertexPointer(3, unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_FLOAT.clone() }.clone(), 0, this.regionRectangleVertexFloatBuffer.clone());
            jcall!(gl).glEnableClientState(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_VERTEX_ARRAY.clone() }.clone());
            jcall!(gl).glEnable(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.clone() }.clone());
            jcall!(gl).glBindTexture(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.clone() }.clone(), jcall!(this.openGLESImageProperties).textureID.clone());
            jcall!(gl).glEnableClientState(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_COORD_ARRAY.clone() }.clone());
            jcall!(this.regionTextureVertexFloatBuffer).put(this.regionTextureRectangleFloatArray.clone());
            jcall!(this.glUtil).position(this.regionTextureVertexFloatBuffer.clone(), 0);
            jcall!(gl).glTexCoordPointer(2, unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_FLOAT.clone() }.clone(), 0, this.regionTextureVertexFloatBuffer.clone());
            jcall!(gl).glDrawArrays(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TRIANGLE_STRIP.clone() }.clone(), 0, 4);
            jcall!(gl).glDisableClientState(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_VERTEX_ARRAY.clone() }.clone());
            jcall!(gl).glDisableClientState(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_COORD_ARRAY.clone() }.clone());
            jcall!(gl).glDisable(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.clone() }.clone());
            jcall!(gl).glPopMatrix();
        }
        

        pub fn draw(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut x: i32, mut y: i32, mut z: i32) {
            let mut width: i32 = this.getWidth();
            let mut height: i32 = this.getHeight();
            this.regionRectangleFloatArray[(7) as usize] = jcall!(this.displayInfoSingleton).getLastHeight();
            this.regionRectangleFloatArray[(1) as usize] = this.regionRectangleFloatArray[(7) as usize] - height;
            this.regionRectangleFloatArray[(3) as usize] = width.clone();
            this.regionRectangleFloatArray[(4) as usize] = this.regionRectangleFloatArray[(1) as usize].clone();
            this.regionRectangleFloatArray[(9) as usize] = this.regionRectangleFloatArray[(3) as usize].clone();
            this.regionRectangleFloatArray[(10) as usize] = this.regionRectangleFloatArray[(7) as usize].clone();
            this.regionTextureRectangleFloatArray[(0) as usize] = 0;
            this.regionTextureRectangleFloatArray[(1) as usize] = 1;
            this.regionTextureRectangleFloatArray[(2) as usize] = 1;
            this.regionTextureRectangleFloatArray[(3) as usize] = 1;
            this.regionTextureRectangleFloatArray[(4) as usize] = 0;
            this.regionTextureRectangleFloatArray[(5) as usize] = 0;
            this.regionTextureRectangleFloatArray[(6) as usize] = 1;
            this.regionTextureRectangleFloatArray[(7) as usize] = 0;
            jcall!(gl).glPushMatrix();
            jcall!(this.openGLESImageTranslate).translate(gl.clone(), this, x.clone(), -y);
            let mut imageProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImageProcessor::OpenGLESImageProcessorDyn>>> = this.imageProcessor.clone();
            jcall!(imageProcessor).scale(gl.clone(), jcall!(this.openGLESImageProperties).scaleX.clone(), jcall!(this.openGLESImageProperties).scaleY.clone());
            jcall!(imageProcessor).colorMask(gl.clone(), jcall!(this.openGLESImageProperties).redf.clone(), jcall!(this.openGLESImageProperties).greenf.clone(), jcall!(this.openGLESImageProperties).bluef.clone(), jcall!(this.openGLESImageProperties).alphaf.clone());
            jcall!(this.openGLESImageTranslate).translate2(gl.clone(), this);
            jcall!(this.regionRectangleVertexFloatBuffer).put(this.regionRectangleFloatArray.clone());
            jcall!(this.glUtil).position(this.regionRectangleVertexFloatBuffer.clone(), 0);
            jcall!(gl).glVertexPointer(3, unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_FLOAT.clone() }.clone(), 0, this.regionRectangleVertexFloatBuffer.clone());
            jcall!(gl).glEnableClientState(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_VERTEX_ARRAY.clone() }.clone());
            jcall!(gl).glEnable(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.clone() }.clone());
            jcall!(gl).glBindTexture(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.clone() }.clone(), jcall!(this.openGLESImageProperties).textureID.clone());
            jcall!(gl).glEnableClientState(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_COORD_ARRAY.clone() }.clone());
            jcall!(this.glUtil).rotateUVs(this.regionTextureRectangleFloatArray.clone(), -jcall!(this.openGLESImageProperties).angle, 0.5_f32, 0.5_f32);
            jcall!(this.glUtil).position(this.textureVertexFloatBuffer.clone(), 0);
            jcall!(this.textureVertexFloatBuffer).put(this.regionTextureRectangleFloatArray.clone());
            jcall!(this.glUtil).position(this.textureVertexFloatBuffer.clone(), 0);
            jcall!(gl).glTexCoordPointer(2, unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_FLOAT.clone() }.clone(), 0, this.textureVertexFloatBuffer.clone());
            jcall!(gl).glDrawArrays(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TRIANGLE_STRIP.clone() }.clone(), 0, 4);
            jcall!(gl).glDisableClientState(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_VERTEX_ARRAY.clone() }.clone());
            jcall!(gl).glDisableClientState(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_COORD_ARRAY.clone() }.clone());
            jcall!(gl).glDisable(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.clone() }.clone());
            jcall!(gl).glPopMatrix();
        }
*/;
        this.openGLESImageDraw = OpenGLESImageDraw::getInstance();
        // super(...) is in the struct literal above
        this.initVertices();
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
        let mut gl10: Option<Rc<RefCell<dyn GL10>>> = (gl).clone() /* as GL10 */;
        if self.base.initTexture(gl10.clone()) {
            jcall!(self.textureFactory).load(gl10.clone(), unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.clone() }.clone(), 0, None /* Java this: a &mut self method has no Rc of itself */, 0, true);
            jcall!(gl10).glDisable(unsafe { crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.clone() }.clone());
            self.openGLESImageDraw = self.realOpenGLESImageDraw.clone();
            jcall!(OpenGLLogUtil::getInstance()).logError2(gl10.clone(), None /* Java this: a &mut self method has no Rc of itself */);
        }
    }
    

    pub fn drawRegion(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut viewHeight: i32, mut x_src: f32, mut y_src: f32, mut width: f32, mut height: f32, mut x: i32, mut y: i32, mut z: i32) {
        jcall!(self.openGLESImageDraw).drawRegion(gl.clone(), viewHeight.clone(), x_src.clone(), y_src.clone(), width.clone(), height.clone(), x.clone(), y.clone(), z.clone());
    }
    

    pub fn draw(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut x: i32, mut y: i32, mut z: i32) {
        jcall!(self.openGLESImageDraw).draw(gl.clone(), x.clone(), y.clone(), z.clone());
    }
}
impl std::ops::Deref for OpenGLESGL10Image {
    type Target = OpenGLESImage;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for OpenGLESGL10Image {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn for OpenGLESGL10Image {
    fn as_OpenGLESImage(&self) -> &crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImage { &self.base }
    fn as_OpenGLESImage_mut(&mut self) -> &mut crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImage { &mut self.base }
    fn drawRegion(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut viewHeight: i32, mut x_src: f32, mut y_src: f32, mut width: f32, mut height: f32, mut x: i32, mut y: i32, mut z: i32) { Self::drawRegion(self, gl, viewHeight, x_src, y_src, width, height, x, y, z) }
    fn draw(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut x: i32, mut y: i32, mut z: i32) { Self::draw(self, gl, x, y, z) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for OpenGLESGL10Image {
    fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) { Self::set(self, gl) }
    fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) { Self::set(self, gl) }
}


