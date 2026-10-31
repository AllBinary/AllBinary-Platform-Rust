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
// package org.allbinary.game.layer.geographic.map
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::animation::vector::CircleFilledAnimation::CircleFilledAnimation;
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::color::ColorCompositeInterface::ColorCompositeInterface;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::event::GeographicMapCellPositionEvent::GeographicMapCellPositionEvent;
use crate::org::allbinary::media::graphics::geography::map::event::GeographicMapCellPositionEventHandler::GeographicMapCellPositionEventHandler;
use crate::org::allbinary::media::graphics::geography::map::event::GeographicMapCellPositionEventListenerInterface::GeographicMapCellPositionEventListenerInterface;
use crate::org::allbinary::view::ViewPositionBase::ViewPositionBase;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MiniMapLayer {
    pub base: AllBinaryLayer,
    pub allBinaryTiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>>,
    pub geographicMapCellPositionFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>>,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub basicColorList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub positionList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub animationInterface: Option<Rc<RefCell<CircleFilledAnimation>>>,
}

impl GeographicMapCellPositionEventListenerInterface for MiniMapLayer {
    fn onGeographicMapCellPositionEvent(&mut self, mut geographicMapCellPositionEvent: Option<Rc<RefCell<GeographicMapCellPositionEvent>>>) { Self::onGeographicMapCellPositionEvent(self, geographicMapCellPositionEvent) }
    fn onRemoveGeographicMapCellPositionEvent(&mut self, mut geographicMapCellPositionEvent: Option<Rc<RefCell<GeographicMapCellPositionEvent>>>) { Self::onRemoveGeographicMapCellPositionEvent(self, geographicMapCellPositionEvent) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for MiniMapLayer {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for MiniMapLayer {
    fn getName(&mut self) -> String { self.base.getName() }
}

impl MiniMapLayer {
    

    pub fn new_struct(mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Self {
        let mut this = Self {
            base: AllBinaryLayer::new_struct(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), Rectangle::new(jcall!(PointFactory::getInstance()).createXY(0, jcall!(viewPosition).getY()), jcall!(jcall!(geographicMapInterface).getAllBinaryTiledLayer()).getWidth(), jcall!(jcall!(geographicMapInterface).getAllBinaryTiledLayer()).getHeight()), viewPosition.clone()),
            allBinaryTiledLayer: None,
            geographicMapCellPositionFactory: None,
            list: None,
            basicColorList: None,
            positionList: None,
            animationInterface: None,
        };
        // super(...) is in the struct literal above
        this.allBinaryTiledLayer = jcall!(geographicMapInterface).getAllBinaryTiledLayer();
        this.geographicMapCellPositionFactory = jcall!(geographicMapInterface).getGeographicMapCellPositionFactory();
        this.animationInterface = CircleFilledAnimation::new(jcall!(this.allBinaryTiledLayer).getCellWidth(), jcall!(this.allBinaryTiledLayer).getCellHeight(), jcall!(BasicColorFactory::getInstance()).WHITE.clone());
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.basicColorList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.positionList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.init();
        this
    }

    pub fn new(mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(geographicMapInterface, viewPosition))));
            jcall!(GeographicMapCellPositionEventHandler::getInstance()).addListener((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    // throws Exception
    pub fn init(&mut self) {
        jcall!(self.allBinaryTiledLayer).setPosition(self.x.clone(), self.y.clone(), self.z.clone());
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log(jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    // throws Exception
    // synchronized
    pub fn onRemoveGeographicMapCellPositionEvent(&mut self, mut geographicMapCellPositionEvent: Option<Rc<RefCell<GeographicMapCellPositionEvent>>>) {
        let mut colorCompositeInterface: Option<Rc<RefCell<dyn ColorCompositeInterface>>> = { let java_object = jcall!(geographicMapCellPositionEvent).getSource(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn ColorCompositeInterface>>>>().expect("ClassCastException").clone()) };
        let mut index: i32 = jcall!(self.list).indexOf(crate::java_into_object(colorCompositeInterface.clone()));
        if index >= 0 {
            jcall!(self.list).removeAt(index.clone());
            jcall!(self.basicColorList).removeAt(index.clone());
            jcall!(self.positionList).removeAt(index.clone());
        }
    }
    

    // throws Exception
    // synchronized
    pub fn onGeographicMapCellPositionEvent(&mut self, mut geographicMapCellPositionEvent: Option<Rc<RefCell<GeographicMapCellPositionEvent>>>) {
        let mut colorCompositeInterface: Option<Rc<RefCell<dyn ColorCompositeInterface>>> = { let java_object = jcall!(geographicMapCellPositionEvent).getSource(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn ColorCompositeInterface>>>>().expect("ClassCastException").clone()) };
        let mut layerIndex: i32 = -1;
        let mut size: i32 = jcall!(self.list).size();
        let mut nextColorCompositeInterface: Option<Rc<RefCell<dyn ColorCompositeInterface>>> = None;
        for index in (0..size) {
            nextColorCompositeInterface = { let java_object = jcall!(self.list).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn ColorCompositeInterface>>>>().expect("ClassCastException").clone()) };
            if match (&nextColorCompositeInterface, &colorCompositeInterface) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                layerIndex = index.clone();
                break;
            }
        }
        let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = { let java_receiver = self.geographicMapCellPositionFactory.clone(); let java_arguments = (jcall!(geographicMapCellPositionEvent).getGeographicMapCellPosition(),); let java_result = jcall!(java_receiver).getInstance(java_arguments.0); java_result };
        let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(geographicMapCellPosition).getPoint();
        if layerIndex == -1 {
            jcall!(self.list).add(crate::java_into_object(colorCompositeInterface.clone()));
            let mut basicColor: Option<Rc<RefCell<BasicColor>>> = jcall!(colorCompositeInterface).getBasicColorP();
            jcall!(self.basicColorList).add(crate::java_into_object(basicColor.clone()));
            jcall!(self.positionList).add(crate::java_into_object(point.clone()));
            layerIndex = jcall!(self.basicColorList).size() - 1;
        } else {
            jcall!(self.positionList).set(layerIndex.clone(), crate::java_into_object(point.clone()));
        }
    }
    

    pub fn paintDots(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut localPositionList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.positionList.clone();
        let mut localBasicColorList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.basicColorList.clone();
        let mut length: i32 = jcall!(localPositionList).size();
        let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = None;
        let mut basicColor: Option<Rc<RefCell<BasicColor>>> = None;
        {
            let mut index: i32 = length.clone();
            while { index -= 1; index } >= 0 {
            point = { let java_object = (jcall!(localPositionList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>>().expect("ClassCastException").clone()) };
            basicColor = { let java_object = (jcall!(localBasicColorList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<BasicColor>>>>().expect("ClassCastException").clone()) };
            jcall!(graphics).setColor(jcall!(basicColor).intValue());
            jcall!(self.animationInterface).setBasicColorP(basicColor.clone());
            { let java_receiver = self.animationInterface.clone(); let java_arguments = (graphics.clone(), jcall!(point).getX() + self.x, jcall!(point).getY() + self.y,); jcall!(java_receiver).paintXY(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
        }
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.allBinaryTiledLayer).paint(graphics.clone());
        self.paintDots(graphics.clone());
    }
}
impl std::ops::Deref for MiniMapLayer {
    type Target = AllBinaryLayer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MiniMapLayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait MiniMapLayerDyn: crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn {
    fn as_MiniMapLayer(&self) -> &MiniMapLayer;
    fn as_MiniMapLayer_mut(&mut self) -> &mut MiniMapLayer;
    fn init(&mut self) { MiniMapLayer::init(self.as_MiniMapLayer_mut()) }
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { MiniMapLayer::onEvent(self.as_MiniMapLayer_mut(), eventObject) }
    fn onRemoveGeographicMapCellPositionEvent(&mut self, mut geographicMapCellPositionEvent: Option<Rc<RefCell<GeographicMapCellPositionEvent>>>) { MiniMapLayer::onRemoveGeographicMapCellPositionEvent(self.as_MiniMapLayer_mut(), geographicMapCellPositionEvent) }
    fn onGeographicMapCellPositionEvent(&mut self, mut geographicMapCellPositionEvent: Option<Rc<RefCell<GeographicMapCellPositionEvent>>>) { MiniMapLayer::onGeographicMapCellPositionEvent(self.as_MiniMapLayer_mut(), geographicMapCellPositionEvent) }
    fn paintDots(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { MiniMapLayer::paintDots(self.as_MiniMapLayer_mut(), graphics) }
}
impl MiniMapLayerDyn for MiniMapLayer {
    fn as_MiniMapLayer(&self) -> &MiniMapLayer { self }
    fn as_MiniMapLayer_mut(&mut self) -> &mut MiniMapLayer { self }
}

impl crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn for MiniMapLayer {
    fn as_AllBinaryLayer(&self) -> &crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &self.base }
    fn as_AllBinaryLayer_mut(&mut self) -> &mut crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &mut self.base }
}
impl crate::org::allbinary::layer::Layer::LayerDyn for MiniMapLayer {
    fn as_Layer(&self) -> &crate::org::allbinary::layer::Layer::Layer { &self.base.base }
    fn as_Layer_mut(&mut self) -> &mut crate::org::allbinary::layer::Layer::Layer { &mut self.base.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn toStringAppend(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::logic::string::StringMaker::StringMaker>>>) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::toStringAppend(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1) }
}


