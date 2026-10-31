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
// package org.allbinary.game.ag.ai
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::bounds::BoundsVisitorInterface::BoundsVisitorInterface;
use crate::org::allbinary::bounds::LayerBounds::LayerBounds;
use crate::org::allbinary::game::ai::BasicAI::BasicAI;
use crate::org::allbinary::game::input::GameInput::GameInput;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BoundBounceAI {
    pub base: BasicAI,
    pub currentRelativeAngle: i32,
    pub layerBounds: Option<Rc<RefCell<dyn crate::org::allbinary::bounds::LayerBounds::LayerBoundsDyn>>>,
    pub boundsVisitorInterface: Option<Rc<RefCell<dyn BoundsVisitorInterface>>>,
}

impl crate::org::allbinary::graphics::IdentifierInterface::IdentifierInterface for BoundBounceAI {
    fn getId(&mut self) -> i32 { self.base.getId() }
}
impl crate::org::allbinary::ai::ArtificialIntelligenceInterface::ArtificialIntelligenceInterface for BoundBounceAI {
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}

impl BoundBounceAI {
    

    pub fn new_struct(mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>, mut layerBounds: Option<Rc<RefCell<dyn crate::org::allbinary::bounds::LayerBounds::LayerBoundsDyn>>>, mut boundsVisitorInterface: Option<Rc<RefCell<dyn BoundsVisitorInterface>>>) -> Self {
        let mut this = Self {
            base: BasicAI::new_struct(ownerLayerInterface.clone(), gameInput.clone()),
            currentRelativeAngle: 0,
            layerBounds: None,
            boundsVisitorInterface: None,
        };
        this.currentRelativeAngle = 0;
        // super(...) is in the struct literal above
        this.layerBounds = layerBounds.clone();
        this.boundsVisitorInterface = boundsVisitorInterface.clone();
        this
    }

    pub fn new(mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>, mut layerBounds: Option<Rc<RefCell<dyn crate::org::allbinary::bounds::LayerBounds::LayerBoundsDyn>>>, mut boundsVisitorInterface: Option<Rc<RefCell<dyn BoundsVisitorInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(ownerLayerInterface, gameInput, layerBounds, boundsVisitorInterface))))
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut rectange: Option<Rc<RefCell<Rectangle>>> = jcall!(self.layerBounds).getRectangleP();
        jcall!(graphics).drawRect(jcall!(jcall!(rectange).getPoint()).getX(), jcall!(jcall!(rectange).getPoint()).getY(), jcall!(rectange).getWidth(), jcall!(rectange).getHeight());
    }
    

    // throws Exception
    pub fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        jcall!(self.layerBounds).visit(self.boundsVisitorInterface.clone());
    }
}
impl std::ops::Deref for BoundBounceAI {
    type Target = BasicAI;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BoundBounceAI {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::ai::BasicAI::BasicAIDyn for BoundBounceAI {
    fn as_BasicAI(&self) -> &crate::org::allbinary::game::ai::BasicAI::BasicAI { &self.base }
    fn as_BasicAI_mut(&mut self) -> &mut crate::org::allbinary::game::ai::BasicAI::BasicAI { &mut self.base }
}
impl crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligenceDyn for BoundBounceAI {
    fn as_ArtificialIntelligence(&self) -> &crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &self.base.base }
    fn as_ArtificialIntelligence_mut(&mut self) -> &mut crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &mut self.base.base }
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for BoundBounceAI {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::ai::BasicAI::BasicAI::getSourceId(crate::org::allbinary::game::ai::BasicAI::BasicAIDyn::as_BasicAI_mut(self)) }
}


