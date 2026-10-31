// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
// package org.allbinary.emulator.device
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Font::Font;
use crate::org::allbinary::AvianUtil::AvianUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TrueTypeFontUtilBase {
    pub shortPattern: String,
    pub patternAsBytes: Vec<i8>,
    pub pattern: String,
    pub charArray: Vec<i16>,
    pub _characterWidth: Vec<i32>,
    pub characterArray: Vec<char>,
    pub size: i32,
    pub lastCapIndex: i32,
    pub currentFont: Option<Rc<RefCell<Font>>>,
    pub scale: i32,
    pub CELLS_PER_ROW: i32,
    pub fontSize: i32,
    pub baseCharWidth: i32,
    pub cellSize: i32,
    pub textureSize: i32,
    pub actualCellsPerRow: i32,
}


impl TrueTypeFontUtilBase {
    

    pub fn new_struct(mut scale: i32) -> Self {
        let mut this = Self {
            shortPattern: String::new(),
            patternAsBytes: Vec::new(),
            pattern: String::new(),
            charArray: Vec::new(),
            _characterWidth: Vec::new(),
            characterArray: Vec::new(),
            size: 0,
            lastCapIndex: 0,
            currentFont: None,
            scale: 0,
            CELLS_PER_ROW: 0,
            fontSize: 0,
            baseCharWidth: 0,
            cellSize: 0,
            textureSize: 0,
            actualCellsPerRow: 0,
        };
        this.patternAsBytes = vec![32, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 32, 32, 32, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 32, 32, 32, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 97, 98, 32, 32, 32, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 32, 32, 32, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 46, 63, 32, 32, 32, 33, 36, 37, 96, ((-62) as i8), ((-84) as i8), ((-62) as i8), ((-84) as i8), 34, ((-62) as i8), ((-93) as i8), ((-62) as i8), ((-93) as i8), 94, 38, 42, 40, 32, 32, 32, 41, 95, 43, 45, 61, 91, 93, 123, 125, 59, 39, 35, 58, 32, 32, 32, 64, 126, 44, 47, 60, 62, 92, 124, ((-62) as i8), ((-82) as i8), ((-62) as i8), ((-82) as i8), ((-62) as i8), ((-87) as i8), ((-62) as i8), ((-87) as i8)];
        this.pattern = String::from_utf8(this.patternAsBytes.clone().iter().map(|value| *value as u8).collect()).unwrap();
        this.charArray = vec![0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x6f_u32 as i32, 0x50_u32 as i32, 0x56_u32 as i32, 0x6b_u32 as i32, 0x51_u32 as i32, 0x52_u32 as i32, 0x5a_u32 as i32, 0x6a_u32 as i32, 0x5c_u32 as i32, 0x60_u32 as i32, 0x5b_u32 as i32, 0x62_u32 as i32, 0x72_u32 as i32, 0x63_u32 as i32, 0x4b_u32 as i32, 0x73_u32 as i32, 0x1_u32 as i32, 0x2_u32 as i32, 0x3_u32 as i32, 0x4_u32 as i32, 0x5_u32 as i32, 0x6_u32 as i32, 0x7_u32 as i32, 0x8_u32 as i32, 0x9_u32 as i32, 0xa_u32 as i32, 0x6c_u32 as i32, 0x69_u32 as i32, 0x74_u32 as i32, 0x64_u32 as i32, 0x75_u32 as i32, 0x4c_u32 as i32, 0x70_u32 as i32, 0xb_u32 as i32, 0xc_u32 as i32, 0x10_u32 as i32, 0x11_u32 as i32, 0x12_u32 as i32, 0x13_u32 as i32, 0x14_u32 as i32, 0x15_u32 as i32, 0x16_u32 as i32, 0x17_u32 as i32, 0x18_u32 as i32, 0x19_u32 as i32, 0x1a_u32 as i32, 0x1b_u32 as i32, 0x1c_u32 as i32, 0x20_u32 as i32, 0x21_u32 as i32, 0x22_u32 as i32, 0x23_u32 as i32, 0x24_u32 as i32, 0x25_u32 as i32, 0x26_u32 as i32, 0x27_u32 as i32, 0x28_u32 as i32, 0x29_u32 as i32, 0x2a_u32 as i32, 0x65_u32 as i32, 0x76_u32 as i32, 0x66_u32 as i32, 0x59_u32 as i32, 0x61_u32 as i32, 0x53_u32 as i32, 0x2b_u32 as i32, 0x2c_u32 as i32, 0x30_u32 as i32, 0x31_u32 as i32, 0x32_u32 as i32, 0x33_u32 as i32, 0x34_u32 as i32, 0x35_u32 as i32, 0x36_u32 as i32, 0x37_u32 as i32, 0x38_u32 as i32, 0x39_u32 as i32, 0x3a_u32 as i32, 0x3b_u32 as i32, 0x3c_u32 as i32, 0x40_u32 as i32, 0x41_u32 as i32, 0x42_u32 as i32, 0x43_u32 as i32, 0x44_u32 as i32, 0x45_u32 as i32, 0x46_u32 as i32, 0x47_u32 as i32, 0x48_u32 as i32, 0x49_u32 as i32, 0x4a_u32 as i32, 0x67_u32 as i32, 0x77_u32 as i32, 0x68_u32 as i32, 0x71_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x58_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x7b_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x55_u32 as i32, 0x0_u32 as i32, 0x79_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32, 0x0_u32 as i32];
        this._characterWidth = vec![0; ((this.pattern.chars().count() as i32)) as usize];
        this.characterArray = vec!['\0'; (1) as usize];
        this.size = (this.pattern.chars().count() as i32);
        this.lastCapIndex = this.pattern.find('Z').map(|index| index as i32).unwrap_or(-1);
        this.currentFont = Font::getDefaultFont();
        let mut shortPattern: String = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        if AvianUtil::isAvian() {
            let mut shortPatterAsByteArray: Vec<i8> = vec![32, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 46, 63, 33, 36, 37, 96, ((-62) as i8), ((-84) as i8), ((-62) as i8), ((-84) as i8), 34, ((-62) as i8), ((-93) as i8), ((-62) as i8), ((-93) as i8), 94, 38, 42, 40, 41, 95, 43, 45, 61, 91, 93, 123, 125, 59, 39, 35, 58, 64, 126, 44, 47, 60, 62, 92, 124, ((-62) as i8), ((-82) as i8), ((-62) as i8), ((-82) as i8), ((-62) as i8), ((-87) as i8), ((-62) as i8), ((-87) as i8)];
            shortPattern = String::from_utf8(shortPatterAsByteArray.clone().iter().map(|value| *value as u8).collect()).unwrap();
        } else {
            let mut shortPatterAsByteArray: Vec<i8> = vec![32, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 46, 63, 33, 36, 37, 96, ((-62) as i8), ((-84) as i8), 34, ((-62) as i8), ((-93) as i8), 94, 38, 42, 40, 41, 95, 43, 45, 61, 91, 93, 123, 125, 59, 39, 35, 58, 64, 126, 44, 47, 60, 62, 92, 124, ((-62) as i8), ((-82) as i8), ((-62) as i8), ((-87) as i8)];
            shortPattern = String::from_utf8(shortPatterAsByteArray.clone().iter().map(|value| *value as u8).collect()).unwrap();
        }
        this.shortPattern = shortPattern.clone();
        this.scale = scale.clone();
        this.CELLS_PER_ROW = 16;
        this.fontSize = (20 + 6) * this.scale;
        this.baseCharWidth = this.fontSize + (6 * this.scale);
        this.cellSize = this.fontSize + (6 * this.scale);
        this.textureSize = this.getAsTextureSize(this.CELLS_PER_ROW * this.cellSize);
        this.actualCellsPerRow = this.textureSize / this.cellSize;
        this
    }

    pub fn new(mut scale: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(scale))))
    }
    

    pub fn getAsTextureSize(&mut self, mut textureSize: i32) -> i32 {
        if textureSize <= 64 {
            textureSize = 64;
        } else if textureSize <= 128 {
            textureSize = 128;
        } else if textureSize <= 256 {
            textureSize = 256;
        } else if textureSize <= 512 {
            textureSize = 512;
        } else if textureSize <= 1024 {
            textureSize = 1024;
        }
        return textureSize.clone();
    }
    

    pub fn getCellSize(&mut self, mut cellSize: i32) -> i32 {
        if cellSize <= 16 {
            cellSize = 16;
        } else if cellSize <= 32 {
            cellSize = 32;
        } else if cellSize <= 64 {
            cellSize = 64;
        } else if cellSize <= 128 {
            cellSize = 128;
        } else if cellSize <= 256 {
            cellSize = 256;
        }
        return cellSize.clone();
    }
    

    pub fn getYOffset(&mut self) -> i32 {
        return ((self.scale - 1) * self.cellSize);
    }
}
pub trait TrueTypeFontUtilBaseDyn {
    fn as_TrueTypeFontUtilBase(&self) -> &TrueTypeFontUtilBase;
    fn as_TrueTypeFontUtilBase_mut(&mut self) -> &mut TrueTypeFontUtilBase;
    fn getAsTextureSize(&mut self, mut textureSize: i32) -> i32 { TrueTypeFontUtilBase::getAsTextureSize(self.as_TrueTypeFontUtilBase_mut(), textureSize) }
    fn getCellSize(&mut self, mut cellSize: i32) -> i32 { TrueTypeFontUtilBase::getCellSize(self.as_TrueTypeFontUtilBase_mut(), cellSize) }
    fn getYOffset(&mut self) -> i32 { TrueTypeFontUtilBase::getYOffset(self.as_TrueTypeFontUtilBase_mut()) }
}
impl TrueTypeFontUtilBaseDyn for TrueTypeFontUtilBase {
    fn as_TrueTypeFontUtilBase(&self) -> &TrueTypeFontUtilBase { self }
    fn as_TrueTypeFontUtilBase_mut(&mut self) -> &mut TrueTypeFontUtilBase { self }
}


