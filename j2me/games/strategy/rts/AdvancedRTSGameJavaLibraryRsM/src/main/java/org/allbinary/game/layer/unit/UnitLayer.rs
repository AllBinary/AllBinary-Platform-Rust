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
// package org.allbinary.game.layer.unit
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::J2MEUtil::J2MEUtil;
use crate::org::allbinary::ag::weapon::media::audio::ExplosionBasicSound::ExplosionBasicSound;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::AnimationInterfaceFactoryInterface::AnimationInterfaceFactoryInterface;
use crate::org::allbinary::animation::FeaturedAnimationInterfaceFactoryInterfaceFactory::FeaturedAnimationInterfaceFactoryInterfaceFactory;
use crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation;
use crate::org::allbinary::animation::NullAnimationFactory::NullAnimationFactory;
use crate::org::allbinary::animation::NullIndexedAnimationFactory::NullIndexedAnimationFactory;
use crate::org::allbinary::animation::ProceduralAnimationInterfaceFactoryInterface::ProceduralAnimationInterfaceFactoryInterface;
use crate::org::allbinary::animation::RotationAnimation::RotationAnimation;
use crate::org::allbinary::animation::caption::CaptionAnimationHelper::CaptionAnimationHelper;
use crate::org::allbinary::animation::caption::CaptionAnimationHelperBase::CaptionAnimationHelperBase;
use crate::org::allbinary::animation::vector::AdjustedCircleAnimation::AdjustedCircleAnimation;
use crate::org::allbinary::direction::Direction::Direction;
use crate::org::allbinary::game::combat::damage::DamageFloaters::DamageFloaters;
use crate::org::allbinary::game::combat::damage::PtsDamageFloaters::PtsDamageFloaters;
use crate::org::allbinary::game::combat::weapon::WeaponProperties::WeaponProperties;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::GameFeatureFactory::GameFeatureFactory;
use crate::org::allbinary::game::graphics::hud::BasicHudFactory::BasicHudFactory;
use crate::org::allbinary::game::health::Health::Health;
use crate::org::allbinary::game::health::HealthBar::HealthBar;
use crate::org::allbinary::game::health::HealthBarTwodAnimation::HealthBarTwodAnimation;
use crate::org::allbinary::game::identification::Group::Group;
use crate::org::allbinary::game::identification::GroupCommonFactory::GroupCommonFactory;
use crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent;
use crate::org::allbinary::game::input::event::GameKeyEventFactory::GameKeyEventFactory;
use crate::org::allbinary::game::input::event::GameKeyEventUtil::GameKeyEventUtil;
use crate::org::allbinary::game::input::form::VisibleCellPositionsSingleton::VisibleCellPositionsSingleton;
use crate::org::allbinary::game::input::form::WaypointRTSFormInput::WaypointRTSFormInput;
use crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer;
use crate::org::allbinary::game::layer::AdvancedRTSProperties::AdvancedRTSProperties;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use crate::org::allbinary::game::layer::CaptionResources::CaptionResources;
use crate::org::allbinary::game::layer::LinePathRelativeAnimation::LinePathRelativeAnimation;
use crate::org::allbinary::game::layer::PathAnimation::PathAnimation;
use crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface;
use crate::org::allbinary::game::layer::RTSLayer2LogHelper::RTSLayer2LogHelper;
use crate::org::allbinary::game::layer::RTSLayer2SelectedLogHelper::RTSLayer2SelectedLogHelper;
use crate::org::allbinary::game::layer::RTSLayerEvent::RTSLayerEvent;
use crate::org::allbinary::game::layer::RTSLayerHudPaintable::RTSLayerHudPaintable;
use crate::org::allbinary::game::layer::RTSLayerLogHelper::RTSLayerLogHelper;
use crate::org::allbinary::game::layer::RTSLayerSelectedLogHelper::RTSLayerSelectedLogHelper;
use crate::org::allbinary::game::layer::RTSPlayerLayerInterface::RTSPlayerLayerInterface;
use crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintable;
use crate::org::allbinary::game::layer::SensorActionFactory::SensorActionFactory;
use crate::org::allbinary::game::layer::SteeringVisitor::SteeringVisitor;
use crate::org::allbinary::game::layer::TiledLayerUtil::TiledLayerUtil;
use crate::org::allbinary::game::layer::VehicleFrictionProperties::VehicleFrictionProperties;
use crate::org::allbinary::game::layer::VehicleProperties::VehicleProperties;
use crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBase;
use crate::org::allbinary::game::layer::building::BuildingLayer::BuildingLayer;
use crate::org::allbinary::game::layer::building::event::BuildingEventHandler::BuildingEventHandler;
use crate::org::allbinary::game::layer::building::event::BuildingEventListenerInterface::BuildingEventListenerInterface;
use crate::org::allbinary::game::layer::capital::event::CapitalEvent::CapitalEvent;
use crate::org::allbinary::game::layer::capital::event::CapitalEventHandlerFactory::CapitalEventHandlerFactory;
use crate::org::allbinary::game::layer::geographic::map::LayerPartialCellPositionsUtil::LayerPartialCellPositionsUtil;
use crate::org::allbinary::game::layer::special::SpecialDownGameInputProcessor::SpecialDownGameInputProcessor;
use crate::org::allbinary::game::layer::special::SpecialFireGameInputProcessor::SpecialFireGameInputProcessor;
use crate::org::allbinary::game::layer::special::SpecialLeftGameInputProcessor::SpecialLeftGameInputProcessor;
use crate::org::allbinary::game::layer::special::SpecialRightGameInputProcessor::SpecialRightGameInputProcessor;
use crate::org::allbinary::game::layer::special::SpecialUpGameInputProcessor::SpecialUpGameInputProcessor;
use crate::org::allbinary::game::layer::waypoint::MultipassNoCacheWaypoint::MultipassNoCacheWaypoint;
use crate::org::allbinary::game::layer::waypoint::NoCacheWaypoint::NoCacheWaypoint;
use crate::org::allbinary::game::layer::waypoint::WaypointBase::WaypointBase;
use crate::org::allbinary::game::layer::waypoint::event::WaypointEventHandlerFactory::WaypointEventHandlerFactory;
use crate::org::allbinary::game::multiplayer::layer::RemoteInfo::RemoteInfo;
use crate::org::allbinary::game::part::weapon::BasicWeaponPart::BasicWeaponPart;
use crate::org::allbinary::game::part::weapon::SalvoInterface::SalvoInterface;
use crate::org::allbinary::game::physics::velocity::VelocityProperties::VelocityProperties;
use crate::org::allbinary::game::tracking::TrackingEvent::TrackingEvent;
use crate::org::allbinary::game::tracking::TrackingEventHandler::TrackingEventHandler;
use crate::org::allbinary::game::tracking::TrackingEventListenerInterface::TrackingEventListenerInterface;
use crate::org::allbinary::graphics::CellPositionFactory::CellPositionFactory;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::paint::NullPaintable::NullPaintable;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::layer::Layer::Layer;
use crate::org::allbinary::layer::LayerInterfaceFactoryInterface::LayerInterfaceFactoryInterface;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::math::BasicDecimal::BasicDecimal;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::math::AngleFactory::AngleFactory;
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use crate::org::allbinary::math::FrameUtil::FrameUtil;
use crate::org::allbinary::math::LayerDistanceUtil::LayerDistanceUtil;
use crate::org::allbinary::math::NamedAngle::NamedAngle;
use crate::org::allbinary::media::audio::AttackSound::AttackSound;
use crate::org::allbinary::media::audio::SecondaryPlayerQueueFactory::SecondaryPlayerQueueFactory;
use crate::org::allbinary::media::audio::Sound::Sound;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellHistory::GeographicMapCellHistory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCompositeInterface::GeographicMapCompositeInterface;
use crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory;
use crate::org::allbinary::media::graphics::geography::map::drop::DropCellPositionHistory::DropCellPositionHistory;
use crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMap::RaceTrackGeographicMap;
use crate::org::allbinary::string::CommonPhoneStrings::CommonPhoneStrings;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::util::BasicArrayListS::BasicArrayListS;
use crate::org::allbinary::view::ViewPositionBase::ViewPositionBase;
use crate::java::lang::Exception::*;
use super::CollidableUnitBehavior::CollidableUnitBehavior;
use super::UnitWaypointBehavior2::UnitWaypointBehavior2;
use super::UnitWaypointBehavior::UnitWaypointBehavior;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UnitLayer {
    pub base: AdvancedRTSGameLayer,
    pub basicColorFactory: Option<Rc<RefCell<BasicColorFactory>>>,
    pub groupCommonFactory: Option<Rc<RefCell<GroupCommonFactory>>>,
    pub layerPartialCellPositionsUtil: Option<Rc<RefCell<LayerPartialCellPositionsUtil>>>,
    pub layerDistanceUtil: Option<Rc<RefCell<LayerDistanceUtil>>>,
    pub angleFactory: Option<Rc<RefCell<AngleFactory>>>,
    pub cellPositionFactory: Option<Rc<RefCell<CellPositionFactory>>>,
    pub maxResourceLoad: i16,
    pub vehicleProperties: Option<Rc<RefCell<VehicleProperties>>>,
    pub accelerationBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>,
    pub decelerationBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>,
    pub trackingEvent: Option<Rc<RefCell<TrackingEvent>>>,
    pub damageFloaters: Option<Rc<RefCell<dyn crate::org::allbinary::game::combat::damage::DamageFloaters::DamageFloatersDyn>>>,
    pub damageFloatersPaintableInterface: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
    pub healthBar: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
    pub moveSoundInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>,
    pub initResourceAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>>,
    pub resourceAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>>,
    pub initPathAnimation: Option<Rc<RefCell<PathAnimation>>>,
    pub pathAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
    pub captionAnimationHelper: Option<Rc<RefCell<dyn crate::org::allbinary::animation::caption::CaptionAnimationHelperBase::CaptionAnimationHelperBaseDyn>>>,
    pub decalAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>>,
    pub waypointLayerInterfaceFactoryInterface: Option<Rc<RefCell<dyn LayerInterfaceFactoryInterface>>>,
    pub resourceLoad: i32,
    pub weaponRange: i32,
    pub rtsLogHelper: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayerLogHelper::RTSLayerLogHelperDyn>>>,
    pub rotationAnimationInterfaceP: Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>>,
    pub movementAngle: Option<Rc<RefCell<NamedAngle>>>,
    pub steeringInsideGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>,
    pub sensorGeographicMapCellPositionList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub CAPITAL_EVENT: Option<Rc<RefCell<CapitalEvent>>>,
}

pub static UnitLayer_partialPositionList: crate::JavaStatic<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>> = crate::JavaStatic::new(|| BasicArrayListS::new(4).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
pub static UnitLayer_SENSOR_RANGE_MULTIPLIER: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 6);
pub static UnitLayer_MOVE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Moving"));
// impl BuildingEventListenerInterface for UnitLayer with the methods of the trait
impl TrackingEventListenerInterface for UnitLayer {
    fn onMovement(&mut self, mut trackingEvent: Option<Rc<RefCell<TrackingEvent>>>) { Self::onMovement(self, trackingEvent) }
}
impl PathFindingLayerInterface for UnitLayer {
    fn getEndGeographicMapCellPositionList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::getEndGeographicMapCellPositionList(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn shouldHandleStartSameAsEnd(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::shouldHandleStartSameAsEnd(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn getGeographicMapCellPositionArea(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::GeographicMapCellPositionAreaBase::GeographicMapCellPositionAreaBaseDyn>>> { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::getGeographicMapCellPositionArea(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn getHudPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn>>> { Self::getHudPaintable(self) }
    fn handleCost(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::handleCost(self, ownerLayer) }
    fn getParentLayer(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface>>> { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::getParentLayer(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn getRTSLayer2LogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer2LogHelper::RTSLayer2LogHelperDyn>>> { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::getRTSLayer2LogHelper(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn setTarget(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface>>>) { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::setTarget(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::input::GameInputInterface::GameInputInterface for UnitLayer {
    fn processInput(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processInput(self, layerManager) }
    fn initInputProcessors(&mut self) { Self::initInputProcessors(self) }
}
impl crate::org::allbinary::game::layer::waypoint::WaypointLayerInterface::WaypointLayerInterface for UnitLayer {
    fn init(&mut self, mut geographicMapCellHistory: Option<Rc<RefCell<GeographicMapCellHistory>>>, mut geographicMapCellPositionBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::init(self, geographicMapCellHistory, geographicMapCellPositionBasicArrayList) }
    fn getMoveOutOfBuildAreaPath(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::getMoveOutOfBuildAreaPath(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self), p1) }
    fn setClosestGeographicMapCellHistory(&mut self, mut pathsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::setClosestGeographicMapCellHistory(self, pathsList) }
    fn teleportTo(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) { Self::teleportTo(self, geographicMapCellPosition) }
    fn setLoad(&mut self, mut resource: i32) { Self::setLoad(self, resource) }
    fn getSurroundingGeographicMapCellPositionList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::getSurroundingGeographicMapCellPositionList(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn isSelected(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::isSelected(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn shouldAddWaypointFromBuilding(&mut self) -> bool { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::shouldAddWaypointFromBuilding(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn isShowMoreCaptionStates(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::isShowMoreCaptionStates(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn trackTo(&mut self, mut reason: String) { Self::trackTo(self, reason) }
    fn trackToDXY(&mut self, mut dx: i32, mut dy: i32) { Self::trackToDXY(self, dx, dy) }
    fn buildingChase(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, p2: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> bool { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::buildingChase(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self), p1, p2) }
    fn getTrackingEvent(&mut self) -> Option<Rc<RefCell<TrackingEvent>>> { Self::getTrackingEvent(self) }
    fn allStop(&mut self) { Self::allStop(self) }
    fn isWaypointListEmptyOrOnlyTargets(&mut self) -> bool { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::isWaypointListEmptyOrOnlyTargets(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn getCurrentGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { Self::getCurrentGeographicMapCellPosition(self) }
    fn getTopLeftGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::getTopLeftGeographicMapCellPosition(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn getCaptionAnimationHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::caption::CaptionAnimationHelperBase::CaptionAnimationHelperBaseDyn>>> { Self::getCaptionAnimationHelper(self) }
    fn getWaypointBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>> { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::getWaypointBehavior(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn getWaypointLogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointLogHelper::WaypointLogHelperDyn>>> { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::getWaypointLogHelper(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn getWaypoint2LogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::Waypoint2LogHelper::Waypoint2LogHelperDyn>>> { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::getWaypoint2LogHelper(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn getWaypointRunnableLogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointRunnableLogHelper::WaypointRunnableLogHelperDyn>>> { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::getWaypointRunnableLogHelper(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
}
impl crate::org::allbinary::layer::LayerInterface::LayerInterface for UnitLayer {
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
    fn implmentsTickableInterface(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::implmentsTickableInterface(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn implmentsCollidableInterface(&mut self) -> bool { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::implmentsCollidableInterface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
    fn implmentsGameInputInterface(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::implmentsGameInputInterface(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn implmentsArtificialIntelligenceCompositeInterface(&mut self) -> bool { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::implmentsArtificialIntelligenceCompositeInterface(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getType(&mut self) -> i32 { Self::getType(self) }
}
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for UnitLayer {
    fn getName(&mut self) -> String { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getName(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
}
impl crate::org::allbinary::layer::PositionInterface::PositionInterface for UnitLayer {
    fn setPosition(&mut self, p1: i32, p2: i32, p3: i32) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::setPosition(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1, p2, p3) }
}
impl crate::org::allbinary::game::combat::damage::DamageableInterface::DamageableInterface for UnitLayer {
    fn damage(&mut self, mut damage: i32, mut damageType: i32) { Self::damage(self, damage, damageType) }
    fn getDamage(&mut self, mut damageType: i32) -> i32 { Self::getDamage(self, damageType) }
}
impl crate::org::allbinary::game::identification::GroupInterfaceCompositeInterface::GroupInterfaceCompositeInterface for UnitLayer {
    fn getGroupInterface(&mut self) -> Vec<Option<Rc<RefCell<crate::org::allbinary::game::identification::Group::Group>>>> { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::getGroupInterface(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
}
impl crate::org::allbinary::game::layer::special::SpecialGameInputInterface::SpecialGameInputInterface for UnitLayer {
    fn up(&mut self) { Self::up(self) }
    fn down(&mut self) { Self::down(self) }
    fn right(&mut self) { Self::right(self) }
    fn left(&mut self) { Self::left(self) }
    fn strafeLeft(&mut self) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::strafeLeft(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
    fn strafeRight(&mut self) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::strafeRight(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
    fn fire(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { Self::fire(self, layerManager, gameKeyEvent) }
    fn special1(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, p2: Option<Rc<RefCell<crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::special1(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1, p2) }
    fn special2(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, p2: Option<Rc<RefCell<crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::special2(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1, p2) }
    fn special3(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, p2: Option<Rc<RefCell<crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::special3(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1, p2) }
    fn special4(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, p2: Option<Rc<RefCell<crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::special4(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1, p2) }
    fn special5(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, p2: Option<Rc<RefCell<crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::special5(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1, p2) }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for UnitLayer {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::paintThreed(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for UnitLayer {
    fn onEvent(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::onEvent(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::combat::destroy::event::DestroyedEventListenerInterface::DestroyedEventListenerInterface for UnitLayer {
    fn onDestroyed(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::game::combat::destroy::event::DestroyedEvent::DestroyedEvent>>>) { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::onDestroyed(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::combat::destroy::DestroyableInterface::DestroyableInterface for UnitLayer {
    fn isDestroyed(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::isDestroyed(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
}
impl crate::org::allbinary::game::tick::TickableInterface::TickableInterface for UnitLayer {
    fn processTick(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::processTick(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self), p1) }
}
impl crate::org::allbinary::view::event::ViewPositionEventListenerInterface::ViewPositionEventListenerInterface for UnitLayer {
    fn onChangeEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::view::event::ViewPositionEvent::ViewPositionEvent>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::onChangeEvent(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
}

impl UnitLayer {
    

    pub fn new_struct(mut remoteInfo: Option<Rc<RefCell<RemoteInfo>>>, mut parentLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut advancedRTSProperties: Option<Rc<RefCell<AdvancedRTSProperties>>>, mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>, mut rootName: String, mut name: String, mut vehicleProperties: Option<Rc<RefCell<VehicleProperties>>>, mut healthInterface: Option<Rc<RefCell<Health>>>, mut maxResourceLoad: Option<i32>, mut moveSoundInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>, mut waypointLayerInterfaceFactoryInterface: Option<Rc<RefCell<dyn LayerInterfaceFactoryInterface>>>, mut animationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut emptyAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut baseAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut buildAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut verticleBuildAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut decalAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut resourceAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut proceduralAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn ProceduralAnimationInterfaceFactoryInterface>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut direction: Option<Rc<RefCell<Direction>>>, mut x: i32, mut y: i32, mut z: i32, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Self {
        let mut this = Self {
            base: AdvancedRTSGameLayer::new_struct(remoteInfo.clone(), parentLayer.clone(), advancedRTSProperties.clone(), groupInterface.clone(), rootName.clone(), name.clone(), healthInterface.clone(), WaypointRTSFormInput::new(groupInterface.clone(), true).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn>>), animationInterfaceFactoryInterface.clone(), emptyAnimationInterfaceFactoryInterface.clone(), baseAnimationInterfaceFactoryInterface.clone(), buildAnimationInterfaceFactoryInterface.clone(), verticleBuildAnimationInterfaceFactoryInterface.clone(), proceduralAnimationInterfaceFactoryInterface.clone(), rectangle.clone(), x.clone(), y.clone(), viewPosition.clone()),
            basicColorFactory: None,
            groupCommonFactory: None,
            layerPartialCellPositionsUtil: None,
            layerDistanceUtil: None,
            angleFactory: None,
            cellPositionFactory: None,
            maxResourceLoad: 0,
            vehicleProperties: None,
            accelerationBasicDecimal: None,
            decelerationBasicDecimal: None,
            trackingEvent: None,
            damageFloaters: None,
            damageFloatersPaintableInterface: None,
            healthBar: None,
            moveSoundInterface: None,
            initResourceAnimation: None,
            resourceAnimation: None,
            initPathAnimation: None,
            pathAnimation: None,
            captionAnimationHelper: None,
            decalAnimation: None,
            waypointLayerInterfaceFactoryInterface: None,
            resourceLoad: 0,
            weaponRange: 0,
            rtsLogHelper: None,
            rotationAnimationInterfaceP: None,
            movementAngle: None,
            steeringInsideGeographicMapCellPosition: None,
            sensorGeographicMapCellPositionList: None,
            CAPITAL_EVENT: None,
        };
        this.basicColorFactory = BasicColorFactory::getInstance();
        this.groupCommonFactory = GroupCommonFactory::getInstance();
        this.layerPartialCellPositionsUtil = LayerPartialCellPositionsUtil::getInstance();
        this.layerDistanceUtil = LayerDistanceUtil::getInstance();
        this.angleFactory = AngleFactory::getInstance();
        this.cellPositionFactory = CellPositionFactory::getInstance();
        this.accelerationBasicDecimal = BasicDecimal::new(1600);
        this.decelerationBasicDecimal = BasicDecimal::new(-1000);
        this.resourceAnimation = (|| -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>> { panic!("ClassCastException: a Rust dyn Animation cannot be cast to IndexedAnimation") })();
        this.pathAnimation = jcall!(NullAnimationFactory::getFactoryInstance()).getInstance(0);
        this.captionAnimationHelper = CaptionAnimationHelper::new(jcall!(jcall!(FeaturedAnimationInterfaceFactoryInterfaceFactory::getInstance()).get({ let java_field = jcall!(CaptionResources::getInstance()).RESOURCE.clone(); java_field }.clone())).getInstance(0), -23, -25, 6, 0).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::caption::CaptionAnimationHelperBase::CaptionAnimationHelperBaseDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::caption::CaptionAnimationHelperBase::CaptionAnimationHelperBaseDyn>>);
        this.resourceLoad = 0;
        this.rtsLogHelper = RTSLayerLogHelper::getInstance();
        this.movementAngle = { let java_field = jcall!(this.angleFactory).NOT_ANGLE.clone(); java_field }.clone();
        this.steeringInsideGeographicMapCellPosition = crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone).clone();
        this.sensorGeographicMapCellPositionList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        // super(...) is in the struct literal above
        this.setCollidableInferface(CollidableUnitBehavior::new(true).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>));
        this.waypointLayerInterfaceFactoryInterface = waypointLayerInterfaceFactoryInterface.clone();
        this.maxResourceLoad = { let java_value = (jcall!(maxResourceLoad).shortValue()).clone(); java_value };
        this.moveSoundInterface = moveSoundInterface.clone();
        this
    }

    pub fn new(mut remoteInfo: Option<Rc<RefCell<RemoteInfo>>>, mut parentLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut advancedRTSProperties: Option<Rc<RefCell<AdvancedRTSProperties>>>, mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>, mut rootName: String, mut name: String, mut vehicleProperties: Option<Rc<RefCell<VehicleProperties>>>, mut healthInterface: Option<Rc<RefCell<Health>>>, mut maxResourceLoad: Option<i32>, mut moveSoundInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>, mut waypointLayerInterfaceFactoryInterface: Option<Rc<RefCell<dyn LayerInterfaceFactoryInterface>>>, mut animationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut emptyAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut baseAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut buildAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut verticleBuildAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut decalAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut resourceAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut proceduralAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn ProceduralAnimationInterfaceFactoryInterface>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut direction: Option<Rc<RefCell<Direction>>>, mut x: i32, mut y: i32, mut z: i32, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(remoteInfo.clone(), parentLayer.clone(), advancedRTSProperties.clone(), groupInterface.clone(), rootName.clone(), name.clone(), vehicleProperties.clone(), healthInterface.clone(), maxResourceLoad.clone(), moveSoundInterface.clone(), waypointLayerInterfaceFactoryInterface.clone(), animationInterfaceFactoryInterface.clone(), emptyAnimationInterfaceFactoryInterface.clone(), baseAnimationInterfaceFactoryInterface.clone(), buildAnimationInterfaceFactoryInterface.clone(), verticleBuildAnimationInterfaceFactoryInterface.clone(), decalAnimationInterfaceFactoryInterface.clone(), resourceAnimationInterfaceFactoryInterface.clone(), proceduralAnimationInterfaceFactoryInterface.clone(), rectangle.clone(), direction.clone(), x.clone(), y.clone(), z.clone(), viewPosition.clone()))));
        jcall!(this).CAPITAL_EVENT = CapitalEvent::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
            if jcall!(Features::getInstance()).isFeature({ let java_field = jcall!(GameFeatureFactory::getInstance()).DAMAGE_FLOATERS.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            jcall!(this).damageFloaters = PtsDamageFloaters::new(this.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::combat::damage::DamageFloaters::DamageFloatersDyn>>);
            jcall!(this).damageFloatersPaintableInterface = jcall!(this).damageFloaters.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>);
        } else {
            jcall!(this).damageFloatersPaintableInterface = { let java_value = (NullPaintable::getInstance()).clone(); java_value };
            jcall!(this).damageFloaters = DamageFloaters::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::combat::damage::DamageFloaters::DamageFloatersDyn>>);
        }
            if jcall!(Features::getInstance()).isFeature({ let java_field = jcall!(GameFeatureFactory::getInstance()).HEALTH_BARS.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            jcall!(this).healthBar = { let java_value = (HealthBar::new(this.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>), jcall!(this).getHealthInterface(), HealthBarTwodAnimation::new(this.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>), { let java_field = jcall!(BasicHudFactory::getInstance()).BOTTOMLEFT.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::health::HealthBarAnimation::HealthBarAnimationDyn>>), -1).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>)).clone(); java_value };
        } else {
            jcall!(this).healthBar = { let java_value = (NullPaintable::getInstance()).clone(); java_value };
        }
            jcall!(this).decalAnimation = { let java_value = ((|| -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>> { panic!("ClassCastException: a Rust dyn Animation cannot be cast to RotationAnimation") })()).clone(); java_value };
            jcall!(this).initResourceAnimation = { let java_value = ((|| -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>> { panic!("ClassCastException: a Rust dyn Animation cannot be cast to RotationAnimation") })()).clone(); java_value };
            jcall!(jcall!(this).initResourceAnimation).setFrameByDirection(direction.clone());
            jcall!(jcall!(this).decalAnimation).setFrameByDirection(direction.clone());
            jcall!(this).rotationAnimationInterfaceP = (|| -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>> { panic!("ClassCastException: a Rust dyn IndexedAnimation cannot be cast to RotationAnimation") })();
            jcall!(jcall!(this).rotationAnimationInterfaceP).setFrameByDirection(direction.clone());
            jcall!(this).setMaxLevel(12);
            jcall!(this).vehicleProperties = vehicleProperties.clone();
            jcall!(this).trackingEvent = TrackingEvent::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
            jcall!(this).initPathAnimation = { let java_value = (PathAnimation::new(this.clone().map(|reference| reference as Rc<RefCell<dyn PathFindingLayerInterface>>), LinePathRelativeAnimation::getInstance())).clone(); java_value };
        this
    }
    

    // throws Exception
    pub fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) {
        { let java_arguments = (allBinaryGameLayerManager.clone(),); self.base.setAllBinaryGameLayerManager(java_arguments.0) };
        jcall!(self.initPathAnimation).setAllBinaryGameLayerManager(allBinaryGameLayerManager.clone());
    }
    

    // throws Exception
    pub fn updateWaypointBehavior(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) {
        let mut hashtable: Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashtable();
        jcall!(hashtable).put(crate::java_into_object({ let java_field = jcall!(self.groupCommonFactory).ID.clone(); java_field }.clone()), crate::java_into_object(self.getGroupInterface()));
        jcall!(hashtable).put(crate::java_into_object(crate::org::allbinary::layer::Layer::Layer_ID.with(Clone::clone).clone()), None /* Java this: a &mut self method has no Rc of itself */);
        jcall!(hashtable).put(crate::java_into_object(crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager_ID.with(Clone::clone).clone()), crate::java_into_object(self.allBinaryGameLayerManagerP.clone()));
        { let java_arguments = (UnitWaypointBehavior2::new(None /* Java this: a &mut self method has no Rc of itself */, (|| -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>> { panic!("ClassCastException: a Rust dyn AllBinaryLayer cannot be cast to AdvancedRTSGameLayer") })()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>),); self.setWaypointBehavior(java_arguments.0) };
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        let mut waypoint: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointBase::WaypointBaseDyn>>> = (if J2MEUtil::isHTML() { MultipassNoCacheWaypoint::new(None /* Java this: a &mut self method has no Rc of itself */, AttackSound::getInstance()) } else { NoCacheWaypoint::new(None /* Java this: a &mut self method has no Rc of itself */, AttackSound::getInstance()) });
        jcall!(self.getWaypointBehavior()).setWaypoint(waypoint.clone());
        { let java_arguments = (geographicMapInterface.clone(),); self.base.updateWaypointBehavior(java_arguments.0) };
        self.initRangeHack();
    }
    

    // throws Exception
    pub fn construct(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>) {
        { let java_arguments = (rtsPlayerLayerInterface.clone(),); self.base.construct(java_arguments.0) };
        jcall!(TrackingEventHandler::getInstance()).addListenerInterface(None /* Java this: a &mut self method has no Rc of itself */);
        jcall!(WaypointEventHandlerFactory::getInstance(self.getGroupInterface()[(0) as usize].clone())).addListenerInterface(self.getUnitWaypointBehavior().map(|_java_reference| -> Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>> { panic!("ClassCastException: the Rust trait of UnitWaypointBehavior does not have EventListenerInterface as a base trait") }));
        jcall!(BuildingEventHandler::getInstance()).addListenerInterface(None /* Java this: a &mut self method has no Rc of itself */);
        jcall!(self.getUnitWaypointBehavior()).setCurrentPathGeographicMapCellPosition(self.getCurrentGeographicMapCellPosition());
        self.updateSensorGeographicMapCellPositionList();
        jcall!(self.getUnitWaypointBehavior()).setLastPathGeographicMapCellPosition({ let java_argument = jcall!(self.getUnitWaypointBehavior()).getCurrentPathGeographicMapCellPosition(); java_argument });
    }
    

    // throws Exception
    pub fn updateSensorGeographicMapCellPositionList(&mut self) {
        if jcall!(VisibleCellPositionsSingleton::getInstance()).shouldProcess() {
            let mut currentGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = self.getCurrentGeographicMapCellPosition();
            jcall!(self.sensorGeographicMapCellPositionList).clear();
            jcall!(self.sensorGeographicMapCellPositionList).add(crate::java_into_object(currentGeographicMapCellPosition.clone()));
            let mut sensorRange: i32 = self.weaponRange * UnitLayer_SENSOR_RANGE_MULTIPLIER.with(Clone::clone);
            let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (|| -> Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> { panic!("ClassCastException: a Rust dyn AllBinaryGameLayerManager cannot be cast to GeographicMapCompositeInterface") })();
            let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
            let mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(geographicMapInterface).getAllBinaryTiledLayer();
            let mut totalCells: i32 = (sensorRange / jcall!(tiledLayer).getCellHeight()) / 2;
            let mut geographicMapCellPositionFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>> = jcall!(geographicMapInterface).getGeographicMapCellPositionFactory();
            let mut column: i32 = jcall!(currentGeographicMapCellPosition).getColumn();
            let mut row: i32 = jcall!(currentGeographicMapCellPosition).getRow();
            let mut lastColumn: i32 = column + totalCells;
            let mut lastRow: i32 = row + totalCells;
            if lastColumn > jcall!(tiledLayer).getColumns() {
                lastColumn = { let java_value = (jcall!(tiledLayer).getColumns()).clone(); java_value };
            }
            if lastRow > jcall!(tiledLayer).getRows() {
                lastRow = { let java_value = (jcall!(tiledLayer).getRows()).clone(); java_value };
            }
            let mut firstColumn: i32 = column - totalCells;
            let mut firstRow: i32 = row - totalCells;
            if firstColumn < 0 {
                firstColumn = 0;
            }
            if firstRow < 0 {
                firstRow = 0;
            }
            for index in (firstColumn..=lastColumn - 1).rev() {
                for index2 in (firstRow..=lastRow - 1).rev() {
                    let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(geographicMapCellPositionFactory).getAt(index.clone(), index2.clone());
                    if !jcall!(self.sensorGeographicMapCellPositionList).contains(crate::java_into_object(geographicMapCellPosition.clone())) {
                        jcall!(self.sensorGeographicMapCellPositionList).add(crate::java_into_object(geographicMapCellPosition.clone()));
                    }
                }
            }
        }
    }
    

    pub fn getSensorGeographicMapCellPositionList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.sensorGeographicMapCellPositionList.clone();
    }
    

    pub fn select(&mut self) {
        self.pathAnimation = self.initPathAnimation.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
        self.base.select();
    }
    

    pub fn deselect(&mut self) {
        self.pathAnimation = { let java_value = (jcall!(NullAnimationFactory::getFactoryInstance()).getInstance(0)).clone(); java_value };
        self.base.deselect();
    }
    

    pub fn setSelected(&mut self, mut selected: bool) {
        { let java_arguments = (selected.clone(),); self.base.setSelected(java_arguments.0) };
        if selected {
            if self.debug {
                self.rtsLogHelper = { let java_value = (RTSLayerSelectedLogHelper::getInstance()).clone(); java_value }.map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayerLogHelper::RTSLayerLogHelperDyn>>);
            }
            self.rtsLayer2LogHelper = { let java_value = (RTSLayer2SelectedLogHelper::getInstance()).clone(); java_value }.map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer2LogHelper::RTSLayer2LogHelperDyn>>);
        } else {
            if self.debug {
                self.rtsLogHelper = { let java_value = (RTSLayerLogHelper::getInstance()).clone(); java_value };
            }
            self.rtsLayer2LogHelper = { let java_value = (RTSLayer2LogHelper::getInstance()).clone(); java_value };
        }
    }
    

    // throws Exception
    pub fn setClosestGeographicMapCellHistory(&mut self, mut pathsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        jcall!(self.rtsLogHelper).setClosestGeographicMapCellHistory(None /* Java this: a &mut self method has no Rc of itself */, pathsList.clone());
        let mut closestIndex: i32 = -1;
        let mut shortestDistance: i32 = i32::MAX.clone();
        let mut currentDistance: i32 = i32::MAX.clone();
        for index in (0..={ let java_bound = jcall!(pathsList).size() - 1; java_bound }).rev() {
            let mut geographicMapCellPositionBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_object = (jcall!(pathsList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) };
            let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = { let java_object = (jcall!(geographicMapCellPositionBasicArrayList).get(jcall!(geographicMapCellPositionBasicArrayList).size() - 1)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) };
            currentDistance = { let java_value = ({ let java_receiver = self.layerDistanceUtil.clone(); let java_arguments = (None /* Java this: a &mut self method has no Rc of itself */, jcall!(geographicMapCellPosition).getMidPoint(),); let java_result = jcall!(java_receiver).getDistanceAt(java_arguments.0, java_arguments.1); java_result }).clone(); java_value };
            if currentDistance < shortestDistance {
                shortestDistance = currentDistance.clone();
                closestIndex = index.clone();
            }
        }
        if closestIndex >= 0 {
            let mut geographicMapCellPositionBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_object = (jcall!(pathsList).get(closestIndex.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) };
            let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = { let java_object = (jcall!(geographicMapCellPositionBasicArrayList).get(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) };
            self.teleportTo(geographicMapCellPosition.clone());
        }
    }
    

    // throws Exception
    pub fn init(&mut self, mut geographicMapCellHistory: Option<Rc<RefCell<GeographicMapCellHistory>>>, mut geographicMapCellPositionBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        jcall!(geographicMapCellHistory).trackAll(geographicMapCellPositionBasicArrayList.clone());
    }
    

    pub fn onMovement(&mut self, mut trackingEvent: Option<Rc<RefCell<TrackingEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>> = (|| -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>> { panic!("ClassCastException: a Rust dyn AllBinaryLayer cannot be cast to AdvancedRTSGameLayer") })();
            if !match (&jcall!(layerInterface).getGroupInterface()[(0) as usize], &self.getGroupInterface()[(0) as usize]) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(jcall!(self.getUnitWaypointBehavior()).getPossibleTargetList()).add(crate::java_into_object(layerInterface.clone()));
                jcall!(layerInterface).onMovementFound(self.getTrackingEvent());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onMovement"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn onMovementFound(&mut self, mut trackingEvent: Option<Rc<RefCell<TrackingEvent>>>) {
        let mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>> = (|| -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>> { panic!("ClassCastException: a Rust dyn AllBinaryLayer cannot be cast to AdvancedRTSGameLayer") })();
        if !match (&jcall!(layerInterface).getGroupInterface()[(0) as usize], &self.getGroupInterface()[(0) as usize]) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(jcall!(self.getUnitWaypointBehavior()).getPossibleTargetList()).add(crate::java_into_object(layerInterface.clone()));
        }
    }
    

    pub fn initRangeHack(&mut self) {
        let mut basicWeaponPart: Option<Rc<RefCell<dyn crate::org::allbinary::game::part::weapon::BasicWeaponPart::BasicWeaponPartDyn>>> = (|| -> Option<Rc<RefCell<dyn crate::org::allbinary::game::part::weapon::BasicWeaponPart::BasicWeaponPartDyn>>> { panic!("ClassCastException: a Rust dyn PartInterface cannot be cast to BasicWeaponPart") })();
        let mut weaponProperties: Option<Rc<RefCell<WeaponProperties>>> = jcall!(basicWeaponPart).getWeaponProperties();
        self.weaponRange = { let java_value = (jcall!(weaponProperties).getRange()).clone(); java_value };
        self.initRangeAnimation = { let java_value = (AdjustedCircleAnimation::createW(self.weaponRange.clone(), self.weaponRange.clone(), self.getWidth(), { let java_field = jcall!(self.basicColorFactory).GREEN.clone(); java_field }.clone())).clone(); java_value }.map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
        let mut sensorRange: i32 = self.weaponRange * UnitLayer_SENSOR_RANGE_MULTIPLIER.with(Clone::clone);
        self.initSensorRangeAnimation = { let java_value = (AdjustedCircleAnimation::createW(sensorRange.clone(), sensorRange.clone(), self.getWidth(), { let java_field = jcall!(self.basicColorFactory).RED.clone(); java_field }.clone())).clone(); java_value }.map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
        jcall!(self.getUnitWaypointBehavior()).initRange(self.weaponRange.clone());
        jcall!(self.fireTimeHelper).as_TimeDelayHelper_mut().delay = (((jcall!(weaponProperties).getReloadTime()) as i32));
    }
    

    // throws Exception
    pub fn processBuiltTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        if !jcall!(self.getHealthInterface()).isAlive() {
            if self.isReadyForExplosion() {
                let mut currentFrame: i32 = jcall!(self.destroyAnimationInterface).getFrame();
                let mut size: i32 = jcall!(self.destroyAnimationInterface).getSize() - 1;
                if currentFrame == size {
                    if !jcall!(self.getHealthInterface()).isAlive() {
                        self.setDestroyed(true);
                    }
                } else {
                    jcall!(self.destroyAnimationInterface).nextFrame();
                }
            } else {
                { let java_arguments = (self.destroyAnimationInterface.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>),); self.setAnimationInterface(java_arguments.0) };
                jcall!(SecondaryPlayerQueueFactory::getInstance()).add(ExplosionBasicSound::getInstance());
                jcall!(self.shakeListener).onSmallShakeEvent();
                jcall!(self.vibration).vibrate(self.duration.clone(), 0, 0);
                self.setReadyForExplosion(true);
            }
        } else {
            { let java_arguments = (allBinaryLayerManager.clone(),); self.base.processBuiltTick(java_arguments.0) };
        }
        jcall!(self.captionAnimationHelper).tick();
        jcall!(self.getUnitWaypointBehavior()).processTick(allBinaryLayerManager.clone());
    }
    

    pub fn teleportTo(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) {
        let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(geographicMapCellPosition).getMidPoint();
        { let java_arguments = ({ let java_value = jcall!(point).getX(); java_value } - self.getHalfWidth(), { let java_value = jcall!(point).getY(); java_value } - self.getHalfHeight(), self.z.clone(),); self.setPosition(java_arguments.0, java_arguments.1, java_arguments.2) };
    }
    

    // throws Exception
    pub fn getCurrentGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (|| -> Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> { panic!("ClassCastException: a Rust dyn AllBinaryGameLayerManager cannot be cast to GeographicMapCompositeInterface") })();
        let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
        let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(geographicMapInterface).getCellPositionAtXY(self.x + self.getHalfWidth(), self.y + self.getHalfHeight());
        let mut raceTrackGeographicMap: Option<Rc<RefCell<RaceTrackGeographicMap>>> = (geographicMapInterface).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<RaceTrackGeographicMap>() }, "ClassCastException: not a RaceTrackGeographicMap"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<RaceTrackGeographicMap>) } });
        if !jcall!(raceTrackGeographicMap).isValid(geographicMapCellPosition.clone()) {
            panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("Position is not really on the map: "), geographicMapCellPosition))));
        }
        return geographicMapCellPosition.clone();
    }
    

    // throws Exception
    pub fn fire(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
        if jcall!(self.fireTimeHelper).isTimeTNT() {
            self.fireAll(layerManager.clone());
        } else {
            self.reload();
        }
    }
    

    // throws Exception
    pub fn left(&mut self) {
        jcall!(self.initResourceAnimation).previousRotation();
        jcall!(self.decalAnimation).previousRotation();
        jcall!(self.rotationAnimationInterfaceP).previousRotation();
    }
    

    // throws Exception
    pub fn right(&mut self) {
        jcall!(self.initResourceAnimation).nextRotation();
        jcall!(self.decalAnimation).nextRotation();
        jcall!(self.rotationAnimationInterfaceP).nextRotation();
    }
    

    pub fn down(&mut self) {
        let mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>> = jcall!(self.getVehicleProperties()).getVelocityProperties();
        if !jcall!(velocityProperties).isOverXYMaxForwardVelocity() {
            { let java_arguments = (self.decelerationBasicDecimal.clone(),); self.accelerate(java_arguments.0) };
        }
    }
    

    pub fn up(&mut self) {
        let mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>> = jcall!(self.getVehicleProperties()).getVelocityProperties();
        if !jcall!(velocityProperties).isOverXYMaxForwardVelocity() {
            { let java_arguments = (self.accelerationBasicDecimal.clone(),); self.accelerate(java_arguments.0) };
        }
    }
    

    pub fn initInputProcessors(&mut self) {
        { let java_index = (crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.with(Clone::clone)) as usize; self.inputProcessorArray[java_index] = SpecialRightGameInputProcessor::new(None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn>>); };
        { let java_index = (crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.with(Clone::clone)) as usize; self.inputProcessorArray[java_index] = SpecialLeftGameInputProcessor::new(None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn>>); };
        { let java_index = (crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM0.with(Clone::clone)) as usize; self.inputProcessorArray[java_index] = SpecialFireGameInputProcessor::new(None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn>>); };
        { let java_index = (crate::javax::microedition::lcdui::Canvas::Canvas_KEY_POUND.with(Clone::clone)) as usize; self.inputProcessorArray[java_index] = { let java_value = (self.inputProcessorArray[(crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM0.with(Clone::clone)) as usize]).clone(); java_value }.clone(); };
        { let java_index = (crate::javax::microedition::lcdui::Canvas::Canvas_DOWN.with(Clone::clone)) as usize; self.inputProcessorArray[java_index] = SpecialDownGameInputProcessor::new(None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn>>); };
        { let java_index = (crate::javax::microedition::lcdui::Canvas::Canvas_UP.with(Clone::clone)) as usize; self.inputProcessorArray[java_index] = SpecialUpGameInputProcessor::new(None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn>>); };
        self.base.initInputProcessors();
    }
    

    // throws Exception
    pub fn processInput(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getGameKeyEventList();
        let mut size: i32 = jcall!(list).size();
        for index in (0..size) {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(list).get(index.clone());
            let mut key: i32 = GameKeyEventUtil::getKey(crate::java_into_object(anyType.clone()));
            jcall!(self.inputProcessorArray[(key) as usize]).processEvent(layerManager.clone(), crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent_NONE.with(Clone::clone).clone());
        }
        jcall!(list).clear();
        self.groundFriction();
        self.r#move();
    }
    

    pub fn accelerate(&mut self, mut accelerate: Option<Rc<RefCell<BasicDecimal>>>) {
        jcall!(jcall!(self.getVehicleProperties()).getVelocityProperties()).addVelocityi(jcall!(accelerate).getUnscaled(), ((jcall!(jcall!(self.rotationAnimationInterfaceP).getAngleInfoP()).getAngle()) as i32), 90);
    }
    

    // throws Exception
    pub fn fireAll(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        let mut angleInfo: Option<Rc<RefCell<AngleInfo>>> = jcall!(self.rotationAnimationInterfaceP).getAngleInfoP();
        let mut angle: i32 = (((jcall!(angleInfo).getAngle()) as i32) + self.slightAngle);
        { let java_receiver = self.hashtable.clone(); let java_arguments = (crate::java_into_object(jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(1)), crate::java_into_object(jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(((jcall!(jcall!(AngleFactory::getInstance()).getAt(angle.clone())).getValue()) as i32))),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1); };
        let mut salvoInterface: Option<Rc<RefCell<dyn SalvoInterface>>> = (|| -> Option<Rc<RefCell<dyn SalvoInterface>>> { panic!("ClassCastException: a Rust dyn PartInterface cannot be cast to SalvoInterface") })();
        jcall!(salvoInterface).process(layerManager.clone(), ((angle) as i16), ((90) as i16));
    }
    

    pub fn downgrade(&mut self) {
        if self.getLevel() > 1 {
            self.base.downgrade();
        }
    }
    

    pub fn upgrade(&mut self) {
        self.base.upgrade();
        self.initRangeHack();
    }
    

    pub fn groundFriction(&mut self) {
        let mut vehicleFrictionProperties: Option<Rc<RefCell<VehicleFrictionProperties>>> = jcall!(self.getVehicleProperties()).getVehicleFrictionProperties();
        jcall!(jcall!(self.getVehicleProperties()).getVehicleFrictionProperties()).friction({ let java_argument = jcall!(self.getVehicleProperties()).getVelocityProperties(); java_argument }, { let java_argument = jcall!(vehicleFrictionProperties).getTireFrictionNominator(); java_argument });
    }
    

    // throws Exception
    pub fn trackTo(&mut self, mut reason: String) {
        let mut waypointBehaviorBase: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>> = (self.waypointBehaviorBase).clone();
        let mut nextUnvisitedPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(waypointBehaviorBase).getNextUnvisitedPathGeographicMapCellPosition();
        let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(nextUnvisitedPathGeographicMapCellPosition).getMidPoint();
        let mut dx: i32 = (self.getXP() + self.getHalfWidth()) - { let java_value = jcall!(point).getX(); java_value };
        let mut dy: i32 = (self.getYP() + self.getHalfHeight()) - { let java_value = jcall!(point).getY(); java_value };
        jcall!(self.rtsLogHelper).trackTo(None /* Java this: a &mut self method has no Rc of itself */, nextUnvisitedPathGeographicMapCellPosition.clone(), dx.clone(), dy.clone(), reason.clone());
        self.trackToDXY(dx.clone(), dy.clone());
    }
    

    // throws Exception
    pub fn trackToDXY(&mut self, mut dx: i32, mut dy: i32) {
        let mut angleOfTarget: i32 = 0;
        self.trackToDXYTargetAngle(dx.clone(), dy.clone(), angleOfTarget.clone());
    }
    

    // throws Exception
    pub fn turnTo(&mut self, mut dx: i32, mut dy: i32, mut targetAngle: i32) -> bool {
        let mut waypointBehaviorBase: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>> = (self.waypointBehaviorBase).clone();
        let mut nextUnvisitedPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(waypointBehaviorBase).getNextUnvisitedPathGeographicMapCellPosition();
        let mut evading: bool = false;
        if match (&{ let java_value = jcall!(self.getUnitWaypointBehavior()).getSensorAction(); java_value }, &{ let java_field = jcall!(SensorActionFactory::getInstance()).EVADE.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(self.rtsLogHelper).evade(None /* Java this: a &mut self method has no Rc of itself */);
            evading = true;
            targetAngle += 180;
        }
        let mut angleInfo: Option<Rc<RefCell<AngleInfo>>> = jcall!(self.rotationAnimationInterfaceP).getAngleInfoP();
        let mut angle: i32 = jcall!(FrameUtil::getInstance()).adjustAngleToFrameAngle(((jcall!(angleInfo).getAngle()) as i32) - 270);
        jcall!(self.rtsLogHelper).turnTo(None /* Java this: a &mut self method has no Rc of itself */, dx.clone(), dy.clone(), angleInfo.clone(), angle.clone(), self.movementAngle.clone(), evading.clone(), targetAngle.clone());
        let mut gameKeyEventFactory: Option<Rc<RefCell<GameKeyEventFactory>>> = GameKeyEventFactory::getInstance();
        if dx == 0 && dy == 0 {
            jcall!(self.rtsLogHelper).doneMoving(None /* Java this: a &mut self method has no Rc of itself */);
            return true;
        } else if (((jcall!(self.movementAngle).getValue()) as i32)) == angle {
            if dx > 0 && match (&self.movementAngle, &{ let java_field = jcall!(self.angleFactory).LEFT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(self.rtsLogHelper).movingLeft(None /* Java this: a &mut self method has no Rc of itself */);
                return false;
            }
            if dx < 0 && match (&self.movementAngle, &{ let java_field = jcall!(self.angleFactory).RIGHT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(self.rtsLogHelper).movingRight(None /* Java this: a &mut self method has no Rc of itself */);
                return false;
            }
            if dy > 0 && match (&self.movementAngle, &{ let java_field = jcall!(self.angleFactory).UP.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(self.rtsLogHelper).movingUp(None /* Java this: a &mut self method has no Rc of itself */);
                return false;
            }
            if dy < 0 && match (&self.movementAngle, &{ let java_field = jcall!(self.angleFactory).DOWN.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(self.rtsLogHelper).movingDown(None /* Java this: a &mut self method has no Rc of itself */);
                return false;
            }
            jcall!(self.rtsLogHelper).currentMoveEnded(None /* Java this: a &mut self method has no Rc of itself */);
            if match (&self.movementAngle, &{ let java_field = jcall!(self.angleFactory).LEFT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&self.movementAngle, &{ let java_field = jcall!(self.angleFactory).RIGHT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                self.handleDeltalY(dx.clone(), dy.clone());
            } else if match (&self.movementAngle, &{ let java_field = jcall!(self.angleFactory).UP.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&self.movementAngle, &{ let java_field = jcall!(self.angleFactory).DOWN.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                self.handleDeltalX(dx.clone(), dy.clone());
            }
            return true;
        } else {
            if !match (&nextUnvisitedPathGeographicMapCellPosition, &crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                if !match (&self.steeringInsideGeographicMapCellPosition, &nextUnvisitedPathGeographicMapCellPosition) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    if (dx.clone()).abs() > (dy.clone()).abs() && dy != 0 {
                        self.handleDeltalY(dx.clone(), dy.clone());
                    } else if dx != 0 {
                        self.handleDeltalX(dx.clone(), dy.clone());
                    } else {
                        self.handleDeltalY(dx.clone(), dy.clone());
                    }
                }
                let mut deltaAngle2: i32 = ((jcall!(self.movementAngle).getValue()) as i32) - angle;
                if deltaAngle2 > 0 {
                    jcall!(self.rtsLogHelper).rotateRight(None /* Java this: a &mut self method has no Rc of itself */);
                    jcall!(self.getGameKeyEventList()).add(crate::java_into_object(jcall!(gameKeyEventFactory).getInstanceForKey(None /* Java this: a &mut self method has no Rc of itself */, crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.with(Clone::clone).clone())));
                } else {
                    jcall!(self.rtsLogHelper).rotateLeft(None /* Java this: a &mut self method has no Rc of itself */);
                    jcall!(self.getGameKeyEventList()).add(crate::java_into_object(jcall!(gameKeyEventFactory).getInstanceForKey(None /* Java this: a &mut self method has no Rc of itself */, crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.with(Clone::clone).clone())));
                }
                return true;
            } else {
                jcall!(self.rtsLogHelper).noRotation(None /* Java this: a &mut self method has no Rc of itself */);
            }
            return false;
        }
    }
    

    pub fn handleDeltalX(&mut self, mut dx: i32, mut dy: i32) {
        let mut waypointBehaviorBase: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>> = (self.waypointBehaviorBase).clone();
        let mut nextUnvisitedPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(waypointBehaviorBase).getNextUnvisitedPathGeographicMapCellPosition();
        if dx > 0 {
            self.movementAngle = { let java_value = ({ let java_field = jcall!(self.angleFactory).LEFT.clone(); java_field }).clone(); java_value }.clone();
            self.steeringInsideGeographicMapCellPosition = nextUnvisitedPathGeographicMapCellPosition.clone();
        } else {
            self.movementAngle = { let java_value = ({ let java_field = jcall!(self.angleFactory).RIGHT.clone(); java_field }).clone(); java_value }.clone();
            self.steeringInsideGeographicMapCellPosition = nextUnvisitedPathGeographicMapCellPosition.clone();
        }
        jcall!(self.rtsLogHelper).handle(None /* Java this: a &mut self method has no Rc of itself */, self.movementAngle.clone());
    }
    

    pub fn handleDeltalY(&mut self, mut dx: i32, mut dy: i32) {
        let mut waypointBehaviorBase: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>> = (self.waypointBehaviorBase).clone();
        let mut nextUnvisitedPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(waypointBehaviorBase).getNextUnvisitedPathGeographicMapCellPosition();
        if dy > 0 {
            self.movementAngle = { let java_value = ({ let java_field = jcall!(self.angleFactory).UP.clone(); java_field }).clone(); java_value }.clone();
            self.steeringInsideGeographicMapCellPosition = nextUnvisitedPathGeographicMapCellPosition.clone();
        } else {
            self.movementAngle = { let java_value = ({ let java_field = jcall!(self.angleFactory).DOWN.clone(); java_field }).clone(); java_value }.clone();
            self.steeringInsideGeographicMapCellPosition = nextUnvisitedPathGeographicMapCellPosition.clone();
        }
        jcall!(self.rtsLogHelper).handle(None /* Java this: a &mut self method has no Rc of itself */, self.movementAngle.clone());
    }
    

    // throws Exception
    pub fn trackToDXYTargetAngle(&mut self, mut dx: i32, mut dy: i32, mut targetAngle: i32) {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(self.getUnitWaypointBehavior()).getSteeringVisitorList();
        if jcall!(list).size() > 0 {
            for index in (0..={ let java_bound = jcall!(list).size() - 1; java_bound }).rev() {
                let mut steeringVisitor: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::SteeringVisitor::SteeringVisitorDyn>>> = { let java_object = (jcall!(list).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::SteeringVisitor::SteeringVisitorDyn>>>>().expect("ClassCastException").clone()) };
                let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(steeringVisitor).as_SteeringVisitor_mut().visit(None /* Java this: a &mut self method has no Rc of itself */);
                if anyType.is_none() {
                    jcall!(list).removeAt(index.clone());
                }
            }
            self.fireOrMove();
        } else if !self.turnTo(dx.clone(), dy.clone(), targetAngle.clone()) {
            self.fireOrMove();
        }
    }
    

    // throws Exception
    pub fn fireOrMove(&mut self) {
        let mut gameKeyEventFactory: Option<Rc<RefCell<GameKeyEventFactory>>> = GameKeyEventFactory::getInstance();
        if jcall!(self.getUnitWaypointBehavior()).needToMove() {
            jcall!(self.rtsLayer2LogHelper).steeringUp(None /* Java this: a &mut self method has no Rc of itself */);
            if self.showMoreCaptionStates && !jcall!(self.captionAnimationHelper).isShowing() {
                jcall!(self.captionAnimationHelper).update(UnitLayer_MOVE.with(Clone::clone).clone(), { let java_field = jcall!(self.basicColorFactory).GREEN.clone(); java_field }.clone());
            }
            jcall!(self.getGameKeyEventList()).add(crate::java_into_object(jcall!(gameKeyEventFactory).getInstanceForKey(None /* Java this: a &mut self method has no Rc of itself */, crate::javax::microedition::lcdui::Canvas::Canvas_UP.with(Clone::clone).clone())));
        } else {
            { let java_receiver = self.captionAnimationHelper.clone(); let java_arguments = ({ let java_field = jcall!(CommonPhoneStrings::getInstance()).FIRE.clone(); java_field }.clone(), { let java_field = jcall!(self.basicColorFactory).RED.clone(); java_field }.clone(),); jcall!(java_receiver).update(java_arguments.0, java_arguments.1); };
            jcall!(self.rtsLayer2LogHelper).steeringFireOrStop(None /* Java this: a &mut self method has no Rc of itself */);
            self.allStop();
            jcall!(self.getGameKeyEventList()).add(crate::java_into_object(jcall!(gameKeyEventFactory).getInstanceForKey(None /* Java this: a &mut self method has no Rc of itself */, crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM0.with(Clone::clone).clone())));
            jcall!(TrackingEventHandler::getInstance()).fireEvent(self.getTrackingEvent().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
        }
    }
    

    pub fn r#move(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>> = jcall!(self.getVehicleProperties()).getVelocityProperties();
            let mut velocityXScaled: i64 = ((jcall!(jcall!(velocityProperties).getVelocityXBasicDecimalP()).getScaled()) as i64);
            let mut velocityYScaled: i64 = ((jcall!(jcall!(velocityProperties).getVelocityYBasicDecimalP()).getScaled()) as i64);
            jcall!(self.getUnitWaypointBehavior()).r#move();
            if velocityXScaled != 0_i64 || velocityYScaled != 0_i64 {
                jcall!(self.getUnitWaypointBehavior()).setMoving(true);
                let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (|| -> Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> { panic!("ClassCastException: a Rust dyn AllBinaryGameLayerManager cannot be cast to GeographicMapCompositeInterface") })();
                let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
                { let java_receiver = self.layerPartialCellPositionsUtil.clone(); let java_arguments = (geographicMapInterface.clone(), None /* Java this: a &mut self method has no Rc of itself */, ((velocityXScaled) as i32), ((velocityYScaled) as i32), UnitLayer::getPartialpositionlist(),); jcall!(java_receiver).getAllDXY(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4); };
                let mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = (|| -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { panic!("ClassCastException: a Rust dyn CellPosition cannot be cast to GeographicMapCellPosition") })();
                if match (&cellPosition, &{ let java_field = jcall!(self.cellPositionFactory).NONE.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    let mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(geographicMapInterface).getAllBinaryTiledLayer();
                    let mut tiledLayerUtil: Option<Rc<RefCell<TiledLayerUtil>>> = TiledLayerUtil::getInstance();
                    let mut x: i32 = self.x + ((velocityXScaled) as i32);
                    let mut y: i32 = self.y + ((velocityYScaled) as i32);
                    x = { let java_value = (jcall!(tiledLayerUtil).keepOnMapX(tiledLayer.clone(), x.clone(), self.getWidth())).clone(); java_value };
                    y = { let java_value = (jcall!(tiledLayerUtil).keepOnMapY(tiledLayer.clone(), y.clone(), self.getHeight())).clone(); java_value };
                    { let java_arguments = (x.clone(), y.clone(), self.z.clone(),); self.setPosition(java_arguments.0, java_arguments.1, java_arguments.2) };
                } else {
                    let mut allbinaryLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = jcall!(DropCellPositionHistory::getInstance()).getLayerInterface(cellPosition.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>));
                    jcall!(self.getUnitWaypointBehavior()).setMovingFromStopped(false);
                    jcall!(self.getUnitWaypointBehavior()).addBuildingChase(allbinaryLayer.clone(), cellPosition.clone());
                }
            }
            if jcall!(self.getUnitWaypointBehavior()).isMoving() {
                jcall!(TrackingEventHandler::getInstance()).fireEvent(self.getTrackingEvent().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
            } else {
                jcall!(self.getUnitWaypointBehavior()).setMovingFromStopped(false);
            }
            if { let java_value = jcall!(self.getUnitWaypointBehavior()).isMovingFromStopped(); java_value } && self.isVisible() {
                jcall!(SecondaryPlayerQueueFactory::getInstance()).add(self.moveSoundInterface.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("move"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn allStop(&mut self) {
        let mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>> = jcall!(self.getVehicleProperties()).getVelocityProperties();
        jcall!(jcall!(velocityProperties).getVelocityXBasicDecimalP()).setint(0);
        jcall!(jcall!(velocityProperties).getVelocityYBasicDecimalP()).setint(0);
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        if self.isVisible() {
            { let java_arguments = (graphics.clone(),); self.base.paint(java_arguments.0) };
            let mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>> = self.getViewPosition();
            let mut viewX: i32 = jcall!(viewPosition).getX();
            let mut viewY: i32 = jcall!(viewPosition).getY();
            jcall!(self.decalAnimation).paintXY(graphics.clone(), viewX.clone(), viewY.clone());
            jcall!(self.rangeAnimation).paintXY(graphics.clone(), viewX.clone(), viewY.clone());
            jcall!(self.sensorRangeAnimation).paintXY(graphics.clone(), viewX.clone(), viewY.clone());
            jcall!(self.damageFloatersPaintableInterface).as_Paintable_mut().paint(graphics.clone());
            jcall!(self.healthBar).as_Paintable_mut().paint(graphics.clone());
            jcall!(self.captionAnimationHelper).paintXY(graphics.clone(), viewX.clone(), viewY.clone());
            jcall!(self.pathAnimation).paintXY(graphics.clone(), viewX.clone(), viewY.clone());
            jcall!(self.resourceAnimation).paintXY(graphics.clone(), viewX.clone(), viewY.clone());
        }
    }
    

    // throws Exception
    pub fn onBuildingEvent(&mut self, mut event: Option<Rc<RefCell<RTSLayerEvent>>>) {
        let mut buildingLayer: Option<Rc<RefCell<BuildingLayer>>> = (jcall!(event).getRtsLayer()).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<BuildingLayer>() }, "ClassCastException: not a BuildingLayer"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<BuildingLayer>) } });
        jcall!(self.getUnitWaypointBehavior()).moveAwayFromBuilding(buildingLayer.clone());
    }
    

    pub fn getVehicleProperties(&mut self) -> Option<Rc<RefCell<VehicleProperties>>> {
        return self.vehicleProperties.clone();
    }
    

    // throws Exception
    pub fn damage(&mut self, mut damage: i32, mut damageType: i32) {
        { let java_arguments = (damage.clone(), damageType.clone(),); self.base.damage(java_arguments.0, java_arguments.1) };
        jcall!(self.damageFloaters).add(damage.clone());
        if damage > 0 {
            jcall!(self.getHealthInterface()).damage(damage.clone());
        }
    }
    

    // throws Exception
    pub fn getDamage(&mut self, mut damageType: i32) -> i32 {
        return 0;
    }
    

    // throws Exception
    pub fn setDestroyed(&mut self, mut destroyed: bool) {
        jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setDestroyed"));
        { let java_arguments = (destroyed.clone(),); self.base.setDestroyed(java_arguments.0) };
        if self.isDestroyed() {
            jcall!(WaypointEventHandlerFactory::getInstance(self.getGroupInterface()[(0) as usize].clone())).removeListener(self.getUnitWaypointBehavior().map(|_java_reference| -> Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>> { panic!("ClassCastException: the Rust trait of UnitWaypointBehavior does not have EventListenerInterface as a base trait") }));
            jcall!(TrackingEventHandler::getInstance()).removeListener(None /* Java this: a &mut self method has no Rc of itself */);
            jcall!(BuildingEventHandler::getInstance()).removeListener(None /* Java this: a &mut self method has no Rc of itself */);
            if !jcall!(self.getHealthInterface()).isAlive() {
                let mut damage: i32 = jcall!(self.getHealthInterface()).getMaxHealth();
                if damage > 10 {
                    jcall!(SecondaryPlayerQueueFactory::getInstance()).add(ExplosionBasicSound::getInstance());
                    if damage < 100 {
                        jcall!(self.shakeListener).onSmallShakeEvent();
                        jcall!(self.vibration).vibrate(self.duration.clone(), 0, 0);
                    } else if damage < 1000 {
                        jcall!(self.shakeListener).onMediumShakeEvent();
                        jcall!(self.vibration).vibrate(self.duration * 2, 0, 0);
                    } else if damage < 3000 {
                        jcall!(self.shakeListener).onLargeShakeEvent();
                        jcall!(self.vibration).vibrate(self.duration * 4, 0, 0);
                    }
                }
            }
        }
    }
    

    pub fn getLoad(&mut self) -> i32 {
        return self.resourceLoad.clone();
    }
    

    // throws Exception
    pub fn clearResourceAnimation(&mut self) {
        self.resourceAnimation = { let java_value = ((|| -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>> { panic!("ClassCastException: a Rust dyn Animation cannot be cast to IndexedAnimation") })()).clone(); java_value };
    }
    

    // throws Exception
    pub fn setLoad(&mut self, mut resource: i32) {
        if resource > 0 {
            self.resourceAnimation = self.initResourceAnimation.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>);
        } else {
            self.clearResourceAnimation();
        }
        self.resourceLoad = resource.clone();
    }
    

    pub fn addLoad(&mut self, mut resource: i32) {
        self.resourceLoad += resource.clone();
    }
    

    // throws Exception
    pub fn handleCost(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        if self.getLoad() > 0 {
            { let java_receiver = self.CAPITAL_EVENT.clone(); let java_arguments = (self.getLoad(),); jcall!(java_receiver).setValue(java_arguments.0); };
            jcall!(CapitalEventHandlerFactory::getInstance(jcall!(ownerLayer).getGroupInterface()[(0) as usize].clone())).fireEvent(self.CAPITAL_EVENT.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
            self.setLoad(0);
        }
    }
    

    pub fn createHudPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn>>> {
        let mut rtsLayerHudPaintable: Option<Rc<RefCell<RTSLayerHudPaintable>>> = RTSLayerHudPaintable::getInstance();
        jcall!(rtsLayerHudPaintable).setBasicColorP(jcall!(self.allBinaryGameLayerManagerP).getForegroundBasicColor());
        jcall!(rtsLayerHudPaintable).setRtsLayer(None /* Java this: a &mut self method has no Rc of itself */);
        return rtsLayerHudPaintable.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn>>);
    }
    

    pub fn getHudPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn>>> {
        let mut rtsLayerHudPaintable: Option<Rc<RefCell<RTSLayerHudPaintable>>> = RTSLayerHudPaintable::getInstance();
        return rtsLayerHudPaintable.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn>>);
    }
    

    pub fn getType(&mut self) -> i32 {
        return UnitLayer::getStaticType();
    }
    

    pub fn getStaticType() -> i32 {
        return 1;
    }
    

    pub fn getUnitWaypointBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehaviorDyn>>> {
        return (|| -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehaviorDyn>>> { panic!("ClassCastException: a Rust dyn WaypointBehaviorBase cannot be cast to UnitWaypointBehavior") })();
    }
    

    pub fn getTrackingEvent(&mut self) -> Option<Rc<RefCell<TrackingEvent>>> {
        return self.trackingEvent.clone();
    }
    

    pub fn getCaptionAnimationHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::caption::CaptionAnimationHelperBase::CaptionAnimationHelperBaseDyn>>> {
        return self.captionAnimationHelper.clone();
    }
    

    pub fn isSelfUpgradeable(&mut self) -> bool {
        return false;
    }
    

    pub fn getMaxResourceLoad(&mut self) -> i16 {
        return self.maxResourceLoad.clone();
    }
    

    pub fn getPartialpositionlist() -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return UnitLayer_partialPositionList.with(Clone::clone).clone();
    }
}
impl std::ops::Deref for UnitLayer {
    type Target = AdvancedRTSGameLayer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for UnitLayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn for UnitLayer {
    fn as_RTSLayer(&self) -> &crate::org::allbinary::game::layer::RTSLayer::RTSLayer { &self.base.base }
    fn as_RTSLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::RTSLayer::RTSLayer { &mut self.base.base }
    fn updateWaypointBehavior(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) { Self::updateWaypointBehavior(self, geographicMapInterface) }
    fn construct(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>) { Self::construct(self, rtsPlayerLayerInterface) }
    fn select(&mut self) { Self::select(self) }
    fn deselect(&mut self) { Self::deselect(self) }
    fn setSelected(&mut self, mut selected: bool) { Self::setSelected(self, selected) }
    fn initInputProcessors(&mut self) { Self::initInputProcessors(self) }
    fn processBuiltTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processBuiltTick(self, allBinaryLayerManager) }
    fn onMovement(&mut self, mut trackingEvent: Option<Rc<RefCell<TrackingEvent>>>) { Self::onMovement(self, trackingEvent) }
    fn onMovementFound(&mut self, mut trackingEvent: Option<Rc<RefCell<TrackingEvent>>>) { Self::onMovementFound(self, trackingEvent) }
    fn createHudPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn>>> { Self::createHudPaintable(self) }
    fn getHudPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn>>> { Self::getHudPaintable(self) }
    fn setDestroyed(&mut self, mut destroyed: bool) { Self::setDestroyed(self, destroyed) }
    fn handleCost(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::handleCost(self, ownerLayer) }
    fn getWaypointBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>> { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::getWaypointBehavior(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn getParentLayer(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface>>> { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::getParentLayer(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn getWaypointLogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointLogHelper::WaypointLogHelperDyn>>> { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::getWaypointLogHelper(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn getWaypoint2LogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::Waypoint2LogHelper::Waypoint2LogHelperDyn>>> { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::getWaypoint2LogHelper(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn getWaypointRunnableLogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointRunnableLogHelper::WaypointRunnableLogHelperDyn>>> { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::getWaypointRunnableLogHelper(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn shouldAddWaypointFromBuilding(&mut self) -> bool { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::shouldAddWaypointFromBuilding(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn getCaptionAnimationHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::caption::CaptionAnimationHelperBase::CaptionAnimationHelperBaseDyn>>> { Self::getCaptionAnimationHelper(self) }
    fn init(&mut self, mut geographicMapCellHistory: Option<Rc<RefCell<GeographicMapCellHistory>>>, mut geographicMapCellPositionBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::init(self, geographicMapCellHistory, geographicMapCellPositionBasicArrayList) }
    fn getCurrentGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { Self::getCurrentGeographicMapCellPosition(self) }
    fn setClosestGeographicMapCellHistory(&mut self, mut pathsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::setClosestGeographicMapCellHistory(self, pathsList) }
    fn teleportTo(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) { Self::teleportTo(self, geographicMapCellPosition) }
    fn setLoad(&mut self, mut resource: i32) { Self::setLoad(self, resource) }
    fn getSurroundingGeographicMapCellPositionList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::getSurroundingGeographicMapCellPositionList(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn trackTo(&mut self, mut reason: String) { Self::trackTo(self, reason) }
    fn trackToDXY(&mut self, mut dx: i32, mut dy: i32) { Self::trackToDXY(self, dx, dy) }
    fn isWaypointListEmptyOrOnlyTargets(&mut self) -> bool { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::isWaypointListEmptyOrOnlyTargets(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self)) }
    fn getTrackingEvent(&mut self) -> Option<Rc<RefCell<TrackingEvent>>> { Self::getTrackingEvent(self) }
    fn buildingChase(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, p2: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> bool { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::buildingChase(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self), p1, p2) }
    fn allStop(&mut self) { Self::allStop(self) }
}
impl crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn for UnitLayer {
    fn as_AdvancedRTSGameLayer(&self) -> &crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer { &self.base }
    fn as_AdvancedRTSGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer { &mut self.base }
}
impl crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn for UnitLayer {
    fn as_CollidableCompositeLayer(&self) -> &crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer { &self.base.base.base.base.base }
    fn as_CollidableCompositeLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer { &mut self.base.base.base.base.base }
}
impl crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn for UnitLayer {
    fn as_AllBinaryGameLayer(&self) -> &crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &self.base.base.base.base.base.base }
    fn as_AllBinaryGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &mut self.base.base.base.base.base.base }
    fn r#move(&mut self) { Self::r#move(self) }
    fn processInput(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processInput(self, layerManager) }
    fn processTick(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::processTick(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self), p1) }
}
impl crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn for UnitLayer {
    fn as_AllBinaryLayer(&self) -> &crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &self.base.base.base.base.base.base.base }
    fn as_AllBinaryLayer_mut(&mut self) -> &mut crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &mut self.base.base.base.base.base.base.base }
    fn onChangeEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::view::event::ViewPositionEvent::ViewPositionEvent>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::onChangeEvent(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
    fn getGroupInterface(&mut self) -> Vec<Option<Rc<RefCell<crate::org::allbinary::game::identification::Group::Group>>>> { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::getGroupInterface(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
    fn implmentsTickableInterface(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::implmentsTickableInterface(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn implmentsGameInputInterface(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::implmentsGameInputInterface(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn implmentsCollidableInterface(&mut self) -> bool { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::implmentsCollidableInterface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
    fn getType(&mut self) -> i32 { Self::getType(self) }
    fn toString(&mut self) -> String { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::toString(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
}
impl crate::org::allbinary::layer::Layer::LayerDyn for UnitLayer {
    fn as_Layer(&self) -> &crate::org::allbinary::layer::Layer::Layer { &self.base.base.base.base.base.base.base.base }
    fn as_Layer_mut(&mut self) -> &mut crate::org::allbinary::layer::Layer::Layer { &mut self.base.base.base.base.base.base.base.base }
    fn setPosition(&mut self, p1: i32, p2: i32, p3: i32) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::setPosition(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1, p2, p3) }
    fn moveDXYZ(&mut self, p1: i32, p2: i32, p3: i32) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::moveDXYZ(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1, p2, p3) }
    fn getXP(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getXP(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getYP(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getYP(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getZP(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getZP(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getWidth(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getHeight(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getHeight(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn setVisible(&mut self, p1: bool) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::setVisible(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1) }
    fn isVisible(&mut self) -> bool { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::isVisible(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn toStringAppend(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::logic::string::StringMaker::StringMaker>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::toStringAppend(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn for UnitLayer {
    fn as_CollidableDestroyableDamageableLayer(&self) -> &crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer { &self.base.base.base.base }
    fn as_CollidableDestroyableDamageableLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer { &mut self.base.base.base.base }
    fn damage(&mut self, mut damage: i32, mut damageType: i32) { Self::damage(self, damage, damageType) }
    fn getDamage(&mut self, mut damageType: i32) -> i32 { Self::getDamage(self, damageType) }
    fn isDestroyed(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::isDestroyed(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) { Self::setAllBinaryGameLayerManager(self, allBinaryGameLayerManager) }
    fn up(&mut self) { Self::up(self) }
    fn down(&mut self) { Self::down(self) }
    fn right(&mut self) { Self::right(self) }
    fn left(&mut self) { Self::left(self) }
    fn fire(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { Self::fire(self, layerManager, gameKeyEvent) }
    fn onDestroyed(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::game::combat::destroy::event::DestroyedEvent::DestroyedEvent>>>) { crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer::onDestroyed(crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn::as_AdvancedRTSGameLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayerDyn for UnitLayer {
    fn as_MultiPlayerGameLayer(&self) -> &crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer { &self.base.base.base }
    fn as_MultiPlayerGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer { &mut self.base.base.base }
}
impl crate::org::allbinary::game::layer::RTSInterface::RTSInterface for UnitLayer {
    fn isSelfUpgradeable(&mut self) -> bool { Self::isSelfUpgradeable(self) }
    fn isCompleted(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::isCompleted(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn getPercentComplete(&mut self) -> i32 { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::getPercentComplete(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn getLevel(&mut self) -> i32 { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::getLevel(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn getMaxLevel(&mut self) -> i32 { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::getMaxLevel(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn getCost(&mut self) -> i32 { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::getCost(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn downgrade(&mut self) { Self::downgrade(self) }
    fn getDowngradeCost(&mut self) -> i32 { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::getDowngradeCost(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn isDowngradeable(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::isDowngradeable(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn getUpgradeCost(&mut self) -> i32 { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::getUpgradeCost(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn isUpgradeable(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::isUpgradeable(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn upgrade(&mut self) { Self::upgrade(self) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for UnitLayer {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::getSourceId(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
}
impl crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayerInterface::MultiPlayerGameLayerInterface for UnitLayer {
    fn getMultiplayerBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::multiplayer::layer::MultiplayerBehavior::MultiplayerBehaviorDyn>>> { crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer::getMultiplayerBehavior(crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayerDyn::as_MultiPlayerGameLayer_mut(self)) }
}
impl crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface for UnitLayer {
    fn getCollidableInferface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>> { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::getCollidableInferface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for UnitLayer {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::set(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::layer::pickup::PickupCompositeInterface::PickupCompositeInterface for UnitLayer {
    fn getPickupBehavior(&mut self) -> Option<Rc<RefCell<crate::org::allbinary::game::layer::pickup::PickupBehavior::PickupBehavior>>> { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::getPickupBehavior(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
    fn setPickupBehavior(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::game::layer::pickup::PickupBehavior::PickupBehavior>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::setPickupBehavior(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1) }
}


