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
// use crate::javax::microedition::khronos::opengles::GL::GL;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
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
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for AllBinaryGameLayer {
    fn getName(&mut self) -> String { self.base.getName() }
}

impl AllBinaryGameLayer {
    

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
        this.RED = jcall!(BasicColorFactory::getInstance()).RED.clone();
        this.paintable = (if unsafe { SWTUtil_isSWT.clone() } { Paintable::new() /* Java anonymous class body: implement it as a named type
        basicSetColorUtil: BasicColorSetUtil::getInstance(),
        BLACK: jcall!(BasicColorFactory::getInstance()).BLACK.clone(),
        

        pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
            jcall!(this.basicSetColorUtil).setBasicColorP(graphics.clone(), this.BLACK.clone());
        }
*/ } else { NullPaintable::getInstance() });
        this.gameKeyEventList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut name: String, mut layerInfo: Option<Rc<RefCell<Rectangle>>>, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, layerInfo, viewPosition))))
    }
    

    // throws Exception
    pub fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(self.commonStrings).NOT_IMPLEMENTED.clone())));
    }
    

    pub fn r#move(&mut self) {
    }
    

    pub fn getGameKeyEventList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.gameKeyEventList.clone();
    }
    

    // throws Exception
    pub fn processInput(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(self.commonStrings).NOT_IMPLEMENTED.clone())));
    }
    

    // throws Exception
    pub fn processTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(self.commonStrings).NOT_IMPLEMENTED.clone())));
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log(jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    // throws Exception
    pub fn onViewPositionChangeEvent(&mut self) {
        self.onChangeEvent(self.viewPositionEvent.clone());
    }
    

    // throws Exception
    pub fn onChangeEvent(&mut self, mut layerManagerEvent: Option<Rc<RefCell<ViewPositionEvent>>>) {
        if GameLayerUtil::GameLayerUtil::isOnScreen(None /* Java this: a &mut self method has no Rc of itself */) {
            self.setVisible(true);
        } else {
            self.setVisible(false);
        }
    }
    

    pub fn paintFirst(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.paintable).paint(graphics.clone());
    }
    

    pub fn paintDebug(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>> = self.getViewPosition();
        let mut viewX: i32 = jcall!(viewPosition).getX();
        let mut viewY: i32 = jcall!(viewPosition).getY();
        jcall!(self.basicSetColorUtil).setBasicColorP(graphics.clone(), self.RED.clone());
        jcall!(graphics).drawRect(viewX.clone(), viewY.clone(), self.getWidth(), self.getHeight());
    }
    

    pub fn setWidth(&mut self, mut width: i32) {
        self.base.setLayerWidth(width.clone());
        self.setHalfWidth(width >> 1);
    }
    

    pub fn setHeight(&mut self, mut height: i32) {
        self.base.setLayerHeight(height.clone());
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
    fn paintFirst(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { AllBinaryGameLayer::paintFirst(self.as_AllBinaryGameLayer_mut(), graphics) }
    fn paintDebug(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { AllBinaryGameLayer::paintDebug(self.as_AllBinaryGameLayer_mut(), graphics) }
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
    fn toStringAppend(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::logic::string::StringMaker::StringMaker>>>) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::toStringAppend(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1) }
}


