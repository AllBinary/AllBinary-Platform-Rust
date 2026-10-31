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
// package org.allbinary.game.layer.waypoint
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::layer::NullPathFindingLayer::NullPathFindingLayer;
use crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface;
use crate::org::allbinary::game::layer::RTSLayer::RTSLayer;
use crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintable;
use crate::org::allbinary::graphics::draw::KeyValueDrawString::KeyValueDrawString;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WaypointInfoHudPaintable {
    pub base: SelectionHudPaintable,
    pub keyvalueDrawString: Option<Rc<RefCell<KeyValueDrawString>>>,
    pub rtsLayerP: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>,
    pub textLine2Y: i32,
}

impl crate::org::allbinary::graphics::paint::InitUpdatePaintableInterface::InitUpdatePaintableInterface for WaypointInfoHudPaintable {
}
impl crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface for WaypointInfoHudPaintable {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}

impl WaypointInfoHudPaintable {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: SelectionHudPaintable::new_struct(),
            keyvalueDrawString: None,
            rtsLayerP: None,
            textLine2Y: 0,
        };
        this.rtsLayerP = crate::org::allbinary::game::layer::NullPathFindingLayer::NullPathFindingLayer_NULL_PATH_FINDING_LAYER.with(Clone::clone).clone();
        this.keyvalueDrawString = KeyValueDrawString::new(String::from("Owner: "), this.textX.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        self.base.updateMeasurement(graphics.clone());
        let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
        self.textLine2Y = (self.y + jcall!(font).getHeight());
    }
    

    pub fn updateSelectionInfo(&mut self) {
        let mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>> = (self.rtsLayerP).clone() /* as RTSLayer */;
        { let java_arguments = (jcall!(rtsLayer).getName(),); self.setName(java_arguments.0) };
        { let java_arguments = (jcall!(rtsLayer).getVerticleBuildAnimationInterface(),); self.setAnimationInterface(java_arguments.0) };
        { let java_receiver = self.keyvalueDrawString.clone(); let java_arguments = (jcall!(jcall!(rtsLayer).getParentLayer()).getName(),); jcall!(java_receiver).update(java_arguments.0); };
    }
    

    pub fn setRtsLayer(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) {
        self.rtsLayerP = rtsLayer.clone().map(|reference| reference as Rc<RefCell<dyn PathFindingLayerInterface>>);
    }
    

    pub fn getRtsLayer(&mut self) -> Option<Rc<RefCell<dyn PathFindingLayerInterface>>> {
        return self.rtsLayerP.clone();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        self.base.paint(graphics.clone());
        jcall!(self.keyvalueDrawString).paint(graphics.clone(), self.textLine2Y.clone());
        jcall!(self.getAnimationInterface()).paintXY(graphics.clone(), self.imageX.clone(), self.y.clone());
    }
}
impl std::ops::Deref for WaypointInfoHudPaintable {
    type Target = SelectionHudPaintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for WaypointInfoHudPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait WaypointInfoHudPaintableDyn: crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn {
    fn as_WaypointInfoHudPaintable(&self) -> &WaypointInfoHudPaintable;
    fn as_WaypointInfoHudPaintable_mut(&mut self) -> &mut WaypointInfoHudPaintable;
    fn setRtsLayer(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) { WaypointInfoHudPaintable::setRtsLayer(self.as_WaypointInfoHudPaintable_mut(), rtsLayer) }
    fn getRtsLayer(&mut self) -> Option<Rc<RefCell<dyn PathFindingLayerInterface>>> { WaypointInfoHudPaintable::getRtsLayer(self.as_WaypointInfoHudPaintable_mut()) }
}
impl WaypointInfoHudPaintableDyn for WaypointInfoHudPaintable {
    fn as_WaypointInfoHudPaintable(&self) -> &WaypointInfoHudPaintable { self }
    fn as_WaypointInfoHudPaintable_mut(&mut self) -> &mut WaypointInfoHudPaintable { self }
}

impl crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn for WaypointInfoHudPaintable {
    fn as_SelectionHudPaintable(&self) -> &crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintable { &self.base }
    fn as_SelectionHudPaintable_mut(&mut self) -> &mut crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintable { &mut self.base }
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
    fn updateSelectionInfo(&mut self) { Self::updateSelectionInfo(self) }
}
impl crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintableDyn for WaypointInfoHudPaintable {
    fn as_InitUpdatePaintable(&self) -> &crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable { &self.base.base }
    fn as_InitUpdatePaintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable { &mut self.base.base }
    fn update(&mut self) { crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintable::update(crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn::as_SelectionHudPaintable_mut(self)) }
}
impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for WaypointInfoHudPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base.base.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base.base.base }
    fn setBasicColorP(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::graphics::color::BasicColor::BasicColor>>>) { crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintable::setBasicColorP(crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn::as_SelectionHudPaintable_mut(self), p1) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}


