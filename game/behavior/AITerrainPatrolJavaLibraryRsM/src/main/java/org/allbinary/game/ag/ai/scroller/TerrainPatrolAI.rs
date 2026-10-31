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
use crate::org::allbinary::game::input::GameInput::GameInput;
use crate::org::allbinary::game::terrain::BasicTerrainInfo::BasicTerrainInfo;
use crate::org::allbinary::game::terrain::TerrainEvent::TerrainEvent;
use crate::org::allbinary::game::terrain::TerrainEventCircularStaticPool::TerrainEventCircularStaticPool;
use crate::org::allbinary::game::terrain::TerrainEventHandler::TerrainEventHandler;
use crate::org::allbinary::game::terrain::TerrainEventListener::TerrainEventListener;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::math::Angle::Angle;
use crate::org::allbinary::math::AngleFactory::AngleFactory;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::PacePatrolAI::PacePatrolAI;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TerrainPatrolAI {
    pub base: PacePatrolAI,
    pub terrainEventListener: Option<Rc<RefCell<TerrainEventListener>>>,
    pub DOWN: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>,
    pub CLIFF: Option<Rc<RefCell<BasicTerrainInfo>>>,
}

impl crate::org::allbinary::graphics::IdentifierInterface::IdentifierInterface for TerrainPatrolAI {
    fn getId(&mut self) -> i32 { self.base.getId() }
}

impl TerrainPatrolAI {
    

    pub fn new_struct(mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Self {
        let mut this = Self {
            base: PacePatrolAI::new_struct(hashtable.clone(), ownerLayerInterface.clone(), gameInput.clone()),
            terrainEventListener: None,
            DOWN: None,
            CLIFF: None,
        };
        this.terrainEventListener = TerrainEventListener::new();
        this.DOWN = jcall!(AngleFactory::getInstance()).DOWN.clone();
        this.CLIFF = BasicTerrainInfo::new(this.DOWN.clone());
        // super(...) is in the struct literal above
        jcall!(TerrainEventHandler::getInstance(ownerLayerInterface.clone())).addListener(this.terrainEventListener.clone());
        { let java_receiver = this.terrainEventListener.clone(); let java_arguments = (jcall!(TerrainEventCircularStaticPool::getInstance()).getNext(this.CLIFF.clone()),); jcall!(java_receiver).onTerrainEvent(java_arguments.0); };
        this
    }

    pub fn new(mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(hashtable, ownerLayerInterface, gameInput))))
    }
    

    pub fn update(&mut self) {
        self.base.update();
        self.changeDirectionIfCliffReached();
    }
    

    pub fn changeDirectionIfCliffReached(&mut self) {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(self.terrainEventListener).getList();
        let mut size: i32 = jcall!(list).size();
        for index in (0..size) {
            let mut terrainEvent: Option<Rc<RefCell<TerrainEvent>>> = { let java_object = (jcall!(list).removeAt(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<TerrainEvent>>>>().expect("ClassCastException").clone()) };
            let mut basicTerrainInfo: Option<Rc<RefCell<BasicTerrainInfo>>> = jcall!(terrainEvent).getBasicTerrainInfo();
            let mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>> = jcall!(basicTerrainInfo).getAngle();
            if match (&angle, &self.DOWN) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                self.nextDirection();
                if !self.isFollowLimitedByTerrain {
                    jcall!(self.logUtil).putF(String::from("Following Limited"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onTerrainEvent"));
                    self.isFollowLimitedByTerrain = true;
                }
            }
        }
    }
}
impl std::ops::Deref for TerrainPatrolAI {
    type Target = PacePatrolAI;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TerrainPatrolAI {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::ag::ai::scroller::BasePatrolAI::BasePatrolAIDyn for TerrainPatrolAI {
    fn as_BasePatrolAI(&self) -> &crate::org::allbinary::game::ag::ai::scroller::BasePatrolAI::BasePatrolAI { &self.base.base }
    fn as_BasePatrolAI_mut(&mut self) -> &mut crate::org::allbinary::game::ag::ai::scroller::BasePatrolAI::BasePatrolAI { &mut self.base.base }
    fn update(&mut self) { Self::update(self) }
    fn disable(&mut self) { crate::org::allbinary::game::ag::ai::scroller::PacePatrolAI::PacePatrolAI::disable(crate::org::allbinary::game::ag::ai::scroller::PacePatrolAI::PacePatrolAIDyn::as_PacePatrolAI_mut(self)) }
}
impl crate::org::allbinary::game::ag::ai::scroller::PacePatrolAI::PacePatrolAIDyn for TerrainPatrolAI {
    fn as_PacePatrolAI(&self) -> &crate::org::allbinary::game::ag::ai::scroller::PacePatrolAI::PacePatrolAI { &self.base }
    fn as_PacePatrolAI_mut(&mut self) -> &mut crate::org::allbinary::game::ag::ai::scroller::PacePatrolAI::PacePatrolAI { &mut self.base }
}
impl crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligenceDyn for TerrainPatrolAI {
    fn as_ArtificialIntelligence(&self) -> &crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &self.base.base.base.base }
    fn as_ArtificialIntelligence_mut(&mut self) -> &mut crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &mut self.base.base.base.base }
    fn processAI(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { crate::org::allbinary::game::ag::ai::scroller::PacePatrolAI::PacePatrolAI::processAI(crate::org::allbinary::game::ag::ai::scroller::PacePatrolAI::PacePatrolAIDyn::as_PacePatrolAI_mut(self), p1) }
}
impl crate::org::allbinary::game::ai::BasicAI::BasicAIDyn for TerrainPatrolAI {
    fn as_BasicAI(&self) -> &crate::org::allbinary::game::ai::BasicAI::BasicAI { &self.base.base.base }
    fn as_BasicAI_mut(&mut self) -> &mut crate::org::allbinary::game::ai::BasicAI::BasicAI { &mut self.base.base.base }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for TerrainPatrolAI {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::ai::BasicAI::BasicAI::getSourceId(crate::org::allbinary::game::ai::BasicAI::BasicAIDyn::as_BasicAI_mut(self)) }
}


