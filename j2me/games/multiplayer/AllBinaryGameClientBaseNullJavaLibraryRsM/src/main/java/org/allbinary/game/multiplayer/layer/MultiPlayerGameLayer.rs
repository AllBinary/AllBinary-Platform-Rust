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
// package org.allbinary.game.multiplayer.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::identification::Group::Group;
use crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::view::ViewPositionBase::ViewPositionBase;
use super::MultiPlayerGameLayerInterface::MultiPlayerGameLayerInterface;
use super::RemoteInfo::RemoteInfo;
use super::MultiplayerBehavior::MultiplayerBehavior;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MultiPlayerGameLayer {
    pub base: CollidableDestroyableDamageableLayer,
}

impl MultiPlayerGameLayerInterface for MultiPlayerGameLayer {
    fn getMultiplayerBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::multiplayer::layer::MultiplayerBehavior::MultiplayerBehaviorDyn>>> { Self::getMultiplayerBehavior(self) }
}
impl crate::org::allbinary::game::combat::destroy::DestroyableInterface::DestroyableInterface for MultiPlayerGameLayer {
    fn isDestroyed(&mut self) -> bool { self.base.isDestroyed() }
}
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for MultiPlayerGameLayer {
    fn getName(&mut self) -> String { self.base.getName() }
}

impl MultiPlayerGameLayer {
    

    pub fn new_struct(mut remoteInfo: Option<Rc<RefCell<RemoteInfo>>>, mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>, mut name: String, mut layerInfo: Option<Rc<RefCell<Rectangle>>>, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Self {
        let mut this = Self {
            base: CollidableDestroyableDamageableLayer::new_struct(groupInterface.clone(), name.clone(), layerInfo.clone(), viewPosition.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut remoteInfo: Option<Rc<RefCell<RemoteInfo>>>, mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>, mut name: String, mut layerInfo: Option<Rc<RefCell<Rectangle>>>, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(remoteInfo, groupInterface, name, layerInfo, viewPosition))))
    }
    

    pub fn getMultiplayerBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::multiplayer::layer::MultiplayerBehavior::MultiplayerBehaviorDyn>>> {
        return MultiplayerBehavior_NULL_MULTIPLAYER_BEHAVIOR.with(Clone::clone).clone();
    }
    

    pub fn setPingInfo(&mut self, mut show: bool) {
    }
    

    pub fn setFiring(&mut self, mut firing: i16) {
    }
    

    pub fn getFiring(&mut self) -> i16 {
        return 0;
    }
}
impl std::ops::Deref for MultiPlayerGameLayer {
    type Target = CollidableDestroyableDamageableLayer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MultiPlayerGameLayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait MultiPlayerGameLayerDyn: crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn + crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayerInterface::MultiPlayerGameLayerInterface {
    fn as_MultiPlayerGameLayer(&self) -> &MultiPlayerGameLayer;
    fn as_MultiPlayerGameLayer_mut(&mut self) -> &mut MultiPlayerGameLayer;
    fn setPingInfo(&mut self, mut show: bool) { MultiPlayerGameLayer::setPingInfo(self.as_MultiPlayerGameLayer_mut(), show) }
    fn setFiring(&mut self, mut firing: i16) { MultiPlayerGameLayer::setFiring(self.as_MultiPlayerGameLayer_mut(), firing) }
    fn getFiring(&mut self) -> i16 { MultiPlayerGameLayer::getFiring(self.as_MultiPlayerGameLayer_mut()) }
}
impl MultiPlayerGameLayerDyn for MultiPlayerGameLayer {
    fn as_MultiPlayerGameLayer(&self) -> &MultiPlayerGameLayer { self }
    fn as_MultiPlayerGameLayer_mut(&mut self) -> &mut MultiPlayerGameLayer { self }
}

impl crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn for MultiPlayerGameLayer {
    fn as_CollidableDestroyableDamageableLayer(&self) -> &crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer { &self.base }
    fn as_CollidableDestroyableDamageableLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer { &mut self.base }
}
impl crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn for MultiPlayerGameLayer {
    fn as_CollidableCompositeLayer(&self) -> &crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer { &self.base.base }
    fn as_CollidableCompositeLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer { &mut self.base.base }
}
impl crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn for MultiPlayerGameLayer {
    fn as_AllBinaryGameLayer(&self) -> &crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &self.base.base.base }
    fn as_AllBinaryGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &mut self.base.base.base }
}
impl crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn for MultiPlayerGameLayer {
    fn as_AllBinaryLayer(&self) -> &crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &self.base.base.base.base }
    fn as_AllBinaryLayer_mut(&mut self) -> &mut crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &mut self.base.base.base.base }
    fn implmentsCollidableInterface(&mut self) -> bool { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::implmentsCollidableInterface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
    fn toString(&mut self) -> String { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::toString(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
}
impl crate::org::allbinary::layer::Layer::LayerDyn for MultiPlayerGameLayer {
    fn as_Layer(&self) -> &crate::org::allbinary::layer::Layer::Layer { &self.base.base.base.base.base }
    fn as_Layer_mut(&mut self) -> &mut crate::org::allbinary::layer::Layer::Layer { &mut self.base.base.base.base.base }
    fn toStringAppend(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::logic::string::StringMaker::StringMaker>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::toStringAppend(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface for MultiPlayerGameLayer {
    fn getCollidableInferface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>> { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::getCollidableInferface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
}
impl crate::org::allbinary::game::layer::pickup::PickupCompositeInterface::PickupCompositeInterface for MultiPlayerGameLayer {
    fn getPickupBehavior(&mut self) -> Option<Rc<RefCell<crate::org::allbinary::game::layer::pickup::PickupBehavior::PickupBehavior>>> { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::getPickupBehavior(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
    fn setPickupBehavior(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::game::layer::pickup::PickupBehavior::PickupBehavior>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::setPickupBehavior(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1) }
}


