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
// package org.allbinary.input.automation
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::*;
// use crate::javax::help::HelpSet::HelpSet;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbstractInputRobot {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub helpSet: Option<Rc<RefCell<HelpSet>>>,
}


impl AbstractInputRobot {
    

    pub fn new_struct(mut helpSet: Option<Rc<RefCell<HelpSet>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            helpSet: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.setHelpSet(helpSet.clone());
        this
    }

    pub fn new(mut helpSet: Option<Rc<RefCell<HelpSet>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(helpSet))))
    }
    

    pub fn getHelpSet(&mut self) -> Option<Rc<RefCell<HelpSet>>> {
        return self.helpSet.clone();
    }
    

    pub fn mouseMove(&mut self, mut point: Option<Rc<RefCell<Point>>>) {
        self.mouseMove2(jcall!(point).x.clone(), jcall!(point).y.clone());
        jcall!(self.logUtil).putF(format!("{}{}", format!("{}{}", format!("{}{}", String::from("Moved Mouse To: x: "), jcall!(point).x), String::from(" y: ")), jcall!(point).y), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("moveMouse"));
    }
    

    // throws Exception
    pub fn mouseMoveToTarget3(&mut self, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut x: Option<i32>, mut y: Option<i32>) {
        let mut point: Option<Rc<RefCell<Point>>> = PointHelper::PointHelper::getCenterPoint(rectangle.clone());
        self.mouseMove2(jcall!(point).x + x, jcall!(point).y + y);
        jcall!(self.logUtil).putF(format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("Moved Mouse To: x: "), jcall!(point).x), String::from(" y: ")), jcall!(point).y), String::from(" in the middle of: ")), rectangle), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("moveMouseToTarget"));
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn mouseMoveToTarget(&mut self, mut rectangle: Option<Rc<RefCell<Rectangle>>>) {
        self.mouseMoveToTarget3(rectangle.clone(), 0, 0);
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn mouseMove2(&mut self, mut x: Option<i32>, mut y: Option<i32>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn setHelpSet(&mut self, mut helpSet: Option<Rc<RefCell<HelpSet>>>) {
        self.helpSet = helpSet.clone();
    }
}
pub trait AbstractInputRobotDyn {
    fn as_AbstractInputRobot(&self) -> &AbstractInputRobot;
    fn as_AbstractInputRobot_mut(&mut self) -> &mut AbstractInputRobot;
    fn getHelpSet(&mut self) -> Option<Rc<RefCell<HelpSet>>> { AbstractInputRobot::getHelpSet(self.as_AbstractInputRobot_mut()) }
    fn mouseMove(&mut self, mut point: Option<Rc<RefCell<Point>>>) { AbstractInputRobot::mouseMove(self.as_AbstractInputRobot_mut(), point) }
    fn mouseMoveToTarget3(&mut self, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut x: Option<i32>, mut y: Option<i32>) { AbstractInputRobot::mouseMoveToTarget3(self.as_AbstractInputRobot_mut(), rectangle, x, y) }
    fn mouseMoveToTarget(&mut self, mut rectangle: Option<Rc<RefCell<Rectangle>>>) { AbstractInputRobot::mouseMoveToTarget(self.as_AbstractInputRobot_mut(), rectangle) }
    fn mouseMove2(&mut self, mut x: Option<i32>, mut y: Option<i32>) { AbstractInputRobot::mouseMove2(self.as_AbstractInputRobot_mut(), x, y) }
    fn setHelpSet(&mut self, mut helpSet: Option<Rc<RefCell<HelpSet>>>) { AbstractInputRobot::setHelpSet(self.as_AbstractInputRobot_mut(), helpSet) }
}
impl AbstractInputRobotDyn for AbstractInputRobot {
    fn as_AbstractInputRobot(&self) -> &AbstractInputRobot { self }
    fn as_AbstractInputRobot_mut(&mut self) -> &mut AbstractInputRobot { self }
}


