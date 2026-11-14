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
// package org.allbinary.direction
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonPhoneStrings::CommonPhoneStrings;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use super::Direction::Direction;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DirectionFactory {
    pub NAME: String,
    pub LEFT: Option<Rc<RefCell<Direction>>>,
    pub RIGHT: Option<Rc<RefCell<Direction>>>,
    pub DOWN: Option<Rc<RefCell<Direction>>>,
    pub UP: Option<Rc<RefCell<Direction>>>,
    pub UP_LEFT: Option<Rc<RefCell<Direction>>>,
    pub UP_RIGHT: Option<Rc<RefCell<Direction>>>,
    pub DOWN_LEFT: Option<Rc<RefCell<Direction>>>,
    pub DOWN_RIGHT: Option<Rc<RefCell<Direction>>>,
    pub NOT_BORDERED_WITH: Option<Rc<RefCell<Direction>>>,
    pub NO_DIRECTION: Option<Rc<RefCell<Direction>>>,
    pub TOP: Option<Rc<RefCell<Direction>>>,
    pub BOTTOM: Option<Rc<RefCell<Direction>>>,
    pub ALL_DIRECTION: Option<Rc<RefCell<Direction>>>,
}

thread_local! { pub static DirectionFactory_instance: Option<Rc<RefCell<DirectionFactory>>> = DirectionFactory::new(); }

impl DirectionFactory {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<DirectionFactory>>> {
        return DirectionFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            NAME: String::new(),
            LEFT: None,
            RIGHT: None,
            DOWN: None,
            UP: None,
            UP_LEFT: None,
            UP_RIGHT: None,
            DOWN_LEFT: None,
            DOWN_RIGHT: None,
            NOT_BORDERED_WITH: None,
            NO_DIRECTION: None,
            TOP: None,
            BOTTOM: None,
            ALL_DIRECTION: None,
        };
        this.NAME = String::from("DIRECTION");
        this.LEFT = Direction::new({ let java_field = jcall!(CommonPhoneStrings::getInstance()).LEFT.clone(); java_field }.clone(), String::from("WEST"), 0, 1);
        this.RIGHT = Direction::new({ let java_field = jcall!(CommonPhoneStrings::getInstance()).RIGHT.clone(); java_field }.clone(), String::from("EAST"), 1, 0);
        this.DOWN = Direction::new({ let java_field = jcall!(CommonPhoneStrings::getInstance()).DOWN.clone(); java_field }.clone(), String::from("SOUTH"), 2, 0);
        this.UP = Direction::new({ let java_field = jcall!(CommonPhoneStrings::getInstance()).UP.clone(); java_field }.clone(), String::from("NORTH"), 3, 0);
        this.NOT_BORDERED_WITH = Direction::new(String::from("NOT_BORDERED_WITH"), String::from("NOT_BORDERED_WITH"), 8, 0);
        this.NO_DIRECTION = Direction::new(String::from("NO_DIRECTION"), String::from("NO_DIRECTION"), 9, 0);
        this.TOP = Direction::new(String::from("TOP"), String::from("TOP"), 10, 0);
        this.BOTTOM = Direction::new(String::from("BOTTOM"), String::from("BOTTOM"), 11, 0);
        this.ALL_DIRECTION = Direction::new(String::from("ALL_DIRECTION"), String::from("ALL_DIRECTION"), 12, 0);
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        this.UP_LEFT = { let java_value = (Direction::new(String::from("UP LEFT"), jcall!(stringBuffer).append(jcall!(this.UP).getOtherName()).append({ let java_field = jcall!(commonSeps).SPACE.clone(); java_field }.clone()).append(jcall!(this.LEFT).getOtherName()).toString(), 4, 0)).clone(); java_value };
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        this.UP_RIGHT = { let java_value = (Direction::new(String::from("UP RIGHT"), jcall!(stringBuffer).append(jcall!(this.UP).getOtherName()).append({ let java_field = jcall!(commonSeps).SPACE.clone(); java_field }.clone()).append(jcall!(this.RIGHT).getOtherName()).toString(), 5, 0)).clone(); java_value };
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        this.DOWN_LEFT = { let java_value = (Direction::new(String::from("DOWN LEFT"), jcall!(stringBuffer).append(jcall!(this.DOWN).getOtherName()).append({ let java_field = jcall!(commonSeps).SPACE.clone(); java_field }.clone()).append(jcall!(this.LEFT).getOtherName()).toString(), 6, 0)).clone(); java_value };
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        this.DOWN_RIGHT = { let java_value = (Direction::new(String::from("DOWN RIGHT"), jcall!(stringBuffer).append(jcall!(this.DOWN).getOtherName()).append({ let java_field = jcall!(commonSeps).SPACE.clone(); java_field }.clone()).append(jcall!(this.RIGHT).getOtherName()).toString(), 7, 0)).clone(); java_value };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

