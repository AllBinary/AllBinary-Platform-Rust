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
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::AnimationInterface::AnimationInterface;
use crate::org::allbinary::graphics::CellPosition::CellPosition;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::graphics::RectangleFactory::RectangleFactory;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::TouchButtonInput::TouchButtonInput;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TouchButton {
    pub base: Paintable,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>,
    pub animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
    pub rawRectangle: Option<Rc<RefCell<Rectangle>>>,
    pub xBorder: i32,
    pub yBorder: i32,
    pub rectangle: Option<Rc<RefCell<Rectangle>>>,
    pub cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>,
    pub animationX: i32,
    pub animationY: i32,
}


impl TouchButton {
    

    pub fn new_struct(mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>, mut animationInterface: Option<Rc<RefCell<dyn AnimationInterface>>>, mut rawRectangle: Option<Rc<RefCell<Rectangle>>>, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>, mut xBorder: i32, mut yBorder: i32) -> Self {
        let mut this = Self {
            base: Paintable::new_struct(),
            logUtil: None,
            commonStrings: None,
            touchButtonInput: None,
            animationInterface: None,
            rawRectangle: None,
            xBorder: 0,
            yBorder: 0,
            rectangle: None,
            cellPosition: None,
            animationX: 0,
            animationY: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.rectangle = crate::org::allbinary::graphics::RectangleFactory::RectangleFactory_SINGLETON.with(Clone::clone).clone();
        this.touchButtonInput = touchButtonInput.clone();
        this.animationInterface = (animationInterface).clone() /* as Animation */;
        this.rawRectangle = rawRectangle.clone();
        this.cellPosition = cellPosition.clone();
        this.xBorder = xBorder.clone();
        this.yBorder = yBorder.clone();
        this.updateRectangle();
        { let java_receiver = this.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Created: ")).append(this.toString()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        this
    }

    pub fn new(mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>, mut animationInterface: Option<Rc<RefCell<dyn AnimationInterface>>>, mut rawRectangle: Option<Rc<RefCell<Rectangle>>>, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>, mut xBorder: i32, mut yBorder: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(touchButtonInput, animationInterface, rawRectangle, cellPosition, xBorder, yBorder))))
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.animationInterface).paintXY(graphics.clone(), self.animationX.clone(), self.animationY.clone());
    }
    

    pub fn updateRectangle(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut x: i32 = jcall!(self.rawRectangle).getWidth() * jcall!(self.cellPosition).getColumn();
            let mut y: i32 = jcall!(self.rawRectangle).getHeight() * jcall!(self.cellPosition).getRow();
            let mut pointFactory: Option<Rc<RefCell<PointFactory>>> = PointFactory::getInstance();
            self.rectangle = Rectangle::new(jcall!(pointFactory).createXY(x + self.xBorder, y + self.yBorder), jcall!(self.rawRectangle).getWidth(), jcall!(self.rawRectangle).getHeight());
            let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(self.rectangle).getPoint();
            self.animationX = jcall!(point).getX();
            self.animationY = jcall!(point).getY();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("updateRectangle"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn getCellPositionP(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>> {
        return self.cellPosition.clone();
    }
    

    pub fn getRectangleP(&mut self) -> Option<Rc<RefCell<Rectangle>>> {
        return self.rectangle.clone();
    }
    

    pub fn getTouchButtonInput(&mut self) -> Option<Rc<RefCell<TouchButtonInput>>> {
        return self.touchButtonInput.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        jcall!(stringBuffer).append(String::from("TouchButton: "));
        jcall!(stringBuffer).append(jcall!(stringUtil).toString(crate::java_into_object(self.rectangle.clone())));
        jcall!(stringBuffer).append(String::from(" CellPosition: "));
        jcall!(stringBuffer).append(jcall!(stringUtil).toString(crate::java_into_object(self.cellPosition.clone())));
        jcall!(stringBuffer).append(String::from(" xBorder: "));
        jcall!(stringBuffer).appendint(self.xBorder.clone());
        jcall!(stringBuffer).append(String::from(" yBorder: "));
        jcall!(stringBuffer).appendint(self.yBorder.clone());
        return jcall!(stringBuffer).toString();
    }
}
impl std::ops::Deref for TouchButton {
    type Target = Paintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TouchButton {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait TouchButtonDyn: crate::org::allbinary::graphics::paint::Paintable::PaintableDyn {
    fn as_TouchButton(&self) -> &TouchButton;
    fn as_TouchButton_mut(&mut self) -> &mut TouchButton;
    fn updateRectangle(&mut self) { TouchButton::updateRectangle(self.as_TouchButton_mut()) }
    fn getCellPositionP(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>> { TouchButton::getCellPositionP(self.as_TouchButton_mut()) }
    fn getRectangleP(&mut self) -> Option<Rc<RefCell<Rectangle>>> { TouchButton::getRectangleP(self.as_TouchButton_mut()) }
    fn getTouchButtonInput(&mut self) -> Option<Rc<RefCell<TouchButtonInput>>> { TouchButton::getTouchButtonInput(self.as_TouchButton_mut()) }
    fn toString(&mut self) -> String { TouchButton::toString(self.as_TouchButton_mut()) }
}
impl TouchButtonDyn for TouchButton {
    fn as_TouchButton(&self) -> &TouchButton { self }
    fn as_TouchButton_mut(&mut self) -> &mut TouchButton { self }
}

impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for TouchButton {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}


