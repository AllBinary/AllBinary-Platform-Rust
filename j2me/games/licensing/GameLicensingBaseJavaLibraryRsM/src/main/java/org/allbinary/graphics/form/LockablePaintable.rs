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
// package org.allbinary.graphics.form
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::FeaturedAnimationInterfaceFactoryInterfaceFactory::FeaturedAnimationInterfaceFactoryInterfaceFactory;
use crate::org::allbinary::logic::system::security::licensing::LockedResources::LockedResources;
use crate::org::allbinary::logic::system::security::licensing::LockedUtil::LockedUtil;
use super::ItemPaintable::ItemPaintable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LockablePaintable {
    pub base: ItemPaintable,
    pub animation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
    pub lockedIndex: i32,
}


impl LockablePaintable {
    

    pub fn new_struct(mut lockedIndex: i32) -> Self {
        let mut this = Self {
            base: ItemPaintable::new_struct(),
            animation: None,
            lockedIndex: 0,
        };
        this.animation = jcall!(jcall!(FeaturedAnimationInterfaceFactoryInterfaceFactory::getInstance()).get(jcall!(LockedResources::getInstance()).LOCKED_DEMO_GAME_FEATURE_RESOURCE.clone())).getInstance(0);
        this.lockedIndex = lockedIndex.clone();
        this
    }

    pub fn new(mut lockedIndex: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(lockedIndex))))
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut currentIndex: i32, mut x: i32, mut y: i32) {
        if jcall!(LockedUtil::getInstance()).isLocked() && currentIndex >= self.getLockedIndex() {
            jcall!(self.getAnimation()).paintXY(graphics.clone(), x.clone(), y.clone());
        }
    }
    

    pub fn getLockedIndex(&mut self) -> i32 {
        return self.lockedIndex.clone();
    }
    

    pub fn getAnimation(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return self.animation.clone();
    }
}
impl std::ops::Deref for LockablePaintable {
    type Target = ItemPaintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for LockablePaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait LockablePaintableDyn: crate::org::allbinary::graphics::form::ItemPaintable::ItemPaintableDyn {
    fn as_LockablePaintable(&self) -> &LockablePaintable;
    fn as_LockablePaintable_mut(&mut self) -> &mut LockablePaintable;
    fn getLockedIndex(&mut self) -> i32 { LockablePaintable::getLockedIndex(self.as_LockablePaintable_mut()) }
    fn getAnimation(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { LockablePaintable::getAnimation(self.as_LockablePaintable_mut()) }
}
impl LockablePaintableDyn for LockablePaintable {
    fn as_LockablePaintable(&self) -> &LockablePaintable { self }
    fn as_LockablePaintable_mut(&mut self) -> &mut LockablePaintable { self }
}

impl crate::org::allbinary::graphics::form::ItemPaintable::ItemPaintableDyn for LockablePaintable {
    fn as_ItemPaintable(&self) -> &crate::org::allbinary::graphics::form::ItemPaintable::ItemPaintable { &self.base }
    fn as_ItemPaintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::ItemPaintable::ItemPaintable { &mut self.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut currentIndex: i32, mut x: i32, mut y: i32) { Self::paint(self, graphics, currentIndex, x, y) }
}


