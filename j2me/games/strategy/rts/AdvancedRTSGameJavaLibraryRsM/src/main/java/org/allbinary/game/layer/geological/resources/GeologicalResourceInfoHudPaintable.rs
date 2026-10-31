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
// package org.allbinary.game.layer.geological.resources
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::layer::RTSLayer::RTSLayer;
use crate::org::allbinary::game::layer::waypoint::WaypointInfoHudPaintable::WaypointInfoHudPaintable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::graphics::draw::KeyValueDrawCharArray::KeyValueDrawCharArray;
use super::GeologicalGeographicMapCellPosition::GeologicalGeographicMapCellPosition;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GeologicalResourceInfoHudPaintable {
    pub base: WaypointInfoHudPaintable,
    pub keyvalueDrawString: Option<Rc<RefCell<KeyValueDrawCharArray>>>,
    pub fontHeight: i32,
}

thread_local! { pub static GeologicalResourceInfoHudPaintable_RESOURCES: String = String::from("Resources: "); }
impl crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface for GeologicalResourceInfoHudPaintable {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}
impl crate::org::allbinary::graphics::paint::InitUpdatePaintableInterface::InitUpdatePaintableInterface for GeologicalResourceInfoHudPaintable {
}

impl GeologicalResourceInfoHudPaintable {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: WaypointInfoHudPaintable::new_struct(),
            keyvalueDrawString: None,
            fontHeight: 0,
        };
        this.fontHeight = 0;
        this.keyvalueDrawString = KeyValueDrawCharArray::new(GeologicalResourceInfoHudPaintable_RESOURCES.with(Clone::clone).clone(), this.textX.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        self.base.updateMeasurement(graphics.clone());
        let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
        self.fontHeight = (2 * jcall!(font).getHeight());
    }
    

    pub fn updateSelectionInfo(&mut self) {
        self.base.updateSelectionInfo();
        let mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>> = (self.rtsLayerP).clone() /* as RTSLayer */;
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(jcall!(rtsLayer).as_RTSLayer_mut().geographicMapCellPositionAreaBase).getOccupyingGeographicMapCellPositionList();
        let mut total: i32 = 0;
        {
            let mut index: i32 = jcall!(list).size();
            while { index -= 1; index } >= 0 {
            let mut geographicMapCellPosition: Option<Rc<RefCell<GeologicalGeographicMapCellPosition>>> = { let java_object = (jcall!(list).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<GeologicalGeographicMapCellPosition>>>>().expect("ClassCastException").clone()) };
            total += jcall!(jcall!(geographicMapCellPosition).getGeologicalResource()).getTotal();
            }
        }
        { let java_receiver = self.keyvalueDrawString.clone(); let java_arguments = (jcall!(self.getPrimitiveLongUtil()).getCharArray(total.clone()), jcall!(self.getPrimitiveLongUtil()).getCurrentTotalDigits(),); jcall!(java_receiver).update(java_arguments.0, java_arguments.1); };
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        self.base.paint(graphics.clone());
        jcall!(self.keyvalueDrawString).paint(graphics.clone(), (self.y + self.fontHeight).clone());
    }
}
impl std::ops::Deref for GeologicalResourceInfoHudPaintable {
    type Target = WaypointInfoHudPaintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GeologicalResourceInfoHudPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn for GeologicalResourceInfoHudPaintable {
    fn as_SelectionHudPaintable(&self) -> &crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintable { &self.base.base }
    fn as_SelectionHudPaintable_mut(&mut self) -> &mut crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintable { &mut self.base.base }
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
    fn updateSelectionInfo(&mut self) { Self::updateSelectionInfo(self) }
}
impl crate::org::allbinary::game::layer::waypoint::WaypointInfoHudPaintable::WaypointInfoHudPaintableDyn for GeologicalResourceInfoHudPaintable {
    fn as_WaypointInfoHudPaintable(&self) -> &crate::org::allbinary::game::layer::waypoint::WaypointInfoHudPaintable::WaypointInfoHudPaintable { &self.base }
    fn as_WaypointInfoHudPaintable_mut(&mut self) -> &mut crate::org::allbinary::game::layer::waypoint::WaypointInfoHudPaintable::WaypointInfoHudPaintable { &mut self.base }
}
impl crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintableDyn for GeologicalResourceInfoHudPaintable {
    fn as_InitUpdatePaintable(&self) -> &crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable { &self.base.base.base }
    fn as_InitUpdatePaintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable { &mut self.base.base.base }
    fn update(&mut self) { crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintable::update(crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn::as_SelectionHudPaintable_mut(self)) }
}
impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for GeologicalResourceInfoHudPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base.base.base.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base.base.base.base }
    fn setBasicColorP(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::graphics::color::BasicColor::BasicColor>>>) { crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintable::setBasicColorP(crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn::as_SelectionHudPaintable_mut(self), p1) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}


