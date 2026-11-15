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
// package org.allbinary.media.image
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::image::BufferedImage::BufferedImage;
use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::microemu::device::j2se::J2SEImmutableImage::J2SEImmutableImage;
use crate::org::microemu::device::j2se::J2SEMutableImage::J2SEMutableImage;
use crate::java::lang::Exception::*;
use super::ImageJ2SERotationUtil::ImageJ2SERotationUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageRotationUtil {
    pub imageJ2SERotationUtil: Option<Rc<RefCell<ImageJ2SERotationUtil>>>,
}

pub static ImageRotationUtil_instance: crate::JavaStatic<Option<Rc<RefCell<ImageRotationUtil>>>> = crate::JavaStatic::new(|| ImageRotationUtil::new());

impl ImageRotationUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ImageRotationUtil>>> {
        return ImageRotationUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            imageJ2SERotationUtil: None,
        };
        this.imageJ2SERotationUtil = ImageJ2SERotationUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn rotateImage(&mut self, mut originalImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut totalAngle: i32) {
        if jcall!(image).isMutable() {
            let mut j2seImage: Option<Rc<RefCell<J2SEMutableImage>>> = (image).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<J2SEMutableImage>() }, "ClassCastException: not a J2SEMutableImage"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<J2SEMutableImage>) } });
            let mut newBufferedImage: Option<Rc<RefCell<BufferedImage>>> = (jcall!(j2seImage).getImage()).clone() /* as BufferedImage */;
            let mut originalAwtImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = None;
            if jcall!(originalImage).isMutable() {
                let mut originalJ2SEImage: Option<Rc<RefCell<J2SEMutableImage>>> = (originalImage).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<J2SEMutableImage>() }, "ClassCastException: not a J2SEMutableImage"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<J2SEMutableImage>) } });
                originalAwtImage = { let java_value = ({ let java_object = (jcall!(originalJ2SEImage).getImage()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>>().expect("ClassCastException").clone()) }).clone(); java_value };
            } else {
                let mut originalJ2SEImage: Option<Rc<RefCell<J2SEImmutableImage>>> = (originalImage).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<J2SEImmutableImage>() }, "ClassCastException: not a J2SEImmutableImage"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<J2SEImmutableImage>) } });
                originalAwtImage = { let java_value = ({ let java_object = (jcall!(originalJ2SEImage).getImage()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>>().expect("ClassCastException").clone()) }).clone(); java_value };
            }
            jcall!(self.imageJ2SERotationUtil).rotateImage(originalAwtImage.clone(), newBufferedImage.clone(), totalAngle.clone());
        } else {
            PreLogUtil::putOE(String::from("Not Mutable"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getRotatedImage"), crate::java_into_object(format!("{}: {:?}", "Exception", ())));
        }
    }
    

    // throws Exception
    pub fn createRotatedImage(&mut self, mut originalImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut rotationInDegrees: i32) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        let mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = jcall!(ImageCreationUtil::ImageCreationUtil::getInstance()).createImageWH(jcall!(originalImage).getWidth(), jcall!(originalImage).getHeight());
        if jcall!(image).isMutable() {
            let mut originalAwtImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = None;
            if jcall!(originalImage).isMutable() {
                let mut originalJ2SEImage: Option<Rc<RefCell<J2SEMutableImage>>> = (originalImage).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<J2SEMutableImage>() }, "ClassCastException: not a J2SEMutableImage"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<J2SEMutableImage>) } });
                originalAwtImage = { let java_value = ({ let java_object = (jcall!(originalJ2SEImage).getImage()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>>().expect("ClassCastException").clone()) }).clone(); java_value };
            } else {
                let mut originalJ2SEImage: Option<Rc<RefCell<J2SEImmutableImage>>> = (originalImage).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<J2SEImmutableImage>() }, "ClassCastException: not a J2SEImmutableImage"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<J2SEImmutableImage>) } });
                originalAwtImage = { let java_value = ({ let java_object = (jcall!(originalJ2SEImage).getImage()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>>().expect("ClassCastException").clone()) }).clone(); java_value };
            }
            let mut j2seImage: Option<Rc<RefCell<J2SEMutableImage>>> = (image).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<J2SEMutableImage>() }, "ClassCastException: not a J2SEMutableImage"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<J2SEMutableImage>) } });
            let mut newBufferedImage: Option<Rc<RefCell<BufferedImage>>> = (jcall!(j2seImage).getImage()).clone() /* as BufferedImage */;
            let mut bufferedImage: Option<Rc<RefCell<BufferedImage>>> = jcall!(self.imageJ2SERotationUtil).createRotatedImage(originalAwtImage.clone(), newBufferedImage.clone(), rotationInDegrees.clone());
            return image.clone();
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Not Mutable"))));
        }
    }
}

