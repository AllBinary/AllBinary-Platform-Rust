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
// package org.allbinary.media.image
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::image::BufferedImage::BufferedImage;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::ImageUnifierProperties::ImageUnifierProperties;
use super::ImageUnifierCell::ImageUnifierCell;
use super::ImageUtil::ImageUtil;
use super::ImageProcessorInput::ImageProcessorInput;
use super::ImageProcessedVisitor::ImageProcessedVisitor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SpriteSplitterUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonSeps: Option<Rc<RefCell<CommonSeps>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub ANIMATIONS_LABELS: Vec<String>,
    pub DIRECTIONAL_ANIMATIONS: String,
    pub HORIZONTAL_ANIMATIONS: String,
    pub HORIZONTAL_SPRITE: String,
    pub ROW_NAME: Vec<String>,
    pub _ROW: String,
}

thread_local! { pub static SpriteSplitterUtil_instance: Option<Rc<RefCell<SpriteSplitterUtil>>> = SpriteSplitterUtil::new(); }

impl SpriteSplitterUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonSeps: None,
            commonStrings: None,
            ANIMATIONS_LABELS: Vec::new(),
            DIRECTIONAL_ANIMATIONS: String::new(),
            HORIZONTAL_ANIMATIONS: String::new(),
            HORIZONTAL_SPRITE: String::new(),
            ROW_NAME: Vec::new(),
            _ROW: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonSeps = CommonSeps::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.ANIMATIONS_LABELS = vec![String::from("Row Total:"), String::from("Column Total:"), String::from("Column Total:"), String::from("Row Total:"), String::from("Column Total:"), String::from("Row Total:")];
        this.DIRECTIONAL_ANIMATIONS = String::from("Directional DLRU");
        this.HORIZONTAL_ANIMATIONS = String::from("Horizontal Animations");
        this.HORIZONTAL_SPRITE = String::from("Horizontal A=R=Y F=C=X");
        this.ROW_NAME = vec![String::from("idle"), String::from("walk"), String::from("attack"), String::from("defeat")];
        this._ROW = String::from("_row");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<SpriteSplitterUtil>>> {
        return SpriteSplitterUtil_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn process(&mut self, mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>, mut totalFrames: i32, mut totalAnimations: i32, mut widthReduction: i32, mut heightReduction: i32, mut increaseWidth: i32, mut increaseHeight: i32, mut spriteType: String, mut visitor: Option<Rc<RefCell<dyn ImageProcessedVisitor>>>) {
        let mut bufferedImage: Option<Rc<RefCell<BufferedImage>>> = None;
        let mut generatedBufferedImageArray: Vec<Vec<Option<Rc<RefCell<BufferedImage>>>>> = Vec::new();
        let mut bufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = jcall!(imageProcessorInput).getBufferedImageArray();
        for index in (0..(bufferedImageArray.len() as i32)) {
            bufferedImage = bufferedImageArray[(index) as usize].clone();
            jcall!(self.logUtil).putF(spriteType.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
            if spriteType == self.HORIZONTAL_SPRITE {
                let mut cellHeight: i32 = jcall!(bufferedImage).getHeight() / totalAnimations;
                let mut cellWidth: i32 = jcall!(bufferedImage).getWidth() / totalFrames;
                let mut columns: i32 = totalFrames.clone();
                let mut rows: i32 = totalAnimations.clone();
                generatedBufferedImageArray = vec![vec![None; (columns) as usize]; (rows) as usize];
                jcall!(self.logUtil).putF(format!("{}{}", format!("{}{}", format!("{}{}", String::from("Processing Individual Cells columns: "), columns), String::from(" rows: ")), rows), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
                jcall!(self.logUtil).putF(format!("{}{}", format!("{}{}", format!("{}{}", String::from("Processing Individual Cells cellHeight: "), cellHeight), String::from(" cellWidth: ")), cellWidth), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
                let mut imageUtil: Option<Rc<RefCell<ImageUtil>>> = ImageUtil::getInstance();
                let mut nameEnding: String = String::new();
                let mut x: i32 = 0;
                let mut y: i32 = 0;
                for index2 in (0..rows) {
                    y = cellHeight * index2;
                    for index3 in (0..columns) {
                        x = cellWidth * index3;
                        generatedBufferedImageArray[(index2) as usize][(index3) as usize] = jcall!(bufferedImage).getSubimage(x + widthReduction, y + heightReduction, cellWidth - (widthReduction * 2), cellHeight - (heightReduction * 2));
                        if increaseWidth != 0 || increaseHeight != 0 {
                            generatedBufferedImageArray[(index2) as usize][(index3) as usize] = jcall!(imageUtil).createBufferedImageAllowTranslate(generatedBufferedImageArray[(index2) as usize][(index3) as usize].clone(), cellWidth + increaseWidth, cellHeight + increaseHeight, false, true);
                        }
                        nameEnding = jcall!(StringMaker::new()).appendint(index2.clone()).append(jcall!(self.commonSeps).UNDERSCORE.clone()).appendint(index3.clone()).toString();
                        jcall!(visitor).visit(generatedBufferedImageArray[(index2) as usize][(index3) as usize].clone(), nameEnding.clone(), index.clone());
                    }
                }
                jcall!(self.logUtil).putF(String::from("Processing Rows from Cells"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
                let mut imageUnifierProperties: Option<Rc<RefCell<ImageUnifierProperties>>> = ImageUnifierProperties::new();
                jcall!(imageUnifierProperties).setRows((1).to_string());
                jcall!(imageUnifierProperties).setColumns((columns.clone()).to_string());
                let mut imageUnifierCell: Option<Rc<RefCell<ImageUnifierCell>>> = ImageUnifierCell::new((cellWidth - (2 * widthReduction)).to_string(), (cellHeight - (2 * heightReduction)).to_string());
                jcall!(imageUnifierProperties).setImageUnifierCell(imageUnifierCell.clone());
                for index2 in (0..rows) {
                    y = cellHeight * index2;
                    let mut tempBufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = vec![None; (columns) as usize];
                    for index3 in (0..columns) {
                        tempBufferedImageArray[(index3) as usize] = generatedBufferedImageArray[(index2) as usize][(index3) as usize].clone();
                        nameEnding = jcall!(StringMaker::new()).appendint(index2.clone()).append(jcall!(self.commonSeps).UNDERSCORE.clone()).appendint(1).append(self._ROW.clone()).toString();
                    }
                    let mut generatedBufferedImage: Option<Rc<RefCell<BufferedImage>>> = jcall!(ImageUnifierUtil::ImageUnifierUtil::getInstance()).getImage(tempBufferedImageArray.clone(), imageUnifierProperties.clone());
                    jcall!(visitor).visit(generatedBufferedImage.clone(), nameEnding.clone(), index.clone());
                }
            } else if spriteType == self.HORIZONTAL_ANIMATIONS {
            } else if spriteType == self.DIRECTIONAL_ANIMATIONS {
                let mut totalFrames2: i32 = 4;
                let mut cellHeight: i32 = jcall!(bufferedImage).getHeight() / totalFrames2;
                let mut cellWidth: i32 = jcall!(bufferedImage).getWidth() / totalAnimations;
                let mut columns: i32 = totalAnimations.clone();
                let mut rows: i32 = totalFrames2.clone();
                generatedBufferedImageArray = vec![vec![None; (columns) as usize]; (rows) as usize];
                jcall!(self.logUtil).putF(String::from("Processing Individual Cells for each Direction"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
                let mut nameEnding: String = String::new();
                let mut x: i32 = 0;
                let mut y: i32 = 0;
                for index2 in (0..rows) {
                    y = cellHeight * index2;
                    for index3 in (0..columns) {
                        x = cellWidth * index3;
                        generatedBufferedImageArray[(index2) as usize][(index3) as usize] = jcall!(bufferedImage).getSubimage(x + widthReduction, y + heightReduction, cellWidth - (widthReduction * 2), cellHeight - (heightReduction * 2));
                        if increaseWidth != 0 || increaseHeight != 0 {
                            generatedBufferedImageArray[(index2) as usize][(index3) as usize] = jcall!(ImageUtil::getInstance()).createBufferedImageAllowTranslate(generatedBufferedImageArray[(index2) as usize][(index3) as usize].clone(), cellWidth + increaseWidth, cellHeight + increaseHeight, false, true);
                        }
                        nameEnding = jcall!(StringMaker::new()).append(jcall!(self.commonStrings).DIRECTION_NAME[(index2) as usize].clone()).append(jcall!(self.commonSeps).UNDERSCORE.clone()).appendint(index3.clone()).toString();
                        jcall!(visitor).visit(generatedBufferedImageArray[(index2) as usize][(index3) as usize].clone(), nameEnding.clone(), index.clone());
                    }
                }
                let mut imageUnifierProperties: Option<Rc<RefCell<ImageUnifierProperties>>> = ImageUnifierProperties::new();
                jcall!(imageUnifierProperties).setRows((1).to_string());
                jcall!(imageUnifierProperties).setColumns((columns.clone()).to_string());
                let mut imageUnifierCell: Option<Rc<RefCell<ImageUnifierCell>>> = ImageUnifierCell::new((cellWidth - (2 * widthReduction)).to_string() + increaseWidth, (cellHeight - (2 * heightReduction) + increaseHeight).to_string());
                jcall!(imageUnifierProperties).setImageUnifierCell(imageUnifierCell.clone());
                jcall!(self.logUtil).putF(String::from("Processing Rows from Cells for each Direction"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
                for index2 in (0..rows) {
                    y = cellHeight * index2;
                    let mut tempBufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = vec![None; (columns) as usize];
                    for index3 in (0..columns) {
                        tempBufferedImageArray[(index3) as usize] = generatedBufferedImageArray[(index2) as usize][(index3) as usize].clone();
                        nameEnding = jcall!(StringMaker::new()).append(jcall!(self.commonStrings).DIRECTION_NAME[(index2) as usize].clone()).append(jcall!(self.commonSeps).UNDERSCORE.clone()).appendint(1).append(self._ROW.clone()).toString();
                    }
                    let mut generatedBufferedImage: Option<Rc<RefCell<BufferedImage>>> = jcall!(ImageUnifierUtil::ImageUnifierUtil::getInstance()).getImage(tempBufferedImageArray.clone(), imageUnifierProperties.clone());
                    jcall!(visitor).visit(generatedBufferedImage.clone(), nameEnding.clone(), index.clone());
                }
            }
        }
    }
}

