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
// package org.allbinary.game.combat.damage
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation;
use crate::org::allbinary::graphics::displayable::CanvasStrings::CanvasStrings;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::CircularIndexUtil::CircularIndexUtil;
use crate::org::allbinary::view::ViewPosition::ViewPosition;
use crate::org::allbinary::view::ViewPositionBase::ViewPositionBase;
use super::DamageFloaters::DamageFloaters;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AnimationDamageFloaters {
    pub base: DamageFloaters,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>,
    pub animationInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>>>,
    pub circularIndexUtil: Option<Rc<RefCell<CircularIndexUtil>>>,
    pub dx: i32,
    pub dy: i32,
}


impl AnimationDamageFloaters {
    

    pub fn new_struct(mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut animationInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>>>, mut dx: i32, mut dy: i32) -> Self {
        let mut this = Self {
            base: DamageFloaters::new_struct(),
            logUtil: None,
            layerInterface: None,
            animationInterfaceArray: Vec::new(),
            circularIndexUtil: None,
            dx: 0,
            dy: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.animationInterfaceArray = animationInterfaceArray.clone();
        for index in (0..=(this.animationInterfaceArray.len() as i32) - 1).rev() {
            jcall!(this.animationInterfaceArray[(index) as usize]).setFrame(jcall!(this.animationInterfaceArray[(index) as usize]).getSize() - 1);
        }
        this.layerInterface = layerInterface.clone();
        this.circularIndexUtil = CircularIndexUtil::createInstance((this.animationInterfaceArray.len() as i32).clone());
        this.dx = dx.clone();
        this.dy = dy.clone();
        this
    }

    pub fn new(mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut animationInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>>>, mut dx: i32, mut dy: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(layerInterface, animationInterfaceArray, dx, dy))))
    }
    

    pub fn add(&mut self, mut damage: i32) {
        let mut i: i32 = jcall!(self.circularIndexUtil).getIndex();
        jcall!(self.animationInterfaceArray[(i) as usize]).setFrame(0);
        jcall!(self.circularIndexUtil).next();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>> = jcall!(self.layerInterface).getViewPosition();
            let mut x: i32 = jcall!(viewPosition).getX();
            let mut y: i32 = jcall!(viewPosition).getY();
            for index in (0..(self.animationInterfaceArray.len() as i32)) {
                let mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>> = self.animationInterfaceArray[(index) as usize].clone();
                if jcall!(animationInterface).getFrame() < jcall!(animationInterface).getAnimationSize() - 1 {
                    let mut delta: i32 = jcall!(animationInterface).getFrame() * 20;
                    jcall!(animationInterface).paintXY(graphics.clone(), x + self.dx, y - delta + self.dy);
                    jcall!(animationInterface).nextFrame();
                }
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            let mut canvasStrings: Option<Rc<RefCell<CanvasStrings>>> = CanvasStrings::getInstance();
            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(canvasStrings).PAINT.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
}
impl std::ops::Deref for AnimationDamageFloaters {
    type Target = DamageFloaters;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AnimationDamageFloaters {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::combat::damage::DamageFloaters::DamageFloatersDyn for AnimationDamageFloaters {
    fn as_DamageFloaters(&self) -> &crate::org::allbinary::game::combat::damage::DamageFloaters::DamageFloaters { &self.base }
    fn as_DamageFloaters_mut(&mut self) -> &mut crate::org::allbinary::game::combat::damage::DamageFloaters::DamageFloaters { &mut self.base }
    fn add(&mut self, mut damage: i32) { Self::add(self, damage) }
}
impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for AnimationDamageFloaters {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}


