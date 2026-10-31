// 
// AllBinary Open License Version 1 
// Copyright (c) 2002 AllBinary 
//  
// By agreeing to this license you and any business entity you represent are 
// legally bound to the AllBinary Open License Version 1 legal agreement. 
//  
// You may obtain the AllBinary Open License Version 1 legal agreement from 
// AllBinary or the root directory of AllBinary's AllBinary Platform repository. 
//  
// Created By: Travis Berthelot  
// Generated Code Do Not Modify
// package org.allbinary.game.layer.waypoint
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintable;
use crate::org::allbinary::animation::AnimationInterfaceFactoryInterface::AnimationInterfaceFactoryInterface;
use crate::org::allbinary::animation::ProceduralAnimationInterfaceFactoryInterface::ProceduralAnimationInterfaceFactoryInterface;
use crate::org::allbinary::game::identification::Group::Group;
use crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::game::multiplayer::layer::RemoteInfo::RemoteInfo;
use super::WaypointLayer::WaypointLayer;
use super::WaypointInfoHudPaintable::WaypointInfoHudPaintable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct VisualWaypointLayer {
    pub base: WaypointLayer,
}

impl crate::org::allbinary::layer::NamedInterface::NamedInterface for VisualWaypointLayer {
    fn getName(&mut self) -> String { self.base.getName() }
}
impl crate::org::allbinary::game::combat::destroy::DestroyableInterface::DestroyableInterface for VisualWaypointLayer {
    fn isDestroyed(&mut self) -> bool { self.base.isDestroyed() }
}

impl VisualWaypointLayer {
    

    pub fn new_struct(mut remoteInfo: Option<Rc<RefCell<RemoteInfo>>>, mut parentLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>, mut rootName: String, mut name: String, mut animationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut proceduralAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn ProceduralAnimationInterfaceFactoryInterface>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut x: i32, mut y: i32) -> Self {
        let mut this = Self {
            base: WaypointLayer::new_struct(remoteInfo.clone(), parentLayer.clone(), groupInterface.clone(), rootName.clone(), name.clone(), animationInterfaceFactoryInterface.clone(), proceduralAnimationInterfaceFactoryInterface.clone(), rectangle.clone(), x.clone(), y.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut remoteInfo: Option<Rc<RefCell<RemoteInfo>>>, mut parentLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>, mut rootName: String, mut name: String, mut animationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut proceduralAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn ProceduralAnimationInterfaceFactoryInterface>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut x: i32, mut y: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(remoteInfo, parentLayer, groupInterface, rootName, name, animationInterfaceFactoryInterface, proceduralAnimationInterfaceFactoryInterface, rectangle, x, y))))
    }
    

    pub fn createHudPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn>>> {
        let mut rtsLayerHudPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointInfoHudPaintable::WaypointInfoHudPaintableDyn>>> = WaypointInfoHudPaintableSingleton::WaypointInfoHudPaintableSingleton::getInstance();
        jcall!(rtsLayerHudPaintable).setBasicColorP(jcall!(self.allBinaryGameLayerManagerP).getForegroundBasicColor());
        jcall!(rtsLayerHudPaintable).setRtsLayer(None /* Java this: a &mut self method has no Rc of itself */);
        return rtsLayerHudPaintable.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn>>);
    }
    

    pub fn getHudPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn>>> {
        let mut rtsLayerHudPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointInfoHudPaintable::WaypointInfoHudPaintableDyn>>> = WaypointInfoHudPaintableSingleton::WaypointInfoHudPaintableSingleton::getInstance();
        return rtsLayerHudPaintable.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn>>);
    }
}
impl std::ops::Deref for VisualWaypointLayer {
    type Target = WaypointLayer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for VisualWaypointLayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::waypoint::WaypointLayer::WaypointLayerDyn for VisualWaypointLayer {
    fn as_WaypointLayer(&self) -> &crate::org::allbinary::game::layer::waypoint::WaypointLayer::WaypointLayer { &self.base }
    fn as_WaypointLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::waypoint::WaypointLayer::WaypointLayer { &mut self.base }
}
impl crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn for VisualWaypointLayer {
    fn as_CollidableCompositeLayer(&self) -> &crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer { &self.base.base.base.base.base.base }
    fn as_CollidableCompositeLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer { &mut self.base.base.base.base.base.base }
}
impl crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn for VisualWaypointLayer {
    fn as_RTSLayer(&self) -> &crate::org::allbinary::game::layer::RTSLayer::RTSLayer { &self.base.base.base }
    fn as_RTSLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::RTSLayer::RTSLayer { &mut self.base.base.base }
    fn updateWaypointBehavior(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::updateWaypointBehavior(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self), p1) }
    fn construct(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSPlayerLayerInterface::RTSPlayerLayerInterface>>>) { crate::org::allbinary::game::layer::waypoint::WaypointLayer::WaypointLayer::construct(crate::org::allbinary::game::layer::waypoint::WaypointLayer::WaypointLayerDyn::as_WaypointLayer_mut(self), p1) }
    fn setSelected(&mut self, p1: bool) { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::setSelected(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self), p1) }
    fn createHudPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn>>> { Self::createHudPaintable(self) }
    fn getHudPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn>>> { Self::getHudPaintable(self) }
    fn setDestroyed(&mut self, p1: bool) { crate::org::allbinary::game::layer::waypoint::WaypointLayer::WaypointLayer::setDestroyed(crate::org::allbinary::game::layer::waypoint::WaypointLayer::WaypointLayerDyn::as_WaypointLayer_mut(self), p1) }
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
impl crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn for VisualWaypointLayer {
    fn as_AllBinaryGameLayer(&self) -> &crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &self.base.base.base.base.base.base.base }
    fn as_AllBinaryGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &mut self.base.base.base.base.base.base.base }
    fn processInput(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::processInput(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self), p1) }
    fn processTick(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { crate::org::allbinary::game::layer::waypoint::WaypointLayer::WaypointLayer::processTick(crate::org::allbinary::game::layer::waypoint::WaypointLayer::WaypointLayerDyn::as_WaypointLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn for VisualWaypointLayer {
    fn as_AdvancedRTSGameLayer(&self) -> &crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer { &self.base.base }
    fn as_AdvancedRTSGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer { &mut self.base.base }
}
impl crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn for VisualWaypointLayer {
    fn as_CollidableDestroyableDamageableLayer(&self) -> &crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer { &self.base.base.base.base.base }
    fn as_CollidableDestroyableDamageableLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer { &mut self.base.base.base.base.base }
    fn damage(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::layer::waypoint::WaypointLayer::WaypointLayer::damage(crate::org::allbinary::game::layer::waypoint::WaypointLayer::WaypointLayerDyn::as_WaypointLayer_mut(self), p1, p2) }
    fn getDamage(&mut self, p1: i32) -> i32 { crate::org::allbinary::game::layer::waypoint::WaypointLayer::WaypointLayer::getDamage(crate::org::allbinary::game::layer::waypoint::WaypointLayer::WaypointLayerDyn::as_WaypointLayer_mut(self), p1) }
    fn isDestroyed(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::isDestroyed(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn setAllBinaryGameLayerManager(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::setAllBinaryGameLayerManager(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self), p1) }
    fn onDestroyed(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::game::combat::destroy::event::DestroyedEvent::DestroyedEvent>>>) { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::onDestroyed(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayerDyn for VisualWaypointLayer {
    fn as_MultiPlayerGameLayer(&self) -> &crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer { &self.base.base.base.base }
    fn as_MultiPlayerGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer { &mut self.base.base.base.base }
}
impl crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn for VisualWaypointLayer {
    fn as_AllBinaryLayer(&self) -> &crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &self.base.base.base.base.base.base.base.base }
    fn as_AllBinaryLayer_mut(&mut self) -> &mut crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &mut self.base.base.base.base.base.base.base.base }
    fn onChangeEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::view::event::ViewPositionEvent::ViewPositionEvent>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::onChangeEvent(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
    fn implmentsTickableInterface(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::implmentsTickableInterface(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn implmentsGameInputInterface(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::implmentsGameInputInterface(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn implmentsCollidableInterface(&mut self) -> bool { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::implmentsCollidableInterface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
    fn getType(&mut self) -> i32 { crate::org::allbinary::game::layer::waypoint::WaypointLayer::WaypointLayer::getType(crate::org::allbinary::game::layer::waypoint::WaypointLayer::WaypointLayerDyn::as_WaypointLayer_mut(self)) }
    fn toString(&mut self) -> String { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::toString(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
}
impl crate::org::allbinary::layer::Layer::LayerDyn for VisualWaypointLayer {
    fn as_Layer(&self) -> &crate::org::allbinary::layer::Layer::Layer { &self.base.base.base.base.base.base.base.base.base }
    fn as_Layer_mut(&mut self) -> &mut crate::org::allbinary::layer::Layer::Layer { &mut self.base.base.base.base.base.base.base.base.base }
    fn toStringAppend(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::logic::string::StringMaker::StringMaker>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::toStringAppend(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::layer::RTSInterface::RTSInterface for VisualWaypointLayer {
    fn isSelfUpgradeable(&mut self) -> bool { crate::org::allbinary::game::layer::waypoint::WaypointLayer::WaypointLayer::isSelfUpgradeable(crate::org::allbinary::game::layer::waypoint::WaypointLayer::WaypointLayerDyn::as_WaypointLayer_mut(self)) }
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
impl crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface for VisualWaypointLayer {
    fn getCollidableInferface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>> { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::getCollidableInferface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
}
impl crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayerInterface::MultiPlayerGameLayerInterface for VisualWaypointLayer {
    fn getMultiplayerBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::multiplayer::layer::MultiplayerBehavior::MultiplayerBehaviorDyn>>> { crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer::getMultiplayerBehavior(crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayerDyn::as_MultiPlayerGameLayer_mut(self)) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for VisualWaypointLayer {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::getSourceId(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for VisualWaypointLayer {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::set(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::layer::pickup::PickupCompositeInterface::PickupCompositeInterface for VisualWaypointLayer {
    fn getPickupBehavior(&mut self) -> Option<Rc<RefCell<crate::org::allbinary::game::layer::pickup::PickupBehavior::PickupBehavior>>> { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::getPickupBehavior(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
    fn setPickupBehavior(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::game::layer::pickup::PickupBehavior::PickupBehavior>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::setPickupBehavior(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1) }
}


