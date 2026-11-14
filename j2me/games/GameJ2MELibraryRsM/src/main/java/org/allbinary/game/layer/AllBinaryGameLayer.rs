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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::khronos::opengles::GL::GL;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::color::BasicColorSetUtil::BasicColorSetUtil;
use crate::org::allbinary::graphics::paint::NullPaintable::NullPaintable;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::view::ViewPositionBase::ViewPositionBase;
use crate::org::allbinary::view::event::ViewPositionEvent::ViewPositionEvent;
use crate::org::allbinary::view::event::ViewPositionEventListenerInterface::ViewPositionEventListenerInterface;
use crate::java::lang::Exception::*;
use super::SWTBlackPaintable::SWTBlackPaintable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryGameLayer {
    pub base: AllBinaryLayer,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub basicSetColorUtil: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::color::BasicColorSetUtil::BasicColorSetUtilDyn>>>,
    pub RED: Option<Rc<RefCell<BasicColor>>>,
    pub paintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
    pub gameKeyEventList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

impl ViewPositionEventListenerInterface for AllBinaryGameLayer {
    fn onChangeEvent(&mut self, mut layerManagerEvent: Option<Rc<RefCell<ViewPositionEvent>>>) { Self::onChangeEvent(self, layerManagerEvent) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for AllBinaryGameLayer {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}
impl OpenGLSurfaceChangedInterface for AllBinaryGameLayer {
    fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) { Self::set(self, gl) }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for AllBinaryGameLayer {
    fn paint(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::paint(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1) }
    fn paintThreed(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::paintThreed(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1) }
}
impl crate::org::allbinary::layer::LayerInterface::LayerInterface for AllBinaryGameLayer {
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
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for AllBinaryGameLayer {
    fn getName(&mut self) -> String { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getName(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
}
impl crate::org::allbinary::layer::PositionInterface::PositionInterface for AllBinaryGameLayer {
    fn setPosition(&mut self, p1: i32, p2: i32, p3: i32) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::setPosition(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1, p2, p3) }
}
impl crate::org::allbinary::game::identification::GroupInterfaceCompositeInterface::GroupInterfaceCompositeInterface for AllBinaryGameLayer {
    fn getGroupInterface(&mut self) -> Vec<Option<Rc<RefCell<crate::org::allbinary::game::identification::Group::Group>>>> { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getGroupInterface(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
}

impl AllBinaryGameLayer {
    

    pub fn createPaintable() -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> {
        if unsafe { SWTUtil_isSWT.clone() } {
            return SWTBlackPaintable::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>);
        }
        return NullPaintable::getInstance();
    }
    

    pub fn new_struct(mut name: String, mut layerInfo: Option<Rc<RefCell<Rectangle>>>, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Self {
        let mut this = Self {
            base: AllBinaryLayer::new_struct(name.clone(), layerInfo.clone(), viewPosition.clone()),
            commonStrings: None,
            basicSetColorUtil: None,
            RED: None,
            paintable: None,
            gameKeyEventList: None,
        };
        this.commonStrings = CommonStrings::getInstance();
        this.basicSetColorUtil = BasicColorSetUtil::getInstance();
        this.RED = { let java_field = jcall!(BasicColorFactory::getInstance()).RED.clone(); java_field }.clone();
        this.paintable = Self::createPaintable();
        this.gameKeyEventList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut name: String, mut layerInfo: Option<Rc<RefCell<Rectangle>>>, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, layerInfo, viewPosition))))
    }
    

    // throws Exception
    pub fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) {
        panic!("{:?}", format!("{}: {:?}", "Exception", ({ let java_field = jcall!(self.commonStrings).NOT_IMPLEMENTED.clone(); java_field }.clone())));
    }
    

    pub fn r#move(&mut self) {
    }
    

    pub fn getGameKeyEventList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.gameKeyEventList.clone();
    }
    

    // throws Exception
    pub fn processInput(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        panic!("{:?}", format!("{}: {:?}", "Exception", ({ let java_field = jcall!(self.commonStrings).NOT_IMPLEMENTED.clone(); java_field }.clone())));
    }
    

    // throws Exception
    pub fn processTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        panic!("{:?}", format!("{}: {:?}", "Exception", ({ let java_field = jcall!(self.commonStrings).NOT_IMPLEMENTED.clone(); java_field }.clone())));
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log({ let java_field = jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    // throws Exception
    pub fn onViewPositionChangeEvent(&mut self) {
        { let java_arguments = (self.viewPositionEvent.clone(),); self.onChangeEvent(java_arguments.0) };
    }
    

    // throws Exception
    pub fn onChangeEvent(&mut self, mut layerManagerEvent: Option<Rc<RefCell<ViewPositionEvent>>>) {
        if GameLayerUtil::GameLayerUtil::isOnScreen(None /* Java this: a &mut self method has no Rc of itself */) {
            self.setVisible(true);
        } else {
            self.setVisible(false);
        }
    }
    

    pub fn paintFirst(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        jcall!(self.paintable).as_Paintable_mut().paint(graphics.clone());
    }
    

    pub fn paintDebug(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        let mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>> = self.getViewPosition();
        let mut viewX: i32 = jcall!(viewPosition).getX();
        let mut viewY: i32 = jcall!(viewPosition).getY();
        jcall!(self.basicSetColorUtil).setBasicColorP(graphics.clone(), self.RED.clone());
        jcall!(graphics).drawRect(viewX.clone(), viewY.clone(), self.getWidth(), self.getHeight());
    }
    

    pub fn setWidth(&mut self, mut width: i32) {
        { let java_arguments = (width.clone(),); self.base.setLayerWidth(java_arguments.0) };
        self.setHalfWidth(width >> 1);
    }
    

    pub fn setHeight(&mut self, mut height: i32) {
        { let java_arguments = (height.clone(),); self.base.setLayerHeight(java_arguments.0) };
        self.setHalfHeight(height >> 1);
    }
}
impl std::ops::Deref for AllBinaryGameLayer {
    type Target = AllBinaryLayer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllBinaryGameLayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AllBinaryGameLayerDyn: crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn + crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface {
    fn as_AllBinaryGameLayer(&self) -> &AllBinaryGameLayer;
    fn as_AllBinaryGameLayer_mut(&mut self) -> &mut AllBinaryGameLayer;
    fn r#move(&mut self) { AllBinaryGameLayer::r#move(self.as_AllBinaryGameLayer_mut()) }
    fn getGameKeyEventList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { AllBinaryGameLayer::getGameKeyEventList(self.as_AllBinaryGameLayer_mut()) }
    fn processInput(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { AllBinaryGameLayer::processInput(self.as_AllBinaryGameLayer_mut(), allBinaryLayerManager) }
    fn processTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { AllBinaryGameLayer::processTick(self.as_AllBinaryGameLayer_mut(), allBinaryLayerManager) }
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { AllBinaryGameLayer::onEvent(self.as_AllBinaryGameLayer_mut(), eventObject) }
    fn onViewPositionChangeEvent(&mut self) { AllBinaryGameLayer::onViewPositionChangeEvent(self.as_AllBinaryGameLayer_mut()) }
    fn paintFirst(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { AllBinaryGameLayer::paintFirst(self.as_AllBinaryGameLayer_mut(), graphics) }
    fn paintDebug(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { AllBinaryGameLayer::paintDebug(self.as_AllBinaryGameLayer_mut(), graphics) }
    fn setWidth(&mut self, mut width: i32) { AllBinaryGameLayer::setWidth(self.as_AllBinaryGameLayer_mut(), width) }
    fn setHeight(&mut self, mut height: i32) { AllBinaryGameLayer::setHeight(self.as_AllBinaryGameLayer_mut(), height) }
}
impl AllBinaryGameLayerDyn for AllBinaryGameLayer {
    fn as_AllBinaryGameLayer(&self) -> &AllBinaryGameLayer { self }
    fn as_AllBinaryGameLayer_mut(&mut self) -> &mut AllBinaryGameLayer { self }
}

impl crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn for AllBinaryGameLayer {
    fn as_AllBinaryLayer(&self) -> &crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &self.base }
    fn as_AllBinaryLayer_mut(&mut self) -> &mut crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &mut self.base }
    fn onChangeEvent(&mut self, mut layerManagerEvent: Option<Rc<RefCell<ViewPositionEvent>>>) { Self::onChangeEvent(self, layerManagerEvent) }
}
impl crate::org::allbinary::layer::Layer::LayerDyn for AllBinaryGameLayer {
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
    fn paint(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::paint(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1) }
    fn toStringAppend(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::logic::string::StringMaker::StringMaker>>>) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::toStringAppend(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1) }
}


