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
// package org.allbinary.input.motion.button
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::FeaturedAnimationInterfaceFactoryInterfaceFactory::FeaturedAnimationInterfaceFactoryInterfaceFactory;
use crate::org::allbinary::graphics::CellPosition::CellPosition;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use super::TouchButton::TouchButton;
use super::TouchButtonInput::TouchButtonInput;
use super::TouchButtonResource::TouchButtonResource;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FullTouchButton {
    pub base: TouchButton,
}


impl FullTouchButton {
    

    // throws Exception
    pub fn createButton(mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>, mut touchButtonResource: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>>, mut rawRectangle: Option<Rc<RefCell<Rectangle>>>, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>, mut xBorder: i32, mut yBorder: i32) -> Option<Rc<RefCell<FullTouchButton>>> {
        return FullTouchButton::new(touchButtonInput.clone(), jcall!(jcall!(FeaturedAnimationInterfaceFactoryInterfaceFactory::getInstance()).get(jcall!(touchButtonResource).as_TouchButtonResource_mut().RESOURCE.clone())).getInstance(0), jcall!(jcall!(FeaturedAnimationInterfaceFactoryInterfaceFactory::getInstance()).get(jcall!(touchButtonResource).as_TouchButtonResource_mut().HINT.clone())).getInstance(0), rawRectangle.clone(), cellPosition.clone(), xBorder.clone(), yBorder.clone());
    }
    

    pub fn new_struct(mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut hintAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut rawRectangle: Option<Rc<RefCell<Rectangle>>>, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>, mut xBorder: i32, mut yBorder: i32) -> Self {
        let mut this = Self {
            base: TouchButton::new_struct(touchButtonInput.clone(), animationInterface.clone(), hintAnimationInterface.clone(), rawRectangle.clone(), cellPosition.clone(), xBorder.clone(), yBorder.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut hintAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut rawRectangle: Option<Rc<RefCell<Rectangle>>>, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>, mut xBorder: i32, mut yBorder: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(touchButtonInput, animationInterface, hintAnimationInterface, rawRectangle, cellPosition, xBorder, yBorder))))
    }
    

    pub fn updateRectangle(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut pointFactory: Option<Rc<RefCell<PointFactory>>> = PointFactory::getInstance();
            let mut x: i32 = jcall!(self.rawRectangle).getWidth() * jcall!(self.cellPositionP).getColumn();
            let mut y: i32 = jcall!(self.rawRectangle).getHeight() * jcall!(self.cellPositionP).getRow();
            self.rectangleP = Rectangle::new(jcall!(pointFactory).createXY(x.clone(), y.clone()), jcall!(self.rawRectangle).getWidth() + (2 * self.xBorder), jcall!(self.rawRectangle).getHeight() + (2 * self.yBorder));
            let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(self.rectangleP).getPoint();
            self.animationX = jcall!(point).getX() + self.xBorder;
            self.animationY = jcall!(point).getY() + self.yBorder;
            self.hintAnimationY = self.animationY - jcall!(self.rectangleP).getHeight() >> 1;
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("updateRectangle"), crate::java_into_object(e.clone()));
            }
        }
    }
}
impl std::ops::Deref for FullTouchButton {
    type Target = TouchButton;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for FullTouchButton {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::input::motion::button::TouchButton::TouchButtonDyn for FullTouchButton {
    fn as_TouchButton(&self) -> &crate::org::allbinary::input::motion::button::TouchButton::TouchButton { &self.base }
    fn as_TouchButton_mut(&mut self) -> &mut crate::org::allbinary::input::motion::button::TouchButton::TouchButton { &mut self.base }
    fn updateRectangle(&mut self) { Self::updateRectangle(self) }
}
impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for FullTouchButton {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base.base }
}


