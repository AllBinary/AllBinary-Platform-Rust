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
// package org.allbinary.game.layer.pickup
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::khronos::opengles::GL::GL;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::NullAnimationFactory::NullAnimationFactory;
use crate::org::allbinary::game::collision::CollidableAlwaysPickupNeverCollideBehaviorFactory::CollidableAlwaysPickupNeverCollideBehaviorFactory;
use crate::org::allbinary::game::combat::destroy::DestroyedLayerProcessor::DestroyedLayerProcessor;
use crate::org::allbinary::game::identification::BasicGroupFactory::BasicGroupFactory;
use crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer;
use crate::org::allbinary::game::multiplayer::layer::RemoteInfo::RemoteInfo;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface;
use crate::org::allbinary::view::ViewPositionBase::ViewPositionBase;
use super::PickedUpLayerInterface::PickedUpLayerInterface;
use super::PickupableInterface::PickupableInterface;
use super::PickedUpLayerInterfaceFactoryInterface::PickedUpLayerInterfaceFactoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PickupLayer {
    pub base: MultiPlayerGameLayer,
    pub pickedUpLayerInterfaceFactoryInterface: Option<Rc<RefCell<dyn PickedUpLayerInterfaceFactoryInterface>>>,
    pub destroyed: bool,
    pub animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
}

// impl PickedUpLayerInterface for PickupLayer with the methods of the trait
impl PickupableInterface for PickupLayer {
}
impl crate::org::allbinary::game::combat::destroy::DestroyableInterface::DestroyableInterface for PickupLayer {
    fn isDestroyed(&mut self) -> bool { Self::isDestroyed(self) }
}
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for PickupLayer {
    fn getName(&mut self) -> String { self.base.getName() }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for PickupLayer {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paintThreed(self, graphics) }
}
impl crate::org::allbinary::game::combat::damage::DamageableInterface::DamageableInterface for PickupLayer {
    fn damage(&mut self, mut damage: i32, mut damageType: i32) { Self::damage(self, damage, damageType) }
    fn getDamage(&mut self, mut damageType: i32) -> i32 { Self::getDamage(self, damageType) }
}

impl PickupLayer {
    

    pub fn new_struct(mut name: String, mut remoteInfo: Option<Rc<RefCell<RemoteInfo>>>, mut total: i32, mut pickedUpLayerInterfaceFactoryInterface: Option<Rc<RefCell<dyn PickedUpLayerInterfaceFactoryInterface>>>, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Self {
        let mut this = Self {
            base: MultiPlayerGameLayer::new_struct(remoteInfo.clone(), jcall!(BasicGroupFactory::getInstance()).NONE_ARRAY.clone(), name.clone(), rectangle.clone(), viewPosition.clone()),
            pickedUpLayerInterfaceFactoryInterface: None,
            destroyed: false,
            animationInterface: None,
        };
        this.pickedUpLayerInterfaceFactoryInterface = CountedPickedUpLayerInterfaceFactory_NULL_COUNTED_PICKUP_LAYER_FACTORY.with(Clone::clone).clone();
        this.animationInterface = jcall!(NullAnimationFactory::getFactoryInstance()).getInstance(0);
        // super(...) is in the struct literal above
        { let java_arguments = (jcall!(CollidableAlwaysPickupNeverCollideBehaviorFactory::getInstance()).createBehavior(),); this.setCollidableInferface(java_arguments.0) };
        this.setLayerWidth(10);
        this.setLayerHeight(10);
        this.init(pickedUpLayerInterfaceFactoryInterface.clone(), animationInterface.clone());
        this
    }

    pub fn new(mut name: String, mut remoteInfo: Option<Rc<RefCell<RemoteInfo>>>, mut total: i32, mut pickedUpLayerInterfaceFactoryInterface: Option<Rc<RefCell<dyn PickedUpLayerInterfaceFactoryInterface>>>, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, remoteInfo, total, pickedUpLayerInterfaceFactoryInterface, animationInterface, rectangle, viewPosition))))
    }
    

    pub fn init(&mut self, mut pickedUpLayerInterfaceFactoryInterface: Option<Rc<RefCell<dyn PickedUpLayerInterfaceFactoryInterface>>>, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) {
        self.pickedUpLayerInterfaceFactoryInterface = pickedUpLayerInterfaceFactoryInterface.clone();
        self.animationInterface = animationInterface.clone();
        self.setDestroyed(false);
    }
    

    pub fn initXYZ(&mut self, mut x: i32, mut y: i32, mut z: i32) {
        self.setPosition(x.clone(), y.clone(), z.clone());
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>> = self.getViewPosition();
        let mut viewX: i32 = jcall!(viewPosition).getX();
        let mut viewY: i32 = jcall!(viewPosition).getY();
        jcall!(self.animationInterface).paintXY(graphics.clone(), viewX.clone(), viewY.clone());
    }
    

    pub fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>> = self.getViewPosition();
        let mut viewX: i32 = jcall!(viewPosition).getX();
        let mut viewY: i32 = jcall!(viewPosition).getY();
        jcall!(self.animationInterface).paintThreedXYZ(graphics.clone(), viewX.clone(), viewY.clone(), 3);
    }
    

    pub fn getPickedUpLayerInterfaceFactoryInterface(&mut self) -> Option<Rc<RefCell<dyn PickedUpLayerInterfaceFactoryInterface>>> {
        return self.pickedUpLayerInterfaceFactoryInterface.clone();
    }
    

    pub fn setPickedUp(&mut self) {
        self.setDestroyed(true);
    }
    

    pub fn isDestroyed(&mut self) -> bool {
        return self.destroyed.clone();
    }
    

    pub fn setDestroyed(&mut self, mut destroyed: bool) {
        self.destroyed = destroyed.clone();
        if self.isDestroyed() {
            jcall!(DestroyedLayerProcessor::getInstance()).add(None /* Java this: a &mut self method has no Rc of itself */);
        }
    }
    

    pub fn damage(&mut self, mut damage: i32, mut damageType: i32) {
    }
    

    pub fn getDamage(&mut self, mut damageType: i32) -> i32 {
        return 0;
    }
    

    // throws Exception
    pub fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) {
        let mut openGLSurfaceChangedInterface: Option<Rc<RefCell<dyn OpenGLSurfaceChangedInterface>>> = (self.animationInterface).clone() /* as OpenGLSurfaceChangedInterface */;
        jcall!(openGLSurfaceChangedInterface).set(gl.clone());
    }
}
impl std::ops::Deref for PickupLayer {
    type Target = MultiPlayerGameLayer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PickupLayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait PickupLayerDyn: crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayerDyn + crate::org::allbinary::game::layer::pickup::PickupableInterface::PickupableInterface {
    fn as_PickupLayer(&self) -> &PickupLayer;
    fn as_PickupLayer_mut(&mut self) -> &mut PickupLayer;
    fn init(&mut self, mut pickedUpLayerInterfaceFactoryInterface: Option<Rc<RefCell<dyn PickedUpLayerInterfaceFactoryInterface>>>, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) { PickupLayer::init(self.as_PickupLayer_mut(), pickedUpLayerInterfaceFactoryInterface, animationInterface) }
    fn initXYZ(&mut self, mut x: i32, mut y: i32, mut z: i32) { PickupLayer::initXYZ(self.as_PickupLayer_mut(), x, y, z) }
    fn getPickedUpLayerInterfaceFactoryInterface(&mut self) -> Option<Rc<RefCell<dyn PickedUpLayerInterfaceFactoryInterface>>> { PickupLayer::getPickedUpLayerInterfaceFactoryInterface(self.as_PickupLayer_mut()) }
    fn setPickedUp(&mut self) { PickupLayer::setPickedUp(self.as_PickupLayer_mut()) }
    fn setDestroyed(&mut self, mut destroyed: bool) { PickupLayer::setDestroyed(self.as_PickupLayer_mut(), destroyed) }
}
impl PickupLayerDyn for PickupLayer {
    fn as_PickupLayer(&self) -> &PickupLayer { self }
    fn as_PickupLayer_mut(&mut self) -> &mut PickupLayer { self }
}

impl crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayerDyn for PickupLayer {
    fn as_MultiPlayerGameLayer(&self) -> &crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer { &self.base }
    fn as_MultiPlayerGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer { &mut self.base }
}
impl crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn for PickupLayer {
    fn as_AllBinaryLayer(&self) -> &crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &self.base.base.base.base.base }
    fn as_AllBinaryLayer_mut(&mut self) -> &mut crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &mut self.base.base.base.base.base }
    fn onChangeEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::view::event::ViewPositionEvent::ViewPositionEvent>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::onChangeEvent(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paintThreed(self, graphics) }
    fn implmentsCollidableInterface(&mut self) -> bool { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::implmentsCollidableInterface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
    fn toString(&mut self) -> String { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::toString(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
}
impl crate::org::allbinary::layer::Layer::LayerDyn for PickupLayer {
    fn as_Layer(&self) -> &crate::org::allbinary::layer::Layer::Layer { &self.base.base.base.base.base.base }
    fn as_Layer_mut(&mut self) -> &mut crate::org::allbinary::layer::Layer::Layer { &mut self.base.base.base.base.base.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn toStringAppend(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::logic::string::StringMaker::StringMaker>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::toStringAppend(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn for PickupLayer {
    fn as_CollidableCompositeLayer(&self) -> &crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer { &self.base.base.base }
    fn as_CollidableCompositeLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer { &mut self.base.base.base }
}
impl crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn for PickupLayer {
    fn as_CollidableDestroyableDamageableLayer(&self) -> &crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer { &self.base.base }
    fn as_CollidableDestroyableDamageableLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer { &mut self.base.base }
    fn damage(&mut self, mut damage: i32, mut damageType: i32) { Self::damage(self, damage, damageType) }
    fn getDamage(&mut self, mut damageType: i32) -> i32 { Self::getDamage(self, damageType) }
    fn isDestroyed(&mut self) -> bool { Self::isDestroyed(self) }
}
impl crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn for PickupLayer {
    fn as_AllBinaryGameLayer(&self) -> &crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &self.base.base.base.base }
    fn as_AllBinaryGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &mut self.base.base.base.base }
}
impl crate::org::allbinary::game::layer::pickup::PickupCompositeInterface::PickupCompositeInterface for PickupLayer {
    fn getPickupBehavior(&mut self) -> Option<Rc<RefCell<crate::org::allbinary::game::layer::pickup::PickupBehavior::PickupBehavior>>> { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::getPickupBehavior(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
    fn setPickupBehavior(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::game::layer::pickup::PickupBehavior::PickupBehavior>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::setPickupBehavior(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface for PickupLayer {
    fn getCollidableInferface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>> { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::getCollidableInferface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
}
impl crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayerInterface::MultiPlayerGameLayerInterface for PickupLayer {
    fn getMultiplayerBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::multiplayer::layer::MultiplayerBehavior::MultiplayerBehaviorDyn>>> { crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer::getMultiplayerBehavior(crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayerDyn::as_MultiPlayerGameLayer_mut(self)) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for PickupLayer {
    fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) { Self::set(self, gl) }
}


