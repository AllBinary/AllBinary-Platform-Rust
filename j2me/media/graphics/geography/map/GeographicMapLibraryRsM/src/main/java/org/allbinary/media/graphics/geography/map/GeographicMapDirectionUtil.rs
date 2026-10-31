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
// package org.allbinary.media.graphics.geography.map
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::direction::Direction::Direction;
use crate::org::allbinary::direction::DirectionFactory::DirectionFactory;
use crate::org::allbinary::graphics::CellPosition::CellPosition;
use super::GeographicMapCellPosition::GeographicMapCellPosition;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GeographicMapDirectionUtil {
    pub MESSAGE: String,
    pub ERROR: String,
    pub EQUAL: String,
}

thread_local! { pub static GeographicMapDirectionUtil_instance: Option<Rc<RefCell<GeographicMapDirectionUtil>>> = GeographicMapDirectionUtil::new(); }

impl GeographicMapDirectionUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<GeographicMapDirectionUtil>>> {
        return GeographicMapDirectionUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            MESSAGE: String::new(),
            ERROR: String::new(),
            EQUAL: String::new(),
        };
        this.MESSAGE = String::from("Cell Position should have been cached!!!");
        this.ERROR = String::from("Error: ");
        this.EQUAL = String::from(" == ");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getDirectionFromCellPositionToAdjacentCellPosition(&mut self, mut fromGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut toGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<Direction>>> {
        let mut fromColumn: i32 = jcall!(fromGeographicMapCellPosition).getColumn();
        let mut fromRow: i32 = jcall!(fromGeographicMapCellPosition).getRow();
        let mut goColumn: i32 = jcall!(toGeographicMapCellPosition).getColumn();
        let mut goRow: i32 = jcall!(toGeographicMapCellPosition).getRow();
        let mut directionFactory: Option<Rc<RefCell<DirectionFactory>>> = DirectionFactory::getInstance();
        if fromColumn - 1 == goColumn {
            return jcall!(directionFactory).LEFT.clone();
        } else if fromColumn + 1 == goColumn {
            return jcall!(directionFactory).RIGHT.clone();
        } else if fromRow - 1 == goRow {
            return jcall!(directionFactory).UP.clone();
        } else if fromRow + 1 == goRow {
            return jcall!(directionFactory).DOWN.clone();
        } else if match (&fromGeographicMapCellPosition, &toGeographicMapCellPosition) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return jcall!(directionFactory).NO_DIRECTION.clone();
        } else if fromRow == goRow && fromColumn == goColumn {
            panic!("{:?}", format!("{}: {:?}", "Exception", (self.MESSAGE.clone())));
        }
        return jcall!(directionFactory).NOT_BORDERED_WITH.clone();
    }
    

    // throws Exception
    pub fn getEightDirectionFromCellPositionToAdjacentCellPosition(&mut self, mut fromGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut toGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<Direction>>> {
        let mut fromColumn: i32 = jcall!(fromGeographicMapCellPosition).getColumn();
        let mut fromRow: i32 = jcall!(fromGeographicMapCellPosition).getRow();
        let mut goColumn: i32 = jcall!(toGeographicMapCellPosition).getColumn();
        let mut goRow: i32 = jcall!(toGeographicMapCellPosition).getRow();
        let mut directionFactory: Option<Rc<RefCell<DirectionFactory>>> = DirectionFactory::getInstance();
        if fromRow == goRow {
            if fromColumn - 1 == goColumn {
                return jcall!(directionFactory).LEFT.clone();
            } else if fromColumn + 1 == goColumn {
                return jcall!(directionFactory).RIGHT.clone();
            }
        } else if fromRow - 1 == goRow {
            if fromColumn == goColumn {
                return jcall!(directionFactory).UP.clone();
            }
            if fromColumn - 1 == goColumn {
                return jcall!(directionFactory).UP_LEFT.clone();
            } else if fromColumn + 1 == goColumn {
                return jcall!(directionFactory).UP_RIGHT.clone();
            }
        } else if fromRow + 1 == goRow {
            if fromColumn == goColumn {
                return jcall!(directionFactory).DOWN.clone();
            }
            if fromColumn - 1 == goColumn {
                return jcall!(directionFactory).DOWN_LEFT.clone();
            } else if fromColumn + 1 == goColumn {
                return jcall!(directionFactory).DOWN_RIGHT.clone();
            }
        } else if match (&fromGeographicMapCellPosition, &toGeographicMapCellPosition) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return jcall!(directionFactory).NO_DIRECTION.clone();
        } else if fromRow == goRow && fromColumn == goColumn {
            panic!("{:?}", format!("{}: {:?}", "Exception", (self.MESSAGE.clone())));
        }
        return jcall!(directionFactory).NOT_BORDERED_WITH.clone();
    }
    

    // throws Exception
    pub fn getDirectionFromCellPositionToCellPosition(&mut self, mut fromGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut toGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<Direction>>> {
        let mut fromColumn: i32 = jcall!(fromGeographicMapCellPosition).getColumn();
        let mut fromRow: i32 = jcall!(fromGeographicMapCellPosition).getRow();
        let mut goColumn: i32 = jcall!(toGeographicMapCellPosition).getColumn();
        let mut goRow: i32 = jcall!(toGeographicMapCellPosition).getRow();
        let mut directionFactory: Option<Rc<RefCell<DirectionFactory>>> = DirectionFactory::getInstance();
        if fromColumn > goColumn {
            return jcall!(directionFactory).LEFT.clone();
        } else if fromColumn < goColumn {
            return jcall!(directionFactory).RIGHT.clone();
        } else if fromRow > goRow {
            return jcall!(directionFactory).UP.clone();
        } else if fromRow < goRow {
            return jcall!(directionFactory).DOWN.clone();
        }
        let mut stringMaker: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut string: String = jcall!(stringMaker).append(CellPosition::toStringCellPosition(fromGeographicMapCellPosition.clone())).append(String::from(" == ")).append(CellPosition::toStringCellPosition(toGeographicMapCellPosition.clone())).toString();
        jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(stringMaker).append(String::from("Error: ")).append(string.clone()).toString())));
    }
    

    // throws Exception
    pub fn getEightDirectionFromCellPositionToCellPosition(&mut self, mut fromGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut toGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<Direction>>> {
        let mut fromColumn: i32 = jcall!(fromGeographicMapCellPosition).getColumn();
        let mut fromRow: i32 = jcall!(fromGeographicMapCellPosition).getRow();
        let mut goColumn: i32 = jcall!(toGeographicMapCellPosition).getColumn();
        let mut goRow: i32 = jcall!(toGeographicMapCellPosition).getRow();
        let mut directionFactory: Option<Rc<RefCell<DirectionFactory>>> = DirectionFactory::getInstance();
        if match (&fromGeographicMapCellPosition, &toGeographicMapCellPosition) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return jcall!(directionFactory).NO_DIRECTION.clone();
        } else if fromRow == goRow {
            if fromColumn > goColumn {
                return jcall!(directionFactory).LEFT.clone();
            } else if fromColumn < goColumn {
                return jcall!(directionFactory).RIGHT.clone();
            }
        } else if fromRow > goRow {
            if fromColumn > goColumn {
                return jcall!(directionFactory).UP_LEFT.clone();
            } else if fromColumn < goColumn {
                return jcall!(directionFactory).UP_RIGHT.clone();
            } else if fromColumn == goColumn {
                return jcall!(directionFactory).UP.clone();
            }
        } else if fromRow < goRow {
            if fromColumn > goColumn {
                return jcall!(directionFactory).DOWN_LEFT.clone();
            } else if fromColumn < goColumn {
                return jcall!(directionFactory).DOWN_RIGHT.clone();
            } else if fromColumn == goColumn {
                return jcall!(directionFactory).DOWN.clone();
            }
        }
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(self.ERROR.clone());
        jcall!(stringBuffer).append(CellPosition::toStringCellPosition(fromGeographicMapCellPosition.clone()));
        jcall!(stringBuffer).append(self.EQUAL.clone());
        jcall!(stringBuffer).append(CellPosition::toStringCellPosition(toGeographicMapCellPosition.clone()));
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(stringBuffer).toString())));
    }
}

