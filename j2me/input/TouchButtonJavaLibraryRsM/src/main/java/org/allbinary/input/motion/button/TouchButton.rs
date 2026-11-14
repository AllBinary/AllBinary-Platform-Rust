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
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::FeaturedAnimationInterfaceFactoryInterfaceFactory::FeaturedAnimationInterfaceFactoryInterfaceFactory;
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
use crate::java::lang::Exception::*;
use super::TouchButtonInput::TouchButtonInput;
use super::TouchButtonResource::TouchButtonResource;

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
    pub hintAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
    pub rawRectangle: Option<Rc<RefCell<Rectangle>>>,
    pub xBorder: i32,
    pub yBorder: i32,
    pub rectangleP: Option<Rc<RefCell<Rectangle>>>,
    pub cellPositionP: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>,
    pub animationX: i32,
    pub animationY: i32,
    pub hintAnimationY: i32,
}


impl TouchButton {
    

    // throws Exception
    pub fn createButton(mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>, mut touchButtonResource: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>>, mut rawRectangle: Option<Rc<RefCell<Rectangle>>>, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>, mut xBorder: i32, mut yBorder: i32) -> Option<Rc<RefCell<dyn TouchButtonDyn>>> {
        return TouchButton::new(touchButtonInput.clone(), jcall!(jcall!(FeaturedAnimationInterfaceFactoryInterfaceFactory::getInstance()).get({ let java_field = jcall!(touchButtonResource).as_TouchButtonResource_mut().RESOURCE.clone(); java_field }.clone())).getInstance(0), jcall!(jcall!(FeaturedAnimationInterfaceFactoryInterfaceFactory::getInstance()).get({ let java_field = jcall!(touchButtonResource).as_TouchButtonResource_mut().HINT.clone(); java_field }.clone())).getInstance(0), rawRectangle.clone(), cellPosition.clone(), xBorder.clone(), yBorder.clone()).map(|reference| reference as Rc<RefCell<dyn TouchButtonDyn>>);
    }
    

    pub fn new_struct(mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut hintAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut rawRectangle: Option<Rc<RefCell<Rectangle>>>, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>, mut xBorder: i32, mut yBorder: i32) -> Self {
        let mut this = Self {
            base: Paintable::new_struct(),
            logUtil: None,
            commonStrings: None,
            touchButtonInput: None,
            animationInterface: None,
            hintAnimationInterface: None,
            rawRectangle: None,
            xBorder: 0,
            yBorder: 0,
            rectangleP: None,
            cellPositionP: None,
            animationX: 0,
            animationY: 0,
            hintAnimationY: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.rectangleP = crate::org::allbinary::graphics::RectangleFactory::RectangleFactory_SINGLETON.with(Clone::clone).clone();
        this.touchButtonInput = touchButtonInput.clone();
        this.animationInterface = animationInterface.clone();
        this.hintAnimationInterface = hintAnimationInterface.clone();
        this.rawRectangle = rawRectangle.clone();
        this.cellPositionP = cellPosition.clone();
        this.xBorder = xBorder.clone();
        this.yBorder = yBorder.clone();
        this.updateRectangle();
        { let java_receiver = this.logUtil.clone(); let java_arguments = ({ let java_argument = jcall!(StringMaker::new()).append(String::from("Created: ")).append(this.toString()).toString(); java_argument }, crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(this.commonStrings).CONSTRUCTOR.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        this
    }

    pub fn new(mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut hintAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut rawRectangle: Option<Rc<RefCell<Rectangle>>>, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>, mut xBorder: i32, mut yBorder: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(touchButtonInput, animationInterface, hintAnimationInterface, rawRectangle, cellPosition, xBorder, yBorder))))
    }
    

    pub fn paintHint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        jcall!(self.hintAnimationInterface).paintXY(graphics.clone(), self.animationX.clone(), self.hintAnimationY.clone());
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        jcall!(self.animationInterface).paintXY(graphics.clone(), self.animationX.clone(), self.animationY.clone());
    }
    

    pub fn updateRectangle(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut pointFactory: Option<Rc<RefCell<PointFactory>>> = PointFactory::getInstance();
            let mut x: i32 = jcall!(self.rawRectangle).getWidth() * jcall!(self.cellPositionP).getColumn();
            let mut y: i32 = jcall!(self.rawRectangle).getHeight() * jcall!(self.cellPositionP).getRow();
            self.rectangleP = { let java_value = (Rectangle::new(jcall!(pointFactory).createXY(x + self.xBorder, y + self.yBorder), jcall!(self.rawRectangle).getWidth(), jcall!(self.rawRectangle).getHeight())).clone(); java_value };
            let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(self.rectangleP).getPoint();
            self.animationX = { let java_value = (jcall!(point).getX()).clone(); java_value };
            self.animationY = { let java_value = (jcall!(point).getY()).clone(); java_value };
            self.hintAnimationY = self.animationY - 32;
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("updateRectangle"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn getCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>> {
        return self.cellPositionP.clone();
    }
    

    pub fn getRectangle(&mut self) -> Option<Rc<RefCell<Rectangle>>> {
        return self.rectangleP.clone();
    }
    

    pub fn getTouchButtonInput(&mut self) -> Option<Rc<RefCell<TouchButtonInput>>> {
        return self.touchButtonInput.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        jcall!(stringBuffer).append(String::from("TouchButton: "));
        jcall!(stringBuffer).append({ let java_argument = jcall!(stringUtil).toString(crate::java_into_object(self.getRectangle())); java_argument });
        jcall!(stringBuffer).append(String::from(" CellPosition: "));
        jcall!(stringBuffer).append(jcall!(stringUtil).toString(crate::java_into_object(self.cellPositionP.clone())));
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
    fn paintHint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { TouchButton::paintHint(self.as_TouchButton_mut(), graphics) }
    fn updateRectangle(&mut self) { TouchButton::updateRectangle(self.as_TouchButton_mut()) }
    fn getCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>> { TouchButton::getCellPosition(self.as_TouchButton_mut()) }
    fn getRectangle(&mut self) -> Option<Rc<RefCell<Rectangle>>> { TouchButton::getRectangle(self.as_TouchButton_mut()) }
    fn getTouchButtonInput(&mut self) -> Option<Rc<RefCell<TouchButtonInput>>> { TouchButton::getTouchButtonInput(self.as_TouchButton_mut()) }
}
impl TouchButtonDyn for TouchButton {
    fn as_TouchButton(&self) -> &TouchButton { self }
    fn as_TouchButton_mut(&mut self) -> &mut TouchButton { self }
}

impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for TouchButton {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for TouchButton {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::graphics::paint::Paintable::Paintable::paintThreed(crate::org::allbinary::graphics::paint::Paintable::PaintableDyn::as_Paintable_mut(self), p1) }
}


