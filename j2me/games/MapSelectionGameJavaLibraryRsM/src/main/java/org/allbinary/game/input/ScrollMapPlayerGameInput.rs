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
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Canvas::Canvas;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::InputFeatureFactory::InputFeatureFactory;
use crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::layer::event::ScrollMapEvent::ScrollMapEvent;
use crate::org::allbinary::layer::event::ScrollMapEventHandler::ScrollMapEventHandler;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::PlayerGameInput::PlayerGameInput;
use super::GameInputProcessor::GameInputProcessor;
use super::ScrollMapUpGameInputProcessor::ScrollMapUpGameInputProcessor;
use super::ScrollMapDownGameInputProcessor::ScrollMapDownGameInputProcessor;
use super::ScrollMapRightGameInputProcessor::ScrollMapRightGameInputProcessor;
use super::ScrollMapLeftGameInputProcessor::ScrollMapLeftGameInputProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ScrollMapPlayerGameInput {
    pub base: PlayerGameInput,
    pub inputProcessorArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInputProcessor::GameInputProcessorDyn>>>>,
    pub border: i32,
    pub inputList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub isSingleKeyProcessing: bool,
    pub geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>,
    pub maxBottom: i32,
    pub maxRight: i32,
    pub scrollMapEvent: Option<Rc<RefCell<ScrollMapEvent>>>,
}

impl crate::org::allbinary::game::input::event::GameKeyEventListenerInterface::GameKeyEventListenerInterface for ScrollMapPlayerGameInput {
}

impl ScrollMapPlayerGameInput {
    

    pub fn new_struct(mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut playerInputId: i32) -> Self {
        let mut this = Self {
            base: PlayerGameInput::new_struct(list.clone(), BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>), playerInputId.clone()),
            inputProcessorArray: Vec::new(),
            border: 0,
            inputList: None,
            isSingleKeyProcessing: false,
            geographicMapInterface: None,
            maxBottom: 0,
            maxRight: 0,
            scrollMapEvent: None,
        };
        this.inputProcessorArray = vec![None; (jcall!(InputFactory::InputFactory::getInstance()).MAX) as usize];
        this.border = 150;
        this.isSingleKeyProcessing = jcall!(Features::getInstance()).isFeature(jcall!(InputFeatureFactory::getInstance()).SINGLE_KEY_REPEAT_PRESS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) || jcall!(Features::getInstance()).isFeature(jcall!(InputFeatureFactory::getInstance()).SINGLE_KEY_PRESS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>));
        this.scrollMapEvent = ScrollMapEvent::new(this);
        // super(...) is in the struct literal above
        this.initInputProcessors();
        this.geographicMapInterface = geographicMapInterface.clone();
        this.inputList = list.clone();
        this.init();
        this
    }

    pub fn new(mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut playerInputId: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(geographicMapInterface, list, playerInputId))))
    }
    

    // throws Exception
    pub fn r#move(&mut self, mut dx: i32, mut dy: i32) {
        let mut terrainTiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(self.geographicMapInterface).getAllBinaryTiledLayer();
        jcall!(terrainTiledLayer).moveDXY(-dx, -dy);
        jcall!(self.scrollMapEvent).setDxDy(-dx, -dy);
        jcall!(ScrollMapEventHandler::getInstance()).fireEvent(self.scrollMapEvent.clone());
    }
    

    pub fn init(&mut self) {
        self.maxBottom = self.getSpecialHeight();
        self.maxRight = self.getSpecialWidth();
    }
    

    // throws Exception
    pub fn scrollMiddleX(&mut self) {
        let mut terrainTiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(self.geographicMapInterface).getAllBinaryTiledLayer();
        let mut displayInfo: Option<Rc<RefCell<DisplayInfoSingleton>>> = DisplayInfoSingleton::getInstance();
        { let java_arguments = (-jcall!(terrainTiledLayer).getXP(), 0,); self.r#move(java_arguments.0, java_arguments.1) };
        if jcall!(terrainTiledLayer).getWidth() < jcall!(displayInfo).getLastWidth() {
            let mut diffX: i32 = ((jcall!(displayInfo).getLastWidth() - jcall!(terrainTiledLayer).getWidth()) >> 1);
            self.r#move(-diffX, 0);
        } else {
            let mut diffX: i32 = ((jcall!(terrainTiledLayer).getWidth() - jcall!(displayInfo).getLastWidth()) >> 1);
            self.r#move(diffX.clone(), 0);
        }
    }
    

    // throws Exception
    pub fn scrollY(&mut self, mut anchor: i32) {
        let mut terrainTiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(self.geographicMapInterface).getAllBinaryTiledLayer();
        let mut displayInfo: Option<Rc<RefCell<DisplayInfoSingleton>>> = DisplayInfoSingleton::getInstance();
        if anchor == unsafe { crate::javax::microedition::lcdui::Graphics::Graphics_TOP.clone() } {
            jcall!(self.logUtil).putF(String::from("Top"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("scrollY"));
        } else if anchor == unsafe { crate::javax::microedition::lcdui::Graphics::Graphics_BOTTOM.clone() } {
            jcall!(self.logUtil).putF(String::from("Bottom"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("scrollY"));
            let mut diffY: i32 = (jcall!(terrainTiledLayer).getHeight() - jcall!(displayInfo).getLastHeight());
            self.r#move(0, diffY.clone());
        } else if anchor == unsafe { crate::javax::microedition::lcdui::Graphics::Graphics_VCENTER.clone() } {
            jcall!(self.logUtil).putF(String::from("Center"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("scrollY"));
            let mut diffY: i32 = (jcall!(terrainTiledLayer).getHeight() - jcall!(displayInfo).getLastHeight()) / 2;
            self.r#move(0, diffY.clone());
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No Such Anchor Supported"))));
        }
    }
    

    // throws Exception
    pub fn up(&mut self) {
        let mut y: i32 = jcall!(jcall!(self.geographicMapInterface).getAllBinaryTiledLayer()).getYP() - 10;
        if self.keepOnMapMinY(y.clone()) {
            self.r#move(0, -10);
        }
    }
    

    // throws Exception
    pub fn down(&mut self) {
        let mut y: i32 = -jcall!(jcall!(self.geographicMapInterface).getAllBinaryTiledLayer()).getYP() + 10;
        if self.keepOnMapMaxY(y.clone()) {
            self.r#move(0, 10);
        }
    }
    

    // throws Exception
    pub fn right(&mut self) {
        let mut x: i32 = jcall!(jcall!(self.geographicMapInterface).getAllBinaryTiledLayer()).getYP() + 10;
        if self.keepOnMapMinX(x.clone()) {
            self.r#move(10, 0);
        }
    }
    

    // throws Exception
    pub fn left(&mut self) {
        let mut x: i32 = -jcall!(jcall!(self.geographicMapInterface).getAllBinaryTiledLayer()).getYP() - 10;
        if self.keepOnMapMaxX(x.clone()) {
            self.r#move(-10, 0);
        }
    }
    

    pub fn initInputProcessors(&mut self) {
        self.inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_UP.clone() }) as usize] = ScrollMapUpGameInputProcessor::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_DOWN.clone() }) as usize] = ScrollMapDownGameInputProcessor::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM9.clone() }) as usize] = ScrollMapRightGameInputProcessor::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.inputProcessorArray[(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM7.clone() }) as usize] = ScrollMapLeftGameInputProcessor::new(None /* Java this: a &mut self method has no Rc of itself */);
        GameInputProcessorUtil::GameInputProcessorUtil::init(self.inputProcessorArray.clone());
    }
    

    // throws Exception
    pub fn processInputKey(&mut self, mut key: i32) {
        jcall!(self.inputProcessorArray[(key) as usize]).processEvent(AllBinaryGameLayerManager::getNullInstance(), crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent_NONE.with(Clone::clone).clone());
    }
    

    // throws Exception
    pub fn processInput(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut size: i32 = jcall!(self.inputList).size();
            let mut key: i32 = 0;
            for index in (0..size) {
                let mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>> = { let java_object = (jcall!(self.inputList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<GameKeyEvent>>>>().expect("ClassCastException").clone()) };
                key = jcall!(gameKeyEvent).getKey();
                self.processInputKey(key.clone());
            }
            if self.isSingleKeyProcessing {
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
    

    pub fn getSpecialHeight(&mut self) -> i32 {
        let mut terrainTiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(self.geographicMapInterface).getAllBinaryTiledLayer();
        let mut displayInfo: Option<Rc<RefCell<DisplayInfoSingleton>>> = DisplayInfoSingleton::getInstance();
        if jcall!(terrainTiledLayer).getHeight() > jcall!(displayInfo).getLastHeight() {
            return jcall!(terrainTiledLayer).getHeight() - jcall!(displayInfo).getLastHeight();
        } else {
            return jcall!(displayInfo).getLastHeight() - jcall!(terrainTiledLayer).getHeight() + jcall!(terrainTiledLayer).getCellHeight();
        }
    }
    

    pub fn getSpecialWidth(&mut self) -> i32 {
        let mut terrainTiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(self.geographicMapInterface).getAllBinaryTiledLayer();
        let mut displayInfo: Option<Rc<RefCell<DisplayInfoSingleton>>> = DisplayInfoSingleton::getInstance();
        if jcall!(terrainTiledLayer).getWidth() > jcall!(displayInfo).getLastWidth() {
            return jcall!(terrainTiledLayer).getWidth() - jcall!(displayInfo).getLastWidth();
        } else {
            return jcall!(displayInfo).getLastWidth() - jcall!(terrainTiledLayer).getWidth() + jcall!(terrainTiledLayer).getCellWidth();
        }
    }
    

    pub fn keepOnMapMaxY(&mut self, mut newY: i32) -> bool {
        let mut y: i32 = jcall!(jcall!(self.geographicMapInterface).getAllBinaryTiledLayer()).getYP();
        if y > self.maxBottom + self.border {
            return false;
        } else {
            return true;
        }
    }
    

    pub fn keepOnMapMinY(&mut self, mut newY: i32) -> bool {
        let mut y: i32 = jcall!(jcall!(self.geographicMapInterface).getAllBinaryTiledLayer()).getYP();
        if y < -self.border {
            return false;
        } else {
            return true;
        }
    }
    

    pub fn keepOnMapMaxX(&mut self, mut newX: i32) -> bool {
        let mut x: i32 = jcall!(jcall!(self.geographicMapInterface).getAllBinaryTiledLayer()).getXP();
        if x < -self.maxRight - self.border {
            return false;
        } else {
            return true;
        }
    }
    

    pub fn keepOnMapMinX(&mut self, mut newX: i32) -> bool {
        let mut x: i32 = jcall!(jcall!(self.geographicMapInterface).getAllBinaryTiledLayer()).getXP();
        if x > self.border {
            return false;
        } else {
            return true;
        }
    }
}
impl std::ops::Deref for ScrollMapPlayerGameInput {
    type Target = PlayerGameInput;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ScrollMapPlayerGameInput {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::input::GameInput::GameInputDyn for ScrollMapPlayerGameInput {
    fn as_GameInput(&self) -> &crate::org::allbinary::game::input::GameInput::GameInput { &self.base.base }
    fn as_GameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::GameInput::GameInput { &mut self.base.base }
}
impl crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn for ScrollMapPlayerGameInput {
    fn as_PlayerGameInput(&self) -> &crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &self.base }
    fn as_PlayerGameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &mut self.base }
}


