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
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::displayable::CanvasStrings::CanvasStrings;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellHistory::GeographicMapCellHistory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCompositeInterface::GeographicMapCompositeInterface;
use crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::LinePathAnimation::LinePathAnimation;
use super::PathFindingLayerInterface::PathFindingLayerInterface;
use super::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use super::WaypointBehaviorBase::WaypointBehaviorBase;
use super::AllBinaryTiledLayer::AllBinaryTiledLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PathAnimation {
    pub base: Animation,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub linePathAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::LinePathAnimation::LinePathAnimationDyn>>>,
    pub pathFindingLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>,
    pub allBinaryGameLayerManagerP: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>,
    pub startColor: Option<Rc<RefCell<BasicColor>>>,
    pub middleColor: Option<Rc<RefCell<BasicColor>>>,
    pub endColor: Option<Rc<RefCell<BasicColor>>>,
}

impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for PathAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl PathAnimation {
    

    pub fn new_struct(mut pathFindingLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut linePathAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::LinePathAnimation::LinePathAnimationDyn>>>) -> Self {
        let mut this = Self {
            base: Animation::new_struct(),
            logUtil: None,
            linePathAnimation: None,
            pathFindingLayer: None,
            allBinaryGameLayerManagerP: None,
            startColor: None,
            middleColor: None,
            endColor: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.allBinaryGameLayerManagerP = AllBinaryGameLayerManager::getNullInstance();
        this.startColor = jcall!(BasicColorFactory::getInstance()).RED.clone();
        this.middleColor = jcall!(BasicColorFactory::getInstance()).BLUE.clone();
        this.endColor = jcall!(BasicColorFactory::getInstance()).GREEN.clone();
        this.pathFindingLayer = pathFindingLayer.clone();
        this.linePathAnimation = linePathAnimation.clone();
        this
    }

    pub fn new(mut pathFindingLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut linePathAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::LinePathAnimation::LinePathAnimationDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(pathFindingLayer, linePathAnimation))))
    }
    

    // throws Exception
    pub fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) {
        self.allBinaryGameLayerManagerP = allBinaryGameLayerManager.clone();
    }
    

    // throws Exception
    pub fn nextFrame(&mut self) {
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut waypointBehaviorBase: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>> = jcall!(self.pathFindingLayer).getWaypointBehavior();
            if waypointBehaviorBase.is_some() {
                let mut geographicMapCellHistory: Option<Rc<RefCell<GeographicMapCellHistory>>> = jcall!(waypointBehaviorBase).getCurrentGeographicMapCellHistory();
                let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(geographicMapCellHistory).getTracked();
                let mut size: i32 = jcall!(list).size();
                if size > 0 {
                    let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (self.allBinaryGameLayerManagerP).clone() /* as GeographicMapCompositeInterface */;
                    let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
                    let mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(geographicMapInterface).getAllBinaryTiledLayer();
                    let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone).clone();
                    let mut nextPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = None;
                    let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = None;
                    let mut start: i32 = size - 2;
                    for index in (0..size - 1) {
                        if index == start {
                            self.setBasicColorP(self.startColor.clone());
                            { let java_receiver = self.basicSetColorUtil.clone(); let java_arguments = (graphics.clone(), self.getBasicColorP(), self.getColor(),); jcall!(java_receiver).setBasicColorP3(java_arguments.0, java_arguments.1, java_arguments.2); };
                        } else if jcall!(geographicMapCellHistory).isVisited(geographicMapCellPosition.clone()) {
                            self.setBasicColorP(self.endColor.clone());
                            { let java_receiver = self.basicSetColorUtil.clone(); let java_arguments = (graphics.clone(), self.getBasicColorP(), self.getColor(),); jcall!(java_receiver).setBasicColorP3(java_arguments.0, java_arguments.1, java_arguments.2); };
                        } else {
                            self.setBasicColorP(self.middleColor.clone());
                            { let java_receiver = self.basicSetColorUtil.clone(); let java_arguments = (graphics.clone(), self.getBasicColorP(), self.getColor(),); jcall!(java_receiver).setBasicColorP3(java_arguments.0, java_arguments.1, java_arguments.2); };
                        }
                        geographicMapCellPosition = { let java_object = (jcall!(list).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) };
                        point = jcall!(geographicMapCellPosition).getMidPoint();
                        geographicMapCellPosition = { let java_object = (jcall!(list).get(index + 1)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) };
                        nextPoint = jcall!(geographicMapCellPosition).getMidPoint();
                        jcall!(self.linePathAnimation).paint(graphics.clone(), point.clone(), nextPoint.clone(), tiledLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::Layer::LayerDyn>>));
                    }
                }
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            { let java_receiver = self.logUtil.clone(); let java_arguments = (String::from("Cleared Target"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(CanvasStrings::getInstance()).PAINT.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
        }
    }
}
impl std::ops::Deref for PathAnimation {
    type Target = Animation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PathAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for PathAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base }
    fn nextFrame(&mut self) { Self::nextFrame(self) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for PathAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


