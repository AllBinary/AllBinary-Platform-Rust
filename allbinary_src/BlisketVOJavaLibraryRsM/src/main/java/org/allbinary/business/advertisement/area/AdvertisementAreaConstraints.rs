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
// package org.allbinary.business.advertisement.area
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::logic::control::contraints::display::browser::DisplayInBrowserContraints::DisplayInBrowserContraints;
use crate::org::allbinary::logic::control::contraints::display::browser::DisplayInBrowserContraintsInterface::DisplayInBrowserContraintsInterface;
use crate::org::allbinary::logic::control::contraints::size::TwoDimensionalConstraint::TwoDimensionalConstraint;
use crate::org::allbinary::logic::control::contraints::size::TwoDimensionalConstraintInterface::TwoDimensionalConstraintInterface;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::AdvertisementAreaConstraintsInterface::AdvertisementAreaConstraintsInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AdvertisementAreaConstraints {
    pub displayInBrowserContraintsInterface: Option<Rc<RefCell<dyn DisplayInBrowserContraintsInterface>>>,
    pub twoDimensionConstraintsInterface: Option<Rc<RefCell<dyn TwoDimensionalConstraintInterface>>>,
}

impl AdvertisementAreaConstraintsInterface for AdvertisementAreaConstraints {
    fn getDisplayInBrowserContraints(&mut self) -> Option<Rc<RefCell<dyn DisplayInBrowserContraintsInterface>>> { Self::getDisplayInBrowserContraints(self) }
    fn getTwoDimensionContraints(&mut self) -> Option<Rc<RefCell<dyn TwoDimensionalConstraintInterface>>> { Self::getTwoDimensionContraints(self) }
}

impl AdvertisementAreaConstraints {
    

    pub fn new_struct(mut document: Option<Rc<RefCell<Document>>>) -> Self {
        let mut this = Self {
            displayInBrowserContraintsInterface: None,
            twoDimensionConstraintsInterface: None,
        };
        let mut node: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(jcall!(AdvertisementAreaData::AdvertisementAreaData::getInstance()).NAME.clone(), jcall!(document).getChildNodes());
        this.displayInBrowserContraintsInterface = DisplayInBrowserContraints::new(node.clone()).map(|reference| reference as Rc<RefCell<dyn DisplayInBrowserContraintsInterface>>);
        this.twoDimensionConstraintsInterface = TwoDimensionalConstraint::new(node.clone()).map(|reference| reference as Rc<RefCell<dyn TwoDimensionalConstraintInterface>>);
        this
    }

    pub fn new(mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(document))))
    }
    

    pub fn setDisplayInBrowserContraints(&mut self, mut displayInBrowserContraintsInterface: Option<Rc<RefCell<dyn DisplayInBrowserContraintsInterface>>>) {
        self.displayInBrowserContraintsInterface = displayInBrowserContraintsInterface.clone();
    }
    

    pub fn getTwoDimensionContraints1(&mut self, mut twoDimensionConstraintsInterface: Option<Rc<RefCell<dyn TwoDimensionalConstraintInterface>>>) {
        self.twoDimensionConstraintsInterface = twoDimensionConstraintsInterface.clone();
    }
    

    pub fn getDisplayInBrowserContraints(&mut self) -> Option<Rc<RefCell<dyn DisplayInBrowserContraintsInterface>>> {
        return self.displayInBrowserContraintsInterface.clone();
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getTwoDimensionContraints(&mut self) -> Option<Rc<RefCell<dyn TwoDimensionalConstraintInterface>>> {
        return self.twoDimensionConstraintsInterface.clone();
    }
}

