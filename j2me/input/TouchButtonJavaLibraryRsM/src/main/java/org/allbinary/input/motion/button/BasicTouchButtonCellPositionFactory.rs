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
use crate::org::allbinary::graphics::CellPosition::CellPosition;
use crate::org::allbinary::graphics::CellPositionFactory::CellPositionFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::TouchButtonLocationHelper::TouchButtonLocationHelper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicTouchButtonCellPositionFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub TOP_RIGHT: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>,
    pub TOP_LEFT: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>,
    pub SECOND_FROM_TOP_LEFT: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>,
    pub SECOND_FROM_TOP_RIGHT: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>,
    pub TOP_SECOND_FROM_LEFT: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>,
    pub TOP_SECOND_FROM_RIGHT: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>,
    pub BOTTOM_RIGHT: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>,
    pub BOTTOM_SECOND_FROM_RIGHT: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>,
    pub SECOND_FROM_BOTTOM_RIGHT: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>,
    pub THIRD_FROM_BOTTOM_RIGHT: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>,
    pub FOURTH_FROM_BOTTOM_RIGHT: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>,
    pub SECOND_FROM_BOTTOM_SECOND_FROM_RIGHT: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>,
    pub SECOND_FROM_BOTTOM_THIRD_FROM_RIGHT: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>,
    pub SECOND_FROM_BOTTOM_FOURTH_FROM_RIGHT: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>,
    pub BOTTOM_THIRD_FROM_RIGHT: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>,
    pub BOTTOM_FOURTH_FROM_RIGHT: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>,
    pub BOTTOM_SECOND_FROM_LEFT: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>,
    pub SECOND_FROM_BOTTOM_LEFT: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>,
    pub THIRD_FROM_BOTTOM_LEFT: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>,
    pub BOTTOM_LEFT: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>,
    pub SECOND_FROM_BOTTOM_SECOND_FROM_LEFT: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>,
}


impl BasicTouchButtonCellPositionFactory {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            TOP_RIGHT: None,
            TOP_LEFT: None,
            SECOND_FROM_TOP_LEFT: None,
            SECOND_FROM_TOP_RIGHT: None,
            TOP_SECOND_FROM_LEFT: None,
            TOP_SECOND_FROM_RIGHT: None,
            BOTTOM_RIGHT: None,
            BOTTOM_SECOND_FROM_RIGHT: None,
            SECOND_FROM_BOTTOM_RIGHT: None,
            THIRD_FROM_BOTTOM_RIGHT: None,
            FOURTH_FROM_BOTTOM_RIGHT: None,
            SECOND_FROM_BOTTOM_SECOND_FROM_RIGHT: None,
            SECOND_FROM_BOTTOM_THIRD_FROM_RIGHT: None,
            SECOND_FROM_BOTTOM_FOURTH_FROM_RIGHT: None,
            BOTTOM_THIRD_FROM_RIGHT: None,
            BOTTOM_FOURTH_FROM_RIGHT: None,
            BOTTOM_SECOND_FROM_LEFT: None,
            SECOND_FROM_BOTTOM_LEFT: None,
            THIRD_FROM_BOTTOM_LEFT: None,
            BOTTOM_LEFT: None,
            SECOND_FROM_BOTTOM_SECOND_FROM_LEFT: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        jcall!(this.logUtil).putF({ let java_field = jcall!(this.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(this.commonStrings).CONSTRUCTOR.clone(); java_field }.clone());
        let mut touchButtonLocationHelper: Option<Rc<RefCell<TouchButtonLocationHelper>>> = TouchButtonLocationHelper::new();
        let mut cellPositionFactory: Option<Rc<RefCell<CellPositionFactory>>> = CellPositionFactory::getInstance();
        jcall!(cellPositionFactory).init(jcall!(touchButtonLocationHelper).getTotalColumns(), jcall!(touchButtonLocationHelper).getTotalRows());
        this.TOP_LEFT = { let java_value = (jcall!(cellPositionFactory).getInstanceColRow(0, 0)).clone(); java_value };
        this.TOP_RIGHT = { let java_value = (jcall!(cellPositionFactory).getInstanceColRow(jcall!(touchButtonLocationHelper).getTotalColumns() - 1, 0)).clone(); java_value };
        this.BOTTOM_LEFT = { let java_value = (jcall!(cellPositionFactory).getInstanceColRow(0, jcall!(touchButtonLocationHelper).getTotalRows() - 1)).clone(); java_value };
        this.BOTTOM_RIGHT = { let java_value = (jcall!(cellPositionFactory).getInstanceColRow(jcall!(touchButtonLocationHelper).getTotalColumns() - 1, jcall!(touchButtonLocationHelper).getTotalRows() - 1)).clone(); java_value };
        this.BOTTOM_SECOND_FROM_RIGHT = { let java_value = (jcall!(cellPositionFactory).getInstanceColRow(jcall!(touchButtonLocationHelper).getTotalColumns() - 2, jcall!(touchButtonLocationHelper).getTotalRows() - 1)).clone(); java_value };
        this.SECOND_FROM_BOTTOM_RIGHT = { let java_value = (jcall!(cellPositionFactory).getInstanceColRow(jcall!(touchButtonLocationHelper).getTotalColumns() - 1, jcall!(touchButtonLocationHelper).getTotalRows() - 2)).clone(); java_value };
        this.SECOND_FROM_BOTTOM_SECOND_FROM_RIGHT = { let java_value = (jcall!(cellPositionFactory).getInstanceColRow(jcall!(touchButtonLocationHelper).getTotalColumns() - 2, jcall!(touchButtonLocationHelper).getTotalRows() - 2)).clone(); java_value };
        this.BOTTOM_SECOND_FROM_LEFT = { let java_value = (jcall!(cellPositionFactory).getInstanceColRow(1, jcall!(touchButtonLocationHelper).getTotalRows() - 1)).clone(); java_value };
        this.SECOND_FROM_BOTTOM_LEFT = { let java_value = (jcall!(cellPositionFactory).getInstanceColRow(0, jcall!(touchButtonLocationHelper).getTotalRows() - 2)).clone(); java_value };
        this.SECOND_FROM_BOTTOM_SECOND_FROM_LEFT = { let java_value = (jcall!(cellPositionFactory).getInstanceColRow(1, jcall!(touchButtonLocationHelper).getTotalRows() - 2)).clone(); java_value };
        if jcall!(touchButtonLocationHelper).getTotalRows() > 2 {
            this.SECOND_FROM_TOP_LEFT = { let java_value = (jcall!(cellPositionFactory).getInstanceColRow(0, 1)).clone(); java_value };
            this.SECOND_FROM_TOP_RIGHT = { let java_value = (jcall!(cellPositionFactory).getInstanceColRow(jcall!(touchButtonLocationHelper).getTotalColumns() - 1, 1)).clone(); java_value };
        } else {
            this.SECOND_FROM_TOP_LEFT = { let java_value = ({ let java_field = jcall!(cellPositionFactory).NONE.clone(); java_field }).clone(); java_value }.clone();
            this.SECOND_FROM_TOP_RIGHT = { let java_value = ({ let java_field = jcall!(cellPositionFactory).NONE.clone(); java_field }).clone(); java_value }.clone();
        }
        if jcall!(touchButtonLocationHelper).getTotalRows() >= 4 {
            this.THIRD_FROM_BOTTOM_RIGHT = { let java_value = (jcall!(cellPositionFactory).getInstanceColRow(jcall!(touchButtonLocationHelper).getTotalColumns() - 1, jcall!(touchButtonLocationHelper).getTotalRows() - 3)).clone(); java_value };
            this.FOURTH_FROM_BOTTOM_RIGHT = { let java_value = (jcall!(cellPositionFactory).getInstanceColRow(jcall!(touchButtonLocationHelper).getTotalColumns() - 1, jcall!(touchButtonLocationHelper).getTotalRows() - 4)).clone(); java_value };
            this.THIRD_FROM_BOTTOM_LEFT = { let java_value = (jcall!(cellPositionFactory).getInstanceColRow(0, jcall!(touchButtonLocationHelper).getTotalRows() - 3)).clone(); java_value };
        } else {
            this.THIRD_FROM_BOTTOM_RIGHT = { let java_value = ({ let java_field = jcall!(cellPositionFactory).NONE.clone(); java_field }).clone(); java_value }.clone();
            this.FOURTH_FROM_BOTTOM_RIGHT = { let java_value = ({ let java_field = jcall!(cellPositionFactory).NONE.clone(); java_field }).clone(); java_value }.clone();
            this.THIRD_FROM_BOTTOM_LEFT = { let java_value = ({ let java_field = jcall!(cellPositionFactory).NONE.clone(); java_field }).clone(); java_value }.clone();
        }
        if jcall!(touchButtonLocationHelper).getTotalColumns() >= 4 {
            this.SECOND_FROM_BOTTOM_THIRD_FROM_RIGHT = { let java_value = (jcall!(cellPositionFactory).getInstanceColRow(jcall!(touchButtonLocationHelper).getTotalColumns() - 3, jcall!(touchButtonLocationHelper).getTotalRows() - 2)).clone(); java_value };
            this.SECOND_FROM_BOTTOM_FOURTH_FROM_RIGHT = { let java_value = (jcall!(cellPositionFactory).getInstanceColRow(jcall!(touchButtonLocationHelper).getTotalColumns() - 4, jcall!(touchButtonLocationHelper).getTotalRows() - 2)).clone(); java_value };
            this.BOTTOM_THIRD_FROM_RIGHT = { let java_value = (jcall!(cellPositionFactory).getInstanceColRow(jcall!(touchButtonLocationHelper).getTotalColumns() - 3, jcall!(touchButtonLocationHelper).getTotalRows() - 1)).clone(); java_value };
            this.BOTTOM_FOURTH_FROM_RIGHT = { let java_value = (jcall!(cellPositionFactory).getInstanceColRow(jcall!(touchButtonLocationHelper).getTotalColumns() - 4, jcall!(touchButtonLocationHelper).getTotalRows() - 1)).clone(); java_value };
            this.TOP_SECOND_FROM_LEFT = { let java_value = (jcall!(cellPositionFactory).getInstanceColRow(1, 0)).clone(); java_value };
            this.TOP_SECOND_FROM_RIGHT = { let java_value = (jcall!(cellPositionFactory).getInstanceColRow(jcall!(touchButtonLocationHelper).getTotalColumns() - 2, 0)).clone(); java_value };
        } else {
            this.SECOND_FROM_BOTTOM_THIRD_FROM_RIGHT = { let java_value = ({ let java_field = jcall!(cellPositionFactory).NONE.clone(); java_field }).clone(); java_value }.clone();
            this.SECOND_FROM_BOTTOM_FOURTH_FROM_RIGHT = { let java_value = ({ let java_field = jcall!(cellPositionFactory).NONE.clone(); java_field }).clone(); java_value }.clone();
            this.BOTTOM_THIRD_FROM_RIGHT = { let java_value = ({ let java_field = jcall!(cellPositionFactory).NONE.clone(); java_field }).clone(); java_value }.clone();
            this.BOTTOM_FOURTH_FROM_RIGHT = { let java_value = ({ let java_field = jcall!(cellPositionFactory).NONE.clone(); java_field }).clone(); java_value }.clone();
            this.TOP_SECOND_FROM_LEFT = { let java_value = ({ let java_field = jcall!(cellPositionFactory).NONE.clone(); java_field }).clone(); java_value }.clone();
            this.TOP_SECOND_FROM_RIGHT = { let java_value = ({ let java_field = jcall!(cellPositionFactory).NONE.clone(); java_field }).clone(); java_value }.clone();
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

