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
use crate::org::allbinary::animation::NullAnimationFactory::NullAnimationFactory;
use crate::org::allbinary::game::identification::BasicGroupFactory::BasicGroupFactory;
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use crate::org::allbinary::game::multiplayer::layer::RemoteInfo::RemoteInfo;
use crate::org::allbinary::game::view::TileLayerPositionIntoViewPosition::TileLayerPositionIntoViewPosition;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::view::ViewPosition::ViewPosition;
use crate::org::allbinary::view::event::ViewPositionEventHandler::ViewPositionEventHandler;
use crate::java::lang::Exception::*;
use super::PickupLayer::PickupLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RaceTrackPickupLayer {
    pub base: PickupLayer,
    pub viewPositionEventHandler: Option<Rc<RefCell<ViewPositionEventHandler>>>,
}

thread_local! { pub static RaceTrackPickupLayer_NAME: String = String::from("RaceTrackPickupLayer"); }
impl crate::org::allbinary::game::layer::pickup::PickedUpLayerInterface::PickedUpLayerInterface for RaceTrackPickupLayer {
    fn getPickedUpLayerInterfaceFactoryInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactoryInterface::PickedUpLayerInterfaceFactoryInterface>>> { crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayer::getPickedUpLayerInterfaceFactoryInterface(crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayerDyn::as_PickupLayer_mut(self)) }
    fn setPickedUp(&mut self) { crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayer::setPickedUp(crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayerDyn::as_PickupLayer_mut(self)) }
}
impl crate::org::allbinary::game::identification::GroupInterfaceCompositeInterface::GroupInterfaceCompositeInterface for RaceTrackPickupLayer {
    fn getGroupInterface(&mut self) -> Vec<Option<Rc<RefCell<crate::org::allbinary::game::identification::Group::Group>>>> { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::getGroupInterface(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
}
impl crate::org::allbinary::layer::LayerInterface::LayerInterface for RaceTrackPickupLayer {
    fn getHeight(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getHeight(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getWidth(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getHalfHeight(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getHalfHeight(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getHalfWidth(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getHalfWidth(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getXP(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getXP(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getYP(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getYP(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getZP(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getZP(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getX2(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getX2(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getY2(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getY2(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getZ2(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getZ2(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn isVisible(&mut self) -> bool { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::isVisible(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn moveDXYZ(&mut self, p1: i32, p2: i32, p3: i32) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::moveDXYZ(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1, p2, p3) }
    fn setVisible(&mut self, p1: bool) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::setVisible(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1) }
    fn implmentsTickableInterface(&mut self) -> bool { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::implmentsTickableInterface(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn implmentsCollidableInterface(&mut self) -> bool { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::implmentsCollidableInterface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
    fn implmentsGameInputInterface(&mut self) -> bool { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::implmentsGameInputInterface(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn implmentsArtificialIntelligenceCompositeInterface(&mut self) -> bool { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::implmentsArtificialIntelligenceCompositeInterface(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getType(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getType(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
}
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for RaceTrackPickupLayer {
    fn getName(&mut self) -> String { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getName(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
}
impl crate::org::allbinary::layer::PositionInterface::PositionInterface for RaceTrackPickupLayer {
    fn setPosition(&mut self, p1: i32, p2: i32, p3: i32) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::setPosition(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1, p2, p3) }
}
impl crate::org::allbinary::view::event::ViewPositionEventListenerInterface::ViewPositionEventListenerInterface for RaceTrackPickupLayer {
    fn onChangeEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::view::event::ViewPositionEvent::ViewPositionEvent>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::onChangeEvent(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::layer::special::SpecialGameInputInterface::SpecialGameInputInterface for RaceTrackPickupLayer {
    fn up(&mut self) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::up(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
    fn down(&mut self) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::down(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
    fn right(&mut self) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::right(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
    fn left(&mut self) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::left(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
    fn strafeLeft(&mut self) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::strafeLeft(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
    fn strafeRight(&mut self) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::strafeRight(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
    fn fire(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, p2: Option<Rc<RefCell<crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::fire(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1, p2) }
    fn special1(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, p2: Option<Rc<RefCell<crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::special1(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1, p2) }
    fn special2(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, p2: Option<Rc<RefCell<crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::special2(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1, p2) }
    fn special3(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, p2: Option<Rc<RefCell<crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::special3(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1, p2) }
    fn special4(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, p2: Option<Rc<RefCell<crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::special4(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1, p2) }
    fn special5(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, p2: Option<Rc<RefCell<crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::special5(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1, p2) }
}
impl crate::org::allbinary::game::combat::destroy::DestroyableInterface::DestroyableInterface for RaceTrackPickupLayer {
    fn isDestroyed(&mut self) -> bool { crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayer::isDestroyed(crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayerDyn::as_PickupLayer_mut(self)) }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for RaceTrackPickupLayer {
    fn paint(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayer::paint(crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayerDyn::as_PickupLayer_mut(self), p1) }
    fn paintThreed(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayer::paintThreed(crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayerDyn::as_PickupLayer_mut(self), p1) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for RaceTrackPickupLayer {
    fn onEvent(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::onEvent(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::combat::damage::DamageableInterface::DamageableInterface for RaceTrackPickupLayer {
    fn damage(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayer::damage(crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayerDyn::as_PickupLayer_mut(self), p1, p2) }
    fn getDamage(&mut self, p1: i32) -> i32 { crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayer::getDamage(crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayerDyn::as_PickupLayer_mut(self), p1) }
}

impl RaceTrackPickupLayer {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: PickupLayer::new_struct(RaceTrackPickupLayer_NAME.with(Clone::clone).clone(), crate::org::allbinary::game::multiplayer::layer::RemoteInfo::RemoteInfo_REMOTE_INFO.with(Clone::clone).clone(), 0, CountedPickedUpLayerInterfaceFactory_NULL_COUNTED_PICKUP_LAYER_FACTORY.with(Clone::clone).clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactoryInterface::PickedUpLayerInterfaceFactoryInterface>>), jcall!(NullAnimationFactory::getFactoryInstance()).getInstance(0), Rectangle::new({ let java_field = jcall!(PointFactory::getInstance()).ZERO_ZERO.clone(); java_field }.clone(), 0, 0), TileLayerPositionIntoViewPosition::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>)),
            viewPositionEventHandler: None,
        };
        this.viewPositionEventHandler = ViewPositionEventHandler::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setTiledLayer(&mut self, mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>>) {
        let mut viewPosition: Option<Rc<RefCell<TileLayerPositionIntoViewPosition>>> = (self.getViewPosition()).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<TileLayerPositionIntoViewPosition>() }, "ClassCastException: not a TileLayerPositionIntoViewPosition"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<TileLayerPositionIntoViewPosition>) } });
        jcall!(viewPosition).setTiledLayer(tiledLayer.clone());
        jcall!(self.viewPositionEventHandler).addListener(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    pub fn setDestroyed(&mut self, mut destroyed: bool) {
        { let java_arguments = (destroyed.clone(),); self.base.setDestroyed(java_arguments.0) };
        if self.isDestroyed() {
            jcall!(self.viewPositionEventHandler).removeListener(None /* Java this: a &mut self method has no Rc of itself */);
        }
    }
}
impl std::ops::Deref for RaceTrackPickupLayer {
    type Target = PickupLayer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RaceTrackPickupLayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayerDyn for RaceTrackPickupLayer {
    fn as_PickupLayer(&self) -> &crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayer { &self.base }
    fn as_PickupLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayer { &mut self.base }
    fn setDestroyed(&mut self, mut destroyed: bool) { Self::setDestroyed(self, destroyed) }
}
impl crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn for RaceTrackPickupLayer {
    fn as_CollidableCompositeLayer(&self) -> &crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer { &self.base.base.base.base }
    fn as_CollidableCompositeLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer { &mut self.base.base.base.base }
}
impl crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn for RaceTrackPickupLayer {
    fn as_AllBinaryGameLayer(&self) -> &crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &self.base.base.base.base.base }
    fn as_AllBinaryGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &mut self.base.base.base.base.base }
}
impl crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayerDyn for RaceTrackPickupLayer {
    fn as_MultiPlayerGameLayer(&self) -> &crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer { &self.base.base }
    fn as_MultiPlayerGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer { &mut self.base.base }
}
impl crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn for RaceTrackPickupLayer {
    fn as_AllBinaryLayer(&self) -> &crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &self.base.base.base.base.base.base }
    fn as_AllBinaryLayer_mut(&mut self) -> &mut crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &mut self.base.base.base.base.base.base }
    fn onChangeEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::view::event::ViewPositionEvent::ViewPositionEvent>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::onChangeEvent(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
    fn paintThreed(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayer::paintThreed(crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayerDyn::as_PickupLayer_mut(self), p1) }
    fn getGroupInterface(&mut self) -> Vec<Option<Rc<RefCell<crate::org::allbinary::game::identification::Group::Group>>>> { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::getGroupInterface(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
    fn implmentsCollidableInterface(&mut self) -> bool { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::implmentsCollidableInterface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
    fn toString(&mut self) -> String { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::toString(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
}
impl crate::org::allbinary::layer::Layer::LayerDyn for RaceTrackPickupLayer {
    fn as_Layer(&self) -> &crate::org::allbinary::layer::Layer::Layer { &self.base.base.base.base.base.base.base }
    fn as_Layer_mut(&mut self) -> &mut crate::org::allbinary::layer::Layer::Layer { &mut self.base.base.base.base.base.base.base }
    fn setPosition(&mut self, p1: i32, p2: i32, p3: i32) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::setPosition(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1, p2, p3) }
    fn moveDXYZ(&mut self, p1: i32, p2: i32, p3: i32) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::moveDXYZ(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1, p2, p3) }
    fn getXP(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getXP(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getYP(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getYP(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getZP(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getZP(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getWidth(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getHeight(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getHeight(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn setVisible(&mut self, p1: bool) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::setVisible(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1) }
    fn isVisible(&mut self) -> bool { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::isVisible(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn paint(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayer::paint(crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayerDyn::as_PickupLayer_mut(self), p1) }
    fn toStringAppend(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::logic::string::StringMaker::StringMaker>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::toStringAppend(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn for RaceTrackPickupLayer {
    fn as_CollidableDestroyableDamageableLayer(&self) -> &crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer { &self.base.base.base }
    fn as_CollidableDestroyableDamageableLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer { &mut self.base.base.base }
    fn damage(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayer::damage(crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayerDyn::as_PickupLayer_mut(self), p1, p2) }
    fn getDamage(&mut self, p1: i32) -> i32 { crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayer::getDamage(crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayerDyn::as_PickupLayer_mut(self), p1) }
    fn isDestroyed(&mut self) -> bool { crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayer::isDestroyed(crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayerDyn::as_PickupLayer_mut(self)) }
}
impl crate::org::allbinary::game::layer::pickup::PickupCompositeInterface::PickupCompositeInterface for RaceTrackPickupLayer {
    fn getPickupBehavior(&mut self) -> Option<Rc<RefCell<crate::org::allbinary::game::layer::pickup::PickupBehavior::PickupBehavior>>> { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::getPickupBehavior(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
    fn setPickupBehavior(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::game::layer::pickup::PickupBehavior::PickupBehavior>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::setPickupBehavior(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::layer::pickup::PickupableInterface::PickupableInterface for RaceTrackPickupLayer {
}
impl crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayerInterface::MultiPlayerGameLayerInterface for RaceTrackPickupLayer {
    fn getMultiplayerBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::multiplayer::layer::MultiplayerBehavior::MultiplayerBehaviorDyn>>> { crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer::getMultiplayerBehavior(crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayerDyn::as_MultiPlayerGameLayer_mut(self)) }
}
impl crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface for RaceTrackPickupLayer {
    fn getCollidableInferface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>> { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::getCollidableInferface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for RaceTrackPickupLayer {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayer::set(crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayerDyn::as_PickupLayer_mut(self), p1) }
}


