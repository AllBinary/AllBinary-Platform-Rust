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
// package org.allbinary.game.layer.hud.basic
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::animation::AnimationInterface::AnimationInterface;
use crate::org::allbinary::game::health::HealthInterface::HealthInterface;
use crate::org::allbinary::graphics::Anchor::Anchor;
use crate::org::allbinary::image::ImageCacheFactory::ImageCacheFactory;
use super::HealthGraphic::HealthGraphic;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageHealthGraphic {
    pub base: HealthGraphic,
    pub image: Vec<Option<Rc<RefCell<Image>>>>,
}


impl ImageHealthGraphic {
    

    pub fn new_struct(mut animationInterface: Option<Rc<RefCell<dyn AnimationInterface>>>, mut healthInterface: Option<Rc<RefCell<dyn HealthInterface>>>, mut location: i32, mut direction: i32) -> Self {
        let mut this = Self {
            base: HealthGraphic::new_struct4(animationInterface.clone(), healthInterface.clone(), location.clone(), direction.clone()),
            image: Vec::new(),
        };
        // super(...) is in the struct literal above
        this.image = vec![None; (jcall!(this.healthInterface).getMaxHealth()) as usize];
        let mut imageCacheFactory: Option<Rc<RefCell<ImageCacheFactory>>> = ImageCacheFactory::getInstance();
        for numOfHealth in (0..=this.base.max) {
            for index in (0..numOfHealth) {
                { let java_receiver = this.animationInterface.clone(); let java_arguments = (jcall!(this.image[(numOfHealth) as usize]).getGraphics(), numOfHealth * 10, 0,); jcall!(java_receiver).paint(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
        }
        this
    }

    pub fn new(mut animationInterface: Option<Rc<RefCell<dyn AnimationInterface>>>, mut healthInterface: Option<Rc<RefCell<dyn HealthInterface>>>, mut location: i32, mut direction: i32) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(animationInterface, healthInterface, location, direction))));
            for numOfHealth in (0..=this.base.max) {
            this.image[(numOfHealth) as usize] = jcall!(imageCacheFactory).get((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), numOfHealth + 1 * 10, 10);
        }
        this
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(graphics).drawImage(self.image[(self.base.max) as usize].clone(), self.base.x.clone(), self.base.y.clone(), unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone());
    }
}
impl std::ops::Deref for ImageHealthGraphic {
    type Target = HealthGraphic;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImageHealthGraphic {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



