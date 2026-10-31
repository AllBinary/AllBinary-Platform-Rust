// 
// AllBinary Open License Version 1 
// Copyright (c) 2006 AllBinary 
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
// package org.allbinary.game.layer.waypoint
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::animation::AnimationInterfaceFactoryInterface::AnimationInterfaceFactoryInterface;
use crate::org::allbinary::animation::ProceduralAnimationInterfaceFactoryInterface::ProceduralAnimationInterfaceFactoryInterface;
use crate::org::allbinary::game::health::Health::Health;
use crate::org::allbinary::game::identification::Group::Group;
use crate::org::allbinary::game::input::form::NullRTSFormInputFactory::NullRTSFormInputFactory;
use crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer;
use crate::org::allbinary::game::layer::AdvancedRTSProperties::AdvancedRTSProperties;
use crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface;
use crate::org::allbinary::game::view::TileLayerPositionIntoViewPosition::TileLayerPositionIntoViewPosition;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::game::layer::RTSPlayerLayerInterface::RTSPlayerLayerInterface;
use crate::org::allbinary::game::multiplayer::layer::RemoteInfo::RemoteInfo;
use crate::org::allbinary::media::audio::AttackSound::AttackSound;
use super::CollidableWaypointBehavior::CollidableWaypointBehavior;
use super::Waypoint::Waypoint;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WaypointLayer {
    pub base: AdvancedRTSGameLayer,
}

impl crate::org::allbinary::layer::NamedInterface::NamedInterface for WaypointLayer {
    fn getName(&mut self) -> String { self.base.getName() }
}
impl crate::org::allbinary::game::combat::destroy::DestroyableInterface::DestroyableInterface for WaypointLayer {
    fn isDestroyed(&mut self) -> bool { self.base.isDestroyed() }
}
impl crate::org::allbinary::game::tick::TickableInterface::TickableInterface for WaypointLayer {
    fn processTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processTick(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::game::combat::damage::DamageableInterface::DamageableInterface for WaypointLayer {
    fn damage(&mut self, mut damage: i32, mut damageType: i32) { Self::damage(self, damage, damageType) }
    fn getDamage(&mut self, mut damageType: i32) -> i32 { Self::getDamage(self, damageType) }
}

impl WaypointLayer {
    

    pub fn new_struct(mut remoteInfo: Option<Rc<RefCell<RemoteInfo>>>, mut parentLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>, mut rootName: String, mut name: String, mut animationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut proceduralAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn ProceduralAnimationInterfaceFactoryInterface>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut x: i32, mut y: i32) -> Self {
        let mut this = Self {
            base: AdvancedRTSGameLayer::new_struct(remoteInfo.clone(), parentLayer.clone(), AdvancedRTSProperties::createProperties(), groupInterface.clone(), rootName.clone(), name.clone(), Health::new(1000), NullRTSFormInputFactory::getInstance(), animationInterfaceFactoryInterface.clone(), animationInterfaceFactoryInterface.clone(), animationInterfaceFactoryInterface.clone(), animationInterfaceFactoryInterface.clone(), animationInterfaceFactoryInterface.clone(), proceduralAnimationInterfaceFactoryInterface.clone(), rectangle.clone(), x.clone(), y.clone(), TileLayerPositionIntoViewPosition::new()),
        };
        // super(...) is in the struct literal above
        this.setCollidableInferface(CollidableWaypointBehavior::new(true));
        this.setAnimationInterface(this.indexedButShouldBeRotationAnimationInterface.clone());
        this
    }

    pub fn new(mut remoteInfo: Option<Rc<RefCell<RemoteInfo>>>, mut parentLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>, mut rootName: String, mut name: String, mut animationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut proceduralAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn ProceduralAnimationInterfaceFactoryInterface>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut x: i32, mut y: i32) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(remoteInfo, parentLayer, groupInterface, rootName, name, animationInterfaceFactoryInterface, proceduralAnimationInterfaceFactoryInterface, rectangle, x, y))));
            jcall!(this.getWaypointBehavior()).setWaypoint(Waypoint::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), AttackSound::getInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::Waypoint::WaypointDyn>>));
        this
    }
    

    pub fn construct(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>) {
        self.percentCompleteP = 100;
        self.initVisibility(rtsPlayerLayerInterface.clone());
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        if self.isVisible() {
            self.base.paint(graphics.clone());
        }
    }
    

    pub fn processTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.indexedButShouldBeRotationAnimationInterface).nextFrame();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("processTick"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn damage(&mut self, mut damage: i32, mut damageType: i32) {
    }
    

    // throws Exception
    pub fn getDamage(&mut self, mut damageType: i32) -> i32 {
        return 0;
    }
    

    // throws Exception
    pub fn setDestroyed(&mut self, mut destroyed: bool) {
        self.base.setDestroyed(destroyed.clone());
        if self.isDestroyed() {
            jcall!(WaypointCellPositionHistory::WaypointCellPositionHistory::getInstance()).removeLayer(None /* Java this: a &mut self method has no Rc of itself */);
        }
    }
    

    pub fn getType(&mut self) -> i32 {
        return WaypointLayer::getStaticType();
    }
    

    pub fn getStaticType() -> i32 {
        return 4;
    }
    

    pub fn isSelfUpgradeable(&mut self) -> bool {
        return false;
    }
}
impl std::ops::Deref for WaypointLayer {
    type Target = AdvancedRTSGameLayer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for WaypointLayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait WaypointLayerDyn: crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn {
    fn as_WaypointLayer(&self) -> &WaypointLayer;
    fn as_WaypointLayer_mut(&mut self) -> &mut WaypointLayer;
}
impl WaypointLayerDyn for WaypointLayer {
    fn as_WaypointLayer(&self) -> &WaypointLayer { self }
    fn as_WaypointLayer_mut(&mut self) -> &mut WaypointLayer { self }
}

impl crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn for WaypointLayer {
    fn as_CollidableCompositeLayer(&self) -> &crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer { &self.base.base.base.base.base }
    fn as_CollidableCompositeLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer { &mut self.base.base.base.base.base }
}
impl crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn for WaypointLayer {
    fn as_RTSLayer(&self) -> &crate::org::allbinary::game::layer::RTSLayer::RTSLayer { &self.base.base }
    fn as_RTSLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::RTSLayer::RTSLayer { &mut self.base.base }
    fn updateWaypointBehavior(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::updateWaypointBehavior(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self), p1) }
    fn construct(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>) { Self::construct(self, rtsPlayerLayerInterface) }
    fn setSelected(&mut self, p1: bool) { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::setSelected(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self), p1) }
    fn setDestroyed(&mut self, mut destroyed: bool) { Self::setDestroyed(self, destroyed) }
    fn getWaypointBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>> { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::getWaypointBehavior(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn getParentLayer(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface>>> { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::getParentLayer(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn getWaypointLogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointLogHelper::WaypointLogHelperDyn>>> { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::getWaypointLogHelper(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn getWaypoint2LogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::Waypoint2LogHelper::Waypoint2LogHelperDyn>>> { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::getWaypoint2LogHelper(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn getWaypointRunnableLogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointRunnableLogHelper::WaypointRunnableLogHelperDyn>>> { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::getWaypointRunnableLogHelper(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn shouldAddWaypointFromBuilding(&mut self) -> bool { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::shouldAddWaypointFromBuilding(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn getSurroundingGeographicMapCellPositionList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::getSurroundingGeographicMapCellPositionList(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn isWaypointListEmptyOrOnlyTargets(&mut self) -> bool { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::isWaypointListEmptyOrOnlyTargets(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn buildingChase(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, p2: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> bool { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::buildingChase(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self), p1, p2) }
}
impl crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn for WaypointLayer {
    fn as_AllBinaryGameLayer(&self) -> &crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &self.base.base.base.base.base.base }
    fn as_AllBinaryGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &mut self.base.base.base.base.base.base }
    fn processInput(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::processInput(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self), p1) }
    fn processTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processTick(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn for WaypointLayer {
    fn as_AdvancedRTSGameLayer(&self) -> &crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer { &self.base }
    fn as_AdvancedRTSGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer { &mut self.base }
}
impl crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn for WaypointLayer {
    fn as_CollidableDestroyableDamageableLayer(&self) -> &crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer { &self.base.base.base.base }
    fn as_CollidableDestroyableDamageableLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer { &mut self.base.base.base.base }
    fn damage(&mut self, mut damage: i32, mut damageType: i32) { Self::damage(self, damage, damageType) }
    fn getDamage(&mut self, mut damageType: i32) -> i32 { Self::getDamage(self, damageType) }
    fn isDestroyed(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::isDestroyed(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn setAllBinaryGameLayerManager(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::setAllBinaryGameLayerManager(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self), p1) }
    fn onDestroyed(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::game::combat::destroy::event::DestroyedEvent::DestroyedEvent>>>) { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::onDestroyed(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayerDyn for WaypointLayer {
    fn as_MultiPlayerGameLayer(&self) -> &crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer { &self.base.base.base }
    fn as_MultiPlayerGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer { &mut self.base.base.base }
}
impl crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn for WaypointLayer {
    fn as_AllBinaryLayer(&self) -> &crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &self.base.base.base.base.base.base.base }
    fn as_AllBinaryLayer_mut(&mut self) -> &mut crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &mut self.base.base.base.base.base.base.base }
    fn onChangeEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::view::event::ViewPositionEvent::ViewPositionEvent>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::onChangeEvent(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
    fn implmentsTickableInterface(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::implmentsTickableInterface(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn implmentsGameInputInterface(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::implmentsGameInputInterface(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn implmentsCollidableInterface(&mut self) -> bool { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::implmentsCollidableInterface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
    fn getType(&mut self) -> i32 { Self::getType(self) }
    fn toString(&mut self) -> String { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::toString(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
}
impl crate::org::allbinary::layer::Layer::LayerDyn for WaypointLayer {
    fn as_Layer(&self) -> &crate::org::allbinary::layer::Layer::Layer { &self.base.base.base.base.base.base.base.base }
    fn as_Layer_mut(&mut self) -> &mut crate::org::allbinary::layer::Layer::Layer { &mut self.base.base.base.base.base.base.base.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn toStringAppend(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::logic::string::StringMaker::StringMaker>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::toStringAppend(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::layer::RTSInterface::RTSInterface for WaypointLayer {
    fn isSelfUpgradeable(&mut self) -> bool { Self::isSelfUpgradeable(self) }
    fn isCompleted(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::isCompleted(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn getPercentComplete(&mut self) -> i32 { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::getPercentComplete(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn getLevel(&mut self) -> i32 { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::getLevel(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn getMaxLevel(&mut self) -> i32 { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::getMaxLevel(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn getCost(&mut self) -> i32 { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::getCost(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn downgrade(&mut self) { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::downgrade(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn getDowngradeCost(&mut self) -> i32 { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::getDowngradeCost(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn isDowngradeable(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::isDowngradeable(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn getUpgradeCost(&mut self) -> i32 { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::getUpgradeCost(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn isUpgradeable(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::isUpgradeable(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn upgrade(&mut self) { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::upgrade(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
}
impl crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface for WaypointLayer {
    fn getCollidableInferface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>> { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::getCollidableInferface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
}
impl crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayerInterface::MultiPlayerGameLayerInterface for WaypointLayer {
    fn getMultiplayerBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::multiplayer::layer::MultiplayerBehavior::MultiplayerBehaviorDyn>>> { crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer::getMultiplayerBehavior(crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayerDyn::as_MultiPlayerGameLayer_mut(self)) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for WaypointLayer {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::getSourceId(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for WaypointLayer {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::set(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::layer::pickup::PickupCompositeInterface::PickupCompositeInterface for WaypointLayer {
    fn getPickupBehavior(&mut self) -> Option<Rc<RefCell<crate::org::allbinary::game::layer::pickup::PickupBehavior::PickupBehavior>>> { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::getPickupBehavior(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
    fn setPickupBehavior(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::game::layer::pickup::PickupBehavior::PickupBehavior>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::setPickupBehavior(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1) }
}


