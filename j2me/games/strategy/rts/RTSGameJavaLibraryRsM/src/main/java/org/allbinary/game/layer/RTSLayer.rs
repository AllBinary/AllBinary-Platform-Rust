// 
// AllBinary Open License Version 1 
// Copyright (c) 2003 AllBinary 
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
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInput;
use crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::AnimationInterfaceFactoryInterface::AnimationInterfaceFactoryInterface;
use crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation;
use crate::org::allbinary::animation::NullAnimationFactory::NullAnimationFactory;
use crate::org::allbinary::animation::NullIndexedAnimationFactory::NullIndexedAnimationFactory;
use crate::org::allbinary::animation::ProceduralAnimationInterfaceFactoryInterface::ProceduralAnimationInterfaceFactoryInterface;
use crate::org::allbinary::animation::RotationAnimation::RotationAnimation;
use crate::org::allbinary::animation::caption::CaptionAnimationHelperBase::CaptionAnimationHelperBase;
use crate::org::allbinary::game::combat::destroy::DestroyedLayerProcessor::DestroyedLayerProcessor;
use crate::org::allbinary::game::health::Health::Health;
use crate::org::allbinary::game::identification::Group::Group;
use crate::org::allbinary::game::identification::GroupFactory::GroupFactory;
use crate::org::allbinary::game::input::GameInputInterface::GameInputInterface;
use crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessor;
use crate::org::allbinary::game::input::GameInputProcessorUtil::GameInputProcessorUtil;
use crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface;
use crate::org::allbinary::game::input::InputFactory::InputFactory;
use crate::org::allbinary::game::input::form::NullRTSFormInputFactory::NullRTSFormInputFactory;
use crate::org::allbinary::game::layer::waypoint::Waypoint2LogHelper::Waypoint2LogHelper;
use crate::org::allbinary::game::layer::waypoint::WaypointLogHelper::WaypointLogHelper;
use crate::org::allbinary::game::layer::waypoint::WaypointRunnableLogHelper::WaypointRunnableLogHelper;
use crate::org::allbinary::game::tick::TickableInterface::TickableInterface;
use crate::org::allbinary::game::tracking::TrackingEvent::TrackingEvent;
use crate::org::allbinary::game::view::TileLayerPositionIntoViewPosition::TileLayerPositionIntoViewPosition;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::graphics::RectangleFactory::RectangleFactory;
use crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionForm;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::org::allbinary::view::event::ViewPositionEventHandler::ViewPositionEventHandler;
use crate::org::allbinary::game::multiplayer::layer::RemoteInfo::RemoteInfo;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::math::LayerDistanceUtil::LayerDistanceUtil;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellHistory::GeographicMapCellHistory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCompositeInterface::GeographicMapCompositeInterface;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::view::ViewPositionBase::ViewPositionBase;
use super::RTSInterface::RTSInterface;
use super::PathFindingLayerInterface::PathFindingLayerInterface;
use super::RTSLayer2LogHelper::RTSLayer2LogHelper;
use super::GeographicMapCellPositionAreaBase::GeographicMapCellPositionAreaBase;
use super::GeographicMapCellPositionArea::GeographicMapCellPositionArea;
use super::AllBinaryTiledLayer::AllBinaryTiledLayer;
use super::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use super::RTSPlayerLayerInterface::RTSPlayerLayerInterface;
use super::RTSLayerUtil::RTSLayerUtil;
use super::WaypointBehaviorBase::WaypointBehaviorBase;
use super::SelectionHudPaintable::SelectionHudPaintable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RTSLayer {
    pub base: MultiPlayerGameLayer,
    pub debug: bool,
    pub showMoreCaptionStates: bool,
    pub inputProcessorArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn>>>>,
    pub rtsLayer2LogHelper: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer2LogHelper::RTSLayer2LogHelperDyn>>>,
    pub waypointLogHelperP: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointLogHelper::WaypointLogHelperDyn>>>,
    pub waypoint2LogHelperP: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::Waypoint2LogHelper::Waypoint2LogHelperDyn>>>,
    pub waypointRunnableLogHelperP: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointRunnableLogHelper::WaypointRunnableLogHelperDyn>>>,
    pub rtsFormInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn>>>,
    pub verticleBuildAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>>,
    pub rangeAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
    pub initRangeAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
    pub sensorRangeAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
    pub initSensorRangeAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
    pub baseAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
    pub buildFrameTimeHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub buildAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
    pub animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
    pub initAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>>,
    pub indexedButShouldBeRotationAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>>,
    pub emptyAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>>,
    pub destroyAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>>,
    pub fireTimeHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub rootName: String,
    pub hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>,
    pub BUILD_VALUE: i32,
    pub geographicMapCellPositionAreaBase: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::GeographicMapCellPositionAreaBase::GeographicMapCellPositionAreaBaseDyn>>>,
    pub healthInterface: Option<Rc<RefCell<Health>>>,
    pub level: i32,
    pub maxLevel: i32,
    pub hackVerticleBuild: i32,
    pub slightAngle: i32,
    pub percentCompleteP: i32,
    pub destroyed: bool,
    pub selected: bool,
    pub rtsLayerUtil: Option<Rc<RefCell<RTSLayerUtil>>>,
    pub layerDistanceUtil: Option<Rc<RefCell<LayerDistanceUtil>>>,
}

thread_local! { pub static RTSLayer_id: i32 = 0; }
thread_local! { pub static RTSLayer_MAIN_INDEX: i32 = 0; }
thread_local! { pub static RTSLayer_SECONDARY_INDEX: i32 = 1; }
// impl TickableInterface for RTSLayer with the methods of the trait
// impl GameInputInterface for RTSLayer with the methods of the trait
impl GameKeyEventSourceInterface for RTSLayer {
    fn getSourceId(&mut self) -> i32 { Self::getSourceId(self) }
}
impl RTSInterface for RTSLayer {
    fn isSelfUpgradeable(&mut self) -> bool { Self::isSelfUpgradeable(self) }
    fn isCompleted(&mut self) -> bool { Self::isCompleted(self) }
    fn getPercentComplete(&mut self) -> i32 { Self::getPercentComplete(self) }
    fn getLevel(&mut self) -> i32 { Self::getLevel(self) }
    fn getMaxLevel(&mut self) -> i32 { Self::getMaxLevel(self) }
    fn getCost(&mut self) -> i32 { Self::getCost(self) }
    fn downgrade(&mut self) { Self::downgrade(self) }
    fn getDowngradeCost(&mut self) -> i32 { Self::getDowngradeCost(self) }
    fn isDowngradeable(&mut self) -> bool { Self::isDowngradeable(self) }
    fn getUpgradeCost(&mut self) -> i32 { Self::getUpgradeCost(self) }
    fn isUpgradeable(&mut self) -> bool { Self::isUpgradeable(self) }
    fn upgrade(&mut self) { Self::upgrade(self) }
}
// impl PathFindingLayerInterface for RTSLayer with the methods of the trait

impl RTSLayer {
    

    // throws Exception
    pub fn createSimulatedInstance(mut remoteInfo: Option<Rc<RefCell<RemoteInfo>>>) -> Option<Rc<RefCell<dyn RTSLayerDyn>>> {
        let mut nullAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>> = NullAnimationFactory::getFactoryInstance();
        let mut nullIndexedAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>> = NullIndexedAnimationFactory::getFactoryInstance();
        return RTSLayer::new(crate::org::allbinary::game::multiplayer::layer::RemoteInfo::RemoteInfo_REMOTE_INFO.with(Clone::clone).clone(), jcall!(GroupFactory::getInstance()).NULL_GROUP_ARRAY.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), crate::org::allbinary::game::health::Health::Health_NULL_HEALTH.with(Clone::clone).clone(), NullRTSFormInputFactory::getInstance(), nullAnimationInterfaceFactoryInterface.clone(), nullIndexedAnimationInterfaceFactoryInterface.clone(), nullAnimationInterfaceFactoryInterface.clone(), nullAnimationInterfaceFactoryInterface.clone(), nullIndexedAnimationInterfaceFactoryInterface.clone(), NullIndexedAnimationFactory::getFactoryInstance(), crate::org::allbinary::graphics::RectangleFactory::RectangleFactory_SINGLETON.with(Clone::clone).clone(), 0, 0, TileLayerPositionIntoViewPosition::new()).map(|reference| reference as Rc<RefCell<dyn RTSLayerDyn>>);
    }
    

    pub fn new_struct(mut remoteInfo: Option<Rc<RefCell<RemoteInfo>>>, mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>, mut rootName: String, mut name: String, mut healthInterface: Option<Rc<RefCell<Health>>>, mut rtsFormInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn>>>, mut animationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut emptyAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut baseAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut buildAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut verticleBuildAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut proceduralAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn ProceduralAnimationInterfaceFactoryInterface>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut x: i32, mut y: i32, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Self {
        let mut this = Self {
            base: MultiPlayerGameLayer::new_struct(remoteInfo.clone(), groupInterface.clone(), name.clone(), rectangle.clone(), viewPosition.clone()),
            debug: false,
            showMoreCaptionStates: false,
            inputProcessorArray: Vec::new(),
            rtsLayer2LogHelper: None,
            waypointLogHelperP: None,
            waypoint2LogHelperP: None,
            waypointRunnableLogHelperP: None,
            rtsFormInput: None,
            verticleBuildAnimationInterface: None,
            rangeAnimation: None,
            initRangeAnimation: None,
            sensorRangeAnimation: None,
            initSensorRangeAnimation: None,
            baseAnimationInterface: None,
            buildFrameTimeHelper: None,
            buildAnimationInterface: None,
            animationInterface: None,
            initAnimationInterface: None,
            indexedButShouldBeRotationAnimationInterface: None,
            emptyAnimationInterface: None,
            destroyAnimationInterface: None,
            fireTimeHelper: None,
            rootName: String::new(),
            hashtable: None,
            BUILD_VALUE: 0,
            geographicMapCellPositionAreaBase: None,
            healthInterface: None,
            level: 0,
            maxLevel: 0,
            hackVerticleBuild: 0,
            slightAngle: 0,
            percentCompleteP: 0,
            destroyed: false,
            selected: false,
            rtsLayerUtil: None,
            layerDistanceUtil: None,
        };
        this.debug = true;
        this.showMoreCaptionStates = this.debug.clone();
        this.inputProcessorArray = vec![None; (jcall!(InputFactory::getInstance()).MAX) as usize];
        this.rtsLayer2LogHelper = RTSLayer2LogHelper::getInstance();
        this.waypointLogHelperP = WaypointLogHelper::getInstance();
        this.waypoint2LogHelperP = Waypoint2LogHelper::getInstance();
        this.waypointRunnableLogHelperP = WaypointRunnableLogHelper::getInstance();
        this.buildFrameTimeHelper = TimeDelayHelper::new(50).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.fireTimeHelper = TimeDelayHelper::new(700).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.hashtable = jcall!(StdUtil::getInstance()).createHashtable();
        this.BUILD_VALUE = 63;
        this.healthInterface = crate::org::allbinary::game::health::Health::Health_NULL_HEALTH.with(Clone::clone).clone();
        this.level = 1;
        this.maxLevel = 1;
        this.hackVerticleBuild = this.BUILD_VALUE.clone();
        this.slightAngle = 0;
        this.destroyed = false;
        this.selected = false;
        this.rtsLayerUtil = RTSLayerUtil::getInstance();
        this.layerDistanceUtil = LayerDistanceUtil::getInstance();
        // super(...) is in the struct literal above
        this.initInputProcessors();
        this.rootName = rootName.clone();
        this.healthInterface = healthInterface.clone();
        this.rtsFormInput = rtsFormInput.clone();
        this.setPosition(x.clone(), y.clone(), this.z.clone());
        this.verticleBuildAnimationInterface = (jcall!(verticleBuildAnimationInterfaceFactoryInterface).getInstance(0)).clone() /* as IndexedAnimation */;
        this.buildAnimationInterface = jcall!(buildAnimationInterfaceFactoryInterface).getInstance(0);
        this.baseAnimationInterface = jcall!(baseAnimationInterfaceFactoryInterface).getInstance(0);
        this.emptyAnimationInterface = (jcall!(emptyAnimationInterfaceFactoryInterface).getInstance(0)).clone() /* as IndexedAnimation */;
        this.indexedButShouldBeRotationAnimationInterface = (jcall!(animationInterfaceFactoryInterface).getInstance(0)).clone() /* as IndexedAnimation */;
        this.initAnimationInterface = this.indexedButShouldBeRotationAnimationInterface.clone();
        this.destroyAnimationInterface = (jcall!(proceduralAnimationInterfaceFactoryInterface).getInstanceAnimation(this.indexedButShouldBeRotationAnimationInterface.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>))).clone() /* as IndexedAnimation */;
        let mut animation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> = jcall!(NullAnimationFactory::getFactoryInstance()).getInstance(0);
        this.rangeAnimation = animation.clone();
        this.initRangeAnimation = animation.clone();
        this.sensorRangeAnimation = animation.clone();
        this.initSensorRangeAnimation = animation.clone();
        this.animationInterface = this.initAnimationInterface.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
        this
    }

    pub fn new(mut remoteInfo: Option<Rc<RefCell<RemoteInfo>>>, mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>, mut rootName: String, mut name: String, mut healthInterface: Option<Rc<RefCell<Health>>>, mut rtsFormInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn>>>, mut animationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut emptyAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut baseAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut buildAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut verticleBuildAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>, mut proceduralAnimationInterfaceFactoryInterface: Option<Rc<RefCell<dyn ProceduralAnimationInterfaceFactoryInterface>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut x: i32, mut y: i32, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(remoteInfo, groupInterface, rootName, name, healthInterface, rtsFormInput, animationInterfaceFactoryInterface, emptyAnimationInterfaceFactoryInterface, baseAnimationInterfaceFactoryInterface, buildAnimationInterfaceFactoryInterface, verticleBuildAnimationInterfaceFactoryInterface, proceduralAnimationInterfaceFactoryInterface, rectangle, x, y, viewPosition))));
            this.geographicMapCellPositionAreaBase = GeographicMapCellPositionArea::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone())).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::GeographicMapCellPositionAreaBase::GeographicMapCellPositionAreaBaseDyn>>);
        this
    }
    

    // throws Exception
    pub fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) {
        self.base.setAllBinaryGameLayerManager(allBinaryGameLayerManager.clone());
        let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (self.allBinaryGameLayerManagerP).clone() /* as GeographicMapCompositeInterface */;
        let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
        let mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(geographicMapInterface).getAllBinaryTiledLayer();
        let mut viewPosition2: Option<Rc<RefCell<TileLayerPositionIntoViewPosition>>> = (self.getViewPosition()).clone() /* as TileLayerPositionIntoViewPosition */;
        jcall!(viewPosition2).setTiledLayer(tiledLayer.clone());
        self.updateWaypointBehavior(geographicMapInterface.clone());
    }
    

    // throws Exception
    pub fn updateWaypointBehavior(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) {
        jcall!(self.geographicMapCellPositionAreaBase).update(geographicMapInterface.clone());
    }
    

    // throws Exception
    pub fn construct(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>) {
        jcall!(ViewPositionEventHandler::getInstance()).addListener((self).clone() /* as AllBinaryLayer */);
        self.animationInterface = self.buildAnimationInterface.clone();
        self.hackVerticleBuild = 0;
    }
    

    pub fn select(&mut self) {
        self.rangeAnimation = self.initRangeAnimation.clone();
        self.sensorRangeAnimation = self.initSensorRangeAnimation.clone();
        self.setSelected(true);
    }
    

    pub fn deselect(&mut self) {
        let mut animation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> = jcall!(NullAnimationFactory::getFactoryInstance()).getInstance(0);
        self.rangeAnimation = animation.clone();
        self.sensorRangeAnimation = animation.clone();
        self.setSelected(false);
    }
    

    pub fn setSelected(&mut self, mut selected: bool) {
        self.selected = selected.clone();
    }
    

    pub fn isSelected(&mut self) -> bool {
        return self.selected.clone();
    }
    

    pub fn getPercentComplete(&mut self) -> i32 {
        return self.percentCompleteP.clone();
    }
    

    pub fn getScrollSelectionForm(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> {
        return ScrollSelectionForm::getNullScrollSelectionForm();
    }
    

    pub fn initInputProcessors(&mut self) {
        GameInputProcessorUtil::init(self.inputProcessorArray.clone());
    }
    

    // throws Exception
    pub fn processBuiltTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        self.animationInterface = self.indexedButShouldBeRotationAnimationInterface.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
    }
    

    pub fn processTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if self.isCompleted() {
                self.processBuiltTick(allBinaryLayerManager.clone());
            } else {
                self.build();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("processTick"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn processInput(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
    }
    

    pub fn reload(&mut self) {
    }
    

    pub fn onMovement(&mut self, mut trackingEvent: Option<Rc<RefCell<TrackingEvent>>>) {
    }
    

    // throws Exception
    pub fn onMovementFound(&mut self, mut trackingEvent: Option<Rc<RefCell<TrackingEvent>>>) {
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        self.base.paintFirst(graphics.clone());
        let mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>> = self.getViewPosition();
        let mut viewX: i32 = jcall!(viewPosition).getX();
        let mut viewY: i32 = jcall!(viewPosition).getY();
        jcall!(self.getAnimationInterface()).paintXY(graphics.clone(), viewX.clone(), viewY.clone());
    }
    

    // throws Exception
    pub fn damage(&mut self, mut damage: i32, mut damageType: i32) {
    }
    

    pub fn getSourceId(&mut self) -> i32 {
        return RTSLayer_id.with(Clone::clone).clone();
    }
    

    pub fn downgrade(&mut self) {
        jcall!(self.rtsLayerUtil).downgrade(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    pub fn upgrade(&mut self) {
        jcall!(self.rtsLayerUtil).upgrade(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    pub fn isCompleted(&mut self) -> bool {
        if self.hackVerticleBuild < self.BUILD_VALUE {
            return false;
        } else {
            return true;
        }
    }
    

    // throws Exception
    pub fn build(&mut self) {
        if jcall!(self.buildFrameTimeHelper).isTimeTNT() {
            self.animationInterface = self.buildAnimationInterface.clone();
            jcall!(self.verticleBuildAnimationInterface).nextFrame();
            self.hackVerticleBuild += 1;
        }
        self.percentCompleteP = 100 * self.hackVerticleBuild / self.BUILD_VALUE;
        jcall!(self.getHudPaintable()).updateInfo();
    }
    

    pub fn isSelfUpgradeable(&mut self) -> bool {
        return true;
    }
    

    pub fn getCost(&mut self) -> i32 {
        return jcall!(self.rtsLayerUtil).getCost(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    pub fn getDowngradeCost(&mut self) -> i32 {
        return jcall!(self.rtsLayerUtil).getDowngradeCost(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    pub fn getUpgradeCost(&mut self) -> i32 {
        return jcall!(self.rtsLayerUtil).getUpgradeCost(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    pub fn isUpgradeable(&mut self) -> bool {
        if self.getLevel() < self.getMaxLevel() {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn isDowngradeable(&mut self) -> bool {
        if self.getLevel() > 1 {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn getLevel(&mut self) -> i32 {
        return self.level.clone();
    }
    

    pub fn setLevel(&mut self, mut level: i32) {
        self.level = level.clone();
    }
    

    pub fn getVerticleBuildAnimationInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return self.verticleBuildAnimationInterface.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
    }
    

    pub fn getAnimationInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return self.animationInterface.clone();
    }
    

    pub fn setAnimationInterface(&mut self, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) {
        self.animationInterface = animationInterface.clone();
    }
    

    // throws Exception
    pub fn setTarget(&mut self, mut targetGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        let mut anotherTargetDistance: i32 = jcall!(self.layerDistanceUtil).getDistance((self).clone() /* as AllBinaryLayer */, (targetGameLayer).clone() /* as AllBinaryLayer */);
        let mut waypointBehaviorBase: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>> = self.getWaypointBehavior();
        jcall!(waypointBehaviorBase).setTargetWithDistance((targetGameLayer).clone() /* as PathFindingLayerInterface */, anotherTargetDistance.clone());
    }
    

    pub fn createHudPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getHudPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getMaxLevel(&mut self) -> i32 {
        return self.maxLevel.clone();
    }
    

    pub fn setMaxLevel(&mut self, mut maxLevel: i32) {
        self.maxLevel = maxLevel.clone();
    }
    

    pub fn getHealthInterface(&mut self) -> Option<Rc<RefCell<Health>>> {
        return self.healthInterface.clone();
    }
    

    pub fn setHealthInterface(&mut self, mut healthInterface: Option<Rc<RefCell<Health>>>) {
        self.healthInterface = healthInterface.clone();
    }
    

    pub fn getRTSFormInput(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn>>> {
        return self.rtsFormInput.clone();
    }
    

    // throws Exception
    pub fn isDestroyed(&mut self) -> bool {
        return self.destroyed.clone();
    }
    

    // throws Exception
    pub fn setDestroyed(&mut self, mut destroyed: bool) {
        self.destroyed = destroyed.clone();
        if self.isDestroyed() {
            jcall!(DestroyedLayerProcessor::getInstance()).add(None /* Java this: a &mut self method has no Rc of itself */);
            jcall!(ViewPositionEventHandler::getInstance()).removeListener(None /* Java this: a &mut self method has no Rc of itself */);
        }
    }
    

    pub fn getEndGeographicMapCellPositionList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return jcall!(self.geographicMapCellPositionAreaBase).getOccupyingGeographicMapCellPositionList();
    }
    

    pub fn getGeographicMapCellPositionArea(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::GeographicMapCellPositionAreaBase::GeographicMapCellPositionAreaBaseDyn>>> {
        return self.geographicMapCellPositionAreaBase.clone();
    }
    

    pub fn shouldHandleStartSameAsEnd(&mut self) -> bool {
        return true;
    }
    

    // throws Exception
    pub fn handleCost(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
    

    pub fn getWaypointBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getParentLayer(&mut self) -> Option<Rc<RefCell<dyn PathFindingLayerInterface>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getRTSLayer2LogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer2LogHelper::RTSLayer2LogHelperDyn>>> {
        return self.rtsLayer2LogHelper.clone();
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
        return false;
    }
    

    pub fn getCaptionAnimationHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::caption::CaptionAnimationHelperBase::CaptionAnimationHelperBaseDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn isShowMoreCaptionStates(&mut self) -> bool {
        return self.showMoreCaptionStates.clone();
    }
    

    // throws Exception
    pub fn init(&mut self, mut geographicMapCellHistory: Option<Rc<RefCell<GeographicMapCellHistory>>>, mut geographicMapCellPositionBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
    }
    

    // throws Exception
    pub fn getCurrentGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn getTopLeftGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getMoveOutOfBuildAreaPath(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn setClosestGeographicMapCellHistory(&mut self, mut pathsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
    }
    

    pub fn teleportTo(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) {
    }
    

    // throws Exception
    pub fn setLoad(&mut self, mut resource: i32) {
    }
    

    // throws Exception
    pub fn getSurroundingGeographicMapCellPositionList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn trackTo(&mut self, mut reason: String) {
    }
    

    // throws Exception
    pub fn trackToDXY(&mut self, mut dx: i32, mut dy: i32) {
    }
    

    pub fn isWaypointListEmptyOrOnlyTargets(&mut self) -> bool {
        return false;
    }
    

    pub fn getTrackingEvent(&mut self) -> Option<Rc<RefCell<TrackingEvent>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn buildingChase(&mut self, mut allbinaryLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> bool {
        return false;
    }
    

    pub fn allStop(&mut self) {
    }
    

    pub fn getMinStaticType() -> i32 {
        return 1;
    }
    

    pub fn getMaxStaticType() -> i32 {
        return 4;
    }
    

    pub fn isRTSLayer(mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> bool {
        if jcall!(layerInterface).getType() >= RTSLayer::getMinStaticType() && jcall!(layerInterface).getType() <= RTSLayer::getMaxStaticType() {
            return true;
        }
        return false;
    }
    

    pub fn implmentsTickableInterface(&mut self) -> bool {
        return true;
    }
    

    pub fn implmentsGameInputInterface(&mut self) -> bool {
        return true;
    }
    

    pub fn getRootName(&mut self) -> String {
        return self.rootName.clone();
    }
    

    pub fn getRotationAnimationInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>> {
        return (self.indexedButShouldBeRotationAnimationInterface).clone() /* as RotationAnimation */;
    }
    

    pub fn setFrame(&mut self, mut index: i32) {
        jcall!(self.getRotationAnimationInterface()).setFrame(index.clone());
    }
    

    pub fn setRotationAnimationInterface(&mut self, mut rotationAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>>) {
        self.indexedButShouldBeRotationAnimationInterface = rotationAnimationInterface.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>);
    }
}
impl std::ops::Deref for RTSLayer {
    type Target = MultiPlayerGameLayer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RTSLayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait RTSLayerDyn: crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayerDyn + crate::org::allbinary::game::layer::RTSInterface::RTSInterface + crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface {
    fn as_RTSLayer(&self) -> &RTSLayer;
    fn as_RTSLayer_mut(&mut self) -> &mut RTSLayer;
    fn updateWaypointBehavior(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) { RTSLayer::updateWaypointBehavior(self.as_RTSLayer_mut(), geographicMapInterface) }
    fn construct(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>) { RTSLayer::construct(self.as_RTSLayer_mut(), rtsPlayerLayerInterface) }
    fn select(&mut self) { RTSLayer::select(self.as_RTSLayer_mut()) }
    fn deselect(&mut self) { RTSLayer::deselect(self.as_RTSLayer_mut()) }
    fn setSelected(&mut self, mut selected: bool) { RTSLayer::setSelected(self.as_RTSLayer_mut(), selected) }
    fn isSelected(&mut self) -> bool { RTSLayer::isSelected(self.as_RTSLayer_mut()) }
    fn getScrollSelectionForm(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> { RTSLayer::getScrollSelectionForm(self.as_RTSLayer_mut()) }
    fn initInputProcessors(&mut self) { RTSLayer::initInputProcessors(self.as_RTSLayer_mut()) }
    fn processBuiltTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { RTSLayer::processBuiltTick(self.as_RTSLayer_mut(), allBinaryLayerManager) }
    fn reload(&mut self) { RTSLayer::reload(self.as_RTSLayer_mut()) }
    fn onMovement(&mut self, mut trackingEvent: Option<Rc<RefCell<TrackingEvent>>>) { RTSLayer::onMovement(self.as_RTSLayer_mut(), trackingEvent) }
    fn onMovementFound(&mut self, mut trackingEvent: Option<Rc<RefCell<TrackingEvent>>>) { RTSLayer::onMovementFound(self.as_RTSLayer_mut(), trackingEvent) }
    fn build(&mut self) { RTSLayer::build(self.as_RTSLayer_mut()) }
    fn setLevel(&mut self, mut level: i32) { RTSLayer::setLevel(self.as_RTSLayer_mut(), level) }
    fn getVerticleBuildAnimationInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { RTSLayer::getVerticleBuildAnimationInterface(self.as_RTSLayer_mut()) }
    fn getAnimationInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { RTSLayer::getAnimationInterface(self.as_RTSLayer_mut()) }
    fn setAnimationInterface(&mut self, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) { RTSLayer::setAnimationInterface(self.as_RTSLayer_mut(), animationInterface) }
    fn setTarget(&mut self, mut targetGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { RTSLayer::setTarget(self.as_RTSLayer_mut(), targetGameLayer) }
    fn createHudPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn>>> { RTSLayer::createHudPaintable(self.as_RTSLayer_mut()) }
    fn getHudPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn>>> { RTSLayer::getHudPaintable(self.as_RTSLayer_mut()) }
    fn setMaxLevel(&mut self, mut maxLevel: i32) { RTSLayer::setMaxLevel(self.as_RTSLayer_mut(), maxLevel) }
    fn getHealthInterface(&mut self) -> Option<Rc<RefCell<Health>>> { RTSLayer::getHealthInterface(self.as_RTSLayer_mut()) }
    fn setHealthInterface(&mut self, mut healthInterface: Option<Rc<RefCell<Health>>>) { RTSLayer::setHealthInterface(self.as_RTSLayer_mut(), healthInterface) }
    fn getRTSFormInput(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn>>> { RTSLayer::getRTSFormInput(self.as_RTSLayer_mut()) }
    fn setDestroyed(&mut self, mut destroyed: bool) { RTSLayer::setDestroyed(self.as_RTSLayer_mut(), destroyed) }
    fn getEndGeographicMapCellPositionList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { RTSLayer::getEndGeographicMapCellPositionList(self.as_RTSLayer_mut()) }
    fn getGeographicMapCellPositionArea(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::GeographicMapCellPositionAreaBase::GeographicMapCellPositionAreaBaseDyn>>> { RTSLayer::getGeographicMapCellPositionArea(self.as_RTSLayer_mut()) }
    fn shouldHandleStartSameAsEnd(&mut self) -> bool { RTSLayer::shouldHandleStartSameAsEnd(self.as_RTSLayer_mut()) }
    fn handleCost(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { RTSLayer::handleCost(self.as_RTSLayer_mut(), ownerLayer) }
    fn getWaypointBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>> { RTSLayer::getWaypointBehavior(self.as_RTSLayer_mut()) }
    fn getParentLayer(&mut self) -> Option<Rc<RefCell<dyn PathFindingLayerInterface>>> { RTSLayer::getParentLayer(self.as_RTSLayer_mut()) }
    fn getRTSLayer2LogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer2LogHelper::RTSLayer2LogHelperDyn>>> { RTSLayer::getRTSLayer2LogHelper(self.as_RTSLayer_mut()) }
    fn getWaypointLogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointLogHelper::WaypointLogHelperDyn>>> { RTSLayer::getWaypointLogHelper(self.as_RTSLayer_mut()) }
    fn getWaypoint2LogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::Waypoint2LogHelper::Waypoint2LogHelperDyn>>> { RTSLayer::getWaypoint2LogHelper(self.as_RTSLayer_mut()) }
    fn getWaypointRunnableLogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointRunnableLogHelper::WaypointRunnableLogHelperDyn>>> { RTSLayer::getWaypointRunnableLogHelper(self.as_RTSLayer_mut()) }
    fn shouldAddWaypointFromBuilding(&mut self) -> bool { RTSLayer::shouldAddWaypointFromBuilding(self.as_RTSLayer_mut()) }
    fn getCaptionAnimationHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::caption::CaptionAnimationHelperBase::CaptionAnimationHelperBaseDyn>>> { RTSLayer::getCaptionAnimationHelper(self.as_RTSLayer_mut()) }
    fn isShowMoreCaptionStates(&mut self) -> bool { RTSLayer::isShowMoreCaptionStates(self.as_RTSLayer_mut()) }
    fn init(&mut self, mut geographicMapCellHistory: Option<Rc<RefCell<GeographicMapCellHistory>>>, mut geographicMapCellPositionBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { RTSLayer::init(self.as_RTSLayer_mut(), geographicMapCellHistory, geographicMapCellPositionBasicArrayList) }
    fn getCurrentGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { RTSLayer::getCurrentGeographicMapCellPosition(self.as_RTSLayer_mut()) }
    fn getTopLeftGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { RTSLayer::getTopLeftGeographicMapCellPosition(self.as_RTSLayer_mut()) }
    fn getMoveOutOfBuildAreaPath(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { RTSLayer::getMoveOutOfBuildAreaPath(self.as_RTSLayer_mut(), geographicMapCellPosition) }
    fn setClosestGeographicMapCellHistory(&mut self, mut pathsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { RTSLayer::setClosestGeographicMapCellHistory(self.as_RTSLayer_mut(), pathsList) }
    fn teleportTo(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) { RTSLayer::teleportTo(self.as_RTSLayer_mut(), geographicMapCellPosition) }
    fn setLoad(&mut self, mut resource: i32) { RTSLayer::setLoad(self.as_RTSLayer_mut(), resource) }
    fn getSurroundingGeographicMapCellPositionList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { RTSLayer::getSurroundingGeographicMapCellPositionList(self.as_RTSLayer_mut()) }
    fn trackTo(&mut self, mut reason: String) { RTSLayer::trackTo(self.as_RTSLayer_mut(), reason) }
    fn trackToDXY(&mut self, mut dx: i32, mut dy: i32) { RTSLayer::trackToDXY(self.as_RTSLayer_mut(), dx, dy) }
    fn isWaypointListEmptyOrOnlyTargets(&mut self) -> bool { RTSLayer::isWaypointListEmptyOrOnlyTargets(self.as_RTSLayer_mut()) }
    fn getTrackingEvent(&mut self) -> Option<Rc<RefCell<TrackingEvent>>> { RTSLayer::getTrackingEvent(self.as_RTSLayer_mut()) }
    fn buildingChase(&mut self, mut allbinaryLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> bool { RTSLayer::buildingChase(self.as_RTSLayer_mut(), allbinaryLayer, cellPosition) }
    fn allStop(&mut self) { RTSLayer::allStop(self.as_RTSLayer_mut()) }
    fn getRootName(&mut self) -> String { RTSLayer::getRootName(self.as_RTSLayer_mut()) }
    fn getRotationAnimationInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>> { RTSLayer::getRotationAnimationInterface(self.as_RTSLayer_mut()) }
    fn setFrame(&mut self, mut index: i32) { RTSLayer::setFrame(self.as_RTSLayer_mut(), index) }
    fn setRotationAnimationInterface(&mut self, mut rotationAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>>) { RTSLayer::setRotationAnimationInterface(self.as_RTSLayer_mut(), rotationAnimationInterface) }
}
impl RTSLayerDyn for RTSLayer {
    fn as_RTSLayer(&self) -> &RTSLayer { self }
    fn as_RTSLayer_mut(&mut self) -> &mut RTSLayer { self }
}

impl crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn for RTSLayer {
    fn as_CollidableCompositeLayer(&self) -> &crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer { &self.base.base.base }
    fn as_CollidableCompositeLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer { &mut self.base.base.base }
}
impl crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn for RTSLayer {
    fn as_AllBinaryGameLayer(&self) -> &crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &self.base.base.base.base }
    fn as_AllBinaryGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &mut self.base.base.base.base }
    fn processInput(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processInput(self, layerManager) }
    fn processTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processTick(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn for RTSLayer {
    fn as_CollidableDestroyableDamageableLayer(&self) -> &crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer { &self.base.base }
    fn as_CollidableDestroyableDamageableLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer { &mut self.base.base }
    fn damage(&mut self, mut damage: i32, mut damageType: i32) { Self::damage(self, damage, damageType) }
    fn isDestroyed(&mut self) -> bool { Self::isDestroyed(self) }
    fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) { Self::setAllBinaryGameLayerManager(self, allBinaryGameLayerManager) }
}
impl crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn for RTSLayer {
    fn as_AllBinaryLayer(&self) -> &crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &self.base.base.base.base.base }
    fn as_AllBinaryLayer_mut(&mut self) -> &mut crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &mut self.base.base.base.base.base }
    fn onChangeEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::view::event::ViewPositionEvent::ViewPositionEvent>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::onChangeEvent(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
    fn implmentsTickableInterface(&mut self) -> bool { Self::implmentsTickableInterface(self) }
    fn implmentsGameInputInterface(&mut self) -> bool { Self::implmentsGameInputInterface(self) }
    fn implmentsCollidableInterface(&mut self) -> bool { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::implmentsCollidableInterface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
    fn toString(&mut self) -> String { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::toString(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
}
impl crate::org::allbinary::layer::Layer::LayerDyn for RTSLayer {
    fn as_Layer(&self) -> &crate::org::allbinary::layer::Layer::Layer { &self.base.base.base.base.base.base }
    fn as_Layer_mut(&mut self) -> &mut crate::org::allbinary::layer::Layer::Layer { &mut self.base.base.base.base.base.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn toStringAppend(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::logic::string::StringMaker::StringMaker>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::toStringAppend(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayerDyn for RTSLayer {
    fn as_MultiPlayerGameLayer(&self) -> &crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer { &self.base }
    fn as_MultiPlayerGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer { &mut self.base }
}
impl crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface for RTSLayer {
    fn getCollidableInferface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>> { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::getCollidableInferface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
}
impl crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayerInterface::MultiPlayerGameLayerInterface for RTSLayer {
    fn getMultiplayerBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::multiplayer::layer::MultiplayerBehavior::MultiplayerBehaviorDyn>>> { crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer::getMultiplayerBehavior(crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayerDyn::as_MultiPlayerGameLayer_mut(self)) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for RTSLayer {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::set(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::layer::pickup::PickupCompositeInterface::PickupCompositeInterface for RTSLayer {
    fn getPickupBehavior(&mut self) -> Option<Rc<RefCell<crate::org::allbinary::game::layer::pickup::PickupBehavior::PickupBehavior>>> { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::getPickupBehavior(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
    fn setPickupBehavior(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::game::layer::pickup::PickupBehavior::PickupBehavior>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::setPickupBehavior(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1) }
}


