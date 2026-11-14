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
use crate::javax::microedition::lcdui::Graphics::Graphics;
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
use crate::java::lang::Exception::*;

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
impl crate::org::allbinary::layer::LayerInterface::LayerInterface for MiniMapLayer {
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
    fn implmentsTickableInterface(&mut self) -> bool { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::implmentsTickableInterface(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn implmentsCollidableInterface(&mut self) -> bool { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::implmentsCollidableInterface(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn implmentsGameInputInterface(&mut self) -> bool { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::implmentsGameInputInterface(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn implmentsArtificialIntelligenceCompositeInterface(&mut self) -> bool { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::implmentsArtificialIntelligenceCompositeInterface(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getType(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getType(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
}
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for MiniMapLayer {
    fn getName(&mut self) -> String { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getName(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
}
impl crate::org::allbinary::layer::PositionInterface::PositionInterface for MiniMapLayer {
    fn setPosition(&mut self, p1: i32, p2: i32, p3: i32) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::setPosition(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1, p2, p3) }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for MiniMapLayer {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::paintThreed(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::identification::GroupInterfaceCompositeInterface::GroupInterfaceCompositeInterface for MiniMapLayer {
    fn getGroupInterface(&mut self) -> Vec<Option<Rc<RefCell<crate::org::allbinary::game::identification::Group::Group>>>> { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getGroupInterface(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
}

impl MiniMapLayer {
    

    pub fn new_struct(mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Self {
        let mut this = Self {
            base: AllBinaryLayer::new_struct({ let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), Rectangle::new(jcall!(PointFactory::getInstance()).createXY(0, jcall!(viewPosition).getY()), jcall!(jcall!(geographicMapInterface).getAllBinaryTiledLayer()).getWidth(), jcall!(jcall!(geographicMapInterface).getAllBinaryTiledLayer()).getHeight()), viewPosition.clone()),
            allBinaryTiledLayer: None,
            geographicMapCellPositionFactory: None,
            list: None,
            basicColorList: None,
            positionList: None,
            animationInterface: None,
        };
        // super(...) is in the struct literal above
        this.allBinaryTiledLayer = { let java_value = (jcall!(geographicMapInterface).getAllBinaryTiledLayer()).clone(); java_value };
        this.geographicMapCellPositionFactory = { let java_value = (jcall!(geographicMapInterface).getGeographicMapCellPositionFactory()).clone(); java_value };
        this.animationInterface = { let java_value = (CircleFilledAnimation::new(jcall!(this.allBinaryTiledLayer).getCellWidth(), jcall!(this.allBinaryTiledLayer).getCellHeight(), { let java_field = jcall!(BasicColorFactory::getInstance()).WHITE.clone(); java_field }.clone())).clone(); java_value };
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.basicColorList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.positionList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new(mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(geographicMapInterface.clone(), viewPosition.clone()))));
            jcall!(GeographicMapCellPositionEventHandler::getInstance()).addListener(this.clone().map(|reference| reference as Rc<RefCell<dyn MiniMapLayerDyn>>));
            jcall!(this).init();
        this
    }
    

    // throws Exception
    pub fn init(&mut self) {
        jcall!(self.allBinaryTiledLayer).setPosition(self.x.clone(), self.y.clone(), self.z.clone());
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log({ let java_field = jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    // throws Exception
    // synchronized
    pub fn onRemoveGeographicMapCellPositionEvent(&mut self, mut geographicMapCellPositionEvent: Option<Rc<RefCell<GeographicMapCellPositionEvent>>>) {
        let mut colorCompositeInterface: Option<Rc<RefCell<dyn ColorCompositeInterface>>> = { let java_object = (jcall!(geographicMapCellPositionEvent).getSource()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn ColorCompositeInterface>>>>().expect("ClassCastException").clone()) };
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
        let mut colorCompositeInterface: Option<Rc<RefCell<dyn ColorCompositeInterface>>> = { let java_object = (jcall!(geographicMapCellPositionEvent).getSource()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn ColorCompositeInterface>>>>().expect("ClassCastException").clone()) };
        let mut layerIndex: i32 = -1;
        let mut size: i32 = jcall!(self.list).size();
        let mut nextColorCompositeInterface: Option<Rc<RefCell<dyn ColorCompositeInterface>>> = None;
        for index in (0..size) {
            nextColorCompositeInterface = { let java_value = ({ let java_object = (jcall!(self.list).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn ColorCompositeInterface>>>>().expect("ClassCastException").clone()) }).clone(); java_value };
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
            layerIndex = { let java_value = (jcall!(self.basicColorList).size() - 1).clone(); java_value };
        } else {
            jcall!(self.positionList).set(layerIndex.clone(), crate::java_into_object(point.clone()));
        }
    }
    

    pub fn paintDots(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        let mut localPositionList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.positionList.clone();
        let mut localBasicColorList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.basicColorList.clone();
        let mut length: i32 = jcall!(localPositionList).size();
        let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = None;
        let mut basicColor: Option<Rc<RefCell<BasicColor>>> = None;
        {
            let mut index: i32 = length.clone();
            while { index -= 1; index } >= 0 {
            point = { let java_value = ({ let java_object = (jcall!(localPositionList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>>().expect("ClassCastException").clone()) }).clone(); java_value };
            basicColor = { let java_value = ({ let java_object = (jcall!(localBasicColorList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<BasicColor>>>>().expect("ClassCastException").clone()) }).clone(); java_value };
            jcall!(graphics).setColor(jcall!(basicColor).intValue());
            jcall!(self.animationInterface).setBasicColorP(basicColor.clone());
            { let java_receiver = self.animationInterface.clone(); let java_arguments = (graphics.clone(), jcall!(point).getX() + self.x, jcall!(point).getY() + self.y,); jcall!(java_receiver).paintXY(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
        }
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        jcall!(self.allBinaryTiledLayer).as_AllBinaryTiledLayer_mut().paint(graphics.clone());
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
    fn paintDots(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { MiniMapLayer::paintDots(self.as_MiniMapLayer_mut(), graphics) }
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
    fn toStringAppend(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::logic::string::StringMaker::StringMaker>>>) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::toStringAppend(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1) }
}


