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
// use crate::javax::microedition::lcdui::Canvas::Canvas;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
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

thread_local! { pub static UnitLayer_partialPositionList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListS::new(4).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>); }
thread_local! { pub static UnitLayer_SENSOR_RANGE_MULTIPLIER: i32 = 6; }
thread_local! { pub static UnitLayer_MOVE: String = String::from("Moving"); }
// impl BuildingEventListenerInterface for UnitLayer with the methods of the trait
// impl TrackingEventListenerInterface for UnitLayer with the methods of the trait
impl crate::org::allbinary::game::input::GameInputInterface::GameInputInterface for UnitLayer {
    fn processInput(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processInput(self, layerManager) }
    fn initInputProcessors(&mut self) { Self::initInputProcessors(self) }
}
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for UnitLayer {
    fn getName(&mut self) -> String { self.base.getName() }
}
impl crate::org::allbinary::game::combat::damage::DamageableInterface::DamageableInterface for UnitLayer {
    fn damage(&mut self, mut damage: i32, mut damageType: i32) { Self::damage(self, damage, damageType) }
    fn getDamage(&mut self, mut damageType: i32) -> i32 { Self::getDamage(self, damageType) }
}
impl crate::org::allbinary::game::combat::destroy::DestroyableInterface::DestroyableInterface for UnitLayer {
    fn isDestroyed(&mut self) -> bool { self.base.isDestroyed() }
}

impl UnitLayer {
    

    pub fn new_struct(mut remoteInfo: Option<Rc<RefCell<RemoteInfo>>>, mut parentLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut advancedRTSProperties: Option<Rc<RefCell<AdvancedRTSProperties>>>, mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>, mut rootName: String, mut name: String, mut vehicleProperties: Option<Rc<RefCell<VehicleProperties>>>, mut healthInterface: Option<Rc<RefCell<Health>>>, mut maxResourceLoad: Option<i32>, mut moveSoundInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>, mut waypointLayerInterfaceFactoryInterface: Option<Rc<RefCell<dyn LayerInterfaceFactoryInterface>>>, mut animationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut emptyAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut baseAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut buildAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut verticleBuildAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut decalAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut resourceAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut proceduralAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn ProceduralAnimationInterfaceFactoryInterface>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut direction: Option<Rc<RefCell<Direction>>>, mut x: i32, mut y: i32, mut z: i32, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Self {
        let mut this = Self {
            base: AdvancedRTSGameLayer::new_struct(remoteInfo.clone(), parentLayer.clone(), advancedRTSProperties.clone(), groupInterface.clone(), rootName.clone(), name.clone(), healthInterface.clone(), WaypointRTSFormInput::new(groupInterface.clone(), true), animationInterfaceFactoryInterface.clone(), emptyAnimationInterfaceFactoryInterface.clone(), baseAnimationInterfaceFactoryInterface.clone(), buildAnimationInterfaceFactoryInterface.clone(), verticleBuildAnimationInterfaceFactoryInterface.clone(), proceduralAnimationInterfaceFactoryInterface.clone(), rectangle.clone(), x.clone(), y.clone(), viewPosition.clone()),
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
        this.resourceAnimation = (jcall!(NullIndexedAnimationFactory::getFactoryInstance()).getInstance(0)).clone() /* as IndexedAnimation */;
        this.pathAnimation = jcall!(NullAnimationFactory::getFactoryInstance()).getInstance(0);
        this.captionAnimationHelper = CaptionAnimationHelper::new(jcall!(jcall!(FeaturedAnimationInterfaceFactoryInterfaceFactory::getInstance()).get(jcall!(CaptionResources::getInstance()).RESOURCE.clone())).getInstance(0), -23, -25, 6, 0).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::caption::CaptionAnimationHelperBase::CaptionAnimationHelperBaseDyn>>);
        this.resourceLoad = 0;
        this.rtsLogHelper = RTSLayerLogHelper::getInstance();
        this.movementAngle = jcall!(this.angleFactory).NOT_ANGLE.clone();
        this.steeringInsideGeographicMapCellPosition = crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone).clone();
        this.sensorGeographicMapCellPositionList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.CAPITAL_EVENT = CapitalEvent::new(this);
        // super(...) is in the struct literal above
        this.setCollidableInferface(CollidableUnitBehavior::new(true));
        this.waypointLayerInterfaceFactoryInterface = waypointLayerInterfaceFactoryInterface.clone();
        this.maxResourceLoad = jcall!(maxResourceLoad).shortValue();
        this.moveSoundInterface = moveSoundInterface.clone();
        this.decalAnimation = (jcall!(decalAnimationInterfaceFactoryInterface).getInstance(0)).clone() /* as RotationAnimation */;
        this.initResourceAnimation = (jcall!(resourceAnimationInterfaceFactoryInterface).getInstance(0)).clone() /* as RotationAnimation */;
        jcall!(this.initResourceAnimation).setFrameByDirection(direction.clone());
        jcall!(this.decalAnimation).setFrameByDirection(direction.clone());
        this.rotationAnimationInterfaceP = (this.indexedButShouldBeRotationAnimationInterface).clone() /* as RotationAnimation */;
        jcall!(this.rotationAnimationInterfaceP).setFrameByDirection(direction.clone());
        this.setMaxLevel(12);
        this.vehicleProperties = vehicleProperties.clone();
        this
    }

    pub fn new(mut remoteInfo: Option<Rc<RefCell<RemoteInfo>>>, mut parentLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut advancedRTSProperties: Option<Rc<RefCell<AdvancedRTSProperties>>>, mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>, mut rootName: String, mut name: String, mut vehicleProperties: Option<Rc<RefCell<VehicleProperties>>>, mut healthInterface: Option<Rc<RefCell<Health>>>, mut maxResourceLoad: Option<i32>, mut moveSoundInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>, mut waypointLayerInterfaceFactoryInterface: Option<Rc<RefCell<dyn LayerInterfaceFactoryInterface>>>, mut animationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut emptyAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut baseAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut buildAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut verticleBuildAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut decalAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut resourceAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut proceduralAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn ProceduralAnimationInterfaceFactoryInterface>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut direction: Option<Rc<RefCell<Direction>>>, mut x: i32, mut y: i32, mut z: i32, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(remoteInfo, parentLayer, advancedRTSProperties, groupInterface, rootName, name, vehicleProperties, healthInterface, maxResourceLoad, moveSoundInterface, waypointLayerInterfaceFactoryInterface, animationInterfaceFactoryInterface, emptyAnimationInterfaceFactoryInterface, baseAnimationInterfaceFactoryInterface, buildAnimationInterfaceFactoryInterface, verticleBuildAnimationInterfaceFactoryInterface, decalAnimationInterfaceFactoryInterface, resourceAnimationInterfaceFactoryInterface, proceduralAnimationInterfaceFactoryInterface, rectangle, direction, x, y, z, viewPosition))));
            if jcall!(Features::getInstance()).isFeature(jcall!(GameFeatureFactory::getInstance()).DAMAGE_FLOATERS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            this.damageFloaters = PtsDamageFloaters::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone())).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::combat::damage::DamageFloaters::DamageFloatersDyn>>);
            this.damageFloatersPaintableInterface = this.damageFloaters.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>);
        } else {
            this.damageFloatersPaintableInterface = NullPaintable::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>);
            this.damageFloaters = DamageFloaters::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::combat::damage::DamageFloaters::DamageFloatersDyn>>);
        }
            if jcall!(Features::getInstance()).isFeature(jcall!(GameFeatureFactory::getInstance()).HEALTH_BARS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            this.healthBar = HealthBar::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), this.getHealthInterface(), HealthBarTwodAnimation::new((this).clone() /* as AllBinaryLayer */, jcall!(BasicHudFactory::getInstance()).BOTTOMLEFT.clone()), -1).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>);
        } else {
            this.healthBar = NullPaintable::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>);
        }
            this.trackingEvent = TrackingEvent::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
            this.initPathAnimation = PathAnimation::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), LinePathRelativeAnimation::getInstance());
        this
    }
    

    // throws Exception
    pub fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) {
        self.base.setAllBinaryGameLayerManager(allBinaryGameLayerManager.clone());
        jcall!(self.initPathAnimation).setAllBinaryGameLayerManager(allBinaryGameLayerManager.clone());
    }
    

    // throws Exception
    pub fn updateWaypointBehavior(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) {
        let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = jcall!(StdUtil::getInstance()).createHashtable();
        jcall!(hashtable).put(crate::java_into_object(jcall!(self.groupCommonFactory).ID.clone()), crate::java_into_object(self.getGroupInterface()));
        jcall!(hashtable).put(crate::java_into_object(crate::org::allbinary::layer::Layer::Layer_ID.with(Clone::clone).clone()), None /* Java this: a &mut self method has no Rc of itself */);
        jcall!(hashtable).put(crate::java_into_object(crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager_ID.with(Clone::clone).clone()), crate::java_into_object(self.allBinaryGameLayerManagerP.clone()));
        { let java_arguments = (UnitWaypointBehavior2::new(None /* Java this: a &mut self method has no Rc of itself */, (jcall!(self.waypointLayerInterfaceFactoryInterface).getNextInstance(hashtable.clone(), self.x.clone(), self.y.clone(), self.z.clone())).clone() /* as AdvancedRTSGameLayer */),); self.setWaypointBehavior(java_arguments.0) };
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        let mut waypoint: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointBase::WaypointBaseDyn>>> = (if J2MEUtil::isHTML() { MultipassNoCacheWaypoint::new(None /* Java this: a &mut self method has no Rc of itself */, AttackSound::getInstance()) } else { NoCacheWaypoint::new(None /* Java this: a &mut self method has no Rc of itself */, AttackSound::getInstance()) });
        jcall!(self.getWaypointBehavior()).setWaypoint(waypoint.clone());
        self.base.updateWaypointBehavior(geographicMapInterface.clone());
        self.initRangeHack();
    }
    

    // throws Exception
    pub fn construct(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>) {
        self.base.construct(rtsPlayerLayerInterface.clone());
        jcall!(TrackingEventHandler::getInstance()).addListenerInterface(None /* Java this: a &mut self method has no Rc of itself */);
        jcall!(WaypointEventHandlerFactory::getInstance(self.getGroupInterface()[(0) as usize].clone())).addListenerInterface(self.getUnitWaypointBehavior());
        jcall!(BuildingEventHandler::getInstance()).addListenerInterface(None /* Java this: a &mut self method has no Rc of itself */);
        jcall!(self.getUnitWaypointBehavior()).setCurrentPathGeographicMapCellPosition(self.getCurrentGeographicMapCellPosition());
        self.updateSensorGeographicMapCellPositionList();
        jcall!(self.getUnitWaypointBehavior()).setLastPathGeographicMapCellPosition(jcall!(self.getUnitWaypointBehavior()).getCurrentPathGeographicMapCellPosition());
    }
    

    // throws Exception
    pub fn updateSensorGeographicMapCellPositionList(&mut self) {
        if jcall!(VisibleCellPositionsSingleton::getInstance()).shouldProcess() {
            let mut currentGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = self.getCurrentGeographicMapCellPosition();
            jcall!(self.sensorGeographicMapCellPositionList).clear();
            jcall!(self.sensorGeographicMapCellPositionList).add(crate::java_into_object(currentGeographicMapCellPosition.clone()));
            let mut sensorRange: i32 = self.weaponRange * UnitLayer_SENSOR_RANGE_MULTIPLIER.with(Clone::clone);
            let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (self.allBinaryGameLayerManagerP).clone() /* as GeographicMapCompositeInterface */;
            let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
            let mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(geographicMapInterface).getAllBinaryTiledLayer();
            let mut totalCells: i32 = (sensorRange / jcall!(tiledLayer).getCellHeight()) / 2;
            let mut geographicMapCellPositionFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>> = jcall!(geographicMapInterface).getGeographicMapCellPositionFactory();
            let mut column: i32 = jcall!(currentGeographicMapCellPosition).getColumn();
            let mut row: i32 = jcall!(currentGeographicMapCellPosition).getRow();
            let mut lastColumn: i32 = column + totalCells;
            let mut lastRow: i32 = row + totalCells;
            if lastColumn > jcall!(tiledLayer).getColumns() {
                lastColumn = jcall!(tiledLayer).getColumns();
            }
            if lastRow > jcall!(tiledLayer).getRows() {
                lastRow = jcall!(tiledLayer).getRows();
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
        self.pathAnimation = jcall!(NullAnimationFactory::getFactoryInstance()).getInstance(0);
        self.base.deselect();
    }
    

    pub fn setSelected(&mut self, mut selected: bool) {
        self.base.setSelected(selected.clone());
        if selected {
            if self.debug {
                self.rtsLogHelper = RTSLayerSelectedLogHelper::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayerLogHelper::RTSLayerLogHelperDyn>>);
            }
            self.rtsLayer2LogHelper = RTSLayer2SelectedLogHelper::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer2LogHelper::RTSLayer2LogHelperDyn>>);
        } else {
            if self.debug {
                self.rtsLogHelper = RTSLayerLogHelper::getInstance();
            }
            self.rtsLayer2LogHelper = RTSLayer2LogHelper::getInstance();
        }
    }
    

    // throws Exception
    pub fn setClosestGeographicMapCellHistory(&mut self, mut pathsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        jcall!(self.rtsLogHelper).setClosestGeographicMapCellHistory(None /* Java this: a &mut self method has no Rc of itself */, pathsList.clone());
        let mut closestIndex: i32 = -1;
        let mut shortestDistance: i32 = i32::MAX.clone();
        let mut currentDistance: i32 = i32::MAX.clone();
        for index in (0..=jcall!(pathsList).size() - 1).rev() {
            let mut geographicMapCellPositionBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_object = (jcall!(pathsList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) };
            let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = { let java_object = (jcall!(geographicMapCellPositionBasicArrayList).get(jcall!(geographicMapCellPositionBasicArrayList).size() - 1)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) };
            currentDistance = { let java_receiver = self.layerDistanceUtil.clone(); let java_arguments = (None /* Java this: a &mut self method has no Rc of itself */, jcall!(geographicMapCellPosition).getMidPoint(),); let java_result = jcall!(java_receiver).getDistanceAt(java_arguments.0, java_arguments.1); java_result };
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
            let mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>> = (jcall!(trackingEvent).getLayerInterface()).clone() /* as AdvancedRTSGameLayer */;
            if !match (&jcall!(layerInterface).getGroupInterface()[(0) as usize], &self.getGroupInterface()[(0) as usize]) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(jcall!(self.getUnitWaypointBehavior()).getPossibleTargetList()).add(layerInterface.clone());
                jcall!(layerInterface).onMovementFound(self.getTrackingEvent());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onMovement"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn onMovementFound(&mut self, mut trackingEvent: Option<Rc<RefCell<TrackingEvent>>>) {
        let mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>> = (jcall!(trackingEvent).getLayerInterface()).clone() /* as AdvancedRTSGameLayer */;
        if !match (&jcall!(layerInterface).getGroupInterface()[(0) as usize], &self.getGroupInterface()[(0) as usize]) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(jcall!(self.getUnitWaypointBehavior()).getPossibleTargetList()).add(layerInterface.clone());
        }
    }
    

    pub fn initRangeHack(&mut self) {
        let mut basicWeaponPart: Option<Rc<RefCell<dyn crate::org::allbinary::game::part::weapon::BasicWeaponPart::BasicWeaponPartDyn>>> = (self.getPartInterfaceArray()[(0) as usize]).clone() /* as BasicWeaponPart */;
        let mut weaponProperties: Option<Rc<RefCell<WeaponProperties>>> = jcall!(basicWeaponPart).getWeaponProperties();
        self.weaponRange = jcall!(weaponProperties).getRange();
        self.initRangeAnimation = AdjustedCircleAnimation::createW(self.weaponRange.clone(), self.weaponRange.clone(), self.getWidth(), jcall!(self.basicColorFactory).GREEN.clone());
        let mut sensorRange: i32 = self.weaponRange * UnitLayer_SENSOR_RANGE_MULTIPLIER.with(Clone::clone);
        self.initSensorRangeAnimation = AdjustedCircleAnimation::createW(sensorRange.clone(), sensorRange.clone(), self.getWidth(), jcall!(self.basicColorFactory).RED.clone());
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
                self.setAnimationInterface(self.destroyAnimationInterface.clone());
                jcall!(SecondaryPlayerQueueFactory::getInstance()).add(ExplosionBasicSound::getInstance());
                jcall!(self.shakeListener).onSmallShakeEvent();
                jcall!(self.vibration).vibrate(self.duration.clone(), 0, 0);
                self.setReadyForExplosion(true);
            }
        } else {
            self.base.processBuiltTick(allBinaryLayerManager.clone());
        }
        jcall!(self.captionAnimationHelper).tick();
        jcall!(self.getUnitWaypointBehavior()).processTick(allBinaryLayerManager.clone());
    }
    

    pub fn teleportTo(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) {
        let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(geographicMapCellPosition).getMidPoint();
        { let java_arguments = (jcall!(point).getX() - self.getHalfWidth(), jcall!(point).getY() - self.getHalfHeight(), self.z.clone(),); self.setPosition(java_arguments.0, java_arguments.1, java_arguments.2) };
    }
    

    // throws Exception
    pub fn getCurrentGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (self.allBinaryGameLayerManagerP).clone() /* as GeographicMapCompositeInterface */;
        let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
        let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(geographicMapInterface).getCellPositionAtXY(self.x + self.getHalfWidth(), self.y + self.getHalfHeight());
        let mut raceTrackGeographicMap: Option<Rc<RefCell<RaceTrackGeographicMap>>> = (geographicMapInterface).clone() /* as RaceTrackGeographicMap */;
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
            self.accelerate(self.decelerationBasicDecimal.clone());
        }
    }
    

    pub fn up(&mut self) {
        let mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>> = jcall!(self.getVehicleProperties()).getVelocityProperties();
        if !jcall!(velocityProperties).isOverXYMaxForwardVelocity() {
            self.accelerate(self.accelerationBasicDecimal.clone());
        }
    }
    

    pub fn initInputProcessors(&mut self) {
        self.inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() }) as usize] = SpecialRightGameInputProcessor::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() }) as usize] = SpecialLeftGameInputProcessor::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM0.clone() }) as usize] = SpecialFireGameInputProcessor::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_POUND.clone() }) as usize] = self.inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM0.clone() }) as usize].clone();
        self.inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_DOWN.clone() }) as usize] = SpecialDownGameInputProcessor::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_UP.clone() }) as usize] = SpecialUpGameInputProcessor::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.base.initInputProcessors();
    }
    

    // throws Exception
    pub fn processInput(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getGameKeyEventList();
        let mut size: i32 = jcall!(list).size();
        for index in (0..size) {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(list).get(index.clone());
            let mut key: i32 = GameKeyEventUtil::getKey(anyType.clone());
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
        let mut angle: i32 = (((jcall!(angleInfo).getAngle() + self.slightAngle)) as i32);
        { let java_receiver = self.hashtable.clone(); let java_arguments = (crate::java_into_object(jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(1)), crate::java_into_object(jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(((jcall!(jcall!(AngleFactory::getInstance()).getAt(angle.clone())).getValue()) as i32))),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1); };
        let mut salvoInterface: Option<Rc<RefCell<dyn SalvoInterface>>> = (self.getPartInterfaceArray()[(0) as usize]).clone() /* as SalvoInterface */;
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
        jcall!(jcall!(self.getVehicleProperties()).getVehicleFrictionProperties()).friction(jcall!(self.getVehicleProperties()).getVelocityProperties(), jcall!(vehicleFrictionProperties).getTireFrictionNominator());
    }
    

    // throws Exception
    pub fn trackTo(&mut self, mut reason: String) {
        let mut waypointBehaviorBase: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>> = (self.waypointBehaviorBase).clone() /* as WaypointBehaviorBase */;
        let mut nextUnvisitedPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(waypointBehaviorBase).getNextUnvisitedPathGeographicMapCellPosition();
        let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(nextUnvisitedPathGeographicMapCellPosition).getMidPoint();
        let mut dx: i32 = (self.getXP() + self.getHalfWidth()) - jcall!(point).getX();
        let mut dy: i32 = (self.getYP() + self.getHalfHeight()) - jcall!(point).getY();
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
        let mut waypointBehaviorBase: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>> = (self.waypointBehaviorBase).clone() /* as WaypointBehaviorBase */;
        let mut nextUnvisitedPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(waypointBehaviorBase).getNextUnvisitedPathGeographicMapCellPosition();
        let mut evading: bool = false;
        if match (&jcall!(self.getUnitWaypointBehavior()).getSensorAction(), &jcall!(SensorActionFactory::getInstance()).EVADE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
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
            if dx > 0 && match (&self.movementAngle, &jcall!(self.angleFactory).LEFT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(self.rtsLogHelper).movingLeft(None /* Java this: a &mut self method has no Rc of itself */);
                return false;
            }
            if dx < 0 && match (&self.movementAngle, &jcall!(self.angleFactory).RIGHT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(self.rtsLogHelper).movingRight(None /* Java this: a &mut self method has no Rc of itself */);
                return false;
            }
            if dy > 0 && match (&self.movementAngle, &jcall!(self.angleFactory).UP) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(self.rtsLogHelper).movingUp(None /* Java this: a &mut self method has no Rc of itself */);
                return false;
            }
            if dy < 0 && match (&self.movementAngle, &jcall!(self.angleFactory).DOWN) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(self.rtsLogHelper).movingDown(None /* Java this: a &mut self method has no Rc of itself */);
                return false;
            }
            jcall!(self.rtsLogHelper).currentMoveEnded(None /* Java this: a &mut self method has no Rc of itself */);
            if match (&self.movementAngle, &jcall!(self.angleFactory).LEFT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&self.movementAngle, &jcall!(self.angleFactory).RIGHT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                self.handleDeltalY(dx.clone(), dy.clone());
            } else if match (&self.movementAngle, &jcall!(self.angleFactory).UP) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&self.movementAngle, &jcall!(self.angleFactory).DOWN) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
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
                    jcall!(self.getGameKeyEventList()).add(jcall!(gameKeyEventFactory).getInstanceForKey(None /* Java this: a &mut self method has no Rc of itself */, unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() }.clone()));
                } else {
                    jcall!(self.rtsLogHelper).rotateLeft(None /* Java this: a &mut self method has no Rc of itself */);
                    jcall!(self.getGameKeyEventList()).add(jcall!(gameKeyEventFactory).getInstanceForKey(None /* Java this: a &mut self method has no Rc of itself */, unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() }.clone()));
                }
                return true;
            } else {
                jcall!(self.rtsLogHelper).noRotation(None /* Java this: a &mut self method has no Rc of itself */);
            }
            return false;
        }
    }
    

    pub fn handleDeltalX(&mut self, mut dx: i32, mut dy: i32) {
        let mut waypointBehaviorBase: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>> = (self.waypointBehaviorBase).clone() /* as WaypointBehaviorBase */;
        let mut nextUnvisitedPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(waypointBehaviorBase).getNextUnvisitedPathGeographicMapCellPosition();
        if dx > 0 {
            self.movementAngle = jcall!(self.angleFactory).LEFT.clone();
            self.steeringInsideGeographicMapCellPosition = nextUnvisitedPathGeographicMapCellPosition.clone();
        } else {
            self.movementAngle = jcall!(self.angleFactory).RIGHT.clone();
            self.steeringInsideGeographicMapCellPosition = nextUnvisitedPathGeographicMapCellPosition.clone();
        }
        jcall!(self.rtsLogHelper).handle(None /* Java this: a &mut self method has no Rc of itself */, self.movementAngle.clone());
    }
    

    pub fn handleDeltalY(&mut self, mut dx: i32, mut dy: i32) {
        let mut waypointBehaviorBase: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>> = (self.waypointBehaviorBase).clone() /* as WaypointBehaviorBase */;
        let mut nextUnvisitedPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(waypointBehaviorBase).getNextUnvisitedPathGeographicMapCellPosition();
        if dy > 0 {
            self.movementAngle = jcall!(self.angleFactory).UP.clone();
            self.steeringInsideGeographicMapCellPosition = nextUnvisitedPathGeographicMapCellPosition.clone();
        } else {
            self.movementAngle = jcall!(self.angleFactory).DOWN.clone();
            self.steeringInsideGeographicMapCellPosition = nextUnvisitedPathGeographicMapCellPosition.clone();
        }
        jcall!(self.rtsLogHelper).handle(None /* Java this: a &mut self method has no Rc of itself */, self.movementAngle.clone());
    }
    

    // throws Exception
    pub fn trackToDXYTargetAngle(&mut self, mut dx: i32, mut dy: i32, mut targetAngle: i32) {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(self.getUnitWaypointBehavior()).getSteeringVisitorList();
        if jcall!(list).size() > 0 {
            for index in (0..=jcall!(list).size() - 1).rev() {
                let mut steeringVisitor: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::SteeringVisitor::SteeringVisitorDyn>>> = { let java_object = (jcall!(list).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::SteeringVisitor::SteeringVisitorDyn>>>>().expect("ClassCastException").clone()) };
                let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(steeringVisitor).visit(None /* Java this: a &mut self method has no Rc of itself */);
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
                jcall!(self.captionAnimationHelper).update(UnitLayer_MOVE.with(Clone::clone).clone(), jcall!(self.basicColorFactory).GREEN.clone());
            }
            jcall!(self.getGameKeyEventList()).add(jcall!(gameKeyEventFactory).getInstanceForKey(None /* Java this: a &mut self method has no Rc of itself */, unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_UP.clone() }.clone()));
        } else {
            { let java_receiver = self.captionAnimationHelper.clone(); let java_arguments = (jcall!(CommonPhoneStrings::getInstance()).FIRE.clone(), jcall!(self.basicColorFactory).RED.clone(),); jcall!(java_receiver).update(java_arguments.0, java_arguments.1); };
            jcall!(self.rtsLayer2LogHelper).steeringFireOrStop(None /* Java this: a &mut self method has no Rc of itself */);
            self.allStop();
            jcall!(self.getGameKeyEventList()).add(jcall!(gameKeyEventFactory).getInstanceForKey(None /* Java this: a &mut self method has no Rc of itself */, unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM0.clone() }.clone()));
            jcall!(TrackingEventHandler::getInstance()).fireEvent(self.getTrackingEvent());
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
                let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (self.allBinaryGameLayerManagerP).clone() /* as GeographicMapCompositeInterface */;
                let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
                { let java_receiver = self.layerPartialCellPositionsUtil.clone(); let java_arguments = (geographicMapInterface.clone(), None /* Java this: a &mut self method has no Rc of itself */, ((velocityXScaled) as i32), ((velocityYScaled) as i32), UnitLayer::getPartialpositionlist(),); jcall!(java_receiver).getAllDXY(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4); };
                let mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = (jcall!(DropCellPositionHistory::getInstance()).getCellPositionWithDrop(UnitLayer::getPartialpositionlist())).clone() /* as GeographicMapCellPosition */;
                if match (&cellPosition, &jcall!(self.cellPositionFactory).NONE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    let mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(geographicMapInterface).getAllBinaryTiledLayer();
                    let mut tiledLayerUtil: Option<Rc<RefCell<TiledLayerUtil>>> = TiledLayerUtil::getInstance();
                    let mut x: i32 = self.x + ((velocityXScaled) as i32);
                    let mut y: i32 = self.y + ((velocityYScaled) as i32);
                    x = jcall!(tiledLayerUtil).keepOnMapX(tiledLayer.clone(), x.clone(), self.getWidth());
                    y = jcall!(tiledLayerUtil).keepOnMapY(tiledLayer.clone(), y.clone(), self.getHeight());
                    self.setPosition(x.clone(), y.clone(), self.z.clone());
                } else {
                    let mut allbinaryLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = jcall!(DropCellPositionHistory::getInstance()).getLayerInterface(cellPosition.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>));
                    jcall!(self.getUnitWaypointBehavior()).setMovingFromStopped(false);
                    jcall!(self.getUnitWaypointBehavior()).addBuildingChase(allbinaryLayer.clone(), cellPosition.clone());
                }
            }
            if jcall!(self.getUnitWaypointBehavior()).isMoving() {
                jcall!(TrackingEventHandler::getInstance()).fireEvent(self.getTrackingEvent());
            } else {
                jcall!(self.getUnitWaypointBehavior()).setMovingFromStopped(false);
            }
            if jcall!(self.getUnitWaypointBehavior()).isMovingFromStopped() && self.isVisible() {
                jcall!(SecondaryPlayerQueueFactory::getInstance()).add(self.moveSoundInterface.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("move"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn allStop(&mut self) {
        let mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>> = jcall!(self.getVehicleProperties()).getVelocityProperties();
        jcall!(jcall!(velocityProperties).getVelocityXBasicDecimalP()).setint(0);
        jcall!(jcall!(velocityProperties).getVelocityYBasicDecimalP()).setint(0);
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        if self.isVisible() {
            self.base.paint(graphics.clone());
            let mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>> = self.getViewPosition();
            let mut viewX: i32 = jcall!(viewPosition).getX();
            let mut viewY: i32 = jcall!(viewPosition).getY();
            jcall!(self.decalAnimation).paintXY(graphics.clone(), viewX.clone(), viewY.clone());
            jcall!(self.rangeAnimation).paintXY(graphics.clone(), viewX.clone(), viewY.clone());
            jcall!(self.sensorRangeAnimation).paintXY(graphics.clone(), viewX.clone(), viewY.clone());
            jcall!(self.damageFloatersPaintableInterface).paint(graphics.clone());
            jcall!(self.healthBar).paint(graphics.clone());
            jcall!(self.captionAnimationHelper).paintXY(graphics.clone(), viewX.clone(), viewY.clone());
            jcall!(self.pathAnimation).paintXY(graphics.clone(), viewX.clone(), viewY.clone());
            jcall!(self.resourceAnimation).paintXY(graphics.clone(), viewX.clone(), viewY.clone());
        }
    }
    

    // throws Exception
    pub fn onBuildingEvent(&mut self, mut event: Option<Rc<RefCell<RTSLayerEvent>>>) {
        let mut buildingLayer: Option<Rc<RefCell<BuildingLayer>>> = (jcall!(event).getRtsLayer()).clone() /* as BuildingLayer */;
        jcall!(self.getUnitWaypointBehavior()).moveAwayFromBuilding(buildingLayer.clone());
    }
    

    pub fn getVehicleProperties(&mut self) -> Option<Rc<RefCell<VehicleProperties>>> {
        return self.vehicleProperties.clone();
    }
    

    // throws Exception
    pub fn damage(&mut self, mut damage: i32, mut damageType: i32) {
        self.base.damage(damage.clone(), damageType.clone());
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
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setDestroyed"));
        self.base.setDestroyed(destroyed.clone());
        if self.isDestroyed() {
            jcall!(WaypointEventHandlerFactory::getInstance(self.getGroupInterface()[(0) as usize].clone())).removeListener(self.getUnitWaypointBehavior());
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
        self.resourceAnimation = (jcall!(NullIndexedAnimationFactory::getFactoryInstance()).getInstance(0)).clone() /* as IndexedAnimation */;
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
            jcall!(CapitalEventHandlerFactory::getInstance(jcall!(ownerLayer).getGroupInterface()[(0) as usize].clone())).fireEvent(self.CAPITAL_EVENT.clone());
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
        return (self.getWaypointBehavior()).clone() /* as UnitWaypointBehavior */;
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
impl crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn for UnitLayer {
    fn as_AllBinaryLayer(&self) -> &crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &self.base.base.base.base.base.base.base }
    fn as_AllBinaryLayer_mut(&mut self) -> &mut crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &mut self.base.base.base.base.base.base.base }
    fn onChangeEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::view::event::ViewPositionEvent::ViewPositionEvent>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::onChangeEvent(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
    fn implmentsTickableInterface(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::implmentsTickableInterface(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn implmentsGameInputInterface(&mut self) -> bool { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::implmentsGameInputInterface(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self)) }
    fn implmentsCollidableInterface(&mut self) -> bool { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::implmentsCollidableInterface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
    fn getType(&mut self) -> i32 { Self::getType(self) }
    fn toString(&mut self) -> String { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::toString(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
}
impl crate::org::allbinary::layer::Layer::LayerDyn for UnitLayer {
    fn as_Layer(&self) -> &crate::org::allbinary::layer::Layer::Layer { &self.base.base.base.base.base.base.base.base }
    fn as_Layer_mut(&mut self) -> &mut crate::org::allbinary::layer::Layer::Layer { &mut self.base.base.base.base.base.base.base.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn toStringAppend(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::logic::string::StringMaker::StringMaker>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::toStringAppend(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn for UnitLayer {
    fn as_AllBinaryGameLayer(&self) -> &crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &self.base.base.base.base.base.base }
    fn as_AllBinaryGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &mut self.base.base.base.base.base.base }
    fn r#move(&mut self) { Self::r#move(self) }
    fn processInput(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processInput(self, layerManager) }
    fn processTick(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { crate::org::allbinary::game::layer::RTSLayer::RTSLayer::processTick(crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn::as_RTSLayer_mut(self), p1) }
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


