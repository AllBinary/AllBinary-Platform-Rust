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
// package org.allbinary.game.input.form
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::game::SimpleTiledLayer::SimpleTiledLayer;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::layer::RTSLayerEvent::RTSLayerEvent;
use crate::org::allbinary::game::layer::building::event::BuildingEventListenerInterface::BuildingEventListenerInterface;
use crate::org::allbinary::game::layer::building::event::LocalPlayerBuildingEventHandler::LocalPlayerBuildingEventHandler;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::graphics::CellPosition::CellPosition;
use crate::org::allbinary::layer::event::ScrollMapEvent::ScrollMapEvent;
use crate::org::allbinary::layer::event::ScrollMapEventHandler::ScrollMapEventHandler;
use crate::org::allbinary::layer::event::ScrollMapEventListenerInterface::ScrollMapEventListenerInterface;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct VisibleCellPositionsSingleton {
    pub stationaryVisibleCellPositions: Vec<Vec<i16>>,
    pub visibleCellPositions: Vec<Vec<i16>>,
    pub currentlyVisibleCellPositions: Vec<Vec<i16>>,
    pub ALL_VISIBLE_TILEDLAYER: Option<Rc<RefCell<SimpleTiledLayer>>>,
    pub simpleTiledLayer: Option<Rc<RefCell<SimpleTiledLayer>>>,
    pub paintSimpleTiledLayer: Option<Rc<RefCell<SimpleTiledLayer>>>,
    pub currentIndex: i32,
}

thread_local! { pub static VisibleCellPositionsSingleton_instance: Option<Rc<RefCell<VisibleCellPositionsSingleton>>> = VisibleCellPositionsSingleton::new(); }
impl BuildingEventListenerInterface for VisibleCellPositionsSingleton {
    fn onBuildingEvent(&mut self, mut event: Option<Rc<RefCell<RTSLayerEvent>>>) { Self::onBuildingEvent(self, event) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for VisibleCellPositionsSingleton {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}
impl ScrollMapEventListenerInterface for VisibleCellPositionsSingleton {
    fn onMoveEvent(&mut self, mut scrollMapEvent: Option<Rc<RefCell<ScrollMapEvent>>>) { Self::onMoveEvent(self, scrollMapEvent) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for VisibleCellPositionsSingleton {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}

impl VisibleCellPositionsSingleton {
    

    pub fn getInstance() -> Option<Rc<RefCell<VisibleCellPositionsSingleton>>> {
        return VisibleCellPositionsSingleton_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            stationaryVisibleCellPositions: Vec::new(),
            visibleCellPositions: Vec::new(),
            currentlyVisibleCellPositions: Vec::new(),
            ALL_VISIBLE_TILEDLAYER: None,
            simpleTiledLayer: None,
            paintSimpleTiledLayer: None,
            currentIndex: 0,
        };
        this.stationaryVisibleCellPositions = jcall!(NullUtil::getInstance()).NULL_SHORT_ARRAY_ARRAY.clone();
        this.visibleCellPositions = jcall!(NullUtil::getInstance()).NULL_SHORT_ARRAY_ARRAY.clone();
        this.currentlyVisibleCellPositions = jcall!(NullUtil::getInstance()).NULL_SHORT_ARRAY_ARRAY.clone();
        this.ALL_VISIBLE_TILEDLAYER = SimpleTiledLayer::new(0, 0, 0, 0);
        this.simpleTiledLayer = this.ALL_VISIBLE_TILEDLAYER.clone();
        this.paintSimpleTiledLayer = this.ALL_VISIBLE_TILEDLAYER.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct())));
            jcall!(LocalPlayerBuildingEventHandler::getInstance()).addListenerInterface((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
            jcall!(ScrollMapEventHandler::getInstance()).addListenerInterface((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    pub fn init(&mut self, mut simpleTiledLayer: Option<Rc<RefCell<SimpleTiledLayer>>>) {
        self.paintSimpleTiledLayer = self.ALL_VISIBLE_TILEDLAYER.clone();
        self.currentIndex = 0;
        let mut rows: i32 = jcall!(simpleTiledLayer).getRows();
        let mut columns: i32 = jcall!(simpleTiledLayer).getColumns();
        self.stationaryVisibleCellPositions = vec![vec![0; (columns) as usize]; (rows) as usize];
        self.visibleCellPositions = vec![vec![0; (columns) as usize]; (rows) as usize];
        self.currentlyVisibleCellPositions = vec![vec![0; (columns) as usize]; (rows) as usize];
        self.setSimpleTiledLayer(simpleTiledLayer.clone());
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log(jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    // throws Exception
    pub fn onBuildingEvent(&mut self, mut event: Option<Rc<RefCell<RTSLayerEvent>>>) {
        self.paintSimpleTiledLayer = self.simpleTiledLayer.clone();
    }
    

    // throws Exception
    pub fn onMoveEvent(&mut self, mut scrollMapEvent: Option<Rc<RefCell<ScrollMapEvent>>>) {
        { let java_arguments = (jcall!(scrollMapEvent).getDx(), jcall!(scrollMapEvent).getDy(),); self.r#move(java_arguments.0, java_arguments.1) };
    }
    

    pub fn addStationaryCellPositions(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        for index in (0..=jcall!(list).size() - 1).rev() {
            let mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>> = { let java_object = (jcall!(list).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>>().expect("ClassCastException").clone()) };
            self.stationaryVisibleCellPositions[(jcall!(cellPosition).getRow()) as usize][(jcall!(cellPosition).getColumn()) as usize] += 1;
        }
    }
    

    pub fn removeStationaryCellPositions(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        for index in (0..=jcall!(list).size() - 1).rev() {
            let mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>> = { let java_object = (jcall!(list).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>>().expect("ClassCastException").clone()) };
            self.stationaryVisibleCellPositions[(jcall!(cellPosition).getRow()) as usize][(jcall!(cellPosition).getColumn()) as usize] -= 1;
        }
    }
    

    pub fn update(&mut self) {
        if self.currentIndex == 0 {
            let mut temp: Vec<Vec<i16>> = self.currentlyVisibleCellPositions.clone();
            self.currentlyVisibleCellPositions = self.visibleCellPositions.clone();
            self.visibleCellPositions = temp.clone();
            for index in (0..=(self.visibleCellPositions.len() as i32) - 1).rev() {
                for index2 in (0..=(self.visibleCellPositions[(0) as usize].len() as i32) - 1).rev() {
                    self.visibleCellPositions[(index) as usize][(index2) as usize] = self.stationaryVisibleCellPositions[(index) as usize][(index2) as usize].clone();
                }
            }
        }
        self.currentIndex += 1;
        if self.currentIndex > 10 {
            self.currentIndex = 0;
        }
    }
    

    pub fn shouldProcess(&mut self) -> bool {
        if self.currentIndex == 0 {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn addCellPositions(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        for index in (0..=jcall!(list).size() - 1).rev() {
            let mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>> = { let java_object = (jcall!(list).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>>().expect("ClassCastException").clone()) };
            self.visibleCellPositions[(jcall!(cellPosition).getRow()) as usize][(jcall!(cellPosition).getColumn()) as usize] += 1;
        }
    }
    

    pub fn isVisible(&mut self, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>) -> bool {
        if self.visibleCellPositions[(jcall!(cellPosition).getRow()) as usize][(jcall!(cellPosition).getColumn()) as usize] > 0 {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn r#move(&mut self, mut dx: i32, mut dy: i32) {
        jcall!(self.getSimpleTiledLayer()).moveDXY(dx.clone(), dy.clone());
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.paintSimpleTiledLayer).paint(graphics.clone(), self.currentlyVisibleCellPositions.clone());
    }
    

    pub fn setSimpleTiledLayer(&mut self, mut simpleTiledLayer: Option<Rc<RefCell<SimpleTiledLayer>>>) {
        self.simpleTiledLayer = simpleTiledLayer.clone();
    }
    

    pub fn getSimpleTiledLayer(&mut self) -> Option<Rc<RefCell<SimpleTiledLayer>>> {
        return self.simpleTiledLayer.clone();
    }
}

