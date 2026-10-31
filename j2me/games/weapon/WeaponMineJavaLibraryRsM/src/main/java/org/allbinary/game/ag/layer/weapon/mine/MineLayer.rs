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
// package org.allbinary.game.ag.layer.weapon.mine
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::ag::weapon::media::audio::DropWeaponSound::DropWeaponSound;
use crate::org::allbinary::animation::FeaturedAnimationInterfaceFactoryInterfaceFactory::FeaturedAnimationInterfaceFactoryInterfaceFactory;
use crate::org::allbinary::game::layer::weapon::SimpleWeaponLayer::SimpleWeaponLayer;
use crate::org::allbinary::game::multiplayer::layer::RemoteInfo::RemoteInfo;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::media::audio::SecondaryPlayerQueueFactory::SecondaryPlayerQueueFactory;
use crate::org::allbinary::physics::movement::NoMovementFactory::NoMovementFactory;
use crate::org::allbinary::view::ViewPositionBase::ViewPositionBase;
use crate::org::allbinary::view::event::ViewPositionEventHandler::ViewPositionEventHandler;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MineLayer {
    pub base: SimpleWeaponLayer,
    pub viewPositionEventHandler: Option<Rc<RefCell<ViewPositionEventHandler>>>,
}

thread_local! { pub static MineLayer_NAME: String = String::from("MineLayer"); }
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for MineLayer {
    fn getName(&mut self) -> String { self.base.getName() }
}
impl crate::org::allbinary::game::combat::damage::DamageableInterface::DamageableInterface for MineLayer {
    fn damage(&mut self, mut damage: i32, mut damageType: i32) { Self::damage(self, damage, damageType) }
    fn getDamage(&mut self, mut damageType: i32) -> i32 { Self::getDamage(self, damageType) }
}
impl crate::org::allbinary::game::combat::destroy::DestroyableInterface::DestroyableInterface for MineLayer {
    fn isDestroyed(&mut self) -> bool { self.base.isDestroyed() }
}

impl MineLayer {
    

    pub fn new_struct(mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Self {
        let mut this = Self {
            base: SimpleWeaponLayer::new_struct(MineLayer_NAME.with(Clone::clone).clone(), crate::org::allbinary::game::multiplayer::layer::RemoteInfo::RemoteInfo_REMOTE_INFO.with(Clone::clone).clone(), -1, jcall!(NoMovementFactory::getInstance()).getMovmentInstance(), jcall!(jcall!(FeaturedAnimationInterfaceFactoryInterfaceFactory::getInstance()).get(jcall!(MineWeaponResources::MineWeaponResources::getInstance()).RESOURCE.clone())).getInstance(0), SimpleWeaponLayer::createDestroyed(), Rectangle::new(jcall!(PointFactory::getInstance()).ZERO_ZERO.clone(), 10, 10), viewPosition.clone()),
            viewPositionEventHandler: None,
        };
        this.viewPositionEventHandler = ViewPositionEventHandler::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(viewPosition))))
    }
    

    pub fn initXYZ(&mut self, mut x: i32, mut y: i32, mut z: i32) {
        self.setPosition(x + 5, y + 5, z.clone());
        jcall!(self.viewPositionEventHandler).addListener((self).clone() /* as AllBinaryLayer */);
        jcall!(SecondaryPlayerQueueFactory::getInstance()).add(DropWeaponSound::getInstance());
    }
    

    pub fn damage(&mut self, mut damage: i32, mut damageType: i32) {
        self.totalDamage = self.getInitDamage() + 1;
    }
    

    pub fn getDamage(&mut self, mut damageType: i32) -> i32 {
        self.base.getDamage(damageType.clone());
        return self.getInitDamage();
    }
    

    pub fn setDestroyed(&mut self, mut destroyed: bool) {
        self.base.setDestroyed(destroyed.clone());
        if self.isDestroyed() {
            jcall!(self.viewPositionEventHandler).removeListener(None /* Java this: a &mut self method has no Rc of itself */);
        }
    }
}
impl std::ops::Deref for MineLayer {
    type Target = SimpleWeaponLayer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MineLayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::weapon::SimpleWeaponLayer::SimpleWeaponLayerDyn for MineLayer {
    fn as_SimpleWeaponLayer(&self) -> &crate::org::allbinary::game::layer::weapon::SimpleWeaponLayer::SimpleWeaponLayer { &self.base }
    fn as_SimpleWeaponLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::weapon::SimpleWeaponLayer::SimpleWeaponLayer { &mut self.base }
}
impl crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn for MineLayer {
    fn as_WeaponLayer(&self) -> &crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer { &self.base.base }
    fn as_WeaponLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer { &mut self.base.base }
    fn initXYZ(&mut self, mut x: i32, mut y: i32, mut z: i32) { Self::initXYZ(self, x, y, z) }
    fn setDestroyed(&mut self, mut destroyed: bool) { Self::setDestroyed(self, destroyed) }
}
impl crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn for MineLayer {
    fn as_AllBinaryLayer(&self) -> &crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &self.base.base.base.base.base.base.base }
    fn as_AllBinaryLayer_mut(&mut self) -> &mut crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &mut self.base.base.base.base.base.base.base }
    fn onChangeEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::view::event::ViewPositionEvent::ViewPositionEvent>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::onChangeEvent(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
    fn implmentsTickableInterface(&mut self) -> bool { crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer::implmentsTickableInterface(crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn::as_WeaponLayer_mut(self)) }
    fn implmentsCollidableInterface(&mut self) -> bool { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::implmentsCollidableInterface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
    fn getType(&mut self) -> i32 { crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer::getType(crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn::as_WeaponLayer_mut(self)) }
    fn getMultiPlayerType(&mut self) -> i32 { crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer::getMultiPlayerType(crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn::as_WeaponLayer_mut(self)) }
    fn toString(&mut self) -> String { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::toString(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
}
impl crate::org::allbinary::layer::Layer::LayerDyn for MineLayer {
    fn as_Layer(&self) -> &crate::org::allbinary::layer::Layer::Layer { &self.base.base.base.base.base.base.base.base }
    fn as_Layer_mut(&mut self) -> &mut crate::org::allbinary::layer::Layer::Layer { &mut self.base.base.base.base.base.base.base.base }
    fn toStringAppend(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::logic::string::StringMaker::StringMaker>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::toStringAppend(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayerDyn for MineLayer {
    fn as_MultiPlayerGameLayer(&self) -> &crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer { &self.base.base.base }
    fn as_MultiPlayerGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer { &mut self.base.base.base }
}
impl crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn for MineLayer {
    fn as_CollidableCompositeLayer(&self) -> &crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer { &self.base.base.base.base.base }
    fn as_CollidableCompositeLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer { &mut self.base.base.base.base.base }
    fn setCollidableInferface(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>>) { crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer::setCollidableInferface(crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn::as_WeaponLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn for MineLayer {
    fn as_CollidableDestroyableDamageableLayer(&self) -> &crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer { &self.base.base.base.base }
    fn as_CollidableDestroyableDamageableLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer { &mut self.base.base.base.base }
    fn damage(&mut self, mut damage: i32, mut damageType: i32) { Self::damage(self, damage, damageType) }
    fn getDamage(&mut self, mut damageType: i32) -> i32 { Self::getDamage(self, damageType) }
    fn isDestroyed(&mut self) -> bool { crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer::isDestroyed(crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn::as_WeaponLayer_mut(self)) }
}
impl crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn for MineLayer {
    fn as_AllBinaryGameLayer(&self) -> &crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &self.base.base.base.base.base.base }
    fn as_AllBinaryGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &mut self.base.base.base.base.base.base }
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer::set(crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn::as_WeaponLayer_mut(self), p1) }
    fn processTick(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer::processTick(crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn::as_WeaponLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayerInterface::MultiPlayerGameLayerInterface for MineLayer {
    fn getMultiplayerBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::multiplayer::layer::MultiplayerBehavior::MultiplayerBehaviorDyn>>> { crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer::getMultiplayerBehavior(crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayerDyn::as_MultiPlayerGameLayer_mut(self)) }
}
impl crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface for MineLayer {
    fn getCollidableInferface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>> { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::getCollidableInferface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
}


