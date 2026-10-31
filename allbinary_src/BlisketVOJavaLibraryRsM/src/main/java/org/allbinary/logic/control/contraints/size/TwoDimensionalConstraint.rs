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
// package org.allbinary.logic.control.contraints.size
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::w3c::dom::Node::Node;
use super::TwoDimensionalConstraintInterface::TwoDimensionalConstraintInterface;
use super::SizeConstraintInterface::SizeConstraintInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TwoDimensionalConstraint {
    pub widthSizeConstraintInterface: Option<Rc<RefCell<dyn SizeConstraintInterface>>>,
    pub heightSizeConstraintInterface: Option<Rc<RefCell<dyn SizeConstraintInterface>>>,
}

impl TwoDimensionalConstraintInterface for TwoDimensionalConstraint {
    fn getHeight(&mut self) -> Option<Rc<RefCell<dyn SizeConstraintInterface>>> { Self::getHeight(self) }
    fn getWidth(&mut self) -> Option<Rc<RefCell<dyn SizeConstraintInterface>>> { Self::getWidth(self) }
    fn setHeight(&mut self, mut heightSizeConstraintInterface: Option<Rc<RefCell<dyn SizeConstraintInterface>>>) { Self::setHeight(self, heightSizeConstraintInterface) }
    fn setWidth(&mut self, mut widthSizeConstraintInterface: Option<Rc<RefCell<dyn SizeConstraintInterface>>>) { Self::setWidth(self, widthSizeConstraintInterface) }
}

impl TwoDimensionalConstraint {
    

    pub fn new_struct(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            widthSizeConstraintInterface: None,
            heightSizeConstraintInterface: None,
        };
        this
    }

    pub fn new(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(node))))
    }
    

    pub fn getHeight(&mut self) -> Option<Rc<RefCell<dyn SizeConstraintInterface>>> {
        return self.heightSizeConstraintInterface.clone();
    }
    

    pub fn getWidth(&mut self) -> Option<Rc<RefCell<dyn SizeConstraintInterface>>> {
        return self.widthSizeConstraintInterface.clone();
    }
    

    pub fn setHeight(&mut self, mut heightSizeConstraintInterface: Option<Rc<RefCell<dyn SizeConstraintInterface>>>) {
        self.heightSizeConstraintInterface = heightSizeConstraintInterface.clone();
    }
    

    pub fn setWidth(&mut self, mut widthSizeConstraintInterface: Option<Rc<RefCell<dyn SizeConstraintInterface>>>) {
        self.widthSizeConstraintInterface = widthSizeConstraintInterface.clone();
    }
}

