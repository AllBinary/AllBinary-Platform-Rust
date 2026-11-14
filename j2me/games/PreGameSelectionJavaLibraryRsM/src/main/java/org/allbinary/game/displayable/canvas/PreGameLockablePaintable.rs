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
// package org.allbinary.game.displayable.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::image::GameFeatureImageCacheFactory::GameFeatureImageCacheFactory;
use crate::org::allbinary::logic::system::security::licensing::LockedResources::LockedResources;
use crate::org::allbinary::logic::system::security::licensing::LockedUtil::LockedUtil;
use crate::org::allbinary::graphics::form::LockablePaintable::LockablePaintable;
use crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionForm;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PreGameLockablePaintable {
    pub base: LockablePaintable,
    pub halfWidth: i32,
    pub paintableForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionFormDyn>>>,
}


impl PreGameLockablePaintable {
    

    pub fn new_struct(mut paintableForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionFormDyn>>>, mut lockedIndex: i32) -> Self {
        let mut this = Self {
            base: LockablePaintable::new_struct(lockedIndex.clone()),
            halfWidth: 0,
            paintableForm: None,
        };
        // super(...) is in the struct literal above
        this.paintableForm = paintableForm.clone();
        this.halfWidth = { let java_value = (jcall!(jcall!(GameFeatureImageCacheFactory::getInstance()).getWithKey(crate::java_into_object({ let java_field = jcall!(LockedResources::getInstance()).LOCKED_DEMO_GAME_FEATURE_RESOURCE.clone(); java_field }.clone()))).getWidth() / 2).clone(); java_value };
        this
    }

    pub fn new(mut paintableForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionFormDyn>>>, mut lockedIndex: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(paintableForm, lockedIndex))))
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut currentIndex: i32, mut x: i32, mut y: i32) {
        if { let java_value = jcall!(LockedUtil::getInstance()).isLocked(); java_value } && { let java_value = jcall!(self.getPaintableForm()).getSelectedIndex(); java_value } >= self.getLockedIndex() {
            jcall!(self.getAnimation()).paintXY(graphics.clone(), { let java_value = jcall!(self.getPaintableForm()).getDx(); java_value } - self.halfWidth, { let java_value = jcall!(self.getPaintableForm()).getDy(); java_value } + 34);
        }
    }
    

    pub fn getPaintableForm(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionFormDyn>>> {
        return self.paintableForm.clone();
    }
}
impl std::ops::Deref for PreGameLockablePaintable {
    type Target = LockablePaintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PreGameLockablePaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::form::ItemPaintable::ItemPaintableDyn for PreGameLockablePaintable {
    fn as_ItemPaintable(&self) -> &crate::org::allbinary::graphics::form::ItemPaintable::ItemPaintable { &self.base.base }
    fn as_ItemPaintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::ItemPaintable::ItemPaintable { &mut self.base.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut currentIndex: i32, mut x: i32, mut y: i32) { Self::paint(self, graphics, currentIndex, x, y) }
}
impl crate::org::allbinary::graphics::form::LockablePaintable::LockablePaintableDyn for PreGameLockablePaintable {
    fn as_LockablePaintable(&self) -> &crate::org::allbinary::graphics::form::LockablePaintable::LockablePaintable { &self.base }
    fn as_LockablePaintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::LockablePaintable::LockablePaintable { &mut self.base }
}


