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
// use crate::java::awt::image::BufferedImage::BufferedImage;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::logic::util::cache::PoolInterface::PoolInterface;
use crate::org::allbinary::media::image::cache::BufferedImageCacheable::BufferedImageCacheable;
use crate::org::allbinary::media::image::cache::BufferedImageInfo::BufferedImageInfo;
use crate::org::allbinary::media::image::cache::BufferedImageInfoFactory::BufferedImageInfoFactory;
use crate::org::allbinary::media::image::cache::BufferedImagePoolSingleton::BufferedImagePoolSingleton;
use crate::org::allbinary::media::image::comparison::pixel::PixelDelta::PixelDelta;
use super::ImageComparisonResult::ImageComparisonResult;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ChangedPixelsUtil {
}

thread_local! { pub static ChangedPixelsUtil_START_X: i32 = 0; }
thread_local! { pub static ChangedPixelsUtil_START_Y: i32 = 0; }
thread_local! { pub static ChangedPixelsUtil_OFFSET: i32 = 0; }
thread_local! { pub static ChangedPixelsUtil_SCAN_SIZE: i32 = 0; }
thread_local! { pub static ChangedPixelsUtil_CLEAR_INT_ARRAY: Vec<i32> = vec![0; (1800 * 1600) as usize]; }

impl ChangedPixelsUtil {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn generateBufferedImageChacheables(mut imageComparisonInfo: Option<Rc<RefCell<ImageComparisonResult>>>) -> Vec<Option<Rc<RefCell<BufferedImageCacheable>>>> {
        let mut bufferedImageCacheables: Vec<Option<Rc<RefCell<BufferedImageCacheable>>>> = Vec::new();
        let mut originalBufferedImage: Option<Rc<RefCell<BufferedImage>>> = jcall!(imageComparisonInfo).getBufferedImages()[(0) as usize].clone();
        bufferedImageCacheables = vec![None; (2) as usize];
        let mut bufferedImageInfo: Option<Rc<RefCell<BufferedImageInfo>>> = BufferedImageInfoFactory::getInstance(jcall!(imageComparisonInfo).imageWidth.clone(), jcall!(imageComparisonInfo).imageHeight.clone(), jcall!(originalBufferedImage).getType());
        let mut poolInterface: Option<Rc<RefCell<dyn PoolInterface>>> = BufferedImagePoolSingleton::getInstance().map(|reference| reference as Rc<RefCell<dyn PoolInterface>>);
        for index in (0..(bufferedImageCacheables.len() as i32)) {
            let mut bufferedImageCacheable: Option<Rc<RefCell<BufferedImageCacheable>>> = (jcall!(poolInterface).remove(crate::java_into_object(bufferedImageInfo.clone()))).clone() /* as BufferedImageCacheable */;
            bufferedImageCacheables[(index) as usize] = bufferedImageCacheable.clone();
            jcall!(jcall!(bufferedImageCacheables[(index) as usize]).getBufferedImage()).setRGB(ChangedPixelsUtil_START_X.with(Clone::clone).clone(), ChangedPixelsUtil_START_Y.with(Clone::clone).clone(), jcall!(bufferedImageInfo).getWidth(), jcall!(bufferedImageInfo).getHeight(), ChangedPixelsUtil_CLEAR_INT_ARRAY.with(Clone::clone).clone(), ChangedPixelsUtil_OFFSET.with(Clone::clone).clone(), ChangedPixelsUtil_SCAN_SIZE.with(Clone::clone).clone());
        }
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(imageComparisonInfo).getNonMatchingPixelVector();
        let mut size: i32 = jcall!(vector).size();
        for index in (0..size) {
            let mut pixelDelta: Option<Rc<RefCell<PixelDelta>>> = { let java_object = (jcall!(vector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<PixelDelta>>>>().expect("ClassCastException").clone()) };
            if jcall!(pixelDelta).getColorDelta().is_none() {
                print!("{}", String::from("ColorDelta"));
                std::process::exit(0);
            }
            jcall!(jcall!(bufferedImageCacheables[(0) as usize]).getBufferedImage()).setRGB(jcall!(jcall!(pixelDelta).getPoint()).getX(), jcall!(jcall!(pixelDelta).getPoint()).getY(), jcall!(jcall!(pixelDelta).getColorDelta()).getRgb1());
            jcall!(jcall!(bufferedImageCacheables[(1) as usize]).getBufferedImage()).setRGB(jcall!(jcall!(pixelDelta).getPoint()).getX(), jcall!(jcall!(pixelDelta).getPoint()).getY(), jcall!(jcall!(pixelDelta).getColorDelta()).getRgb2());
        }
        return bufferedImageCacheables.clone();
    }
}

