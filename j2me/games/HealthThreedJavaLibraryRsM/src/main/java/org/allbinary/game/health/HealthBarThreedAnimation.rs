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
// package org.allbinary.game.health
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::khronos::opengles::GL10::GL10;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::min3d::core::ColorBufferList::ColorBufferList;
use crate::min3d::core::Number3dBufferList::Number3dBufferList;
use crate::min3d::objectPrimitives::Rectangle::Rectangle;
use crate::min3d::vos::Number3d::Number3d;
use crate::org::allbinary::device::OpenGLESGraphics::OpenGLESGraphics;
use crate::org::allbinary::graphics::threed::min3d::renderer::Object3dProcessor::Object3dProcessor;
use crate::org::allbinary::graphics::threed::min3d::renderer::Object3dProcessorUtil::Object3dProcessorUtil;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::view::ViewPositionBase::ViewPositionBase;
use super::HealthBarAnimation::HealthBarAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HealthBarThreedAnimation {
    pub base: HealthBarAnimation,
    pub rectangle: Option<Rc<RefCell<Rectangle>>>,
    pub basicColorFactory: Option<Rc<RefCell<BasicColorFactory>>>,
    pub object3dProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::threed::min3d::renderer::Object3dProcessor::Object3dProcessorDyn>>>,
}

impl crate::org::allbinary::animation::AnimationInterface::AnimationInterface for HealthBarThreedAnimation {
    fn nextFrame(&mut self) { self.base.nextFrame() }
    fn paintThreedXYZ(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32, mut z: i32) { Self::paintThreedXYZ(self, graphics, x, y, z) }
}

impl HealthBarThreedAnimation {
    

    pub fn new_struct(mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut location: i32) -> Self {
        let mut this = Self {
            base: HealthBarAnimation::new_struct(layerInterface.clone(), location.clone()),
            rectangle: None,
            basicColorFactory: None,
            object3dProcessor: None,
        };
        this.basicColorFactory = BasicColorFactory::getInstance();
        this.object3dProcessor = jcall!(Object3dProcessorUtil::getInstance()).getInstanceObject3dProcessor();
        // super(...) is in the struct literal above
        this.basicColor = jcall!(this.basicColorFactory).GREEN.clone();
        this.colorP = jcall!(this.basicColor).intValue();
        this.rectangle = Rectangle::new(0, 0, 1, 1, this.basicColor.clone());
        jcall!(this.rectangle).setDoubleSidedEnabled(true);
        jcall!(this.rectangle).setNormalsEnabled(false);
        jcall!(this.rectangle).setTexturesEnabled(false);
        this
    }

    pub fn new(mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut location: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(layerInterface, location))))
    }
    

    pub fn updateColor(&mut self) {
        let mut colorBufferList: Option<Rc<RefCell<ColorBufferList>>> = jcall!(jcall!(self.rectangle).getVertices()).getColor4BufferList();
        let mut size: i32 = jcall!(colorBufferList).size();
        {
            let mut index: i32 = size.clone();
            while { index -= 1; index } >= 0 {
            jcall!(colorBufferList).set(index.clone(), self.basicColor.clone());
            }
        }
    }
    

    pub fn updateSize(&mut self) {
        let mut number3dBufferList: Option<Rc<RefCell<Number3dBufferList>>> = jcall!(jcall!(self.rectangle).getVertices()).getPointsOnFacesNumber3dBufferList();
        let mut width: i32 = self.x2.clone();
        let mut height: i32 = self.thickness * 2;
        let mut segsH: i32 = 1;
        let mut segsW: i32 = 1;
        let mut row: i32 = 0;
        let mut col: i32 = 0;
        let mut w: f32 = ((width / segsW) as f32);
        let mut h: f32 = ((height / segsH) as f32);
        let mut width5: f32 = ((width) as f32) / 2.0_f32;
        let mut height5: f32 = ((height) as f32) / 2.0_f32;
        let mut index: i32 = 0;
        for row in (0..=segsH) {
            for col in (0..=segsW) {
                jcall!(number3dBufferList).set({ let value = index; index += 1; value }, ((col) as f32) * w - width5, ((row) as f32) * h - height5, 0.0_f32);
            }
        }
    }
    

    pub fn onHealthChange(&mut self, mut newX2: i32) {
        self.base.onHealthChange(newX2.clone());
        self.updateColor();
        self.updateSize();
    }
    

    pub fn paintThreedXYZ(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32, mut z: i32) {
        let mut openGLESGraphics: Option<Rc<RefCell<dyn crate::org::allbinary::device::OpenGLESGraphics::OpenGLESGraphicsDyn>>> = (graphics).clone() /* as OpenGLESGraphics */;
        let mut gl: Option<Rc<RefCell<dyn GL10>>> = jcall!(openGLESGraphics).getGl10();
        let mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>> = jcall!(self.allbinaryLayer).getViewPosition();
        let mut positionNumber3d: Option<Rc<RefCell<Number3d>>> = jcall!(self.rectangle).getPosition();
        jcall!(positionNumber3d).x = -(((jcall!(viewPosition).getX()) as f32));
        jcall!(positionNumber3d).z = -(((jcall!(viewPosition).getY()) as f32));
        jcall!(positionNumber3d).y = ((jcall!(viewPosition).getZ() + jcall!(self.allbinaryLayer).getHeight()) as f32);
        jcall!(self.object3dProcessor).drawObject(gl.clone(), crate::java_into_object(self.rectangle.clone()));
    }
}
impl std::ops::Deref for HealthBarThreedAnimation {
    type Target = HealthBarAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HealthBarThreedAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::health::HealthBarAnimation::HealthBarAnimationDyn for HealthBarThreedAnimation {
    fn as_HealthBarAnimation(&self) -> &crate::org::allbinary::game::health::HealthBarAnimation::HealthBarAnimation { &self.base }
    fn as_HealthBarAnimation_mut(&mut self) -> &mut crate::org::allbinary::game::health::HealthBarAnimation::HealthBarAnimation { &mut self.base }
    fn onHealthChange(&mut self, mut newX2: i32) { Self::onHealthChange(self, newX2) }
}
impl crate::org::allbinary::animation::Animation::AnimationDyn for HealthBarThreedAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base }
    fn paintThreedXYZ(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32, mut z: i32) { Self::paintThreedXYZ(self, graphics, x, y, z) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for HealthBarThreedAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


