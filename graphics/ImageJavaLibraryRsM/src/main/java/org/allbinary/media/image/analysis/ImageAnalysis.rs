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
// package org.allbinary.media.image.analysis
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::*;
// use crate::java::awt::image::BufferedImage::BufferedImage;
use crate::org::allbinary::graphics::color::ColorCacheFactory::ColorCacheFactory;
use crate::org::allbinary::graphics::color::ColorCacheable::ColorCacheable;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageAnalysis {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static ImageAnalysis_instance: Option<Rc<RefCell<ImageAnalysis>>> = ImageAnalysis::new(); }

impl ImageAnalysis {
    

    pub fn getInstance() -> Option<Rc<RefCell<ImageAnalysis>>> {
        return ImageAnalysis_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn process(&mut self, mut bufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>>, mut colorRangeInterface: Option<Rc<RefCell<dyn ColorRangeInterface>>>) -> Vec<Option<Rc<RefCell<ImageAnalysisResults>>>> {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", jcall!(CommonLabels::getInstance()).START, jcall!(colorRangeInterface).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).PROCESS.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut imageAnalysisResultsArray: Vec<Option<Rc<RefCell<ImageAnalysisResults>>>> = vec![None; ((bufferedImageArray.len() as i32)) as usize];
        for index in (0..(bufferedImageArray.len() as i32)) {
            imageAnalysisResultsArray[(index) as usize] = ImageAnalysis::process(bufferedImageArray[(index) as usize].clone(), colorRangeInterface.clone());
        }
        return imageAnalysisResultsArray.clone();
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn process(mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>, mut colorRangeInterface: Option<Rc<RefCell<dyn ColorRangeInterface>>>) -> Option<Rc<RefCell<ImageAnalysisResults>>> {
        let mut imageAnalysisResults: Option<Rc<RefCell<ImageAnalysisResults>>> = ImageAnalysisResults::new();
        let mut redTotal: i64 = 0_i64;
        let mut greenTotal: i64 = 0_i64;
        let mut blueTotal: i64 = 0_i64;
        for indexY in (0..jcall!(bufferedImage).getHeight()) {
            for indexX in (0..jcall!(bufferedImage).getWidth()) {
                let mut keyInteger: Option<i32> = (jcall!(bufferedImage).getRGB(indexX.clone(), indexY.clone())).to_string();
                let mut colorCacheable: Option<Rc<RefCell<ColorCacheable>>> = (jcall!(ColorCacheFactory::getInstance()).get(keyInteger.clone())).clone() /* as ColorCacheable */;
                let mut color: Option<Rc<RefCell<Color>>> = jcall!(colorCacheable).getColor();
                ImageAnalysis::processColorRangeResults(imageAnalysisResults.clone(), colorRangeInterface.clone(), color.clone());
                ImageAnalysis::processImageColorResults(jcall!(imageAnalysisResults).getImageColorResults(), colorRangeInterface.clone(), color.clone());
                redTotal += jcall!(color).getRed();
                greenTotal += jcall!(color).getGreen();
                blueTotal += jcall!(color).getBlue();
            }
        }
        let mut totalPixels: i64 = jcall!(jcall!(imageAnalysisResults).getImageColorRangeResults()).getTotalPixelsChecked();
        let mut colorAverage: Option<Rc<RefCell<ColorAverage>>> = jcall!(jcall!(imageAnalysisResults).getImageColorResults()).getColorAverage();
        jcall!(colorAverage).setAvgRed(((redTotal) as f32) / ((totalPixels) as f32));
        jcall!(colorAverage).setAvgGreen(((greenTotal) as f32) / ((totalPixels) as f32));
        jcall!(colorAverage).setAvgBlue(((blueTotal) as f32) / ((totalPixels) as f32));
        return imageAnalysisResults.clone();
    }
    

    pub fn processColorRangeResults(mut imageAnalysisResults: Option<Rc<RefCell<ImageAnalysisResults>>>, mut colorRangeInterface: Option<Rc<RefCell<dyn ColorRangeInterface>>>, mut color: Option<Rc<RefCell<Color>>>) {
        if jcall!(colorRangeInterface).isInRange(color.clone()) {
            jcall!(jcall!(imageAnalysisResults).getImageColorRangeResults()).addMatchingPixelsChecked();
        } else {
        }
        jcall!(jcall!(imageAnalysisResults).getImageColorRangeResults()).addTotalPixelsChecked();
    }
    

    pub fn processImageColorResults(mut imageColorResults: Option<Rc<RefCell<ImageColorResults>>>, mut colorRangeInterface: Option<Rc<RefCell<dyn ColorRangeInterface>>>, mut color: Option<Rc<RefCell<Color>>>) {
        if jcall!(color).getRed() < jcall!(jcall!(imageColorResults).getColorRange()).getMinRed() {
            jcall!(jcall!(imageColorResults).getColorRange()).setMinRed(jcall!(color).getRed());
        }
        if jcall!(color).getGreen() < jcall!(jcall!(imageColorResults).getColorRange()).getMinGreen() {
            jcall!(jcall!(imageColorResults).getColorRange()).setMinGreen(jcall!(color).getGreen());
        }
        if jcall!(color).getBlue() < jcall!(jcall!(imageColorResults).getColorRange()).getMinBlue() {
            jcall!(jcall!(imageColorResults).getColorRange()).setMinBlue(jcall!(color).getBlue());
        }
        if jcall!(color).getRed() > jcall!(jcall!(imageColorResults).getColorRange()).getMaxRed() {
            jcall!(jcall!(imageColorResults).getColorRange()).setMaxRed(jcall!(color).getRed());
        }
        if jcall!(color).getGreen() > jcall!(jcall!(imageColorResults).getColorRange()).getMaxGreen() {
            jcall!(jcall!(imageColorResults).getColorRange()).setMaxGreen(jcall!(color).getGreen());
        }
        if jcall!(color).getBlue() > jcall!(jcall!(imageColorResults).getColorRange()).getMaxBlue() {
            jcall!(jcall!(imageColorResults).getColorRange()).setMaxBlue(jcall!(color).getBlue());
        }
    }
}

