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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
// use crate::javax::microedition::lcdui::Screen::Screen;
use crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem;
use crate::org::allbinary::graphics::form::item::ABCustomItemInterface::ABCustomItemInterface;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use super::AllBinaryTiledLayer::AllBinaryTiledLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TileLayerItem {
    pub base: ABCustomItem,
    pub tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>>,
}

// impl ABCustomItemInterface for TileLayerItem with the methods of the trait

impl TileLayerItem {
    

    pub fn new_struct(mut label: String, mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>>, mut layout: i32, mut altText: String, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: ABCustomItem::new_struct(label.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone()),
            tiledLayer: None,
        };
        // super(...) is in the struct literal above
        this.tiledLayer = tiledLayer.clone();
        this
    }

    pub fn new(mut label: String, mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>>, mut layout: i32, mut altText: String, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(label, tiledLayer, layout, altText, backgroundBasicColor, foregroundBasicColor))))
    }
    

    pub fn setOwner(&mut self, mut owner: Option<Rc<RefCell<Screen>>>) {
    }
    

    pub fn getMinimumWidth(&mut self) -> i32 {
        return jcall!(self.tiledLayer).getWidth();
    }
    

    pub fn getMinimumHeight(&mut self) -> i32 {
        return jcall!(self.tiledLayer).getHeight();
    }
    

    pub fn getMinContentHeight(&mut self) -> i32 {
        return jcall!(self.tiledLayer).getHeight();
    }
    

    pub fn getMinContentWidth(&mut self) -> i32 {
        return jcall!(self.tiledLayer).getWidth();
    }
    

    pub fn getPrefContentHeight(&mut self, mut width: i32) -> i32 {
        return jcall!(self.tiledLayer).getHeight();
    }
    

    pub fn getPrefContentWidth(&mut self, mut height: i32) -> i32 {
        return jcall!(self.tiledLayer).getWidth();
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        { let java_receiver = self.tiledLayer.clone(); let java_arguments = (x.clone(), y.clone(), jcall!(self.tiledLayer).getZP(),); jcall!(java_receiver).setPosition(java_arguments.0, java_arguments.1, java_arguments.2); };
        jcall!(self.tiledLayer).paint(graphics.clone());
    }
    

    pub fn paintUnselected(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
    }
}
impl std::ops::Deref for TileLayerItem {
    type Target = ABCustomItem;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TileLayerItem {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn for TileLayerItem {
    fn as_ABCustomItem(&self) -> &crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem { &self.base }
    fn as_ABCustomItem_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem { &mut self.base }
    fn setOwner(&mut self, mut owner: Option<Rc<RefCell<Screen>>>) { Self::setOwner(self, owner) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
    fn paintUnselected(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintUnselected(self, graphics, x, y) }
    fn getMinimumWidth(&mut self) -> i32 { Self::getMinimumWidth(self) }
    fn getMinimumHeight(&mut self) -> i32 { Self::getMinimumHeight(self) }
}


