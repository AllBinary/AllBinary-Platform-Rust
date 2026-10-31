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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput;
use crate::org::allbinary::game::input::PlayerGameInputCompositeInterface::PlayerGameInputCompositeInterface;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::graphics::displayable::CanvasStrings::CanvasStrings;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::view::StaticViewPosition::StaticViewPosition;
use super::AllBinaryGameLayer::AllBinaryGameLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PlayerGameInputGameLayer {
    pub base: AllBinaryGameLayer,
    pub playerGameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn>>>,
}

// impl PlayerGameInputCompositeInterface for PlayerGameInputGameLayer with the methods of the trait
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for PlayerGameInputGameLayer {
    fn getName(&mut self) -> String { self.base.getName() }
}

impl PlayerGameInputGameLayer {
    

    pub fn new_struct(mut playerInputId: i32) -> Self {
        let mut this = Self {
            base: AllBinaryGameLayer::new_struct(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), Rectangle::new(jcall!(PointFactory::getInstance()).ZERO_ZERO.clone(), 0, 0), StaticViewPosition::new(0, 0, 0)),
            playerGameInput: None,
        };
        // super(...) is in the struct literal above
        this.playerGameInput = PlayerGameInput::new(this.getGameKeyEventList(), BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>), playerInputId.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn>>);
        let mut canvasStrings: Option<Rc<RefCell<CanvasStrings>>> = CanvasStrings::getInstance();
        jcall!(this.logUtil).putF(String::from("Danger Danger Danger: Should Not Be Called Except For Testing Input"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(canvasStrings).PAINT.clone());
        this
    }

    pub fn new(mut playerInputId: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(playerInputId))))
    }
    

    pub fn getPlayerGameInput(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn>>> {
        return self.playerGameInput.clone();
    }
    

    pub fn initInputProcessors(&mut self) {
    }
    

    // throws Exception
    pub fn processInput(&mut self, mut myManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
}
impl std::ops::Deref for PlayerGameInputGameLayer {
    type Target = AllBinaryGameLayer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PlayerGameInputGameLayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn for PlayerGameInputGameLayer {
    fn as_AllBinaryGameLayer(&self) -> &crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &self.base }
    fn as_AllBinaryGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &mut self.base }
    fn processInput(&mut self, mut myManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processInput(self, myManager) }
}
impl crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn for PlayerGameInputGameLayer {
    fn as_AllBinaryLayer(&self) -> &crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &self.base.base }
    fn as_AllBinaryLayer_mut(&mut self) -> &mut crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &mut self.base.base }
    fn onChangeEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::view::event::ViewPositionEvent::ViewPositionEvent>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::onChangeEvent(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
}
impl crate::org::allbinary::layer::Layer::LayerDyn for PlayerGameInputGameLayer {
    fn as_Layer(&self) -> &crate::org::allbinary::layer::Layer::Layer { &self.base.base.base }
    fn as_Layer_mut(&mut self) -> &mut crate::org::allbinary::layer::Layer::Layer { &mut self.base.base.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn toStringAppend(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::logic::string::StringMaker::StringMaker>>>) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::toStringAppend(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for PlayerGameInputGameLayer {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::set(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
}


