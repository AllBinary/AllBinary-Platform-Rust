// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
// package javax.microedition.lcdui
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::Image::Image;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ChoiceGroupImageFactory {
    pub multiOff: Vec<i8>,
    pub multiOn: Vec<i8>,
    pub radioOff: Vec<i8>,
    pub radioOn: Vec<i8>,
    pub imgMultiOffO: Option<Rc<RefCell<Image>>>,
    pub imgMultiOnO: Option<Rc<RefCell<Image>>>,
    pub imgRadioOffO: Option<Rc<RefCell<Image>>>,
    pub imgRadioOnO: Option<Rc<RefCell<Image>>>,
    pub imgMultiOff: Option<Rc<RefCell<Image>>>,
    pub imgMultiOn: Option<Rc<RefCell<Image>>>,
    pub imgRadioOff: Option<Rc<RefCell<Image>>>,
    pub imgRadioOn: Option<Rc<RefCell<Image>>>,
    pub imageArray: Vec<Option<Rc<RefCell<Image>>>>,
}

thread_local! { pub static ChoiceGroupImageFactory_instance: Option<Rc<RefCell<ChoiceGroupImageFactory>>> = ChoiceGroupImageFactory::new(); }

impl ChoiceGroupImageFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            multiOff: Vec::new(),
            multiOn: Vec::new(),
            radioOff: Vec::new(),
            radioOn: Vec::new(),
            imgMultiOffO: None,
            imgMultiOnO: None,
            imgRadioOffO: None,
            imgRadioOnO: None,
            imgMultiOff: None,
            imgMultiOn: None,
            imgRadioOff: None,
            imgRadioOn: None,
            imageArray: Vec::new(),
        };
        this.multiOff = vec![((-119) as i8), 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 10, 0, 0, 0, 11, 2, 3, 0, 0, 0, 59, 0, ((-12) as i8), ((-117) as i8), 0, 0, 0, 6, 80, 76, 84, 69, ((-1) as i8), ((-1) as i8), ((-1) as i8), ((-69) as i8), ((-69) as i8), ((-69) as i8), ((-57) as i8), 75, ((-33) as i8), ((-8) as i8), 0, 0, 0, 30, 73, 68, 65, 84, 120, ((-38) as i8), 99, 96, 96, 96, 96, 12, 101, ((-8) as i8), ((-51) as i8), ((-32) as i8), ((-64) as i8), 32, ((-64) as i8), ((-60) as i8), ((-64) as i8), ((-64) as i8), ((-128) as i8), 11, 51, ((-122) as i8), 50, ((-4) as i8), 6, 0, 63, 116, 3, 1, 53, ((-108) as i8), 39, ((-26) as i8), 0, 0, 0, 0, 73, 69, 78, 68, ((-82) as i8), 66, 96, ((-126) as i8)];
        this.multiOn = vec![((-119) as i8), 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 10, 0, 0, 0, 11, 2, 3, 0, 0, 0, 59, 0, ((-12) as i8), ((-117) as i8), 0, 0, 0, 12, 80, 76, 84, 69, ((-1) as i8), ((-1) as i8), ((-1) as i8), ((-69) as i8), ((-69) as i8), ((-69) as i8), 106, 106, 106, 2, 2, 2, 106, ((-103) as i8), 14, ((-47) as i8), 0, 0, 0, 53, 73, 68, 65, 84, 120, ((-38) as i8), 99, 96, 96, 124, ((-64) as i8), ((-16) as i8), ((-1) as i8), ((-77) as i8), 3, ((-45) as i8), 65, ((-111) as i8), 15, 76, 12, 108, 12, 76, 12, ((-4) as i8), 12, 76, 12, 18, 12, 76, ((-68) as i8), 127, 24, ((-104) as i8), 126, 45, 96, 96, ((-7) as i8), ((-11) as i8), ((-109) as i8), ((-127) as i8), ((-23) as i8), ((-65) as i8), 3, 3, ((-29) as i8), 127, ((-122) as i8), ((-113) as i8), 0, 5, 37, 12, ((-34) as i8), 1, ((-99) as i8), ((-83) as i8), 100, 0, 0, 0, 0, 73, 69, 78, 68, ((-82) as i8), 66, 96, ((-126) as i8)];
        this.radioOff = vec![((-119) as i8), 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 11, 0, 0, 0, 11, 2, 3, 0, 0, 0, ((-44) as i8), ((-62) as i8), ((-97) as i8), ((-75) as i8), 0, 0, 0, 9, 80, 76, 84, 69, ((-1) as i8), ((-1) as i8), ((-1) as i8), ((-69) as i8), ((-69) as i8), ((-69) as i8), 106, 106, 106, ((-44) as i8), 13, ((-1) as i8), ((-24) as i8), 0, 0, 0, 42, 73, 68, 65, 84, 120, ((-38) as i8), 99, 96, 90, ((-59) as i8), ((-64) as i8), 32, ((-63) as i8), 48, ((-127) as i8), 65, ((-127) as i8), 65, ((-127) as i8), 41, ((-127) as i8), ((-31) as i8), 5, 19, 3, 3, 3, 50, 102, 80, 96, 80, 96, ((-6) as i8), ((-63) as i8), 80, ((-64) as i8), ((-64) as i8), ((-76) as i8), ((-118) as i8), 1, 0, 113, 24, 5, 61, 73, ((-68) as i8), ((-100) as i8), 98, 0, 0, 0, 0, 73, 69, 78, 68, ((-82) as i8), 66, 96, ((-126) as i8)];
        this.radioOn = vec![((-119) as i8), 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 11, 0, 0, 0, 11, 2, 3, 0, 0, 0, ((-44) as i8), ((-62) as i8), ((-97) as i8), ((-75) as i8), 0, 0, 0, 12, 80, 76, 84, 69, ((-1) as i8), ((-1) as i8), ((-1) as i8), ((-69) as i8), ((-69) as i8), ((-69) as i8), 106, 106, 106, 2, 2, 2, 106, ((-103) as i8), 14, ((-47) as i8), 0, 0, 0, 50, 73, 68, 65, 84, 120, ((-38) as i8), 5, ((-63) as i8), 65, 13, 0, 32, 12, 4, ((-63) as i8), ((-19) as i8), ((-11) as i8), ((-117) as i8), 1, 18, 68, ((-100) as i8), 10, 52, 19, 94, 72, 64, 17, 101, ((-122) as i8), 44, ((-44) as i8), ((-29) as i8), 98, ((-52) as i8), 89, 77, ((-102) as i8), 40, 2, 85, ((-95) as i8), ((-73) as i8), ((-63) as i8), ((-104) as i8), ((-63) as i8), 37, ((-117) as i8), 15, ((-40) as i8), 119, 10, 41, 78, 26, ((-79) as i8), 59, 0, 0, 0, 0, 73, 69, 78, 68, ((-82) as i8), 66, 96, ((-126) as i8)];
        this.imgMultiOffO = Image::createImage(this.multiOff.clone(), 0, (this.multiOff.len() as i32).clone());
        this.imgMultiOnO = Image::createImage(this.multiOn.clone(), 0, (this.multiOn.len() as i32).clone());
        this.imgRadioOffO = Image::createImage(this.radioOff.clone(), 0, (this.radioOff.len() as i32).clone());
        this.imgRadioOnO = Image::createImage(this.radioOn.clone(), 0, (this.radioOn.len() as i32).clone());
        this.imgMultiOff = this.imgMultiOffO.clone();
        this.imgMultiOn = this.imgMultiOnO.clone();
        this.imgRadioOff = this.imgRadioOffO.clone();
        this.imgRadioOn = this.imgRadioOnO.clone();
        this.imageArray = vec![this.imgMultiOff.clone(), this.imgMultiOn.clone(), this.imgRadioOff.clone(), this.imgRadioOn.clone()];
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ChoiceGroupImageFactory>>> {
        return ChoiceGroupImageFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn getImageArray(&mut self) -> Vec<Option<Rc<RefCell<Image>>>> {
        return self.imageArray.clone();
    }
    

    pub fn init(&mut self, mut imageArray: Vec<Option<Rc<RefCell<Image>>>>) {
        self.imgMultiOff = imageArray[(0) as usize].clone();
        self.imageArray[(0) as usize] = self.imgMultiOff.clone();
        self.imgMultiOn = imageArray[(1) as usize].clone();
        self.imageArray[(1) as usize] = self.imgMultiOn.clone();
        self.imgRadioOff = imageArray[(2) as usize].clone();
        self.imageArray[(2) as usize] = self.imgRadioOff.clone();
        self.imgRadioOn = imageArray[(3) as usize].clone();
        self.imageArray[(3) as usize] = self.imgRadioOn.clone();
    }
}

