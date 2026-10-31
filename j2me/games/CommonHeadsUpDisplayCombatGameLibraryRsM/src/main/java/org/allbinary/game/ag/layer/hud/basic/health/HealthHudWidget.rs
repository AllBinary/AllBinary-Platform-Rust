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
// package org.allbinary.game.ag.layer.hud.basic.health
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud;
use crate::org::allbinary::game::health::Health::Health;
use crate::org::allbinary::game::health::HealthListenerInterface::HealthListenerInterface;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface;
use crate::org::allbinary::time::GameTickTimeDelayHelper::GameTickTimeDelayHelper;
use crate::org::allbinary::time::GameTickTimeDelayHelperFactory::GameTickTimeDelayHelperFactory;
use crate::org::allbinary::time::NoTimeDelayHelper::NoTimeDelayHelper;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HealthHudWidget {
    pub base: BasicHud,
    pub healthScale: i32,
    pub animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
    pub healthInterface: Option<Rc<RefCell<Health>>>,
    pub max: i32,
    pub xArray: Vec<i32>,
    pub gameTickTimeDelayHelper: Option<Rc<RefCell<GameTickTimeDelayHelper>>>,
    pub timeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub slowBeatTimeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub mediumBeatTimeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub fastBeatTimeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
}

impl PaintableInterface for HealthHudWidget {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paintThreed(self, graphics) }
}
impl HealthListenerInterface for HealthHudWidget {
    fn onHealthChange(&mut self) { Self::onHealthChange(self) }
}

impl HealthHudWidget {
    

    pub fn new_struct(mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut healthInterface: Option<Rc<RefCell<Health>>>, mut location: i32, mut direction: i32) -> Self {
        let mut this = Self {
            base: BasicHud::new_struct(location.clone(), direction.clone(), 2, jcall!(BasicColorFactory::getInstance()).WHITE.clone()),
            healthScale: 0,
            animationInterface: None,
            healthInterface: None,
            max: 0,
            xArray: Vec::new(),
            gameTickTimeDelayHelper: None,
            timeDelayHelper: None,
            slowBeatTimeDelayHelper: None,
            mediumBeatTimeDelayHelper: None,
            fastBeatTimeDelayHelper: None,
        };
        this.gameTickTimeDelayHelper = GameTickTimeDelayHelperFactory::getInstance();
        this.timeDelayHelper = crate::org::allbinary::time::NoTimeDelayHelper::NoTimeDelayHelper_SINGLETON.with(Clone::clone).clone();
        this.slowBeatTimeDelayHelper = TimeDelayHelper::new(1280).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.mediumBeatTimeDelayHelper = TimeDelayHelper::new(640).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.fastBeatTimeDelayHelper = TimeDelayHelper::new(320).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        // super(...) is in the struct literal above
        this.animationInterface = animationInterface.clone();
        this.healthInterface = healthInterface.clone();
        this.healthScale = (jcall!(this.healthInterface).getMaxHealth() / 6) + 1;
        this.onHealthChange();
        this.updateMaxWidth = jcall!(healthInterface).getMaxHealth() * 16;
        this.updateMaxHeight = 16;
        this.xArray = vec![0; (30) as usize];
        this.update();
        this
    }

    pub fn new(mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut healthInterface: Option<Rc<RefCell<Health>>>, mut location: i32, mut direction: i32) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(animationInterface, healthInterface, location, direction))));
            jcall!(this.healthInterface).addListener((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    pub fn update(&mut self) {
        if self.xArray.is_some() {
            for index in (0..(self.xArray.len() as i32)) {
                self.xArray[(index) as usize] = self.getX() + (index * 16);
            }
        }
    }
    

    pub fn setX(&mut self, mut x: i32) {
        self.base.setX(x.clone());
        self.update();
    }
    

    pub fn onHealthChange(&mut self) {
        self.max = (jcall!(self.healthInterface).getHealth() / self.healthScale);
        self.timeDelayHelper = crate::org::allbinary::time::NoTimeDelayHelper::NoTimeDelayHelper_SINGLETON.with(Clone::clone).clone();
        if self.max <= 1 && jcall!(self.healthInterface).isAlive() {
            self.max = 1;
            if self.healthScale - jcall!(self.healthInterface).getHealth() > (self.healthScale * 2) / 3 {
                self.timeDelayHelper = self.slowBeatTimeDelayHelper.clone();
            } else if self.healthScale - jcall!(self.healthInterface).getHealth() > self.healthScale / 3 {
                self.timeDelayHelper = self.mediumBeatTimeDelayHelper.clone();
            } else {
                self.timeDelayHelper = self.fastBeatTimeDelayHelper.clone();
            }
        }
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        for index in (0..self.max) {
            if jcall!(self.timeDelayHelper).isTime(jcall!(self.gameTickTimeDelayHelper).startTime.clone()) {
                { let java_receiver = self.animationInterface.clone(); let java_arguments = (graphics.clone(), self.xArray[(index) as usize].clone(), self.getY(),); jcall!(java_receiver).paintXY(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
        }
    }
    

    pub fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
}
impl std::ops::Deref for HealthHudWidget {
    type Target = BasicHud;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HealthHudWidget {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::graphics::hud::BasicHud::BasicHudDyn for HealthHudWidget {
    fn as_BasicHud(&self) -> &crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &self.base }
    fn as_BasicHud_mut(&mut self) -> &mut crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &mut self.base }
    fn setX(&mut self, mut x: i32) { Self::setX(self, x) }
}


