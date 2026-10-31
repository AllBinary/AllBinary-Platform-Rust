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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::org::allbinary::animation::AnimationInterfaceFactoryInterface::AnimationInterfaceFactoryInterface;
use crate::org::allbinary::animation::NullAnimationFactory::NullAnimationFactory;
use crate::org::allbinary::animation::NullIndexedAnimationFactory::NullIndexedAnimationFactory;
use crate::org::allbinary::animation::ProceduralAnimationInterfaceFactoryInterface::ProceduralAnimationInterfaceFactoryInterface;
use crate::org::allbinary::animation::transition::shake::ShakeAnimationListener::ShakeAnimationListener;
use crate::org::allbinary::direction::Direction::Direction;
use crate::org::allbinary::direction::DirectionFactory::DirectionFactory;
use crate::org::allbinary::game::GameTypeFactory::GameTypeFactory;
use crate::org::allbinary::game::combat::destroy::event::DestroyedEvent::DestroyedEvent;
use crate::org::allbinary::game::combat::destroy::event::DestroyedEventHandler::DestroyedEventHandler;
use crate::org::allbinary::game::combat::destroy::event::DestroyedEventListenerInterface::DestroyedEventListenerInterface;
use crate::org::allbinary::game::health::Health::Health;
use crate::org::allbinary::game::identification::Group::Group;
use crate::org::allbinary::game::identification::GroupFactory::GroupFactory;
use crate::org::allbinary::game::input::event::GameKeyEventFactory::GameKeyEventFactory;
use crate::org::allbinary::game::input::form::NullRTSFormInputFactory::NullRTSFormInputFactory;
use crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInput;
use crate::org::allbinary::game::layer::building::event::BuildingEventHandler::BuildingEventHandler;
use crate::org::allbinary::game::layer::unit::UnitLayer::UnitLayer;
use crate::org::allbinary::game::layer::waypoint::Waypoint2LogHelper::Waypoint2LogHelper;
use crate::org::allbinary::game::layer::waypoint::Waypoint2SelectedLogHelper::Waypoint2SelectedLogHelper;
use crate::org::allbinary::game::layer::waypoint::WaypointBase::WaypointBase;
use crate::org::allbinary::game::layer::waypoint::WaypointLogHelper::WaypointLogHelper;
use crate::org::allbinary::game::layer::waypoint::WaypointRunnableLogHelper::WaypointRunnableLogHelper;
use crate::org::allbinary::game::layer::waypoint::WaypointRunnableSelectedLogHelper::WaypointRunnableSelectedLogHelper;
use crate::org::allbinary::game::layer::waypoint::WaypointSelectedLogHelper::WaypointSelectedLogHelper;
use crate::org::allbinary::game::multiplayer::layer::RemoteInfo::RemoteInfo;
use crate::org::allbinary::game::tick::NullTickable::NullTickable;
use crate::org::allbinary::game::tick::TickableInterface::TickableInterface;
use crate::org::allbinary::game::view::TileLayerPositionIntoViewPosition::TileLayerPositionIntoViewPosition;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::graphics::RectangleFactory::RectangleFactory;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use crate::org::allbinary::math::FrameUtil::FrameUtil;
use crate::org::allbinary::media::AllBinaryVibrationME::AllBinaryVibrationME;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCompositeInterface::GeographicMapCompositeInterface;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapDirectionUtil::GeographicMapDirectionUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::view::ViewPositionBase::ViewPositionBase;
use super::RTSLayer::RTSLayer;
use super::PathFindingLayerInterface::PathFindingLayerInterface;
use super::WaypointBehaviorBase::WaypointBehaviorBase;
use super::AdvancedRTSProperties::AdvancedRTSProperties;
use super::AdvancedRTSPlayerLayerInterface::AdvancedRTSPlayerLayerInterface;
use super::RTSPlayerLayerInterface::RTSPlayerLayerInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AdvancedRTSGameLayer {
    pub base: RTSLayer,
    pub shakeListener: Option<Rc<RefCell<dyn crate::org::allbinary::animation::transition::shake::ShakeAnimationListener::ShakeAnimationListenerDyn>>>,
    pub vibration: Option<Rc<RefCell<dyn crate::org::allbinary::media::AllBinaryVibrationME::AllBinaryVibrationMEDyn>>>,
    pub duration: i32,
    pub parentLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>,
    pub waypointBehaviorBase: Option<Rc<RefCell<dyn TickableInterface>>>,
}

// impl DestroyedEventListenerInterface for AdvancedRTSGameLayer with the methods of the trait
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for AdvancedRTSGameLayer {
    fn getName(&mut self) -> String { self.base.getName() }
}
impl crate::org::allbinary::game::combat::destroy::DestroyableInterface::DestroyableInterface for AdvancedRTSGameLayer {
    fn isDestroyed(&mut self) -> bool { self.base.isDestroyed() }
}

impl AdvancedRTSGameLayer {
    

    // throws Exception
    pub fn createSimulated() -> Option<Rc<RefCell<dyn AdvancedRTSGameLayerDyn>>> {
        let mut nullAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>> = NullAnimationFactory::getFactoryInstance();
        let mut nullIndexedAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>> = NullIndexedAnimationFactory::getFactoryInstance();
        return AdvancedRTSGameLayer::new(crate::org::allbinary::game::multiplayer::layer::RemoteInfo::RemoteInfo_REMOTE_INFO.with(Clone::clone).clone(), NullPathFindingLayer_NULL_PATH_FINDING_LAYER.with(Clone::clone).clone(), AdvancedRTSProperties::createPropertiesSimulated(), jcall!(GroupFactory::getInstance()).NULL_GROUP_ARRAY.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), crate::org::allbinary::game::health::Health::Health_NULL_HEALTH.with(Clone::clone).clone(), NullRTSFormInputFactory::getInstance(), nullAnimationInterfaceFactoryInterface.clone(), nullIndexedAnimationInterfaceFactoryInterface.clone(), nullAnimationInterfaceFactoryInterface.clone(), nullAnimationInterfaceFactoryInterface.clone(), nullIndexedAnimationInterfaceFactoryInterface.clone(), NullIndexedAnimationFactory::getFactoryInstance(), crate::org::allbinary::graphics::RectangleFactory::RectangleFactory_SINGLETON.with(Clone::clone).clone(), 0, 0, TileLayerPositionIntoViewPosition::new()).map(|reference| reference as Rc<RefCell<dyn AdvancedRTSGameLayerDyn>>);
    }
    

    pub fn new_struct(mut remoteInfo: Option<Rc<RefCell<RemoteInfo>>>, mut parentLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut advancedRTSProperties: Option<Rc<RefCell<AdvancedRTSProperties>>>, mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>, mut rootName: String, mut name: String, mut healthInterface: Option<Rc<RefCell<Health>>>, mut rtsFormInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn>>>, mut animationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut emptyAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut baseAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut buildAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut verticleBuildAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut proceduralAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn ProceduralAnimationInterfaceFactoryInterface>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut x: i32, mut y: i32, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Self {
        let mut this = Self {
            base: RTSLayer::new_struct(remoteInfo.clone(), groupInterface.clone(), rootName.clone(), name.clone(), healthInterface.clone(), rtsFormInput.clone(), animationInterfaceFactoryInterface.clone(), emptyAnimationInterfaceFactoryInterface.clone(), baseAnimationInterfaceFactoryInterface.clone(), buildAnimationInterfaceFactoryInterface.clone(), verticleBuildAnimationInterfaceFactoryInterface.clone(), proceduralAnimationInterfaceFactoryInterface.clone(), rectangle.clone(), x.clone(), y.clone(), viewPosition.clone()),
            shakeListener: None,
            vibration: None,
            duration: 0,
            parentLayer: None,
            waypointBehaviorBase: None,
        };
        this.parentLayer = NullPathFindingLayer_NULL_PATH_FINDING_LAYER.with(Clone::clone).clone();
        this.waypointBehaviorBase = NullTickable::getInstance();
        // super(...) is in the struct literal above
        this.setVisible(false);
        this.setWaypointBehavior(WaypointBehaviorBase::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>));
        this.shakeListener = jcall!(advancedRTSProperties).shakeListener.clone();
        this.vibration = jcall!(advancedRTSProperties).vibration.clone();
        this.duration = jcall!(advancedRTSProperties).duration.clone();
        this.setParentLayer(parentLayer.clone());
        this
    }

    pub fn new(mut remoteInfo: Option<Rc<RefCell<RemoteInfo>>>, mut parentLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut advancedRTSProperties: Option<Rc<RefCell<AdvancedRTSProperties>>>, mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>, mut rootName: String, mut name: String, mut healthInterface: Option<Rc<RefCell<Health>>>, mut rtsFormInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn>>>, mut animationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut emptyAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut baseAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut buildAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut verticleBuildAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut proceduralAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn ProceduralAnimationInterfaceFactoryInterface>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut x: i32, mut y: i32, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(remoteInfo, parentLayer, advancedRTSProperties, groupInterface, rootName, name, healthInterface, rtsFormInput, animationInterfaceFactoryInterface, emptyAnimationInterfaceFactoryInterface, baseAnimationInterfaceFactoryInterface, buildAnimationInterfaceFactoryInterface, verticleBuildAnimationInterfaceFactoryInterface, proceduralAnimationInterfaceFactoryInterface, rectangle, x, y, viewPosition))))
    }
    

    // throws Exception
    pub fn updateWaypointBehavior(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) {
        self.base.updateWaypointBehavior(geographicMapInterface.clone());
        jcall!(jcall!(self.getWaypointBehavior()).getWaypoint()).setAllBinaryGameLayerManager(self.allBinaryGameLayerManagerP.clone());
    }
    

    pub fn initVisibility(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>) {
        let mut advancedRTSPlayerLayerInterface: Option<Rc<RefCell<dyn AdvancedRTSPlayerLayerInterface>>> = panic!("ClassCastException: a Rust dyn RTSPlayerLayerInterface cannot be cast to dyn AdvancedRTSPlayerLayerInterface");
        if jcall!(advancedRTSPlayerLayerInterface).isLocalPlayer() || match (&jcall!(advancedRTSPlayerLayerInterface).getGameType(), &jcall!(GameTypeFactory::getInstance()).BOT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.setVisible(true);
        } else {
            self.setVisible(false);
        }
    }
    

    // throws Exception
    pub fn construct(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>) {
        self.base.construct(rtsPlayerLayerInterface.clone());
        self.initVisibility(rtsPlayerLayerInterface.clone());
        jcall!(BuildingEventHandler::getInstance()).addListenerInterface(jcall!(self.getWaypointBehavior()).getWaypoint());
    }
    

    // throws Exception
    pub fn setDestroyed(&mut self, mut destroyed: bool) {
        self.base.setDestroyed(destroyed.clone());
        if self.isDestroyed() {
            jcall!(BuildingEventHandler::getInstance()).removeListener(jcall!(self.getWaypointBehavior()).getWaypoint());
            jcall!(BuildingEventHandler::getInstance()).removeListener(None /* Java this: a &mut self method has no Rc of itself */);
            jcall!(DestroyedEventHandler::getInstance()).addListener(None /* Java this: a &mut self method has no Rc of itself */);
            jcall!(RTSLayerUtil::RTSLayerUtil::getInstance()).destroyAndClear(jcall!(self.getWaypointBehavior()).getOwnedWaypointList());
        }
    }
    

    pub fn getParentLayer(&mut self) -> Option<Rc<RefCell<dyn PathFindingLayerInterface>>> {
        return self.parentLayer.clone();
    }
    

    pub fn setParentLayer(&mut self, mut parentLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        self.parentLayer = parentLayer.clone();
    }
    

    pub fn getWaypointBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>> {
        return (self.waypointBehaviorBase).clone() /* as WaypointBehaviorBase */;
    }
    

    pub fn setWaypointBehavior(&mut self, mut unitWaypointHelper: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>) {
        self.waypointBehaviorBase = unitWaypointHelper.clone().map(|reference| reference as Rc<RefCell<dyn TickableInterface>>);
    }
    

    pub fn getWaypointLogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointLogHelper::WaypointLogHelperDyn>>> {
        return self.waypointLogHelperP.clone();
    }
    

    pub fn getWaypoint2LogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::Waypoint2LogHelper::Waypoint2LogHelperDyn>>> {
        return self.waypoint2LogHelperP.clone();
    }
    

    pub fn getWaypointRunnableLogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointRunnableLogHelper::WaypointRunnableLogHelperDyn>>> {
        return self.waypointRunnableLogHelperP.clone();
    }
    

    pub fn shouldAddWaypointFromBuilding(&mut self) -> bool {
        if !match (&self.parentLayer, &NullPathFindingLayer_NULL_PATH_FINDING_LAYER.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut parentAdvancedRTSGameLayer: Option<Rc<RefCell<dyn AdvancedRTSGameLayerDyn>>> = (self.parentLayer).clone() /* as AdvancedRTSGameLayer */;
            if jcall!(parentAdvancedRTSGameLayer).getType() != UnitLayer::getStaticType() {
                return true;
            }
        }
        return false;
    }
    

    pub fn isWaypointListEmptyOrOnlyTargets(&mut self) -> bool {
        if !match (&self.getType(), &UnitLayer::getStaticType()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    // throws Exception
    pub fn buildingChase(&mut self, mut allbinaryLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> bool {
        let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = self.getCurrentGeographicMapCellPosition();
        let mut buildingDirection: Option<Rc<RefCell<Direction>>> = jcall!(GeographicMapDirectionUtil::getInstance()).getDirectionFromCellPositionToAdjacentCellPosition(geographicMapCellPosition.clone(), cellPosition.clone());
        let mut angleInfo: Option<Rc<RefCell<AngleInfo>>> = jcall!(self.getRotationAnimationInterface()).getAngleInfoP();
        let mut angle: i32 = jcall!(FrameUtil::getInstance()).adjustAngleToFrameAngle(((jcall!(angleInfo).getAngle()) as i32) + 90);
        let mut gameKeyEventFactory: Option<Rc<RefCell<GameKeyEventFactory>>> = GameKeyEventFactory::getInstance();
        if match (&buildingDirection, &jcall!(DirectionFactory::getInstance()).UP) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(self.rtsLayer2LogHelper).buildingAbove(None /* Java this: a &mut self method has no Rc of itself */);
            if angle > 180 && angle < 360 {
                jcall!(self.rtsLayer2LogHelper).buildingChaseLeft(None /* Java this: a &mut self method has no Rc of itself */, angle.clone());
                jcall!(self.getGameKeyEventList()).add(jcall!(gameKeyEventFactory).getInstanceForKey(None /* Java this: a &mut self method has no Rc of itself */, unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() }.clone()));
                if angle <= 190 {
                    return true;
                }
            } else {
                jcall!(self.rtsLayer2LogHelper).buildingChaseRight(None /* Java this: a &mut self method has no Rc of itself */, angle.clone());
                jcall!(self.getGameKeyEventList()).add(jcall!(gameKeyEventFactory).getInstanceForKey(None /* Java this: a &mut self method has no Rc of itself */, unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() }.clone()));
                if angle >= 170 {
                    return true;
                }
            }
        } else if match (&buildingDirection, &jcall!(DirectionFactory::getInstance()).DOWN) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(self.rtsLayer2LogHelper).buildingDown(None /* Java this: a &mut self method has no Rc of itself */);
            if angle > 180 && angle < 360 {
                jcall!(self.rtsLayer2LogHelper).buildingChaseLeft(None /* Java this: a &mut self method has no Rc of itself */, angle.clone());
                jcall!(self.getGameKeyEventList()).add(jcall!(gameKeyEventFactory).getInstanceForKey(None /* Java this: a &mut self method has no Rc of itself */, unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() }.clone()));
                if angle > 315 {
                    return true;
                }
            } else {
                jcall!(self.rtsLayer2LogHelper).buildingChaseRight(None /* Java this: a &mut self method has no Rc of itself */, angle.clone());
                jcall!(self.getGameKeyEventList()).add(jcall!(gameKeyEventFactory).getInstanceForKey(None /* Java this: a &mut self method has no Rc of itself */, unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() }.clone()));
                if angle < 45 {
                    return true;
                }
            }
        } else if match (&buildingDirection, &jcall!(DirectionFactory::getInstance()).LEFT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(self.rtsLayer2LogHelper).buildingLeft();
            if angle > 90 && angle < 270 {
                jcall!(self.rtsLayer2LogHelper).buildingChaseDown(None /* Java this: a &mut self method has no Rc of itself */, angle.clone());
                jcall!(self.getGameKeyEventList()).add(jcall!(gameKeyEventFactory).getInstanceForKey(None /* Java this: a &mut self method has no Rc of itself */, unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() }.clone()));
                if angle < 135 {
                    return true;
                }
            } else {
                jcall!(self.rtsLayer2LogHelper).buildingChaseUp(None /* Java this: a &mut self method has no Rc of itself */, angle.clone());
                jcall!(self.getGameKeyEventList()).add(jcall!(gameKeyEventFactory).getInstanceForKey(None /* Java this: a &mut self method has no Rc of itself */, unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() }.clone()));
                if angle > 45 {
                    return true;
                }
            }
        } else if match (&buildingDirection, &jcall!(DirectionFactory::getInstance()).RIGHT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(self.rtsLayer2LogHelper).buildingRight();
            if angle > 90 && angle < 270 {
                jcall!(self.rtsLayer2LogHelper).buildingChaseDown(None /* Java this: a &mut self method has no Rc of itself */, angle.clone());
                jcall!(self.getGameKeyEventList()).add(jcall!(gameKeyEventFactory).getInstanceForKey(None /* Java this: a &mut self method has no Rc of itself */, unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() }.clone()));
                if angle > 225 {
                    return true;
                }
            } else {
                jcall!(self.rtsLayer2LogHelper).buildingChaseUp(None /* Java this: a &mut self method has no Rc of itself */, angle.clone());
                jcall!(self.getGameKeyEventList()).add(jcall!(gameKeyEventFactory).getInstanceForKey(None /* Java this: a &mut self method has no Rc of itself */, unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() }.clone()));
                if angle < 315 && angle > 180 {
                    return true;
                }
            }
        } else {
            return true;
        }
        return false;
    }
    

    // throws Exception
    pub fn getSurroundingGeographicMapCellPositionList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (self.allBinaryGameLayerManagerP).clone() /* as GeographicMapCompositeInterface */;
        let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
        jcall!(self.geographicMapCellPositionAreaBase).update(geographicMapInterface.clone());
        return jcall!(self.geographicMapCellPositionAreaBase).getSurroundingGeographicMapCellPositionList();
    }
    

    pub fn setSelected(&mut self, mut selected: bool) {
        self.base.setSelected(selected.clone());
        if self.debug {
            if selected {
                self.waypointLogHelperP = WaypointSelectedLogHelper::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointLogHelper::WaypointLogHelperDyn>>);
                self.waypoint2LogHelperP = Waypoint2SelectedLogHelper::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::Waypoint2LogHelper::Waypoint2LogHelperDyn>>);
                self.waypointRunnableLogHelperP = WaypointRunnableSelectedLogHelper::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointRunnableLogHelper::WaypointRunnableLogHelperDyn>>);
            } else {
                self.waypointLogHelperP = WaypointLogHelper::getInstance();
                self.waypoint2LogHelperP = Waypoint2LogHelper::getInstance();
                self.waypointRunnableLogHelperP = WaypointRunnableLogHelper::getInstance();
            }
        }
    }
    

    pub fn onDestroyed(&mut self, mut destroyedEvent: Option<Rc<RefCell<DestroyedEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = jcall!(destroyedEvent).getLayerInterface();
            if match &(layerInterface) { Some(reference) => std::ptr::addr_eq(&*self as *const Self, reference.as_ptr()), None => false } {
                jcall!(DestroyedEventHandler::getInstance()).removeListener(None /* Java this: a &mut self method has no Rc of itself */);
                let mut waypoint: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointBase::WaypointBaseDyn>>> = (jcall!(self.getWaypointBehavior()).getWaypoint()).clone() /* as WaypointBase */;
                jcall!(waypoint).reset();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onDestroyed"), crate::java_into_object(e.clone()));
            }
        }
    }
}
impl std::ops::Deref for AdvancedRTSGameLayer {
    type Target = RTSLayer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AdvancedRTSGameLayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AdvancedRTSGameLayerDyn: crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn {
    fn as_AdvancedRTSGameLayer(&self) -> &AdvancedRTSGameLayer;
    fn as_AdvancedRTSGameLayer_mut(&mut self) -> &mut AdvancedRTSGameLayer;
    fn initVisibility(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>) { AdvancedRTSGameLayer::initVisibility(self.as_AdvancedRTSGameLayer_mut(), rtsPlayerLayerInterface) }
    fn setParentLayer(&mut self, mut parentLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { AdvancedRTSGameLayer::setParentLayer(self.as_AdvancedRTSGameLayer_mut(), parentLayer) }
    fn setWaypointBehavior(&mut self, mut unitWaypointHelper: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>) { AdvancedRTSGameLayer::setWaypointBehavior(self.as_AdvancedRTSGameLayer_mut(), unitWaypointHelper) }
}
impl AdvancedRTSGameLayerDyn for AdvancedRTSGameLayer {
    fn as_AdvancedRTSGameLayer(&self) -> &AdvancedRTSGameLayer { self }
    fn as_AdvancedRTSGameLayer_mut(&mut self) -> &mut AdvancedRTSGameLayer { self }
}

impl crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn for AdvancedRTSGameLayer {
    fn as_CollidableCompositeLayer(&self) -> &crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer { &self.base.base.base.base }
    fn as_CollidableCompositeLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer { &mut self.base.base.base.base }
}
impl crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn for AdvancedRTSGameLayer {
    fn as_AllBinaryGameLayer(&self) -> &crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &self.base.base.base.base.base }
    fn as_AllBinaryGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &mut self.base.base.base.base.base }
    fn processInput(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::processInput(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self), p1) }
    fn processTick(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::processTick(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn for AdvancedRTSGameLayer {
    fn as_RTSLayer(&self) -> &crate::org::allbinary::game::layer::RTSLayer::RTSLayer { &self.base }
    fn as_RTSLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::RTSLayer::RTSLayer { &mut self.base }
    fn updateWaypointBehavior(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) { Self::updateWaypointBehavior(self, geographicMapInterface) }
    fn construct(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>) { Self::construct(self, rtsPlayerLayerInterface) }
    fn setSelected(&mut self, mut selected: bool) { Self::setSelected(self, selected) }
    fn setDestroyed(&mut self, mut destroyed: bool) { Self::setDestroyed(self, destroyed) }
    fn getWaypointBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>> { Self::getWaypointBehavior(self) }
    fn getParentLayer(&mut self) -> Option<Rc<RefCell<dyn PathFindingLayerInterface>>> { Self::getParentLayer(self) }
    fn getWaypointLogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointLogHelper::WaypointLogHelperDyn>>> { Self::getWaypointLogHelper(self) }
    fn getWaypoint2LogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::Waypoint2LogHelper::Waypoint2LogHelperDyn>>> { Self::getWaypoint2LogHelper(self) }
    fn getWaypointRunnableLogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointRunnableLogHelper::WaypointRunnableLogHelperDyn>>> { Self::getWaypointRunnableLogHelper(self) }
    fn shouldAddWaypointFromBuilding(&mut self) -> bool { Self::shouldAddWaypointFromBuilding(self) }
    fn getSurroundingGeographicMapCellPositionList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getSurroundingGeographicMapCellPositionList(self) }
    fn isWaypointListEmptyOrOnlyTargets(&mut self) -> bool { Self::isWaypointListEmptyOrOnlyTargets(self) }
    fn buildingChase(&mut self, mut allbinaryLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> bool { Self::buildingChase(self, allbinaryLayer, cellPosition) }
}
impl crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayerDyn for AdvancedRTSGameLayer {
    fn as_MultiPlayerGameLayer(&self) -> &crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer { &self.base.base }
    fn as_MultiPlayerGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer { &mut self.base.base }
}
impl crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn for AdvancedRTSGameLayer {
    fn as_AllBinaryLayer(&self) -> &crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &self.base.base.base.base.base.base }
    fn as_AllBinaryLayer_mut(&mut self) -> &mut crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &mut self.base.base.base.base.base.base }
    fn onChangeEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::view::event::ViewPositionEvent::ViewPositionEvent>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::onChangeEvent(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
    fn implmentsTickableInterface(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::implmentsTickableInterface(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn implmentsGameInputInterface(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::implmentsGameInputInterface(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn implmentsCollidableInterface(&mut self) -> bool { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::implmentsCollidableInterface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
    fn toString(&mut self) -> String { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::toString(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
}
impl crate::org::allbinary::layer::Layer::LayerDyn for AdvancedRTSGameLayer {
    fn as_Layer(&self) -> &crate::org::allbinary::layer::Layer::Layer { &self.base.base.base.base.base.base.base }
    fn as_Layer_mut(&mut self) -> &mut crate::org::allbinary::layer::Layer::Layer { &mut self.base.base.base.base.base.base.base }
    fn toStringAppend(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::logic::string::StringMaker::StringMaker>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::toStringAppend(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn for AdvancedRTSGameLayer {
    fn as_CollidableDestroyableDamageableLayer(&self) -> &crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer { &self.base.base.base }
    fn as_CollidableDestroyableDamageableLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer { &mut self.base.base.base }
    fn damage(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::damage(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self), p1, p2) }
    fn isDestroyed(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::isDestroyed(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn setAllBinaryGameLayerManager(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::setAllBinaryGameLayerManager(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self), p1) }
    fn onDestroyed(&mut self, mut destroyedEvent: Option<Rc<RefCell<DestroyedEvent>>>) { Self::onDestroyed(self, destroyedEvent) }
}
impl crate::org::allbinary::game::layer::RTSInterface::RTSInterface for AdvancedRTSGameLayer {
    fn isSelfUpgradeable(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::isSelfUpgradeable(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
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
impl crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayerInterface::MultiPlayerGameLayerInterface for AdvancedRTSGameLayer {
    fn getMultiplayerBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::multiplayer::layer::MultiplayerBehavior::MultiplayerBehaviorDyn>>> { crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer::getMultiplayerBehavior(crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayerDyn::as_MultiPlayerGameLayer_mut(self)) }
}
impl crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface for AdvancedRTSGameLayer {
    fn getCollidableInferface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>> { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::getCollidableInferface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for AdvancedRTSGameLayer {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::set(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for AdvancedRTSGameLayer {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::getSourceId(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
}
impl crate::org::allbinary::game::layer::pickup::PickupCompositeInterface::PickupCompositeInterface for AdvancedRTSGameLayer {
    fn getPickupBehavior(&mut self) -> Option<Rc<RefCell<crate::org::allbinary::game::layer::pickup::PickupBehavior::PickupBehavior>>> { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::getPickupBehavior(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
    fn setPickupBehavior(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::game::layer::pickup::PickupBehavior::PickupBehavior>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::setPickupBehavior(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1) }
}


