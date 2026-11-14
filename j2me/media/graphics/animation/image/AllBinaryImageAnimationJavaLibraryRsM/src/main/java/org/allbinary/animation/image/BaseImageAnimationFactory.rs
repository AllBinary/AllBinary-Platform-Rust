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
// package org.allbinary.animation.image
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactory;
use crate::org::allbinary::animation::AnimationInterfaceFactoryInterface::AnimationInterfaceFactoryInterface;
use crate::org::allbinary::animation::NullAnimationFactory::NullAnimationFactory;
use crate::org::allbinary::image::AnimationFactoryImageScaleUtil::AnimationFactoryImageScaleUtil;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::media::ScaleProperties::ScaleProperties;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::java::lang::Exception::*;
use super::AnimationFactoryInitializationVisitor::AnimationFactoryInitializationVisitor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BaseImageAnimationFactory {
    pub animationFactoryImageScaleUtil: Option<Rc<RefCell<AnimationFactoryImageScaleUtil>>>,
    pub image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>,
    pub animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>,
    pub sequenceArray: Vec<i32>,
    pub animationFactoryInitializationVisitor: Option<Rc<RefCell<AnimationFactoryInitializationVisitor>>>,
    pub scaleProperties: Option<Rc<RefCell<ScaleProperties>>>,
}

impl AnimationInterfaceFactoryInterface for BaseImageAnimationFactory {
    fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstance(self, instanceId) }
    fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) { Self::setInitialScale(self, scaleProperties) }
}
impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for BaseImageAnimationFactory {
}

impl BaseImageAnimationFactory {
    

    // throws Exception
    pub fn createFactoryBase(mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut sequenceArray: Vec<i32>, mut width: i32, mut height: i32, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Option<Rc<RefCell<dyn BaseImageAnimationFactoryDyn>>> {
        return BaseImageAnimationFactory::new(image.clone(), sequenceArray.clone(), width.clone(), height.clone(), 0, 0, animationBehaviorFactory.clone()).map(|reference| reference as Rc<RefCell<dyn BaseImageAnimationFactoryDyn>>);
    }
    

    pub fn new_struct(mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut sequenceArray: Vec<i32>, mut width: i32, mut height: i32, mut dx: i32, mut dy: i32, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Self {
        let mut this = Self {
            animationFactoryImageScaleUtil: None,
            image: None,
            animationBehaviorFactory: None,
            sequenceArray: Vec::new(),
            animationFactoryInitializationVisitor: None,
            scaleProperties: None,
        };
        this.animationFactoryImageScaleUtil = AnimationFactoryImageScaleUtil::getInstance();
        this.scaleProperties = crate::org::allbinary::media::ScaleProperties::ScaleProperties_instance.with(Clone::clone).clone();
        this.animationFactoryInitializationVisitor = AnimationFactoryInitializationVisitor::new();
        this.image = image.clone();
        jcall!(this.animationFactoryInitializationVisitor).width = width.clone();
        jcall!(this.animationFactoryInitializationVisitor).height = height.clone();
        this.animationBehaviorFactory = animationBehaviorFactory.clone();
        this.sequenceArray = sequenceArray.clone();
        jcall!(this.animationFactoryInitializationVisitor).dx = dx.clone();
        jcall!(this.animationFactoryInitializationVisitor).dy = dy.clone();
        this
    }

    pub fn new(mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut sequenceArray: Vec<i32>, mut width: i32, mut height: i32, mut dx: i32, mut dy: i32, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(image, sequenceArray, width, height, dx, dy, animationBehaviorFactory))))
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return jcall!(NullAnimationFactory::getFactoryInstance()).getInstance(instanceId.clone());
    }
    

    pub fn getImage(&mut self) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        return self.image.clone();
    }
    

    pub fn getSequenceArray(&mut self) -> Vec<i32> {
        return self.sequenceArray.clone();
    }
    

    pub fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) {
        self.scaleProperties = scaleProperties.clone();
        if { let java_field = jcall!(self.scaleProperties).shouldScale.clone(); java_field } {
            jcall!(self.scaleProperties).scaleWidth = ((((({ let java_field = jcall!(self.animationFactoryInitializationVisitor).width.clone(); java_field }) as f32) * { let java_field = jcall!(self.scaleProperties).scaleX.clone(); java_field })) as i32);
            jcall!(self.scaleProperties).scaleHeight = ((((({ let java_field = jcall!(self.animationFactoryInitializationVisitor).height.clone(); java_field }) as f32) * { let java_field = jcall!(self.scaleProperties).scaleY.clone(); java_field })) as i32);
        } else {
        }
    }
    

    pub fn getAnimationFactoryInitializationVisitorP(&mut self) -> Option<Rc<RefCell<AnimationFactoryInitializationVisitor>>> {
        return self.animationFactoryInitializationVisitor.clone();
    }
    

    pub fn getScalePropertiesP(&mut self) -> Option<Rc<RefCell<ScaleProperties>>> {
        return self.scaleProperties.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        let mut commonLabels: Option<Rc<RefCell<CommonLabels>>> = CommonLabels::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from(std::any::type_name::<Self>()));
        jcall!(stringBuffer).append({ let java_field = jcall!(commonLabels).HEIGHT_LABEL.clone(); java_field }.clone());
        jcall!(stringBuffer).appendint(jcall!(self.image).getHeight());
        jcall!(stringBuffer).append({ let java_field = jcall!(commonSeps).SPACE.clone(); java_field }.clone());
        jcall!(stringBuffer).append(jcall!(self.animationFactoryInitializationVisitor).toString());
        return jcall!(stringBuffer).toString();
    }
}
pub trait BaseImageAnimationFactoryDyn: std::any::Any {
    fn as_BaseImageAnimationFactory(&self) -> &BaseImageAnimationFactory;
    fn as_BaseImageAnimationFactory_mut(&mut self) -> &mut BaseImageAnimationFactory;
    fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { BaseImageAnimationFactory::getInstance(self.as_BaseImageAnimationFactory_mut(), instanceId) }
    fn getImage(&mut self) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> { BaseImageAnimationFactory::getImage(self.as_BaseImageAnimationFactory_mut()) }
    fn getSequenceArray(&mut self) -> Vec<i32> { BaseImageAnimationFactory::getSequenceArray(self.as_BaseImageAnimationFactory_mut()) }
    fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) { BaseImageAnimationFactory::setInitialScale(self.as_BaseImageAnimationFactory_mut(), scaleProperties) }
    fn getAnimationFactoryInitializationVisitorP(&mut self) -> Option<Rc<RefCell<AnimationFactoryInitializationVisitor>>> { BaseImageAnimationFactory::getAnimationFactoryInitializationVisitorP(self.as_BaseImageAnimationFactory_mut()) }
    fn getScalePropertiesP(&mut self) -> Option<Rc<RefCell<ScaleProperties>>> { BaseImageAnimationFactory::getScalePropertiesP(self.as_BaseImageAnimationFactory_mut()) }
    fn toString(&mut self) -> String { BaseImageAnimationFactory::toString(self.as_BaseImageAnimationFactory_mut()) }
}
impl BaseImageAnimationFactoryDyn for BaseImageAnimationFactory {
    fn as_BaseImageAnimationFactory(&self) -> &BaseImageAnimationFactory { self }
    fn as_BaseImageAnimationFactory_mut(&mut self) -> &mut BaseImageAnimationFactory { self }
}


