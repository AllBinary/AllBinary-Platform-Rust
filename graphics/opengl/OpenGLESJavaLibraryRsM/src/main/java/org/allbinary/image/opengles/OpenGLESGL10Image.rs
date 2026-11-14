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
use crate::java::nio::ByteBuffer::ByteBuffer;
// use crate::java::nio::ByteOrder::ByteOrder;
use crate::java::nio::FloatBuffer::FloatBuffer;
use crate::javax::microedition::khronos::opengles::GL::GL;
use crate::javax::microedition::khronos::opengles::GL10::GL10;
use crate::javax::microedition::lcdui::Image::Image;
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
    

    pub fn new_struct(mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut bitmapFactory: Option<Rc<RefCell<dyn crate::org::allbinary::platform::graphics::PlatformBitmapBaseFactory::PlatformBitmapBaseFactoryDyn>>>, mut textureFactory: Option<Rc<RefCell<dyn crate::org::allbinary::platform::opengles::PlatformTextureBaseFactory::PlatformTextureBaseFactoryDyn>>>) -> Self {
        let mut this = Self {
            base: OpenGLESImage::new_struct(image.clone(), bitmapFactory.clone(), textureFactory.clone()),
            textureVertexFloatBuffer: None,
            realOpenGLESImageDraw: None,
            openGLESImageDraw: None,
        };
        this.textureVertexFloatBuffer = jcall!(jcall!(ByteBuffer::allocateDirect(4 * 4 * 2)).order(ByteOrder::nativeOrder())).asFloatBuffer();
        this.openGLESImageDraw = OpenGLESImageDraw::getInstance();
        // super(...) is in the struct literal above
        this.initVertices();
        this
    }

    pub fn new(mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut bitmapFactory: Option<Rc<RefCell<dyn crate::org::allbinary::platform::graphics::PlatformBitmapBaseFactory::PlatformBitmapBaseFactoryDyn>>>, mut textureFactory: Option<Rc<RefCell<dyn crate::org::allbinary::platform::opengles::PlatformTextureBaseFactory::PlatformTextureBaseFactoryDyn>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(image.clone(), bitmapFactory.clone(), textureFactory.clone()))));
        jcall!(this).realOpenGLESImageDraw = OpenGLESImageDraw::new() /* Java anonymous class body: implement it as a named type
        glUtil: GLUtil::getInstance(),
        displayInfoSingleton: DisplayInfoSingleton::getInstance(),
        textureVertexFloatBuffer: jcall!(this).textureVertexFloatBuffer.clone(),
        regionTextureRectangleFloatArray: vec![0.0_f32; (8) as usize],
        regionTextureVertexFloatBuffer: jcall!(jcall!(ByteBuffer::allocateDirect(4 * 4 * 2)).order(ByteOrder::nativeOrder())).asFloatBuffer(),
        regionRectangleFloatArray: vec![0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32],
        regionRectangleVertexFloatBuffer: jcall!(jcall!(ByteBuffer::allocateDirect(4 * 4 * 3)).order(ByteOrder::nativeOrder())).asFloatBuffer(),
        openGLESImageProperties: jcall!(this).openGLESImageProperties.clone(),
        

        pub fn drawRegion(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut viewHeight: i32, mut x_src: f32, mut y_src: f32, mut width: f32, mut height: f32, mut x: i32, mut y: i32, mut z: i32) {
            let mut imageWidth: i32 = jcall!(this).getWidth();
            let mut imageHeight: i32 = jcall!(this).getHeight();
            jcall!(this).regionRectangleFloatArray[(7) as usize] = ((viewHeight) as f32);
            jcall!(this).regionRectangleFloatArray[(1) as usize] = jcall!(this).regionRectangleFloatArray[(7) as usize] - height;
            jcall!(this).regionRectangleFloatArray[(3) as usize] = width.clone();
            jcall!(this).regionRectangleFloatArray[(4) as usize] = { let java_value = (jcall!(this).regionRectangleFloatArray[(1) as usize]).clone(); java_value }.clone();
            jcall!(this).regionRectangleFloatArray[(9) as usize] = { let java_value = (jcall!(this).regionRectangleFloatArray[(3) as usize]).clone(); java_value }.clone();
            jcall!(this).regionRectangleFloatArray[(10) as usize] = { let java_value = (jcall!(this).regionRectangleFloatArray[(7) as usize]).clone(); java_value }.clone();
            jcall!(this).regionTextureRectangleFloatArray[(0) as usize] = x_src / imageWidth;
            jcall!(this).regionTextureRectangleFloatArray[(1) as usize] = (y_src + height) / imageHeight;
            jcall!(this).regionTextureRectangleFloatArray[(2) as usize] = (x_src + width) / imageWidth;
            jcall!(this).regionTextureRectangleFloatArray[(3) as usize] = { let java_value = (jcall!(this).regionTextureRectangleFloatArray[(1) as usize]).clone(); java_value }.clone();
            jcall!(this).regionTextureRectangleFloatArray[(4) as usize] = { let java_value = (jcall!(this).regionTextureRectangleFloatArray[(0) as usize]).clone(); java_value }.clone();
            jcall!(this).regionTextureRectangleFloatArray[(5) as usize] = y_src / imageHeight;
            jcall!(this).regionTextureRectangleFloatArray[(6) as usize] = { let java_value = (jcall!(this).regionTextureRectangleFloatArray[(2) as usize]).clone(); java_value }.clone();
            jcall!(this).regionTextureRectangleFloatArray[(7) as usize] = { let java_value = (jcall!(this).regionTextureRectangleFloatArray[(5) as usize]).clone(); java_value }.clone();
            let mut u_center: f32 = (jcall!(this).regionTextureRectangleFloatArray[(0) as usize] + jcall!(this).regionTextureRectangleFloatArray[(2) as usize]) / 2.0_f32;
            let mut v_center: f32 = (jcall!(this).regionTextureRectangleFloatArray[(5) as usize] + jcall!(this).regionTextureRectangleFloatArray[(1) as usize]) / 2.0_f32;
            jcall!(jcall!(this).glUtil).rotateUVs(jcall!(this).regionTextureRectangleFloatArray.clone(), ((-{ let java_field = jcall!(jcall!(this).openGLESImageProperties).angle.clone(); java_field }) as f32), u_center.clone(), v_center.clone());
            jcall!(gl).glPushMatrix();
            let mut imageProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImageProcessor::OpenGLESImageProcessorDyn>>> = jcall!(this).imageProcessor.clone();
            jcall!(jcall!(this).openGLESImageTranslate).translate(gl.clone(), this, ((x) as f32), ((-y) as f32));
            jcall!(imageProcessor).scale(gl.clone(), { let java_field = jcall!(jcall!(this).openGLESImageProperties).scaleX.clone(); java_field }.clone(), { let java_field = jcall!(jcall!(this).openGLESImageProperties).scaleY.clone(); java_field }.clone());
            jcall!(imageProcessor).colorMask(gl.clone(), { let java_field = jcall!(jcall!(this).openGLESImageProperties).redf.clone(); java_field }.clone(), { let java_field = jcall!(jcall!(this).openGLESImageProperties).greenf.clone(); java_field }.clone(), { let java_field = jcall!(jcall!(this).openGLESImageProperties).bluef.clone(); java_field }.clone(), { let java_field = jcall!(jcall!(this).openGLESImageProperties).alphaf.clone(); java_field }.clone());
            jcall!(jcall!(this).openGLESImageTranslate).translate2(gl.clone(), this);
            jcall!(jcall!(this).regionRectangleVertexFloatBuffer).put(jcall!(this).regionRectangleFloatArray.clone());
            jcall!(jcall!(this).glUtil).position(jcall!(this).regionRectangleVertexFloatBuffer.clone(), 0);
            jcall!(gl).glVertexPointer(3, crate::javax::microedition::khronos::opengles::GL10::GL10_GL_FLOAT.with(Clone::clone).clone(), 0, jcall!(this).regionRectangleVertexFloatBuffer.clone());
            jcall!(gl).glEnableClientState(crate::javax::microedition::khronos::opengles::GL10::GL10_GL_VERTEX_ARRAY.with(Clone::clone).clone());
            jcall!(gl).glEnable(crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.with(Clone::clone).clone());
            jcall!(gl).glBindTexture(crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.with(Clone::clone).clone(), { let java_field = jcall!(jcall!(this).openGLESImageProperties).textureID.clone(); java_field }.clone());
            jcall!(gl).glEnableClientState(crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_COORD_ARRAY.with(Clone::clone).clone());
            jcall!(jcall!(this).regionTextureVertexFloatBuffer).put(jcall!(this).regionTextureRectangleFloatArray.clone());
            jcall!(jcall!(this).glUtil).position(jcall!(this).regionTextureVertexFloatBuffer.clone(), 0);
            jcall!(gl).glTexCoordPointer(2, crate::javax::microedition::khronos::opengles::GL10::GL10_GL_FLOAT.with(Clone::clone).clone(), 0, jcall!(this).regionTextureVertexFloatBuffer.clone());
            jcall!(gl).glDrawArrays(crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TRIANGLE_STRIP.with(Clone::clone).clone(), 0, 4);
            jcall!(gl).glDisableClientState(crate::javax::microedition::khronos::opengles::GL10::GL10_GL_VERTEX_ARRAY.with(Clone::clone).clone());
            jcall!(gl).glDisableClientState(crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_COORD_ARRAY.with(Clone::clone).clone());
            jcall!(gl).glDisable(crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.with(Clone::clone).clone());
            jcall!(gl).glPopMatrix();
        }
        

        pub fn draw(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut x: i32, mut y: i32, mut z: i32) {
            let mut width: i32 = jcall!(this).getWidth();
            let mut height: i32 = jcall!(this).getHeight();
            jcall!(this).regionRectangleFloatArray[(7) as usize] = ((jcall!(jcall!(this).displayInfoSingleton).getLastHeight()) as f32);
            jcall!(this).regionRectangleFloatArray[(1) as usize] = jcall!(this).regionRectangleFloatArray[(7) as usize] - height;
            jcall!(this).regionRectangleFloatArray[(3) as usize] = ((width) as f32);
            jcall!(this).regionRectangleFloatArray[(4) as usize] = { let java_value = (jcall!(this).regionRectangleFloatArray[(1) as usize]).clone(); java_value }.clone();
            jcall!(this).regionRectangleFloatArray[(9) as usize] = { let java_value = (jcall!(this).regionRectangleFloatArray[(3) as usize]).clone(); java_value }.clone();
            jcall!(this).regionRectangleFloatArray[(10) as usize] = { let java_value = (jcall!(this).regionRectangleFloatArray[(7) as usize]).clone(); java_value }.clone();
            jcall!(this).regionTextureRectangleFloatArray[(0) as usize] = 0.0_f32;
            jcall!(this).regionTextureRectangleFloatArray[(1) as usize] = 1.0_f32;
            jcall!(this).regionTextureRectangleFloatArray[(2) as usize] = 1.0_f32;
            jcall!(this).regionTextureRectangleFloatArray[(3) as usize] = 1.0_f32;
            jcall!(this).regionTextureRectangleFloatArray[(4) as usize] = 0.0_f32;
            jcall!(this).regionTextureRectangleFloatArray[(5) as usize] = 0.0_f32;
            jcall!(this).regionTextureRectangleFloatArray[(6) as usize] = 1.0_f32;
            jcall!(this).regionTextureRectangleFloatArray[(7) as usize] = 0.0_f32;
            jcall!(gl).glPushMatrix();
            jcall!(jcall!(this).openGLESImageTranslate).translate(gl.clone(), this, ((x) as f32), ((-y) as f32));
            let mut imageProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImageProcessor::OpenGLESImageProcessorDyn>>> = jcall!(this).imageProcessor.clone();
            jcall!(imageProcessor).scale(gl.clone(), { let java_field = jcall!(jcall!(this).openGLESImageProperties).scaleX.clone(); java_field }.clone(), { let java_field = jcall!(jcall!(this).openGLESImageProperties).scaleY.clone(); java_field }.clone());
            jcall!(imageProcessor).colorMask(gl.clone(), { let java_field = jcall!(jcall!(this).openGLESImageProperties).redf.clone(); java_field }.clone(), { let java_field = jcall!(jcall!(this).openGLESImageProperties).greenf.clone(); java_field }.clone(), { let java_field = jcall!(jcall!(this).openGLESImageProperties).bluef.clone(); java_field }.clone(), { let java_field = jcall!(jcall!(this).openGLESImageProperties).alphaf.clone(); java_field }.clone());
            jcall!(jcall!(this).openGLESImageTranslate).translate2(gl.clone(), this);
            jcall!(jcall!(this).regionRectangleVertexFloatBuffer).put(jcall!(this).regionRectangleFloatArray.clone());
            jcall!(jcall!(this).glUtil).position(jcall!(this).regionRectangleVertexFloatBuffer.clone(), 0);
            jcall!(gl).glVertexPointer(3, crate::javax::microedition::khronos::opengles::GL10::GL10_GL_FLOAT.with(Clone::clone).clone(), 0, jcall!(this).regionRectangleVertexFloatBuffer.clone());
            jcall!(gl).glEnableClientState(crate::javax::microedition::khronos::opengles::GL10::GL10_GL_VERTEX_ARRAY.with(Clone::clone).clone());
            jcall!(gl).glEnable(crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.with(Clone::clone).clone());
            jcall!(gl).glBindTexture(crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.with(Clone::clone).clone(), { let java_field = jcall!(jcall!(this).openGLESImageProperties).textureID.clone(); java_field }.clone());
            jcall!(gl).glEnableClientState(crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_COORD_ARRAY.with(Clone::clone).clone());
            jcall!(jcall!(this).glUtil).rotateUVs(jcall!(this).regionTextureRectangleFloatArray.clone(), ((-{ let java_field = jcall!(jcall!(this).openGLESImageProperties).angle.clone(); java_field }) as f32), 0.5_f32, 0.5_f32);
            jcall!(jcall!(this).glUtil).position(jcall!(this).textureVertexFloatBuffer.clone(), 0);
            jcall!(jcall!(this).textureVertexFloatBuffer).put(jcall!(this).regionTextureRectangleFloatArray.clone());
            jcall!(jcall!(this).glUtil).position(jcall!(this).textureVertexFloatBuffer.clone(), 0);
            jcall!(gl).glTexCoordPointer(2, crate::javax::microedition::khronos::opengles::GL10::GL10_GL_FLOAT.with(Clone::clone).clone(), 0, jcall!(this).textureVertexFloatBuffer.clone());
            jcall!(gl).glDrawArrays(crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TRIANGLE_STRIP.with(Clone::clone).clone(), 0, 4);
            jcall!(gl).glDisableClientState(crate::javax::microedition::khronos::opengles::GL10::GL10_GL_VERTEX_ARRAY.with(Clone::clone).clone());
            jcall!(gl).glDisableClientState(crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_COORD_ARRAY.with(Clone::clone).clone());
            jcall!(gl).glDisable(crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.with(Clone::clone).clone());
            jcall!(gl).glPopMatrix();
        }
*/;
        this
    }
    

    pub fn initVertices(&mut self) {
        jcall!(self.textureVertexFloatBuffer).put(0.0_f32);
        jcall!(self.textureVertexFloatBuffer).put(1.0_f32);
        jcall!(self.textureVertexFloatBuffer).put(1.0_f32);
        jcall!(self.textureVertexFloatBuffer).put(1.0_f32);
        jcall!(self.textureVertexFloatBuffer).put(0.0_f32);
        jcall!(self.textureVertexFloatBuffer).put(0.0_f32);
        jcall!(self.textureVertexFloatBuffer).put(1.0_f32);
        jcall!(self.textureVertexFloatBuffer).put(0.0_f32);
        jcall!(self.textureVertexFloatBuffer).rewind();
    }
    

    pub fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) {
        let mut gl10: Option<Rc<RefCell<dyn GL10>>> = (|| -> Option<Rc<RefCell<dyn GL10>>> { panic!("ClassCastException: a Rust dyn GL cannot be cast to dyn GL10") })();
        if { let java_arguments = (gl10.clone(),); self.base.initTexture(java_arguments.0) } {
            jcall!(self.textureFactory).load(gl10.clone(), crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.with(Clone::clone).clone(), 0, None /* Java this: a &mut self method has no Rc of itself */, 0, true);
            jcall!(gl10).glDisable(crate::javax::microedition::khronos::opengles::GL10::GL10_GL_TEXTURE_2D.with(Clone::clone).clone());
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
impl crate::javax::microedition::lcdui::Image::ImageDyn for OpenGLESGL10Image {
    fn as_Image(&self) -> &crate::javax::microedition::lcdui::Image::Image { &self.base.base }
    fn as_Image_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Image::Image { &mut self.base.base }
    fn getType(&mut self) -> i32 { crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImage::getType(crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn::as_OpenGLESImage_mut(self)) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for OpenGLESGL10Image {
    fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) { Self::set(self, gl) }
}


