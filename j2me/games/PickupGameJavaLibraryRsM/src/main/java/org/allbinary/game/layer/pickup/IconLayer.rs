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
// package org.allbinary.game.layer.pickup
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::NullAnimationFactory::NullAnimationFactory;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::view::ViewPosition::ViewPosition;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct IconLayer {
    pub base: AllBinaryLayer,
    pub animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
}

thread_local! { pub static IconLayer_NULL_ICON_LAYER: Option<Rc<RefCell<IconLayer>>> = IconLayer::new(jcall!(NullAnimationFactory::getFactoryInstance()).getInstance(0), 0, 0); }
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for IconLayer {
    fn getName(&mut self) -> String { self.base.getName() }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for IconLayer {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paintThreed(self, graphics) }
}

impl IconLayer {
    

    pub fn new_struct(mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut width: i32, mut height: i32) -> Self {
        let mut this = Self {
            base: AllBinaryLayer::new_struct(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), Rectangle::new(jcall!(PointFactory::getInstance()).ZERO_ZERO.clone(), width.clone(), height.clone()), ViewPosition::getInstanceD()),
            animationInterface: None,
        };
        // super(...) is in the struct literal above
        this.animationInterface = animationInterface.clone();
        this
    }

    pub fn new(mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut width: i32, mut height: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(animationInterface, width, height))))
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.animationInterface).paintXY(graphics.clone(), self.x.clone(), self.y.clone());
    }
    

    pub fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.animationInterface).paintThreedXYZ(graphics.clone(), self.x.clone(), self.y.clone(), self.z.clone());
    }
    

    pub fn getAnimationInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return self.animationInterface.clone();
    }
}
impl std::ops::Deref for IconLayer {
    type Target = AllBinaryLayer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for IconLayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn for IconLayer {
    fn as_AllBinaryLayer(&self) -> &crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &self.base }
    fn as_AllBinaryLayer_mut(&mut self) -> &mut crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &mut self.base }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paintThreed(self, graphics) }
}
impl crate::org::allbinary::layer::Layer::LayerDyn for IconLayer {
    fn as_Layer(&self) -> &crate::org::allbinary::layer::Layer::Layer { &self.base.base }
    fn as_Layer_mut(&mut self) -> &mut crate::org::allbinary::layer::Layer::Layer { &mut self.base.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn toStringAppend(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::logic::string::StringMaker::StringMaker>>>) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::toStringAppend(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1) }
}


