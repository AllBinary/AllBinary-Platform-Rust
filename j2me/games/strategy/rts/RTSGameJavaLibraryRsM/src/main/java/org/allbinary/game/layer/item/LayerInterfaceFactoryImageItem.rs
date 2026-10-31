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
// package org.allbinary.game.layer.item
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::graphics::form::item::ABCustomImageItem::ABCustomImageItem;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::layer::LayerInterfaceFactoryInterface::LayerInterfaceFactoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LayerInterfaceFactoryImageItem {
    pub base: ABCustomImageItem,
    pub animationArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>>,
    pub layerInterfaceFactoryInterface: Option<Rc<RefCell<dyn LayerInterfaceFactoryInterface>>>,
}


impl LayerInterfaceFactoryImageItem {
    

    pub fn new_struct(mut label: String, mut img: Option<Rc<RefCell<Image>>>, mut layout: i32, mut altText: String, mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut animationArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>>, mut layerInterfaceFactoryInterface: Option<Rc<RefCell<dyn LayerInterfaceFactoryInterface>>>) -> Self {
        let mut this = Self {
            base: ABCustomImageItem::new_struct(label.clone(), img.clone(), layout.clone(), altText.clone(), basicColor.clone(), 0),
            animationArray: Vec::new(),
            layerInterfaceFactoryInterface: None,
        };
        // super(...) is in the struct literal above
        this.animationArray = animationArray.clone();
        this.layerInterfaceFactoryInterface = layerInterfaceFactoryInterface.clone();
        this
    }

    pub fn new(mut label: String, mut img: Option<Rc<RefCell<Image>>>, mut layout: i32, mut altText: String, mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut animationArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>>, mut layerInterfaceFactoryInterface: Option<Rc<RefCell<dyn LayerInterfaceFactoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(label, img, layout, altText, basicColor, animationArray, layerInterfaceFactoryInterface))))
    }
    

    pub fn getLayerInterfaceFactoryInterface(&mut self) -> Option<Rc<RefCell<dyn LayerInterfaceFactoryInterface>>> {
        return self.layerInterfaceFactoryInterface.clone();
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        self.base.paintXY(graphics.clone(), x.clone(), y.clone());
        {
            let mut index: i32 = (self.animationArray.len() as i32).clone();
            while { index -= 1; index } >= 0 {
            jcall!(self.animationArray[(index) as usize]).paintXY(graphics.clone(), x.clone(), y.clone());
            }
        }
    }
}
impl std::ops::Deref for LayerInterfaceFactoryImageItem {
    type Target = ABCustomImageItem;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for LayerInterfaceFactoryImageItem {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::form::item::ABCustomImageItem::ABCustomImageItemDyn for LayerInterfaceFactoryImageItem {
    fn as_ABCustomImageItem(&self) -> &crate::org::allbinary::graphics::form::item::ABCustomImageItem::ABCustomImageItem { &self.base }
    fn as_ABCustomImageItem_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::item::ABCustomImageItem::ABCustomImageItem { &mut self.base }
}


