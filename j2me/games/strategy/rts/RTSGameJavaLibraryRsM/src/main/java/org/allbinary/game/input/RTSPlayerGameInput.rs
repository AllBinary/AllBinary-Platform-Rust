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
// use crate::javax::microedition::lcdui::Canvas::Canvas;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInput;
use crate::org::allbinary::game::layer::RTSLayer::RTSLayer;
use crate::org::allbinary::game::layer::RTSLayerInfoPaintable::RTSLayerInfoPaintable;
use crate::org::allbinary::game::layer::RTSPlayerLayerInterface::RTSPlayerLayerInterface;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::InputFeatureFactory::InputFeatureFactory;
use crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas;
use crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent;
use crate::org::allbinary::game::input::form::NullRTSFormInputFactory::NullRTSFormInputFactory;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use crate::org::allbinary::game::layer::NullRTSLayer::NullRTSLayer;
use crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::displayable::event::DisplayChangeEvent::DisplayChangeEvent;
use crate::org::allbinary::input::motion::gesture::observer::MotionGestureEvent::MotionGestureEvent;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::media::audio::SecondaryPlayerQueueFactory::SecondaryPlayerQueueFactory;
use crate::org::allbinary::media::audio::SelectSound::SelectSound;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCompositeInterface::GeographicMapCompositeInterface;
use crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use super::PlayerGameInput::PlayerGameInput;
use super::GameInputProcessor::GameInputProcessor;
use super::ScrollMapPlayerGameInput::ScrollMapPlayerGameInput;
use super::SelectedRTSLayersPlayerGameInput::SelectedRTSLayersPlayerGameInput;
use super::LayerPositionFinderInterface::LayerPositionFinderInterface;
use super::SelectRTSLayerVisitorFactoryInterface::SelectRTSLayerVisitorFactoryInterface;
use super::RTSPlayerLeftGameInputProcessor::RTSPlayerLeftGameInputProcessor;
use super::RTSPlayerRightGameInputProcessor::RTSPlayerRightGameInputProcessor;
use super::RTSPlayerFireGameInputProcessor::RTSPlayerFireGameInputProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RTSPlayerGameInput {
    pub base: PlayerGameInput,
    pub inputProcessorArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn>>>>,
    pub removeInputProcessorArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn>>>>,
    pub inputList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub isSingleKeyProcessing: bool,
    pub gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>,
    pub motionGestureInputList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub scrollPlayerGameInput: Option<Rc<RefCell<ScrollMapPlayerGameInput>>>,
    pub selectedRTSLayerPlayerGameInput: Option<Rc<RefCell<SelectedRTSLayersPlayerGameInput>>>,
    pub towerInfoPaintable: Option<Rc<RefCell<RTSLayerInfoPaintable>>>,
    pub rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>,
    pub layerPositionFinderInterface: Option<Rc<RefCell<dyn LayerPositionFinderInterface>>>,
    pub selectedRtsFormInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn>>>,
}

impl crate::org::allbinary::game::input::event::GameKeyEventListenerInterface::GameKeyEventListenerInterface for RTSPlayerGameInput {
}

impl RTSPlayerGameInput {
    

    pub fn new_struct(mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>, mut inputList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut playerInputId: i32, mut towerInfoPaintable: Option<Rc<RefCell<RTSLayerInfoPaintable>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerPositionFinderInterface: Option<Rc<RefCell<dyn LayerPositionFinderInterface>>>, mut selectRTSLayerVisitorFactoryInterface: Option<Rc<RefCell<dyn SelectRTSLayerVisitorFactoryInterface>>>) -> Self {
        let mut this = Self {
            base: PlayerGameInput::new_struct(inputList.clone(), BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>), playerInputId.clone()),
            inputProcessorArray: Vec::new(),
            removeInputProcessorArray: Vec::new(),
            inputList: None,
            isSingleKeyProcessing: false,
            gameCanvas: None,
            motionGestureInputList: None,
            scrollPlayerGameInput: None,
            selectedRTSLayerPlayerGameInput: None,
            towerInfoPaintable: None,
            rtsPlayerLayerInterface: None,
            layerPositionFinderInterface: None,
            selectedRtsFormInput: None,
        };
        this.inputProcessorArray = vec![None; (jcall!(InputFactory::InputFactory::getInstance()).MAX) as usize];
        this.removeInputProcessorArray = vec![None; (jcall!(InputFactory::InputFactory::getInstance()).MAX) as usize];
        this.isSingleKeyProcessing = jcall!(Features::getInstance()).isFeature(jcall!(InputFeatureFactory::getInstance()).SINGLE_KEY_REPEAT_PRESS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) || jcall!(Features::getInstance()).isFeature(jcall!(InputFeatureFactory::getInstance()).SINGLE_KEY_PRESS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>));
        this.motionGestureInputList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.selectedRtsFormInput = NullRTSFormInputFactory::getInstance();
        // super(...) is in the struct literal above
        this.initInputProcessors();
        this.gameCanvas = gameCanvas.clone();
        this.inputList = inputList.clone();
        this.towerInfoPaintable = towerInfoPaintable.clone();
        this.rtsPlayerLayerInterface = rtsPlayerLayerInterface.clone();
        this.selectedRTSLayerPlayerGameInput = SelectedRTSLayersPlayerGameInput::new(this.getRTSLayerInfoPaintable(), this.getRtsPlayerLayerInterface(), this.inputList.clone(), playerInputId.clone(), selectRTSLayerVisitorFactoryInterface.clone());
        if !match (&this.rtsPlayerLayerInterface, &crate::org::allbinary::game::layer::NullRTSLayer::NullRTSLayer_NULL_RTS_LAYER.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            { let java_arguments = (jcall!(this.rtsPlayerLayerInterface).getRTSFormInput(),); this.setSelectedRtsFormInput(java_arguments.0) };
        }
        let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (jcall!(this.gameCanvas).getLayerManager()).clone() /* as GeographicMapCompositeInterface */;
        let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
        this.scrollPlayerGameInput = ScrollMapPlayerGameInput::new(geographicMapInterface.clone(), this.inputList.clone(), playerInputId.clone());
        this.layerPositionFinderInterface = layerPositionFinderInterface.clone();
        this
    }

    pub fn new(mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>, mut inputList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut playerInputId: i32, mut towerInfoPaintable: Option<Rc<RefCell<RTSLayerInfoPaintable>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerPositionFinderInterface: Option<Rc<RefCell<dyn LayerPositionFinderInterface>>>, mut selectRTSLayerVisitorFactoryInterface: Option<Rc<RefCell<dyn SelectRTSLayerVisitorFactoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(gameCanvas, inputList, playerInputId, towerInfoPaintable, rtsPlayerLayerInterface, layerPositionFinderInterface, selectRTSLayerVisitorFactoryInterface))))
    }
    

    // throws Exception
    pub fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) {
        jcall!(self.selectedRTSLayerPlayerGameInput).setAllBinaryGameLayerManager(allBinaryGameLayerManager.clone());
        if !match (&self.selectedRtsFormInput, &NullRTSFormInputFactory::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(self.selectedRtsFormInput).setAllBinaryGameLayerManager(allBinaryGameLayerManager.clone());
        }
    }
    

    pub fn onDisplayChangeEvent(&mut self, mut displayChangeEvent: Option<Rc<RefCell<DisplayChangeEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onDisplayChangeEvent"));
            jcall!(self.getRTSLayerInfoPaintable()).update();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onDisplayChangeEvent"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn left(&mut self) {
        jcall!(jcall!(self.rtsPlayerLayerInterface).getCurrentScrollSelectionForm()).processInputKey(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() }.clone());
    }
    

    // throws Exception
    pub fn right(&mut self) {
        jcall!(jcall!(self.rtsPlayerLayerInterface).getCurrentScrollSelectionForm()).processInputKey(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() }.clone());
    }
    

    pub fn initInputProcessors(&mut self) {
        self.inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() }) as usize] = RTSPlayerLeftGameInputProcessor::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM0.clone() }) as usize] = self.inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() }) as usize].clone();
        self.inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() }) as usize] = RTSPlayerRightGameInputProcessor::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_POUND.clone() }) as usize] = self.inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() }) as usize].clone();
        self.removeInputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM1.clone() }) as usize] = RTSPlayerFireGameInputProcessor::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.removeInputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM3.clone() }) as usize] = self.removeInputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM1.clone() }) as usize].clone();
        self.removeInputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM3.clone() }) as usize] = self.removeInputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM1.clone() }) as usize].clone();
        self.removeInputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() }) as usize] = self.removeInputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM1.clone() }) as usize].clone();
        self.removeInputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() }) as usize] = self.removeInputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM1.clone() }) as usize].clone();
        self.removeInputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM0.clone() }) as usize] = self.removeInputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM1.clone() }) as usize].clone();
        self.removeInputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_POUND.clone() }) as usize] = self.removeInputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM1.clone() }) as usize].clone();
        GameInputProcessorUtil::GameInputProcessorUtil::init(self.inputProcessorArray.clone());
        GameInputProcessorUtil::GameInputProcessorUtil::init(self.removeInputProcessorArray.clone());
    }
    

    // throws Exception
    pub fn processInput(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.processMotionInput(layerManager.clone());
            let mut size: i32 = jcall!(self.inputList).size();
            let mut key: i32 = 0;
            for index in (0..size) {
                let mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>> = { let java_object = (jcall!(self.inputList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<GameKeyEvent>>>>().expect("ClassCastException").clone()) };
                key = jcall!(gameKeyEvent).getKey();
                jcall!(self.getScrollPlayerGameInput()).processInputKey(key.clone());
                jcall!(self.getSelectedBuildingPlayerGameInput()).processInputKey(key.clone());
                jcall!(self.inputProcessorArray[(key) as usize]).processEvent(layerManager.clone(), gameKeyEvent.clone());
                jcall!(self.removeInputProcessorArray[(key) as usize]).processEvent(layerManager.clone(), gameKeyEvent.clone());
            }
            if self.isIsSingleKeyProcessing() {
                self.clear();
            } else {
                self.update();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameInputStrings).PROCESS_INPUT.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn processMotionInput(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
    }
    

    // throws Exception
    pub fn select(&mut self, mut motionGestureEvent: Option<Rc<RefCell<MotionGestureEvent>>>) {
        let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(motionGestureEvent).getCurrentPoint();
        let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (jcall!(self.gameCanvas).getLayerManager()).clone() /* as GeographicMapCompositeInterface */;
        let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
        let mut allBinaryTiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(geographicMapInterface).getAllBinaryTiledLayer();
        let mut x: i32 = jcall!(point).getX() + jcall!(allBinaryTiledLayer).getXP();
        let mut y: i32 = jcall!(point).getY() + jcall!(allBinaryTiledLayer).getYP();
        let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(geographicMapInterface).getCellPositionAtXYNoThrow(x.clone(), y.clone());
        if !match (&geographicMapCellPosition, &crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(SecondaryPlayerQueueFactory::getInstance()).add(SelectSound::getInstance());
            let mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = jcall!(self.layerPositionFinderInterface).getLayerInterface(geographicMapCellPosition.clone());
            if match (&layer, &crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer_NULL_ALLBINARY_LAYER.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                layer = CollidableDestroyableDamageableLayer::getNullInstance();
            } else {
                geographicMapCellPosition = jcall!(geographicMapInterface).getCellPositionAtXY(jcall!(layer).getXP(), jcall!(layer).getYP());
            }
            let mut foundRTSLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>> = (layer).clone() /* as CollidableDestroyableDamageableLayer */;
            self.setSelectedRTSLayer(foundRTSLayer.clone(), geographicMapCellPosition.clone());
        } else {
            let mut commonLabels: Option<Rc<RefCell<CommonLabels>>> = CommonLabels::getInstance();
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Off Of Map -")).append(jcall!(commonLabels).WIDTH_LABEL.clone()).appendint(jcall!(allBinaryTiledLayer).getWidth()).append(jcall!(commonLabels).HEIGHT_LABEL.clone()).appendint(jcall!(allBinaryTiledLayer).getHeight()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("select"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
    }
    

    // throws Exception
    pub fn setSelectedRTSLayer(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) {
        jcall!(self.getSelectedBuildingPlayerGameInput()).setSelectedRTSLayer(rtsLayer.clone());
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(self.getSelectedRtsFormInput()).getSelectedGeographicCellPosition();
        let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (jcall!(self.gameCanvas).getLayerManager()).clone() /* as GeographicMapCompositeInterface */;
        let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
        let mut allBinaryTiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(geographicMapInterface).getAllBinaryTiledLayer();
        jcall!(graphics).setColor(jcall!(jcall!(BasicColorFactory::getInstance()).GREEN).intValue());
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(self.getSelectedBuildingPlayerGameInput()).getPaintSelectedRTSLayersList();
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        if jcall!(list).size() > 0 {
            for index in (0..=jcall!(list).size() - 1).rev() {
                let mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>> = { let java_object = (jcall!(list).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>>().expect("ClassCastException").clone()) };
                width = jcall!(rtsLayer).getWidth();
                height = jcall!(rtsLayer).getHeight();
                jcall!(graphics).drawRect(jcall!(rtsLayer).getXP() - jcall!(allBinaryTiledLayer).getXP(), jcall!(rtsLayer).getYP() - jcall!(allBinaryTiledLayer).getYP(), width.clone(), height.clone());
            }
        } else if !match (&geographicMapCellPosition, &crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(geographicMapCellPosition).getPoint();
            width = jcall!(allBinaryTiledLayer).getCellWidth();
            height = jcall!(allBinaryTiledLayer).getCellHeight();
            jcall!(graphics).drawRect(jcall!(point).getX() - jcall!(allBinaryTiledLayer).getXP(), jcall!(point).getY() - jcall!(allBinaryTiledLayer).getYP(), width.clone(), height.clone());
        }
    }
    

    pub fn updatePaintable(&mut self) {
    }
    

    pub fn getSelectedBuildingPlayerGameInput(&mut self) -> Option<Rc<RefCell<SelectedRTSLayersPlayerGameInput>>> {
        return self.selectedRTSLayerPlayerGameInput.clone();
    }
    

    pub fn getSelectedRtsFormInput(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn>>> {
        return self.selectedRtsFormInput.clone();
    }
    

    pub fn setSelectedRtsFormInput(&mut self, mut selectedRtsFormInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("RTSFormInput: ")).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(selectedRtsFormInput.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setSelectedRtsFormInput"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        self.selectedRtsFormInput = selectedRtsFormInput.clone();
    }
    

    pub fn getRtsPlayerLayerInterface(&mut self) -> Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>> {
        return self.rtsPlayerLayerInterface.clone();
    }
    

    pub fn getRTSLayerInfoPaintable(&mut self) -> Option<Rc<RefCell<RTSLayerInfoPaintable>>> {
        return self.towerInfoPaintable.clone();
    }
    

    pub fn getMotionGestureInputList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.motionGestureInputList.clone();
    }
    

    pub fn getGameCanvas(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>> {
        return self.gameCanvas.clone();
    }
    

    pub fn isIsSingleKeyProcessing(&mut self) -> bool {
        return self.isSingleKeyProcessing.clone();
    }
    

    pub fn getScrollPlayerGameInput(&mut self) -> Option<Rc<RefCell<ScrollMapPlayerGameInput>>> {
        return self.scrollPlayerGameInput.clone();
    }
}
impl std::ops::Deref for RTSPlayerGameInput {
    type Target = PlayerGameInput;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RTSPlayerGameInput {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait RTSPlayerGameInputDyn: crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn {
    fn as_RTSPlayerGameInput(&self) -> &RTSPlayerGameInput;
    fn as_RTSPlayerGameInput_mut(&mut self) -> &mut RTSPlayerGameInput;
    fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) { RTSPlayerGameInput::setAllBinaryGameLayerManager(self.as_RTSPlayerGameInput_mut(), allBinaryGameLayerManager) }
    fn onDisplayChangeEvent(&mut self, mut displayChangeEvent: Option<Rc<RefCell<DisplayChangeEvent>>>) { RTSPlayerGameInput::onDisplayChangeEvent(self.as_RTSPlayerGameInput_mut(), displayChangeEvent) }
    fn left(&mut self) { RTSPlayerGameInput::left(self.as_RTSPlayerGameInput_mut()) }
    fn right(&mut self) { RTSPlayerGameInput::right(self.as_RTSPlayerGameInput_mut()) }
    fn initInputProcessors(&mut self) { RTSPlayerGameInput::initInputProcessors(self.as_RTSPlayerGameInput_mut()) }
    fn processInput(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { RTSPlayerGameInput::processInput(self.as_RTSPlayerGameInput_mut(), layerManager) }
    fn processMotionInput(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { RTSPlayerGameInput::processMotionInput(self.as_RTSPlayerGameInput_mut(), layerManager) }
    fn select(&mut self, mut motionGestureEvent: Option<Rc<RefCell<MotionGestureEvent>>>) { RTSPlayerGameInput::select(self.as_RTSPlayerGameInput_mut(), motionGestureEvent) }
    fn setSelectedRTSLayer(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) { RTSPlayerGameInput::setSelectedRTSLayer(self.as_RTSPlayerGameInput_mut(), rtsLayer, geographicMapCellPosition) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { RTSPlayerGameInput::paint(self.as_RTSPlayerGameInput_mut(), graphics) }
    fn updatePaintable(&mut self) { RTSPlayerGameInput::updatePaintable(self.as_RTSPlayerGameInput_mut()) }
    fn getSelectedBuildingPlayerGameInput(&mut self) -> Option<Rc<RefCell<SelectedRTSLayersPlayerGameInput>>> { RTSPlayerGameInput::getSelectedBuildingPlayerGameInput(self.as_RTSPlayerGameInput_mut()) }
    fn getSelectedRtsFormInput(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn>>> { RTSPlayerGameInput::getSelectedRtsFormInput(self.as_RTSPlayerGameInput_mut()) }
    fn setSelectedRtsFormInput(&mut self, mut selectedRtsFormInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn>>>) { RTSPlayerGameInput::setSelectedRtsFormInput(self.as_RTSPlayerGameInput_mut(), selectedRtsFormInput) }
    fn getRtsPlayerLayerInterface(&mut self) -> Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>> { RTSPlayerGameInput::getRtsPlayerLayerInterface(self.as_RTSPlayerGameInput_mut()) }
    fn getRTSLayerInfoPaintable(&mut self) -> Option<Rc<RefCell<RTSLayerInfoPaintable>>> { RTSPlayerGameInput::getRTSLayerInfoPaintable(self.as_RTSPlayerGameInput_mut()) }
    fn getMotionGestureInputList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { RTSPlayerGameInput::getMotionGestureInputList(self.as_RTSPlayerGameInput_mut()) }
    fn getGameCanvas(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>> { RTSPlayerGameInput::getGameCanvas(self.as_RTSPlayerGameInput_mut()) }
    fn isIsSingleKeyProcessing(&mut self) -> bool { RTSPlayerGameInput::isIsSingleKeyProcessing(self.as_RTSPlayerGameInput_mut()) }
    fn getScrollPlayerGameInput(&mut self) -> Option<Rc<RefCell<ScrollMapPlayerGameInput>>> { RTSPlayerGameInput::getScrollPlayerGameInput(self.as_RTSPlayerGameInput_mut()) }
}
impl RTSPlayerGameInputDyn for RTSPlayerGameInput {
    fn as_RTSPlayerGameInput(&self) -> &RTSPlayerGameInput { self }
    fn as_RTSPlayerGameInput_mut(&mut self) -> &mut RTSPlayerGameInput { self }
}

impl crate::org::allbinary::game::input::GameInput::GameInputDyn for RTSPlayerGameInput {
    fn as_GameInput(&self) -> &crate::org::allbinary::game::input::GameInput::GameInput { &self.base.base }
    fn as_GameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::GameInput::GameInput { &mut self.base.base }
}
impl crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn for RTSPlayerGameInput {
    fn as_PlayerGameInput(&self) -> &crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &self.base }
    fn as_PlayerGameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &mut self.base }
}


