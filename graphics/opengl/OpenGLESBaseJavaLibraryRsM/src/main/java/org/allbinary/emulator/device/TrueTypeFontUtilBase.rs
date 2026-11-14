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
use crate::javax::microedition::lcdui::Font::Font;
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
    pub currentFont: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Font::FontDyn>>>,
    pub scale: i32,
    pub CELLS_PER_ROW: i32,
    pub fontSize: i32,
    pub baseCharWidth: i32,
    pub cellSize: i32,
    pub textureSize: i32,
    pub actualCellsPerRow: i32,
}


impl TrueTypeFontUtilBase {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

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
        this.charArray = vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x6f, 0x50, 0x56, 0x6b, 0x51, 0x52, 0x5a, 0x6a, 0x5c, 0x60, 0x5b, 0x62, 0x72, 0x63, 0x4b, 0x73, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0x6c, 0x69, 0x74, 0x64, 0x75, 0x4c, 0x70, 0xb, 0xc, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x65, 0x76, 0x66, 0x59, 0x61, 0x53, 0x2b, 0x2c, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x67, 0x77, 0x68, 0x71, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x58, 0x0, 0x0, 0x0, 0x0, 0x0, 0x7b, 0x0, 0x0, 0x55, 0x0, 0x79, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
        this._characterWidth = vec![0; ((this.pattern.chars().count() as i32)) as usize];
        this.characterArray = vec!['\0'; (1) as usize];
        this.size = (this.pattern.chars().count() as i32);
        this.lastCapIndex = this.pattern.find('Z').map(|index| index as i32).unwrap_or(-1);
        this.currentFont = Font::getDefaultFont();
        let mut shortPattern: String = { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone();
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
        this.textureSize = { let java_arguments = (this.CELLS_PER_ROW * this.cellSize,); this.getAsTextureSize(java_arguments.0) };
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
pub trait TrueTypeFontUtilBaseDyn: std::any::Any {
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


