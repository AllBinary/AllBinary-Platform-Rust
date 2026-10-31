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
// package org.allbinary.game.ag.life
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::health::Health::Health;
use crate::org::allbinary::game::health::HealthBarAnimation::HealthBarAnimation;
use crate::org::allbinary::game::health::HealthBarTwodAnimation::HealthBarTwodAnimation;
use crate::org::allbinary::game::health::HealthInterface::HealthInterface;
use crate::org::allbinary::game::health::HealthListenerInterface::HealthListenerInterface;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use super::Life::Life;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HealthIncludingLifeBar {
    pub base: Paintable,
    pub animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::health::HealthBarAnimation::HealthBarAnimationDyn>>>,
    pub life: Option<Rc<RefCell<Life>>>,
    pub healthInterface: Option<Rc<RefCell<dyn HealthInterface>>>,
    pub allbinaryLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>,
}

impl HealthListenerInterface for HealthIncludingLifeBar {
    fn onHealthChange(&mut self) { Self::onHealthChange(self) }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for HealthIncludingLifeBar {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paintThreed(self, graphics) }
}

impl HealthIncludingLifeBar {
    

    // throws Exception
    pub fn createHealthIncludingLifeBar(mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut life: Option<Rc<RefCell<Life>>>, mut healthInterface: Option<Rc<RefCell<Health>>>, mut location: i32, mut direction: i32) -> Option<Rc<RefCell<HealthIncludingLifeBar>>> {
        return HealthIncludingLifeBar::new(layerInterface.clone(), life.clone(), healthInterface.clone(), HealthBarTwodAnimation::new(layerInterface.clone(), location.clone()), direction.clone());
    }
    

    pub fn new_struct(mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut life: Option<Rc<RefCell<Life>>>, mut healthInterface: Option<Rc<RefCell<Health>>>, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::health::HealthBarAnimation::HealthBarAnimationDyn>>>, mut direction: i32) -> Self {
        let mut this = Self {
            base: Paintable::new_struct(),
            animationInterface: None,
            life: None,
            healthInterface: None,
            allbinaryLayer: None,
        };
        this.allbinaryLayer = layerInterface.clone();
        this.animationInterface = animationInterface.clone();
        this.life = life.clone();
        this.healthInterface = healthInterface.clone();
        this.onHealthChange();
        this
    }

    pub fn new(mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut life: Option<Rc<RefCell<Life>>>, mut healthInterface: Option<Rc<RefCell<Health>>>, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::health::HealthBarAnimation::HealthBarAnimationDyn>>>, mut direction: i32) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(layerInterface, life, healthInterface, animationInterface, direction))));
            jcall!(this.healthInterface).addListener((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    pub fn onHealthChange(&mut self) {
        { let java_receiver = self.animationInterface.clone(); let java_arguments = (((jcall!(self.healthInterface).getHealth() + (jcall!(self.healthInterface).getMaxHealth() * ((jcall!(self.life).get()) as i32))) * jcall!(self.allbinaryLayer).getWidth()) / (jcall!(self.healthInterface).getMaxHealth() + (jcall!(self.healthInterface).getMaxHealth() * ((jcall!(self.life).getStartLives()) as i32))),); jcall!(java_receiver).onHealthChange(java_arguments.0); };
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.animationInterface).paintXY(graphics.clone(), 0, 0);
    }
    

    pub fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.animationInterface).paintThreedXYZ(graphics.clone(), 0, 0, 0);
    }
}
impl std::ops::Deref for HealthIncludingLifeBar {
    type Target = Paintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HealthIncludingLifeBar {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for HealthIncludingLifeBar {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paintThreed(self, graphics) }
}


