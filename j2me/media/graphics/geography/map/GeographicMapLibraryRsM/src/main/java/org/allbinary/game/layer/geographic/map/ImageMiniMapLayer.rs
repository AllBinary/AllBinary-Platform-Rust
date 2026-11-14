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
use crate::javax::microedition::lcdui::Image::Image;
use crate::javax::microedition::lcdui::NullImage::NullImage;
use crate::org::allbinary::graphics::Anchor::Anchor;
use crate::org::allbinary::image::PaintableToImageUtil::PaintableToImageUtil;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::view::ViewPositionBase::ViewPositionBase;
use crate::java::lang::Exception::*;
use super::MiniMapLayer::MiniMapLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageMiniMapLayer {
    pub base: MiniMapLayer,
    pub image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>,
    pub anchor: i32,
}

impl crate::org::allbinary::layer::LayerInterface::LayerInterface for ImageMiniMapLayer {
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
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for ImageMiniMapLayer {
    fn getName(&mut self) -> String { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getName(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
}
impl crate::org::allbinary::layer::PositionInterface::PositionInterface for ImageMiniMapLayer {
    fn setPosition(&mut self, p1: i32, p2: i32, p3: i32) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::setPosition(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1, p2, p3) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for ImageMiniMapLayer {
    fn onEvent(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { crate::org::allbinary::game::layer::geographic::map::MiniMapLayer::MiniMapLayer::onEvent(crate::org::allbinary::game::layer::geographic::map::MiniMapLayer::MiniMapLayerDyn::as_MiniMapLayer_mut(self), p1) }
}
impl crate::org::allbinary::media::graphics::geography::map::event::GeographicMapCellPositionEventListenerInterface::GeographicMapCellPositionEventListenerInterface for ImageMiniMapLayer {
    fn onGeographicMapCellPositionEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::media::graphics::geography::map::event::GeographicMapCellPositionEvent::GeographicMapCellPositionEvent>>>) { crate::org::allbinary::game::layer::geographic::map::MiniMapLayer::MiniMapLayer::onGeographicMapCellPositionEvent(crate::org::allbinary::game::layer::geographic::map::MiniMapLayer::MiniMapLayerDyn::as_MiniMapLayer_mut(self), p1) }
    fn onRemoveGeographicMapCellPositionEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::media::graphics::geography::map::event::GeographicMapCellPositionEvent::GeographicMapCellPositionEvent>>>) { crate::org::allbinary::game::layer::geographic::map::MiniMapLayer::MiniMapLayer::onRemoveGeographicMapCellPositionEvent(crate::org::allbinary::game::layer::geographic::map::MiniMapLayer::MiniMapLayerDyn::as_MiniMapLayer_mut(self), p1) }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for ImageMiniMapLayer {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::paintThreed(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::identification::GroupInterfaceCompositeInterface::GroupInterfaceCompositeInterface for ImageMiniMapLayer {
    fn getGroupInterface(&mut self) -> Vec<Option<Rc<RefCell<crate::org::allbinary::game::identification::Group::Group>>>> { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getGroupInterface(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
}

impl ImageMiniMapLayer {
    

    pub fn new_struct(mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Self {
        let mut this = Self {
            base: MiniMapLayer::new_struct(geographicMapInterface.clone(), viewPosition.clone()),
            image: None,
            anchor: 0,
        };
        this.image = crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone).clone();
        this.anchor = unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(geographicMapInterface, viewPosition))))
    }
    

    // throws Exception
    pub fn init(&mut self) {
        let mut aWidth: i32 = jcall!(self.allBinaryTiledLayer).getWidth();
        let mut aHeight: i32 = jcall!(self.allBinaryTiledLayer).getHeight();
        self.image = { let java_value = (PaintableToImageUtil::getImage(self.allBinaryTiledLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface>>), aWidth.clone(), aHeight.clone())).clone(); java_value };
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        jcall!(graphics).drawImage(self.image.clone(), self.x.clone(), self.y.clone(), self.anchor.clone());
        self.paintDots(graphics.clone());
    }
}
impl std::ops::Deref for ImageMiniMapLayer {
    type Target = MiniMapLayer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImageMiniMapLayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::geographic::map::MiniMapLayer::MiniMapLayerDyn for ImageMiniMapLayer {
    fn as_MiniMapLayer(&self) -> &crate::org::allbinary::game::layer::geographic::map::MiniMapLayer::MiniMapLayer { &self.base }
    fn as_MiniMapLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::geographic::map::MiniMapLayer::MiniMapLayer { &mut self.base }
    fn init(&mut self) { Self::init(self) }
}
impl crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn for ImageMiniMapLayer {
    fn as_AllBinaryLayer(&self) -> &crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &self.base.base }
    fn as_AllBinaryLayer_mut(&mut self) -> &mut crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &mut self.base.base }
}
impl crate::org::allbinary::layer::Layer::LayerDyn for ImageMiniMapLayer {
    fn as_Layer(&self) -> &crate::org::allbinary::layer::Layer::Layer { &self.base.base.base }
    fn as_Layer_mut(&mut self) -> &mut crate::org::allbinary::layer::Layer::Layer { &mut self.base.base.base }
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


