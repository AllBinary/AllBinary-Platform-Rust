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
// package org.allbinary.media.image.comparison
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::*;
// use crate::java::awt::image::BufferedImage::BufferedImage;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::graphics::color::ColorCacheFactory::ColorCacheFactory;
use crate::org::allbinary::graphics::color::ColorCacheable::ColorCacheable;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::util::cache::AutomaticCacheInterface::AutomaticCacheInterface;
use crate::org::allbinary::media::image::comparison::pixel::PixelDeltaFactory::PixelDeltaFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageComparator {
    pub imageComparatorConstraintsInterface: Option<Rc<RefCell<dyn ImageComparatorConstraintsInterface>>>,
}


impl ImageComparator {
    

    pub fn new_struct(mut imageComparatorConstraintsInterface: Option<Rc<RefCell<dyn ImageComparatorConstraintsInterface>>>) -> Self {
        let mut this = Self {
            imageComparatorConstraintsInterface: None,
        };
        this.imageComparatorConstraintsInterface = imageComparatorConstraintsInterface.clone();
        this
    }

    pub fn new(mut imageComparatorConstraintsInterface: Option<Rc<RefCell<dyn ImageComparatorConstraintsInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(imageComparatorConstraintsInterface))))
    }
    

    // throws Exception
    pub fn process(&mut self, mut imageComparisonInfo: Option<Rc<RefCell<ImageComparisonResult>>>, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) {
        let mut isCollsionWithAvoidRectangles: bool = jcall!(self.imageComparatorConstraintsInterface).isCollisionWithAvoidRectangles(point.clone());
        if !isCollsionWithAvoidRectangles {
            let mut rgb1: i32 = jcall!(jcall!(imageComparisonInfo).getBufferedImages()[(0) as usize]).getRGB(jcall!(point).getX(), jcall!(point).getY());
            let mut rgb2: i32 = jcall!(jcall!(imageComparisonInfo).getBufferedImages()[(1) as usize]).getRGB(jcall!(point).getX(), jcall!(point).getY());
            let mut automaticCacheInterface: Option<Rc<RefCell<dyn AutomaticCacheInterface>>> = ColorCacheFactory::getInstance().map(|reference| reference as Rc<RefCell<dyn AutomaticCacheInterface>>);
            let mut colorInteger: Option<i32> = (rgb1.clone()).to_string();
            let mut colorCacheable: Option<Rc<RefCell<ColorCacheable>>> = (jcall!(automaticCacheInterface).get(crate::java_into_object((colorInteger.clone()).expect("NullPointerException")))).clone() /* as ColorCacheable */;
            let mut color: Option<Rc<RefCell<Color>>> = jcall!(colorCacheable).getColor();
            let mut colorInteger2: Option<i32> = (rgb2.clone()).to_string();
            let mut colorCacheable2: Option<Rc<RefCell<ColorCacheable>>> = (jcall!(automaticCacheInterface).get(crate::java_into_object((colorInteger.clone()).expect("NullPointerException")))).clone() /* as ColorCacheable */;
            let mut color2: Option<Rc<RefCell<Color>>> = jcall!(colorCacheable).getColor();
            if jcall!(self.imageComparatorConstraintsInterface).isColorAllowed(0, point.clone(), color.clone()) || jcall!(self.imageComparatorConstraintsInterface).isColorAllowed(1, point.clone(), color2.clone()) {
                if (rgb1 - rgb2).abs() <= jcall!(imageComparisonInfo).getTolerance() {
                    jcall!(imageComparisonInfo).pixelsThatMatch += 1;
                } else {
                    jcall!(imageComparisonInfo).add(PixelDeltaFactory::getInstance(jcall!(point).getX(), jcall!(point).getY(), rgb1.clone(), rgb2.clone()));
                }
            }
        } else {
            jcall!(imageComparisonInfo).pixelsIgnored += 1;
        }
    }
    

    // throws Exception
    pub fn compare(&mut self, mut bufferedImage1: Option<Rc<RefCell<BufferedImage>>>, mut bufferedImage2: Option<Rc<RefCell<BufferedImage>>>, mut frameOne: Option<i64>, mut frameTwo: Option<i64>, mut tolerance: i32) -> Option<Rc<RefCell<ImageComparisonResult>>> {
        return { let java_arguments = (bufferedImage1.clone(), bufferedImage2.clone(), frameOne.clone(), frameTwo.clone(), tolerance.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(),); self.compare6(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4, java_arguments.5) };
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn compare6(&mut self, mut bufferedImage1: Option<Rc<RefCell<BufferedImage>>>, mut bufferedImage2: Option<Rc<RefCell<BufferedImage>>>, mut frameOne: Option<i64>, mut frameTwo: Option<i64>, mut tolerance: i32, mut name: String) -> Option<Rc<RefCell<ImageComparisonResult>>> {
        if bufferedImage1.is_none() || bufferedImage2.is_none() {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Input images must not be null."))));
        }
        let mut imageComparisonInfo: Option<Rc<RefCell<ImageComparisonResult>>> = ImageComparisonResult::new(name.clone(), bufferedImage1.clone(), bufferedImage2.clone(), frameOne.clone(), frameTwo.clone(), tolerance.clone());
        for indexY in (0..jcall!(imageComparisonInfo).imageHeight) {
            for indexX in (0..jcall!(imageComparisonInfo).imageWidth) {
                { let java_arguments = (imageComparisonInfo.clone(), jcall!(PointFactory::getInstance()).createXY(indexX.clone(), indexY.clone()),); self.process(java_arguments.0, java_arguments.1) };
                if jcall!(jcall!(imageComparisonInfo).getNonMatchingPixelVector()).size() > jcall!(self.imageComparatorConstraintsInterface).getMaxNonMatchingPixelDeltas() {
                    break;
                }
            }
        }
        return imageComparisonInfo.clone();
    }
}

