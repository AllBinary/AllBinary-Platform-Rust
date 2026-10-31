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
// package org.allbinary.game.ag.layer.pickup.health
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::health::HealthInterface::HealthInterface;
use crate::org::allbinary::game::health::HealthVisitorInterface::HealthVisitorInterface;
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

pub struct HealLayer {
    pub base: AllBinaryLayer,
}

impl HealthVisitorInterface for HealLayer {
    fn visit(&mut self, mut healthInterface: Option<Rc<RefCell<dyn HealthInterface>>>) { Self::visit(self, healthInterface) }
}
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for HealLayer {
    fn getName(&mut self) -> String { self.base.getName() }
}

impl HealLayer {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AllBinaryLayer::new_struct(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), Rectangle::new(jcall!(PointFactory::getInstance()).ZERO_ZERO.clone(), 0, 0), ViewPosition::getInstanceD()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
    

    pub fn visit(&mut self, mut healthInterface: Option<Rc<RefCell<dyn HealthInterface>>>) {
        if jcall!(healthInterface).isAlive() {
            jcall!(healthInterface).healMax();
        } else {
            jcall!(self.logUtil).putF(String::from("Heal attempt on dead: Does not currently occur"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("visit"));
        }
    }
}
impl std::ops::Deref for HealLayer {
    type Target = AllBinaryLayer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HealLayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn for HealLayer {
    fn as_AllBinaryLayer(&self) -> &crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &self.base }
    fn as_AllBinaryLayer_mut(&mut self) -> &mut crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &mut self.base }
}
impl crate::org::allbinary::layer::Layer::LayerDyn for HealLayer {
    fn as_Layer(&self) -> &crate::org::allbinary::layer::Layer::Layer { &self.base.base }
    fn as_Layer_mut(&mut self) -> &mut crate::org::allbinary::layer::Layer::Layer { &mut self.base.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn toStringAppend(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::logic::string::StringMaker::StringMaker>>>) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::toStringAppend(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1) }
}


