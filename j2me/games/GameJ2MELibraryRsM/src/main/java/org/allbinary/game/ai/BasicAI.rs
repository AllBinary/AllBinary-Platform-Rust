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
// package org.allbinary.game.ai
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence;
use crate::org::allbinary::game::displayable::canvas::CanvasUtil::CanvasUtil;
use crate::org::allbinary::game::input::GameInput::GameInput;
use crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface;
use crate::org::allbinary::game::input::event::GameKeyEventFactory::GameKeyEventFactory;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicAI {
    pub base: ArtificialIntelligence,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub gameKeyEventFactory: Option<Rc<RefCell<GameKeyEventFactory>>>,
    pub ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>,
    pub gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>,
    pub lastKey: i32,
}

thread_local! { pub static BasicAI_AI_VISITOR: Option<i32> = jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(2); }
thread_local! { pub static BasicAI_ID: Option<i32> = jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(1); }
impl GameKeyEventSourceInterface for BasicAI {
    fn getSourceId(&mut self) -> i32 { Self::getSourceId(self) }
}
impl crate::org::allbinary::ai::ArtificialIntelligenceInterface::ArtificialIntelligenceInterface for BasicAI {
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::graphics::IdentifierInterface::IdentifierInterface for BasicAI {
    fn getId(&mut self) -> i32 { self.base.getId() }
}

impl BasicAI {
    

    pub fn new_struct(mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Self {
        let mut this = Self {
            base: ArtificialIntelligence::new_struct(),
            logUtil: None,
            commonStrings: None,
            gameKeyEventFactory: None,
            ownerLayerInterface: None,
            gameInput: None,
            lastKey: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.gameKeyEventFactory = GameKeyEventFactory::getInstance();
        this.lastKey = -1;
        this.ownerLayerInterface = ownerLayerInterface.clone();
        this.gameInput = gameInput.clone();
        this
    }

    pub fn new(mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(ownerLayerInterface, gameInput))))
    }
    

    // throws Exception
    pub fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(self.commonStrings).NOT_IMPLEMENTED.clone())));
    }
    

    // throws Exception
    pub fn processKeyAI(&mut self, mut key: i32) {
        if key != -1 {
            { let java_receiver = self.gameInput.clone(); let java_arguments = (jcall!(self.gameKeyEventFactory).getInstanceForKey(None /* Java this: a &mut self method has no Rc of itself */, key.clone()),); jcall!(java_receiver).add(java_arguments.0); };
        }
    }
    

    pub fn setLastKey(&mut self, mut lastKey: i32) {
        self.lastKey = lastKey.clone();
    }
    

    pub fn getLastKey(&mut self) -> i32 {
        return self.lastKey.clone();
    }
    

    pub fn getOwnerLayerInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> {
        return self.ownerLayerInterface.clone();
    }
    

    pub fn getGameInput(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>> {
        return self.gameInput.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return std::any::type_name_of_val(&self).to_string();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).NEW_LINE.clone());
        jcall!(stringBuffer).append(self.getName());
        jcall!(stringBuffer).append(String::from(" LastKey: "));
        jcall!(stringBuffer).append(CanvasUtil::getKeyName(self.getLastKey()));
        return jcall!(stringBuffer).toString();
    }
    

    pub fn getSourceId(&mut self) -> i32 {
        return 1;
    }
}
impl std::ops::Deref for BasicAI {
    type Target = ArtificialIntelligence;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicAI {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait BasicAIDyn: crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligenceDyn + crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface {
    fn as_BasicAI(&self) -> &BasicAI;
    fn as_BasicAI_mut(&mut self) -> &mut BasicAI;
    fn processKeyAI(&mut self, mut key: i32) { BasicAI::processKeyAI(self.as_BasicAI_mut(), key) }
    fn setLastKey(&mut self, mut lastKey: i32) { BasicAI::setLastKey(self.as_BasicAI_mut(), lastKey) }
    fn getLastKey(&mut self) -> i32 { BasicAI::getLastKey(self.as_BasicAI_mut()) }
    fn getOwnerLayerInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> { BasicAI::getOwnerLayerInterface(self.as_BasicAI_mut()) }
    fn getGameInput(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>> { BasicAI::getGameInput(self.as_BasicAI_mut()) }
    fn getName(&mut self) -> String { BasicAI::getName(self.as_BasicAI_mut()) }
}
impl BasicAIDyn for BasicAI {
    fn as_BasicAI(&self) -> &BasicAI { self }
    fn as_BasicAI_mut(&mut self) -> &mut BasicAI { self }
}

impl crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligenceDyn for BasicAI {
    fn as_ArtificialIntelligence(&self) -> &crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &self.base }
    fn as_ArtificialIntelligence_mut(&mut self) -> &mut crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &mut self.base }
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}


