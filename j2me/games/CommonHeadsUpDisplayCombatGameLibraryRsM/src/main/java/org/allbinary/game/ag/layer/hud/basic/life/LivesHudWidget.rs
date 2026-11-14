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
// package org.allbinary.game.ag.layer.hud.basic.life
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::game::ag::life::Life::Life;
use crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::displayable::event::DisplayChangeEvent::DisplayChangeEvent;
use crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LivesHudWidget {
    pub base: BasicHud,
    pub lifeInterface: Option<Rc<RefCell<Life>>>,
    pub xArray: Vec<i32>,
    pub animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
}

impl PaintableInterface for LivesHudWidget {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paintThreed(self, graphics) }
}

impl LivesHudWidget {
    

    pub fn new_struct(mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut lifeInterface: Option<Rc<RefCell<Life>>>, mut location: i32, mut direction: i32) -> Self {
        let mut this = Self {
            base: BasicHud::new_struct(location.clone(), direction.clone(), 2, { let java_field = jcall!(BasicColorFactory::getInstance()).WHITE.clone(); java_field }.clone()),
            lifeInterface: None,
            xArray: Vec::new(),
            animationInterface: None,
        };
        // super(...) is in the struct literal above
        this.lifeInterface = lifeInterface.clone();
        let mut size: i32 = ((jcall!(this.getLifeInterface()).getMaxlives()) as i32);
        this.xArray = vec![0; (size) as usize];
        this.animationInterface = animationInterface.clone();
        this.updateMaxWidth = { let java_value = (((jcall!(this.lifeInterface).getMaxlives()) as i32) * 16).clone(); java_value };
        this.updateMaxHeight = 16;
        this
    }

    pub fn new(mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut lifeInterface: Option<Rc<RefCell<Life>>>, mut location: i32, mut direction: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(animationInterface, lifeInterface, location, direction))))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        { let java_arguments = (graphics.clone(),); self.base.updateMeasurement(java_arguments.0) };
        let mut maxLives: i32 = ((jcall!(self.getLifeInterface()).getMaxlives()) as i32);
        for index in (0..maxLives) {
            { let java_index = (index) as usize; self.xArray[java_index] = self.getX() + (index * 16); };
        }
    }
    

    pub fn onDisplayChangeEvent(&mut self, mut displayChangeEvent: Option<Rc<RefCell<DisplayChangeEvent>>>) {
        { let java_arguments = (displayChangeEvent.clone(),); self.base.onDisplayChangeEvent(java_arguments.0) };
        self.myFontProcessor = self.updateMyFontProcessor.clone();
    }
    

    pub fn setX(&mut self, mut x: i32) {
        { let java_arguments = (x.clone(),); self.base.setX(java_arguments.0) };
        self.myFontProcessor = self.updateMyFontProcessor.clone();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        {
            let mut index: i32 = ((jcall!(self.getLifeInterface()).get()) as i32);
            while { index -= 1; index } >= 0 {
            { let java_receiver = self.animationInterface.clone(); let java_arguments = (graphics.clone(), self.xArray[(index) as usize].clone(), self.getY(),); jcall!(java_receiver).paintXY(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
        }
    }
    

    pub fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
    }
    

    pub fn getLifeInterface(&mut self) -> Option<Rc<RefCell<Life>>> {
        return self.lifeInterface.clone();
    }
}
impl std::ops::Deref for LivesHudWidget {
    type Target = BasicHud;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for LivesHudWidget {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::graphics::hud::BasicHud::BasicHudDyn for LivesHudWidget {
    fn as_BasicHud(&self) -> &crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &self.base }
    fn as_BasicHud_mut(&mut self) -> &mut crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &mut self.base }
    fn onDisplayChangeEvent(&mut self, mut displayChangeEvent: Option<Rc<RefCell<DisplayChangeEvent>>>) { Self::onDisplayChangeEvent(self, displayChangeEvent) }
    fn setX(&mut self, mut x: i32) { Self::setX(self, x) }
}
impl crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface for LivesHudWidget {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::updateMeasurement(self, graphics) }
}


