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
// package org.allbinary.game.configuration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Gauge::Gauge;
use super::GameConfiguration::GameConfiguration;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameConfigurationGauge {
    pub base: Gauge,
}


impl GameConfigurationGauge {
    

    pub fn new_struct(mut gameConfiguration: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>>) -> Self {
        let mut this = Self {
            base: Gauge::new_struct(jcall!(gameConfiguration).getName(), jcall!(gameConfiguration).isModifiable().unwrap(), ((jcall!(gameConfiguration).getMaxValue()).unwrap() as i32) - ((jcall!(gameConfiguration).getMinValue()).unwrap() as i32), ((jcall!(gameConfiguration).getValue()).unwrap() as i32) - ((jcall!(gameConfiguration).getMinValue()).unwrap() as i32)),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut gameConfiguration: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(gameConfiguration))))
    }
}
impl std::ops::Deref for GameConfigurationGauge {
    type Target = Gauge;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GameConfigurationGauge {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::javax::microedition::lcdui::Gauge::GaugeDyn for GameConfigurationGauge {
    fn as_Gauge(&self) -> &crate::javax::microedition::lcdui::Gauge::Gauge { &self.base }
    fn as_Gauge_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Gauge::Gauge { &mut self.base }
}
impl crate::javax::microedition::lcdui::Item::ItemDyn for GameConfigurationGauge {
    fn as_Item(&self) -> &crate::javax::microedition::lcdui::Item::Item { &self.base.base }
    fn as_Item_mut(&mut self) -> &mut crate::javax::microedition::lcdui::Item::Item { &mut self.base.base }
    fn addCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::javax::microedition::lcdui::Gauge::Gauge::addCommand(crate::javax::microedition::lcdui::Gauge::GaugeDyn::as_Gauge_mut(self), p1) }
    fn setDefaultCommand(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>) { crate::javax::microedition::lcdui::Gauge::Gauge::setDefaultCommand(crate::javax::microedition::lcdui::Gauge::GaugeDyn::as_Gauge_mut(self), p1) }
    fn setItemCommandListener(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::ItemCommandListener::ItemCommandListener>>>) { crate::javax::microedition::lcdui::Gauge::Gauge::setItemCommandListener(crate::javax::microedition::lcdui::Gauge::GaugeDyn::as_Gauge_mut(self), p1) }
    fn setLabel(&mut self, p1: String) { crate::javax::microedition::lcdui::Gauge::Gauge::setLabel(crate::javax::microedition::lcdui::Gauge::GaugeDyn::as_Gauge_mut(self), p1) }
    fn setLayout(&mut self, p1: i32) { crate::javax::microedition::lcdui::Gauge::Gauge::setLayout(crate::javax::microedition::lcdui::Gauge::GaugeDyn::as_Gauge_mut(self), p1) }
    fn setPreferredSize(&mut self, p1: i32, p2: i32) { crate::javax::microedition::lcdui::Gauge::Gauge::setPreferredSize(crate::javax::microedition::lcdui::Gauge::GaugeDyn::as_Gauge_mut(self), p1, p2) }
    fn getHeight(&mut self) -> i32 { crate::javax::microedition::lcdui::Gauge::Gauge::getHeight(crate::javax::microedition::lcdui::Gauge::GaugeDyn::as_Gauge_mut(self)) }
    fn isFocusable(&mut self) -> bool { crate::javax::microedition::lcdui::Gauge::Gauge::isFocusable(crate::javax::microedition::lcdui::Gauge::GaugeDyn::as_Gauge_mut(self)) }
    fn keyPressed(&mut self, p1: i32) { crate::javax::microedition::lcdui::Gauge::Gauge::keyPressed(crate::javax::microedition::lcdui::Gauge::GaugeDyn::as_Gauge_mut(self), p1) }
    fn paintItem(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) -> i32 { crate::javax::microedition::lcdui::Gauge::Gauge::paintItem(crate::javax::microedition::lcdui::Gauge::GaugeDyn::as_Gauge_mut(self), p1) }
    fn traverseItem(&mut self, p1: i32, p2: i32, p3: i32, p4: bool) -> i32 { crate::javax::microedition::lcdui::Gauge::Gauge::traverseItem(crate::javax::microedition::lcdui::Gauge::GaugeDyn::as_Gauge_mut(self), p1, p2, p3, p4) }
}


