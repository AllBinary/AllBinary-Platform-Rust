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
// package org.allbinary.game.ag.ai.scroller
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::org::allbinary::game::ai::BasicAI::BasicAI;
use crate::org::allbinary::game::input::GameInput::GameInput;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::rand::MyRandomFactory::MyRandomFactory;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::media::audio::PlayerQueue::PlayerQueue;
use crate::org::allbinary::media::audio::SecondaryPlayerQueueFactory::SecondaryPlayerQueueFactory;
use crate::org::allbinary::media::audio::Sound::Sound;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::org::allbinary::util::ABHashtable::ABHashtable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HorizontalScreenPatrolAI {
    pub base: BasicAI,
    pub keyDirection: i32,
    pub displayInfoSingleton: Option<Rc<RefCell<DisplayInfoSingleton>>>,
    pub firingX: i32,
    pub firedIndex: i32,
    pub MAX_FIRE: i32,
    pub timeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub secondaryPlayerQueue: Option<Rc<RefCell<PlayerQueue>>>,
    pub sound: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>,
}

thread_local! { pub static HorizontalScreenPatrolAI_SOUND: Option<i32> = jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(433); }
impl crate::org::allbinary::ai::ArtificialIntelligenceInterface::ArtificialIntelligenceInterface for HorizontalScreenPatrolAI {
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::graphics::IdentifierInterface::IdentifierInterface for HorizontalScreenPatrolAI {
    fn getId(&mut self) -> i32 { self.base.getId() }
}

impl HorizontalScreenPatrolAI {
    

    pub fn new_struct(mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Self {
        let mut this = Self {
            base: BasicAI::new_struct(ownerLayerInterface.clone(), gameInput.clone()),
            keyDirection: 0,
            displayInfoSingleton: None,
            firingX: 0,
            firedIndex: 0,
            MAX_FIRE: 0,
            timeDelayHelper: None,
            secondaryPlayerQueue: None,
            sound: None,
        };
        this.keyDirection = unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() }.clone();
        this.displayInfoSingleton = DisplayInfoSingleton::getInstance();
        this.firingX = jcall!(this.displayInfoSingleton).getLastHalfWidth();
        this.firedIndex = 0;
        this.MAX_FIRE = 28;
        this.timeDelayHelper = TimeDelayHelper::new(240).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.secondaryPlayerQueue = SecondaryPlayerQueueFactory::getInstance();
        // super(...) is in the struct literal above
        this.sound = { let java_object = (jcall!(hashtable).get(crate::java_into_object(HorizontalScreenPatrolAI_SOUND.with(Clone::clone).clone()))).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>>().expect("ClassCastException").clone()) };
        if this.sound.is_none() {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No Sound Provided!!"))));
        }
        this
    }

    pub fn new(mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(hashtable, ownerLayerInterface, gameInput))))
    }
    

    // throws Exception
    pub fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        self.base.processKeyAI(self.keyDirection.clone());
        if jcall!(self.timeDelayHelper).isTimeTNT() {
            let mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = self.getOwnerLayerInterface();
            let mut currentOwnerLayerX: i32 = jcall!(layerInterface).getXP();
            if currentOwnerLayerX < -jcall!(layerInterface).getWidth() {
                let mut y: i32 = jcall!(MyRandomFactory::getInstance()).getAbsoluteNextIntAllowZero(50);
                jcall!(layerInterface).setPosition(jcall!(self.displayInfoSingleton).getLastWidth() + jcall!(layerInterface).getWidth() + 50, y.clone(), jcall!(layerInterface).getZP());
                self.firedIndex = 0;
                self.firingX = jcall!(self.displayInfoSingleton).getLastHalfWidth() + jcall!(MyRandomFactory::getInstance()).getNextInt(jcall!(self.displayInfoSingleton).getLastHalfWidth());
                jcall!(self.secondaryPlayerQueue).add(self.sound.clone());
            }
            if currentOwnerLayerX < self.firingX && currentOwnerLayerX > -jcall!(layerInterface).getWidth() {
                self.base.processKeyAI(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM1.clone() }.clone());
                let mut gameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>> = (allBinaryLayerManager).clone() /* as AllBinaryGameLayerManager */;
                let mut FIRE: i32 = (jcall!(jcall!(gameLayerManager).getGameInfo()).getCurrentLevel() + 12) >> 2;
                if FIRE > self.MAX_FIRE {
                    FIRE = self.MAX_FIRE.clone();
                }
                if self.firedIndex > FIRE {
                    self.firedIndex = 0;
                    self.firingX = i32::MIN.clone();
                }
                self.firedIndex += 1;
            }
        }
    }
    

    pub fn nextDirection(&mut self) {
    }
}
impl std::ops::Deref for HorizontalScreenPatrolAI {
    type Target = BasicAI;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HorizontalScreenPatrolAI {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::ai::BasicAI::BasicAIDyn for HorizontalScreenPatrolAI {
    fn as_BasicAI(&self) -> &crate::org::allbinary::game::ai::BasicAI::BasicAI { &self.base }
    fn as_BasicAI_mut(&mut self) -> &mut crate::org::allbinary::game::ai::BasicAI::BasicAI { &mut self.base }
}
impl crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligenceDyn for HorizontalScreenPatrolAI {
    fn as_ArtificialIntelligence(&self) -> &crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &self.base.base }
    fn as_ArtificialIntelligence_mut(&mut self) -> &mut crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &mut self.base.base }
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for HorizontalScreenPatrolAI {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::ai::BasicAI::BasicAI::getSourceId(crate::org::allbinary::game::ai::BasicAI::BasicAIDyn::as_BasicAI_mut(self)) }
}


