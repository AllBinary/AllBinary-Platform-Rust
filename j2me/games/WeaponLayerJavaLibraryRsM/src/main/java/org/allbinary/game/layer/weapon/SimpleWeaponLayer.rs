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
// package org.allbinary.game.layer.weapon
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::FeaturedAnimationInterfaceFactoryInterfaceFactory::FeaturedAnimationInterfaceFactoryInterfaceFactory;
use crate::org::allbinary::animation::NullIndexedAnimationFactory::NullIndexedAnimationFactory;
use crate::org::allbinary::game::combat::damage::ExplosionResources::ExplosionResources;
use crate::org::allbinary::game::multiplayer::layer::RemoteInfo::RemoteInfo;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::physics::movement::Movement::Movement;
use crate::org::allbinary::view::ViewPositionBase::ViewPositionBase;
use super::WeaponLayer::WeaponLayer;
use super::CollidableWeaponBehavior::CollidableWeaponBehavior;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SimpleWeaponLayer {
    pub base: WeaponLayer,
}

impl crate::org::allbinary::layer::NamedInterface::NamedInterface for SimpleWeaponLayer {
    fn getName(&mut self) -> String { self.base.getName() }
}
impl crate::org::allbinary::game::combat::destroy::DestroyableInterface::DestroyableInterface for SimpleWeaponLayer {
    fn isDestroyed(&mut self) -> bool { self.base.isDestroyed() }
}

impl SimpleWeaponLayer {
    

    // throws Exception
    pub fn createDestroyed() -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return jcall!(jcall!(FeaturedAnimationInterfaceFactoryInterfaceFactory::getInstance()).getProcedural(jcall!(ExplosionResources::getInstance()).THIRD_EXPLOSION_RESOURCE.clone())).getInstanceAnimation(jcall!(NullIndexedAnimationFactory::getFactoryInstance()).getInstance(0));
    }
    

    pub fn new_struct(mut name: String, mut remoteInfo: Option<Rc<RefCell<RemoteInfo>>>, mut multiPlayerType: i32, mut movement: Option<Rc<RefCell<dyn crate::org::allbinary::physics::movement::Movement::MovementDyn>>>, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut destroyedAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Self {
        let mut this = Self {
            base: WeaponLayer::new_struct(name.clone(), remoteInfo.clone(), multiPlayerType.clone(), movement.clone(), animationInterface.clone(), destroyedAnimationInterface.clone(), rectangle.clone(), viewPosition.clone()),
        };
        // super(...) is in the struct literal above
        this.setCollidableInferface(CollidableWeaponBehavior::new(true));
        this
    }

    pub fn new(mut name: String, mut remoteInfo: Option<Rc<RefCell<RemoteInfo>>>, mut multiPlayerType: i32, mut movement: Option<Rc<RefCell<dyn crate::org::allbinary::physics::movement::Movement::MovementDyn>>>, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut destroyedAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, remoteInfo, multiPlayerType, movement, animationInterface, destroyedAnimationInterface, rectangle, viewPosition))))
    }
}
impl std::ops::Deref for SimpleWeaponLayer {
    type Target = WeaponLayer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SimpleWeaponLayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait SimpleWeaponLayerDyn: crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn {
    fn as_SimpleWeaponLayer(&self) -> &SimpleWeaponLayer;
    fn as_SimpleWeaponLayer_mut(&mut self) -> &mut SimpleWeaponLayer;
}
impl SimpleWeaponLayerDyn for SimpleWeaponLayer {
    fn as_SimpleWeaponLayer(&self) -> &SimpleWeaponLayer { self }
    fn as_SimpleWeaponLayer_mut(&mut self) -> &mut SimpleWeaponLayer { self }
}

impl crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn for SimpleWeaponLayer {
    fn as_WeaponLayer(&self) -> &crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer { &self.base }
    fn as_WeaponLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer { &mut self.base }
}
impl crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn for SimpleWeaponLayer {
    fn as_CollidableCompositeLayer(&self) -> &crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer { &self.base.base.base.base }
    fn as_CollidableCompositeLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer { &mut self.base.base.base.base }
    fn setCollidableInferface(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>>) { crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer::setCollidableInferface(crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn::as_WeaponLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn for SimpleWeaponLayer {
    fn as_CollidableDestroyableDamageableLayer(&self) -> &crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer { &self.base.base.base }
    fn as_CollidableDestroyableDamageableLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer { &mut self.base.base.base }
    fn damage(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer::damage(crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn::as_WeaponLayer_mut(self), p1, p2) }
    fn getDamage(&mut self, p1: i32) -> i32 { crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer::getDamage(crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn::as_WeaponLayer_mut(self), p1) }
    fn isDestroyed(&mut self) -> bool { crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer::isDestroyed(crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn::as_WeaponLayer_mut(self)) }
}
impl crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn for SimpleWeaponLayer {
    fn as_AllBinaryLayer(&self) -> &crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &self.base.base.base.base.base.base }
    fn as_AllBinaryLayer_mut(&mut self) -> &mut crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &mut self.base.base.base.base.base.base }
    fn onChangeEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::view::event::ViewPositionEvent::ViewPositionEvent>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::onChangeEvent(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
    fn implmentsTickableInterface(&mut self) -> bool { crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer::implmentsTickableInterface(crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn::as_WeaponLayer_mut(self)) }
    fn implmentsCollidableInterface(&mut self) -> bool { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::implmentsCollidableInterface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
    fn getType(&mut self) -> i32 { crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer::getType(crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn::as_WeaponLayer_mut(self)) }
    fn getMultiPlayerType(&mut self) -> i32 { crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer::getMultiPlayerType(crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn::as_WeaponLayer_mut(self)) }
    fn toString(&mut self) -> String { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::toString(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
}
impl crate::org::allbinary::layer::Layer::LayerDyn for SimpleWeaponLayer {
    fn as_Layer(&self) -> &crate::org::allbinary::layer::Layer::Layer { &self.base.base.base.base.base.base.base }
    fn as_Layer_mut(&mut self) -> &mut crate::org::allbinary::layer::Layer::Layer { &mut self.base.base.base.base.base.base.base }
    fn toStringAppend(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::logic::string::StringMaker::StringMaker>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::toStringAppend(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayerDyn for SimpleWeaponLayer {
    fn as_MultiPlayerGameLayer(&self) -> &crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer { &self.base.base }
    fn as_MultiPlayerGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer { &mut self.base.base }
}
impl crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn for SimpleWeaponLayer {
    fn as_AllBinaryGameLayer(&self) -> &crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &self.base.base.base.base.base }
    fn as_AllBinaryGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &mut self.base.base.base.base.base }
    fn processTick(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer::processTick(crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn::as_WeaponLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface for SimpleWeaponLayer {
    fn getCollidableInferface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>> { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::getCollidableInferface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
}
impl crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayerInterface::MultiPlayerGameLayerInterface for SimpleWeaponLayer {
    fn getMultiplayerBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::multiplayer::layer::MultiplayerBehavior::MultiplayerBehaviorDyn>>> { crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer::getMultiplayerBehavior(crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayerDyn::as_MultiPlayerGameLayer_mut(self)) }
}
impl crate::org::allbinary::game::layer::pickup::PickupCompositeInterface::PickupCompositeInterface for SimpleWeaponLayer {
    fn getPickupBehavior(&mut self) -> Option<Rc<RefCell<crate::org::allbinary::game::layer::pickup::PickupBehavior::PickupBehavior>>> { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::getPickupBehavior(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
    fn setPickupBehavior(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::game::layer::pickup::PickupBehavior::PickupBehavior>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::setPickupBehavior(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for SimpleWeaponLayer {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer::set(crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn::as_WeaponLayer_mut(self), p1) }
}


