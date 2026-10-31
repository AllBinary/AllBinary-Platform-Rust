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
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInput;
use crate::org::allbinary::game::layer::MultiSelectPaintable::MultiSelectPaintable;
use crate::org::allbinary::game::layer::RTSGameStrings::RTSGameStrings;
use crate::org::allbinary::game::layer::RTSLayer::RTSLayer;
use crate::org::allbinary::game::layer::RTSLayerInfoPaintable::RTSLayerInfoPaintable;
use crate::org::allbinary::game::layer::RTSPlayerLayerInterface::RTSPlayerLayerInterface;
use crate::org::allbinary::game::state::GameStateFactory::GameStateFactory;
use crate::org::allbinary::input::motion::button::EndLevelNoBuildingSelectedTouchButtonsBuilder::EndLevelNoBuildingSelectedTouchButtonsBuilder;
use crate::org::allbinary::input::motion::button::EndLevelTouchButtonsBuilder::EndLevelTouchButtonsBuilder;
use crate::org::allbinary::input::motion::button::NoBuildingSelectedTouchButtonsBuilder::NoBuildingSelectedTouchButtonsBuilder;
use crate::org::allbinary::input::motion::button::RTSTouchButtonsBuilder::RTSTouchButtonsBuilder;
use crate::org::allbinary::media::audio::SelectBuildingSound::SelectBuildingSound;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::java::bool::BooleanFactory::BooleanFactory;
use crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::layer::hud::event::GameNotificationEvent::GameNotificationEvent;
use crate::org::allbinary::game::layer::hud::event::GameNotificationEventHandler::GameNotificationEventHandler;
use crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionForm;
use crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem;
use crate::org::allbinary::graphics::paint::NullInitUpdatePaintable::NullInitUpdatePaintable;
use crate::org::allbinary::input::motion::button::TouchButtonsBuilderFactory::TouchButtonsBuilderFactory;
use crate::org::allbinary::input::motion::gesture::MotionGestureInput::MotionGestureInput;
use crate::org::allbinary::input::motion::gesture::TouchMotionGestureFactory::TouchMotionGestureFactory;
use crate::org::allbinary::input::motion::gesture::observer::BaseMotionGestureEventListener::BaseMotionGestureEventListener;
use crate::org::allbinary::input::motion::gesture::observer::BasicMotionGesturesHandler::BasicMotionGesturesHandler;
use crate::org::allbinary::input::motion::gesture::observer::MotionGestureEvent::MotionGestureEvent;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::math::RectangleCollisionUtil::RectangleCollisionUtil;
use crate::org::allbinary::media::audio::ErrorSound::ErrorSound;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCompositeInterface::GeographicMapCompositeInterface;
use crate::org::allbinary::view::ViewPositionBase::ViewPositionBase;
use super::RTSPlayerGameInput::RTSPlayerGameInput;
use super::LayerPositionFinderInterface::LayerPositionFinderInterface;
use super::SelectRTSLayerVisitorFactoryInterface::SelectRTSLayerVisitorFactoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HumanRTSPlayerGameInput {
    pub base: RTSPlayerGameInput,
    pub isDragging: bool,
    pub gameStateFactory: Option<Rc<RefCell<GameStateFactory>>>,
    pub rectangleCollisionUtil: Option<Rc<RefCell<RectangleCollisionUtil>>>,
    pub touchMotionGestureFactory: Option<Rc<RefCell<TouchMotionGestureFactory>>>,
    pub PRESSED: Option<Rc<RefCell<MotionGestureInput>>>,
    pub RELEASED: Option<Rc<RefCell<MotionGestureInput>>>,
    pub touchButtonsBuilderFactory: Option<Rc<RefCell<TouchButtonsBuilderFactory>>>,
    pub notYoursGameNotificationEvent: Option<Rc<RefCell<GameNotificationEvent>>>,
    pub selectedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>,
    pub startPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>,
    pub endPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>,
    pub POSSIBLE: String,
    pub AT: String,
    pub METHOD: String,
    pub ADDING: String,
    pub SPACE: String,
    pub multiSelectPaintable: Option<Rc<RefCell<MultiSelectPaintable>>>,
}

// impl BaseMotionGestureEventListener for HumanRTSPlayerGameInput with the methods of the trait
impl crate::org::allbinary::game::input::event::GameKeyEventListenerInterface::GameKeyEventListenerInterface for HumanRTSPlayerGameInput {
}

impl HumanRTSPlayerGameInput {
    

    pub fn new_struct(mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>, mut inputList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut playerInputId: i32, mut towerInfoPaintable: Option<Rc<RefCell<RTSLayerInfoPaintable>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerPositionFinderInterface: Option<Rc<RefCell<dyn LayerPositionFinderInterface>>>, mut selectRTSLayerVisitorFactoryInterface: Option<Rc<RefCell<dyn SelectRTSLayerVisitorFactoryInterface>>>, mut touchButtonsBuilderFactory: Option<Rc<RefCell<TouchButtonsBuilderFactory>>>) -> Self {
        let mut this = Self {
            base: RTSPlayerGameInput::new_struct(gameCanvas.clone(), inputList.clone(), playerInputId.clone(), towerInfoPaintable.clone(), rtsPlayerLayerInterface.clone(), layerPositionFinderInterface.clone(), selectRTSLayerVisitorFactoryInterface.clone()),
            isDragging: false,
            gameStateFactory: None,
            rectangleCollisionUtil: None,
            touchMotionGestureFactory: None,
            PRESSED: None,
            RELEASED: None,
            touchButtonsBuilderFactory: None,
            notYoursGameNotificationEvent: None,
            selectedRtsLayer: None,
            startPoint: None,
            endPoint: None,
            POSSIBLE: String::new(),
            AT: String::new(),
            METHOD: String::new(),
            ADDING: String::new(),
            SPACE: String::new(),
            multiSelectPaintable: None,
        };
        this.isDragging = false;
        this.gameStateFactory = GameStateFactory::getInstance();
        this.rectangleCollisionUtil = RectangleCollisionUtil::getInstance();
        this.touchMotionGestureFactory = TouchMotionGestureFactory::getInstance();
        this.PRESSED = jcall!(this.touchMotionGestureFactory).PRESSED.clone();
        this.RELEASED = jcall!(this.touchMotionGestureFactory).RELEASED.clone();
        this.selectedRtsLayer = CollidableDestroyableDamageableLayer::getNullInstance();
        this.startPoint = jcall!(PointFactory::getInstance()).ZERO_ZERO.clone();
        this.endPoint = jcall!(PointFactory::getInstance()).ZERO_ZERO.clone();
        this.POSSIBLE = String::from("Possible: ");
        this.AT = String::from("At: ");
        this.METHOD = String::from("makeSelection");
        this.ADDING = String::from("Adding: ");
        this.SPACE = jcall!(CommonSeps::getInstance()).SPACE.clone();
        this.multiSelectPaintable = MultiSelectPaintable::new();
        // super(...) is in the struct literal above
        this.touchButtonsBuilderFactory = touchButtonsBuilderFactory.clone();
        let mut basicColorFactory: Option<Rc<RefCell<BasicColorFactory>>> = BasicColorFactory::getInstance();
        this
    }

    pub fn new(mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>, mut inputList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut playerInputId: i32, mut towerInfoPaintable: Option<Rc<RefCell<RTSLayerInfoPaintable>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerPositionFinderInterface: Option<Rc<RefCell<dyn LayerPositionFinderInterface>>>, mut selectRTSLayerVisitorFactoryInterface: Option<Rc<RefCell<dyn SelectRTSLayerVisitorFactoryInterface>>>, mut touchButtonsBuilderFactory: Option<Rc<RefCell<TouchButtonsBuilderFactory>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(gameCanvas, inputList, playerInputId, towerInfoPaintable, rtsPlayerLayerInterface, layerPositionFinderInterface, selectRTSLayerVisitorFactoryInterface, touchButtonsBuilderFactory))));
            this.notYoursGameNotificationEvent = GameNotificationEvent::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(RTSGameStrings::getInstance()).NOT_YOURS.clone(), jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(2), jcall!(basicColorFactory).PINK.clone(), jcall!(BooleanFactory::getInstance()).FALSE.clone());
            jcall!(BasicMotionGesturesHandler::getInstance()).addListenerInterface((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    // throws Exception
    pub fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) {
        let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (allBinaryGameLayerManager).clone() /* as GeographicMapCompositeInterface */;
        let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
        { let java_receiver = self.notYoursGameNotificationEvent.clone(); let java_arguments = (jcall!(geographicMapInterface).getForegroundBasicColor(),); jcall!(java_receiver).setBasicColorP(java_arguments.0); };
    }
    

    // throws Exception
    pub fn processDraggingMotionInput(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) -> bool {
        let mut motionGestureEvent: Option<Rc<RefCell<MotionGestureEvent>>> = (jcall!(self.getMotionGestureInputList()).get(jcall!(self.getMotionGestureInputList()).size() - 1)).clone() /* as MotionGestureEvent */;
        let mut motionGestureInput: Option<Rc<RefCell<MotionGestureInput>>> = jcall!(motionGestureEvent).getMotionGesture();
        let mut rtsFormInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn>>> = self.getSelectedRtsFormInput();
        if match (&motionGestureInput, &self.PRESSED) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(motionGestureEvent).getCurrentPoint();
            if jcall!(jcall!(self.getRtsPlayerLayerInterface()).getCurrentScrollSelectionForm()).isInForm(point.clone()) {
                self.isDragging = jcall!(rtsFormInput).processSticky(self.getSelectedRtsLayer(), self.getRtsPlayerLayerInterface(), layerManager.clone(), point.clone());
            }
        } else if match (&motionGestureInput, &self.RELEASED) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(motionGestureEvent).getCurrentPoint();
            if jcall!(jcall!(self.getRtsPlayerLayerInterface()).getCurrentScrollSelectionForm()).isInForm(point.clone()) {
                jcall!(rtsFormInput).processAtPoint(self.getSelectedRtsLayer(), self.getRtsPlayerLayerInterface(), layerManager.clone(), point.clone());
            } else if jcall!(rtsFormInput).isStickyItemSelected() {
                let mut previousRtsFormInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn>>> = rtsFormInput.clone();
                self.select(motionGestureEvent.clone());
                jcall!(self.getSelectedBuildingPlayerGameInput()).selectAllPreselected();
                jcall!(previousRtsFormInput).processAtPoint(self.getSelectedRtsLayer(), self.getRtsPlayerLayerInterface(), layerManager.clone(), point.clone());
                jcall!(self.getSelectedBuildingPlayerGameInput()).deselectAllPreselected();
            } else {
                self.select(motionGestureEvent.clone());
            }
            jcall!(rtsFormInput).setStickyItemSelected(false);
            jcall!(rtsFormInput).setSelectedStickyItem(ABCustomItem::getNullInstance());
            jcall!(rtsFormInput).setSelectedStickyItemIndex(-1);
            return true;
        }
        return false;
    }
    

    pub fn makeSelection(&mut self) {
        let mut rtsLayerList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(jcall!(self.getRtsPlayerLayerInterface()).getPlayerOwnedRTSLayers()).rtsLayerList.clone();
        let mut rectX1: i32 = jcall!(self.startPoint).getX();
        let mut rectY1: i32 = jcall!(self.startPoint).getY();
        let mut rectX2: i32 = jcall!(self.endPoint).getX();
        let mut rectY2: i32 = jcall!(self.endPoint).getY();
        if rectX1 > rectX2 {
            rectX2 = rectX1.clone();
            rectX1 = jcall!(self.endPoint).getX();
        }
        if rectY1 > rectY2 {
            rectY2 = rectY1.clone();
            rectY1 = jcall!(self.endPoint).getY();
        }
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(self.AT.clone());
        jcall!(stringBuffer).appendint(rectX1.clone());
        jcall!(stringBuffer).append(self.SPACE.clone());
        jcall!(stringBuffer).appendint(rectY1.clone());
        jcall!(stringBuffer).append(self.SPACE.clone());
        jcall!(stringBuffer).appendint(rectX2.clone());
        jcall!(stringBuffer).append(self.SPACE.clone());
        jcall!(stringBuffer).appendint(rectY2.clone());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>> = None;
        for index in (0..=jcall!(rtsLayerList).size() - 1).rev() {
            rtsLayer = { let java_object = (jcall!(rtsLayerList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>>().expect("ClassCastException").clone()) };
            jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
            let mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>> = jcall!(rtsLayer).getViewPosition();
            jcall!(stringBuffer).append(self.POSSIBLE.clone());
            jcall!(stringBuffer).append(jcall!(rtsLayer).getName());
            jcall!(stringBuffer).append(self.SPACE.clone());
            jcall!(stringBuffer).append(self.AT.clone());
            jcall!(stringBuffer).appendint((jcall!(viewPosition).getX() + jcall!(rtsLayer).getHalfWidth()).clone());
            jcall!(stringBuffer).append(self.SPACE.clone());
            jcall!(stringBuffer).appendint((jcall!(viewPosition).getY() + jcall!(rtsLayer).getHalfHeight()).clone());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            if { let java_receiver = self.rectangleCollisionUtil.clone(); let java_arguments = (rectX1.clone(), rectY1.clone(), rectX2.clone(), rectY2.clone(), jcall!(viewPosition).getX() + jcall!(rtsLayer).getHalfWidth(), jcall!(viewPosition).getY() + jcall!(rtsLayer).getHalfHeight(),); let java_result = jcall!(java_receiver).isInside(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4, java_arguments.5); java_result } {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(self.ADDING.clone()).append(jcall!(rtsLayer).getName()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.METHOD.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                jcall!(rtsLayer).select();
                jcall!(self.getSelectedBuildingPlayerGameInput()).addSelectedRTSLayer(rtsLayer.clone());
            }
        }
        let mut gameLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>> = jcall!(self.getSelectedBuildingPlayerGameInput()).getLastSelectedRtsLayer();
        if !match (&gameLayer, &CollidableDestroyableDamageableLayer::getNullInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut lastRTSLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>> = (gameLayer).clone() /* as RTSLayer */;
            if !match (&jcall!(lastRTSLayer).getScrollSelectionForm(), &ScrollSelectionForm::getNullScrollSelectionForm()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                { let java_arguments = (jcall!(lastRTSLayer).getScrollSelectionForm(), jcall!(lastRTSLayer).getRTSFormInput(),); self.updateForm(java_arguments.0, java_arguments.1) };
                self.setSelectedRtsLayer(lastRTSLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>));
            }
        }
        self.updatePaintable();
    }
    

    // throws Exception
    pub fn processSelectionBoxMotionInput(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        for index in (0..=jcall!(self.getMotionGestureInputList()).size() - 1).rev() {
            let mut motionGestureEvent: Option<Rc<RefCell<MotionGestureEvent>>> = (jcall!(self.getMotionGestureInputList()).get(index.clone())).clone() /* as MotionGestureEvent */;
            let mut motionGestureInput: Option<Rc<RefCell<MotionGestureInput>>> = jcall!(motionGestureEvent).getMotionGesture();
            let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(motionGestureEvent).getCurrentPoint();
            if match (&motionGestureInput, &jcall!(self.touchMotionGestureFactory).PRESSED) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                self.startPoint = point.clone();
                self.endPoint = point.clone();
            } else if match (&motionGestureInput, &jcall!(self.touchMotionGestureFactory).RELEASED) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                self.endPoint = point.clone();
                self.makeSelection();
                self.startPoint = jcall!(PointFactory::getInstance()).ZERO_ZERO.clone();
                self.endPoint = jcall!(PointFactory::getInstance()).ZERO_ZERO.clone();
            } else {
                self.endPoint = point.clone();
            }
        }
    }
    

    // throws Exception
    pub fn processMotionInput(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        if jcall!(self.getMotionGestureInputList()).size() > 0 {
            let mut endDrag: bool = self.processDraggingMotionInput(layerManager.clone());
            if !self.isDragging {
                self.processSelectionBoxMotionInput(layerManager.clone());
            }
            if endDrag {
                self.isDragging = false;
            }
        }
        jcall!(self.getMotionGestureInputList()).clear();
    }
    

    pub fn setSelectedRtsFormInput(&mut self, mut selectedRtsFormInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn>>>) {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(CommonLabels::getInstance()).START.clone()).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(selectedRtsFormInput.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setSelectedRtsFormInput"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        self.base.setSelectedRtsFormInput(selectedRtsFormInput.clone());
        { let java_arguments = (CollidableDestroyableDamageableLayer::getNullInstance(),); self.setSelectedRtsLayer(java_arguments.0) };
    }
    

    pub fn updateFormForLayer(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) {
        let mut scrollSelectionForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> = jcall!(rtsLayer).getScrollSelectionForm();
        if !match (&scrollSelectionForm, &ScrollSelectionForm::getNullScrollSelectionForm()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } && match (&jcall!(self.getRtsPlayerLayerInterface()).getGroupInterface()[(0) as usize], &jcall!(rtsLayer).getGroupInterface()[(0) as usize]) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            { let java_arguments = (scrollSelectionForm.clone(), jcall!(rtsLayer).getRTSFormInput(),); self.updateForm(java_arguments.0, java_arguments.1) };
            jcall!(self.getSelectedBuildingPlayerGameInput()).setSelectedRTSLayer(CollidableDestroyableDamageableLayer::getNullInstance());
            self.setSelectedRtsLayer(rtsLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>));
        } else {
            jcall!(self.getRtsPlayerLayerInterface()).setCurrentScrollSelectionForm(jcall!(self.getRtsPlayerLayerInterface()).getBuildingScrollSelectionForm());
            { let java_arguments = (jcall!(self.getRtsPlayerLayerInterface()).getRTSFormInput(),); self.setSelectedRtsFormInput(java_arguments.0) };
            jcall!(self.getSelectedBuildingPlayerGameInput()).setSelectedRTSLayer(CollidableDestroyableDamageableLayer::getNullInstance());
        }
    }
    

    pub fn updateForm(&mut self, mut scrollSelectionForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>>, mut rtfFormInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn>>>) {
        jcall!(self.getRtsPlayerLayerInterface()).setCurrentScrollSelectionForm(scrollSelectionForm.clone());
        self.setSelectedRtsFormInput(rtfFormInput.clone());
    }
    

    // throws Exception
    pub fn setSelectedRTSLayer(&mut self, mut rtSLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) {
        if !match (&rtSLayer, &CollidableDestroyableDamageableLayer::getNullInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } && !match (&jcall!(self.getRtsPlayerLayerInterface()).getGroupInterface()[(0) as usize], &jcall!(rtSLayer).getGroupInterface()[(0) as usize]) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(self.getRtsPlayerLayerInterface()).add(ErrorSound::getInstance());
            jcall!(GameNotificationEventHandler::getInstance()).fireEvent(self.notYoursGameNotificationEvent.clone());
            return;
        }
        let mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>> = self.getGameCanvas();
        if !match (&rtSLayer, &CollidableDestroyableDamageableLayer::getNullInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(self.getRtsPlayerLayerInterface()).add(SelectBuildingSound::getInstance());
            let mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>> = (rtSLayer).clone() /* as RTSLayer */;
            jcall!(rtsLayer).select();
            self.updateFormForLayer(rtsLayer.clone());
            if match (&jcall!(gameCanvas).getGameState(), &jcall!(self.gameStateFactory).PLAYING_GAME_STATE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                if jcall!(rtsLayer).isSelfUpgradeable() {
                    jcall!(gameCanvas).updateCurrentTouchInputFactory(RTSTouchButtonsBuilder::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::input::motion::button::BaseTouchInput::BaseTouchInputDyn>>));
                } else {
                    jcall!(gameCanvas).updateCurrentTouchInputFactory(NoBuildingSelectedTouchButtonsBuilder::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::input::motion::button::BaseTouchInput::BaseTouchInputDyn>>));
                }
            } else {
                let mut endLevelTouchButtonsBuilder: Option<Rc<RefCell<EndLevelTouchButtonsBuilder>>> = EndLevelTouchButtonsBuilder::new();
                if self.touchButtonsBuilderFactory.is_some() {
                    jcall!(endLevelTouchButtonsBuilder).add(jcall!(self.touchButtonsBuilderFactory).getInstance());
                }
                jcall!(gameCanvas).updateCurrentTouchInputFactory(endLevelTouchButtonsBuilder.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::input::motion::button::BaseTouchInput::BaseTouchInputDyn>>));
            }
        } else {
            jcall!(self.logUtil).putF(String::from("Set Player Default Form"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setSelectedRTSLayer"));
            jcall!(self.getRtsPlayerLayerInterface()).setCurrentScrollSelectionForm(jcall!(self.getRtsPlayerLayerInterface()).getBuildingScrollSelectionForm());
            { let java_arguments = (jcall!(self.getRtsPlayerLayerInterface()).getRTSFormInput(),); self.setSelectedRtsFormInput(java_arguments.0) };
            jcall!(self.getSelectedBuildingPlayerGameInput()).setSelectedRTSLayer(CollidableDestroyableDamageableLayer::getNullInstance());
            if match (&jcall!(gameCanvas).getGameState(), &jcall!(self.gameStateFactory).PLAYING_GAME_STATE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(gameCanvas).updateCurrentTouchInputFactory(NoBuildingSelectedTouchButtonsBuilder::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::input::motion::button::BaseTouchInput::BaseTouchInputDyn>>));
            } else {
                let mut endLevelNoBuildingSelectedTouchButtonsBuilder: Option<Rc<RefCell<EndLevelNoBuildingSelectedTouchButtonsBuilder>>> = EndLevelNoBuildingSelectedTouchButtonsBuilder::new();
                if self.touchButtonsBuilderFactory.is_some() {
                    jcall!(endLevelNoBuildingSelectedTouchButtonsBuilder).add(jcall!(self.touchButtonsBuilderFactory).getInstance());
                }
                jcall!(gameCanvas).updateCurrentTouchInputFactory(endLevelNoBuildingSelectedTouchButtonsBuilder.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::input::motion::button::BaseTouchInput::BaseTouchInputDyn>>));
            }
        }
        jcall!(self.getSelectedRtsFormInput()).setSelectedGeographicCellPosition(geographicMapCellPosition.clone());
        jcall!(self.getSelectedBuildingPlayerGameInput()).setSelectedRTSLayer(rtSLayer.clone());
        self.updatePaintable();
    }
    

    pub fn updatePaintable(&mut self) {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(self.getSelectedBuildingPlayerGameInput()).getSelectedBasicArrayList();
        if jcall!(list).size() > 1 {
            let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (self.getGameCanvas()).clone() /* as GeographicMapCompositeInterface */;
            let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
            { let java_receiver = self.multiSelectPaintable.clone(); let java_arguments = (jcall!(geographicMapInterface).getForegroundBasicColor(),); jcall!(java_receiver).setBasicColorP(java_arguments.0); };
            jcall!(self.multiSelectPaintable).update(list.clone());
            jcall!(self.getRTSLayerInfoPaintable()).updateRTSLayerInfoSelection(self.multiSelectPaintable.clone());
        } else if jcall!(list).size() == 1 {
            let mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>> = { let java_object = (jcall!(list).get(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>>().expect("ClassCastException").clone()) };
            jcall!(self.getRTSLayerInfoPaintable()).updateRTSLayerInfoSelection(jcall!(rtsLayer).createHudPaintable());
        } else {
            jcall!(self.getRTSLayerInfoPaintable()).updateRTSLayerInfoInit(NullInitUpdatePaintable::getInstance());
        }
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        self.base.paint(graphics.clone());
        if !match (&self.startPoint, &jcall!(PointFactory::getInstance()).ZERO_ZERO) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } && !match (&self.endPoint, &jcall!(PointFactory::getInstance()).ZERO_ZERO) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(graphics).setColor(jcall!(jcall!(BasicColorFactory::getInstance()).RED).intValue());
            let mut rectX1: i32 = jcall!(self.startPoint).getX();
            let mut rectY1: i32 = jcall!(self.startPoint).getY();
            let mut rectX2: i32 = jcall!(self.endPoint).getX();
            let mut rectY2: i32 = jcall!(self.endPoint).getY();
            if rectX1 > rectX2 {
                rectX2 = rectX1.clone();
                rectX1 = jcall!(self.endPoint).getX();
            }
            if rectY1 > rectY2 {
                rectY2 = rectY1.clone();
                rectY1 = jcall!(self.endPoint).getY();
            }
            jcall!(graphics).drawRect(rectX1.clone(), rectY1.clone(), rectX2 - rectX1, rectY2 - rectY1);
        }
    }
    

    pub fn onMotionGestureEvent(&mut self, mut motionGestureEvent: Option<Rc<RefCell<MotionGestureEvent>>>) {
        jcall!(self.getMotionGestureInputList()).add(motionGestureEvent.clone());
    }
    

    pub fn onScrolledMotionGestureEvent(&mut self, mut motionGestureEvent: Option<Rc<RefCell<MotionGestureEvent>>>) {
    }
    

    pub fn getSelectedRtsLayer(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>> {
        return self.selectedRtsLayer.clone();
    }
    

    pub fn setSelectedRtsLayer(&mut self, mut selectedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>) {
        self.selectedRtsLayer = selectedRtsLayer.clone();
    }
}
impl std::ops::Deref for HumanRTSPlayerGameInput {
    type Target = RTSPlayerGameInput;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HumanRTSPlayerGameInput {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::input::RTSPlayerGameInput::RTSPlayerGameInputDyn for HumanRTSPlayerGameInput {
    fn as_RTSPlayerGameInput(&self) -> &crate::org::allbinary::game::input::RTSPlayerGameInput::RTSPlayerGameInput { &self.base }
    fn as_RTSPlayerGameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::RTSPlayerGameInput::RTSPlayerGameInput { &mut self.base }
    fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) { Self::setAllBinaryGameLayerManager(self, allBinaryGameLayerManager) }
    fn processMotionInput(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processMotionInput(self, layerManager) }
    fn setSelectedRTSLayer(&mut self, mut rtSLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) { Self::setSelectedRTSLayer(self, rtSLayer, geographicMapCellPosition) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn updatePaintable(&mut self) { Self::updatePaintable(self) }
    fn setSelectedRtsFormInput(&mut self, mut selectedRtsFormInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn>>>) { Self::setSelectedRtsFormInput(self, selectedRtsFormInput) }
}
impl crate::org::allbinary::game::input::GameInput::GameInputDyn for HumanRTSPlayerGameInput {
    fn as_GameInput(&self) -> &crate::org::allbinary::game::input::GameInput::GameInput { &self.base.base.base }
    fn as_GameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::GameInput::GameInput { &mut self.base.base.base }
}
impl crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn for HumanRTSPlayerGameInput {
    fn as_PlayerGameInput(&self) -> &crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &self.base.base }
    fn as_PlayerGameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &mut self.base.base }
}


