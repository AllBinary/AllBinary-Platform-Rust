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
// package org.allbinary.game.ag.layer.pickup
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::animation::NullAnimationFactory::NullAnimationFactory;
use crate::org::allbinary::game::layer::pickup::CountedPickedUpLayerInterfaceFactory::CountedPickedUpLayerInterfaceFactory;
use crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayer;
use crate::org::allbinary::game::multiplayer::layer::RemoteInfo::RemoteInfo;
use crate::org::allbinary::game::tick::TickableInterface::TickableInterface;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::view::ViewPosition::ViewPosition;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FallingPickupLayer {
    pub base: PickupLayer,
}

thread_local! { pub static FallingPickupLayer_NAME: String = String::from("FallingPickupLayer"); }
// impl TickableInterface for FallingPickupLayer with the methods of the trait
impl crate::org::allbinary::game::combat::destroy::DestroyableInterface::DestroyableInterface for FallingPickupLayer {
    fn isDestroyed(&mut self) -> bool { self.base.isDestroyed() }
}

impl FallingPickupLayer {
    

    pub fn new_struct(mut remoteInfo: Option<Rc<RefCell<RemoteInfo>>>) -> Self {
        let mut this = Self {
            base: PickupLayer::new_struct(FallingPickupLayer_NAME.with(Clone::clone).clone(), remoteInfo.clone(), 0, crate::org::allbinary::game::layer::pickup::CountedPickedUpLayerInterfaceFactory::CountedPickedUpLayerInterfaceFactory_NULL_COUNTED_PICKUP_LAYER_FACTORY.with(Clone::clone).clone(), jcall!(NullAnimationFactory::getFactoryInstance()).getInstance(0), Rectangle::new(jcall!(PointFactory::getInstance()).ZERO_ZERO.clone(), 0, 0), ViewPosition::getInstanceD()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut remoteInfo: Option<Rc<RefCell<RemoteInfo>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(remoteInfo))))
    }
    

    pub fn processTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        self.setPosition(self.x.clone(), self.y + 1, self.z.clone());
    }
    

    pub fn implmentsTickableInterface(&mut self) -> bool {
        return true;
    }
}
impl std::ops::Deref for FallingPickupLayer {
    type Target = PickupLayer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for FallingPickupLayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayerDyn for FallingPickupLayer {
    fn as_PickupLayer(&self) -> &crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayer { &self.base }
    fn as_PickupLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayer { &mut self.base }
}
impl crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayerDyn for FallingPickupLayer {
    fn as_MultiPlayerGameLayer(&self) -> &crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer { &self.base.base }
    fn as_MultiPlayerGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer { &mut self.base.base }
}
impl crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn for FallingPickupLayer {
    fn as_CollidableDestroyableDamageableLayer(&self) -> &crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer { &self.base.base.base }
    fn as_CollidableDestroyableDamageableLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer { &mut self.base.base.base }
    fn damage(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayer::damage(crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayerDyn::as_PickupLayer_mut(self), p1, p2) }
    fn getDamage(&mut self, p1: i32) -> i32 { crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayer::getDamage(crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayerDyn::as_PickupLayer_mut(self), p1) }
    fn isDestroyed(&mut self) -> bool { crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayer::isDestroyed(crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayerDyn::as_PickupLayer_mut(self)) }
}
impl crate::org::allbinary::game::layer::pickup::PickupableInterface::PickupableInterface for FallingPickupLayer {
}
impl crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayerInterface::MultiPlayerGameLayerInterface for FallingPickupLayer {
    fn getMultiplayerBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::multiplayer::layer::MultiplayerBehavior::MultiplayerBehaviorDyn>>> { crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer::getMultiplayerBehavior(crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayerDyn::as_MultiPlayerGameLayer_mut(self)) }
}


