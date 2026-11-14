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
// package org.allbinary.input.motion.button
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactory;
use crate::org::allbinary::animation::NullAnimationFactory::NullAnimationFactory;
use crate::org::allbinary::animation::image::sprite::OneRowSpriteIndexedAnimationFactory::OneRowSpriteIndexedAnimationFactory;
use crate::org::allbinary::animation::resource::BaseResourceAnimationInterfaceFactoryInterfaceFactory::BaseResourceAnimationInterfaceFactoryInterfaceFactory;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::resource::ResourceLoadingLevelFactory::ResourceLoadingLevelFactory;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvasFactory::ProgressCanvasFactory;
use crate::org::allbinary::graphics::opengles::OpenGLFeatureFactory::OpenGLFeatureFactory;
use crate::org::allbinary::image::ImageCache::ImageCache;
use crate::org::allbinary::image::ImageCacheFactory::ImageCacheFactory;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::os::OperatingSystemFactory::OperatingSystemFactory;
use crate::org::allbinary::media::image::ImageCompleteUtil::ImageCompleteUtil;
use crate::java::lang::Exception::*;
use super::TouchButtonResource::TouchButtonResource;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory {
    pub base: BaseResourceAnimationInterfaceFactoryInterfaceFactory,
}

impl crate::org::allbinary::animation::resource::ResourceAnimationInterfaceFactoryInterface::ResourceAnimationInterfaceFactoryInterface for TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory {
    fn getBasicAnimationInterfaceFactoryInstance(&mut self, p1: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface>>> { crate::org::allbinary::animation::resource::BaseResourceAnimationInterfaceFactoryInterfaceFactory::BaseResourceAnimationInterfaceFactoryInterfaceFactory::getBasicAnimationInterfaceFactoryInstance(crate::org::allbinary::animation::resource::BaseResourceAnimationInterfaceFactoryInterfaceFactory::BaseResourceAnimationInterfaceFactoryInterfaceFactoryDyn::as_BaseResourceAnimationInterfaceFactoryInterfaceFactory_mut(self), p1) }
}
impl crate::org::allbinary::game::configuration::feature::GameFeatureControlledInterface::GameFeatureControlledInterface for TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory {
    fn init(&mut self, mut level: i32) { Self::init(self, level) }
    fn isFeature(&mut self) -> bool { Self::isFeature(self) }
    fn isLoadingLevel(&mut self, mut level: i32) -> bool { Self::isLoadingLevel(self, level) }
}

impl TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory {
    

    pub fn createFactory() -> Option<Rc<RefCell<dyn TouchButtonResourceAnimationInterfaceFactoryInterfaceFactoryDyn>>> {
        return TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory::new(String::from("TouchButton Animations")).map(|reference| reference as Rc<RefCell<dyn TouchButtonResourceAnimationInterfaceFactoryInterfaceFactoryDyn>>);
    }
    

    pub fn new_struct(mut name: String) -> Self {
        let mut this = Self {
            base: BaseResourceAnimationInterfaceFactoryInterfaceFactory::new_struct(name.clone(), jcall!(StdUtil::getInstance()).createHashtable(), jcall!(StdUtil::getInstance()).createHashtable(), jcall!(StdUtil::getInstance()).createHashtable()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut name: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name))))
    }
    

    // throws Exception
    pub fn init(&mut self, mut level: i32) {
        { let java_arguments = (ImageCacheFactory::getInstance(), level.clone(),); self.initImageCache(java_arguments.0, java_arguments.1) };
    }
    

    // throws Exception
    pub fn initImageCache(&mut self, mut imageCache: Option<Rc<RefCell<dyn crate::org::allbinary::image::ImageCache::ImageCacheDyn>>>, mut level: i32) {
        if self.isInitialized() {
            return;
        }
        let mut portion: i32 = 140;
        let mut loadingString: String = jcall!(StringMaker::new()).append(self.toString()).append(String::from(" Loading: ")).toString();
        let mut index: i32 = 0;
        let mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>> = ProgressCanvasFactory::getInstance();
        jcall!(progressCanvas).addPortion(portion.clone(), loadingString.clone(), { let value = index; index += 1; value });
        let mut touchButtonBlankResource: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>> = TouchButtonBlankResource::TouchButtonBlankResource::getInstance();
        let mut touchButtonGenericActionResource: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>> = TouchButtonGenericActionResource::TouchButtonGenericActionResource::getInstance();
        let mut touchButtonStartResource: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>> = TouchButtonStartResource::TouchButtonStartResource::getInstance();
        let mut touchButtonUpResource: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>> = TouchButtonUpResource::TouchButtonUpResource::getInstance();
        let mut touchButtonDownResource: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>> = TouchButtonDownResource::TouchButtonDownResource::getInstance();
        let mut touchButtonTurnLeftResource: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>> = TouchButtonTurnLeftResource::TouchButtonTurnLeftResource::getInstance();
        let mut touchButtonTurnRightResource: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>> = TouchButtonTurnRightResource::TouchButtonTurnRightResource::getInstance();
        let mut touchButtonStrafeLeftResource: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>> = TouchButtonStrafeLeftResource::TouchButtonStrafeLeftResource::getInstance();
        let mut touchButtonStrafeRightResource: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>> = TouchButtonStrafeRightResource::TouchButtonStrafeRightResource::getInstance();
        let mut touchButtonBlankResourceImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = jcall!(imageCache).getWithKey(crate::java_into_object({ let java_field = jcall!(touchButtonBlankResource).as_TouchButtonResource_mut().RESOURCE.clone(); java_field }.clone()));
        let mut touchButtonGenericActionResourceImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = jcall!(imageCache).getWithKey(crate::java_into_object({ let java_field = jcall!(touchButtonGenericActionResource).as_TouchButtonResource_mut().RESOURCE.clone(); java_field }.clone()));
        let mut touchButtonStartResourceImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = jcall!(imageCache).getWithKey(crate::java_into_object({ let java_field = jcall!(touchButtonStartResource).as_TouchButtonResource_mut().RESOURCE.clone(); java_field }.clone()));
        let mut touchButtonUpResourceImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = jcall!(imageCache).getWithKey(crate::java_into_object({ let java_field = jcall!(touchButtonUpResource).as_TouchButtonResource_mut().RESOURCE.clone(); java_field }.clone()));
        let mut touchButtonDownResourceImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = jcall!(imageCache).getWithKey(crate::java_into_object({ let java_field = jcall!(touchButtonDownResource).as_TouchButtonResource_mut().RESOURCE.clone(); java_field }.clone()));
        let mut touchButtonTurnLeftResourceImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = jcall!(imageCache).getWithKey(crate::java_into_object({ let java_field = jcall!(touchButtonTurnLeftResource).as_TouchButtonResource_mut().RESOURCE.clone(); java_field }.clone()));
        let mut touchButtonTurnRightResourceImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = jcall!(imageCache).getWithKey(crate::java_into_object({ let java_field = jcall!(touchButtonTurnRightResource).as_TouchButtonResource_mut().RESOURCE.clone(); java_field }.clone()));
        let mut touchButtonStrafeLeftResourceImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = jcall!(imageCache).getWithKey(crate::java_into_object({ let java_field = jcall!(touchButtonStrafeLeftResource).as_TouchButtonResource_mut().RESOURCE.clone(); java_field }.clone()));
        let mut touchButtonStrafeRightResourceImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = jcall!(imageCache).getWithKey(crate::java_into_object({ let java_field = jcall!(touchButtonStrafeRightResource).as_TouchButtonResource_mut().RESOURCE.clone(); java_field }.clone()));
        let mut nullAnimationFactory: Option<Rc<RefCell<NullAnimationFactory>>> = NullAnimationFactory::getFactoryInstance();
        if jcall!(jcall!(OperatingSystemFactory::getInstance()).getOperatingSystemInstance()).as_GenericOperatingSystem_mut().isOverScan() {
            jcall!(imageCache).getWithKey(crate::java_into_object({ let java_field = jcall!(TouchButtonStartResource::TouchButtonStartResource::getInstance()).as_TouchButtonResource_mut().HINT.clone(); java_field }.clone()));
        }
        jcall!(progressCanvas).addPortion(portion.clone(), loadingString.clone(), { let value = index; index += 1; value });
        let mut imageCompleteUtil: Option<Rc<RefCell<dyn crate::org::allbinary::media::image::ImageCompleteUtil::ImageCompleteUtilDyn>>> = ImageCompleteUtil::getInstance();
        jcall!(progressCanvas).addPortion(portion.clone(), loadingString.clone(), { let value = index; index += 1; value });
        jcall!(imageCompleteUtil).waitForImage(touchButtonBlankResourceImage.clone(), { let java_field = jcall!(touchButtonBlankResource).as_TouchButtonResource_mut().RESOURCE.clone(); java_field }.clone());
        { let java_arguments = ({ let java_field = jcall!(touchButtonBlankResource).as_TouchButtonResource_mut().RESOURCE.clone(); java_field }.clone(), OneRowSpriteIndexedAnimationFactory::createFactory(touchButtonBlankResourceImage.clone(), AnimationBehaviorFactory::getInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface>>),); self.add(java_arguments.0, java_arguments.1) };
        { let java_arguments = ({ let java_field = jcall!(touchButtonBlankResource).as_TouchButtonResource_mut().HINT.clone(); java_field }.clone(), nullAnimationFactory.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface>>),); self.add(java_arguments.0, java_arguments.1) };
        jcall!(progressCanvas).addPortion(portion.clone(), loadingString.clone(), { let value = index; index += 1; value });
        jcall!(imageCompleteUtil).waitForImage(touchButtonGenericActionResourceImage.clone(), { let java_field = jcall!(touchButtonGenericActionResource).as_TouchButtonResource_mut().RESOURCE.clone(); java_field }.clone());
        { let java_arguments = ({ let java_field = jcall!(touchButtonGenericActionResource).as_TouchButtonResource_mut().RESOURCE.clone(); java_field }.clone(), OneRowSpriteIndexedAnimationFactory::createFactory(touchButtonGenericActionResourceImage.clone(), AnimationBehaviorFactory::getInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface>>),); self.add(java_arguments.0, java_arguments.1) };
        { let java_arguments = ({ let java_field = jcall!(touchButtonGenericActionResource).as_TouchButtonResource_mut().HINT.clone(); java_field }.clone(), nullAnimationFactory.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface>>),); self.add(java_arguments.0, java_arguments.1) };
        jcall!(progressCanvas).addPortion(portion.clone(), loadingString.clone(), { let value = index; index += 1; value });
        jcall!(imageCompleteUtil).waitForImage(touchButtonStartResourceImage.clone(), { let java_field = jcall!(touchButtonStartResource).as_TouchButtonResource_mut().RESOURCE.clone(); java_field }.clone());
        { let java_arguments = ({ let java_field = jcall!(touchButtonStartResource).as_TouchButtonResource_mut().RESOURCE.clone(); java_field }.clone(), OneRowSpriteIndexedAnimationFactory::createFactory(touchButtonStartResourceImage.clone(), AnimationBehaviorFactory::getInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface>>),); self.add(java_arguments.0, java_arguments.1) };
        jcall!(progressCanvas).addPortion(portion.clone(), loadingString.clone(), { let value = index; index += 1; value });
        jcall!(imageCompleteUtil).waitForImage(touchButtonUpResourceImage.clone(), { let java_field = jcall!(touchButtonUpResource).as_TouchButtonResource_mut().RESOURCE.clone(); java_field }.clone());
        { let java_arguments = ({ let java_field = jcall!(touchButtonUpResource).as_TouchButtonResource_mut().RESOURCE.clone(); java_field }.clone(), OneRowSpriteIndexedAnimationFactory::createFactory(touchButtonUpResourceImage.clone(), AnimationBehaviorFactory::getInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface>>),); self.add(java_arguments.0, java_arguments.1) };
        { let java_arguments = ({ let java_field = jcall!(touchButtonUpResource).as_TouchButtonResource_mut().HINT.clone(); java_field }.clone(), nullAnimationFactory.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface>>),); self.add(java_arguments.0, java_arguments.1) };
        jcall!(progressCanvas).addPortion(portion.clone(), loadingString.clone(), { let value = index; index += 1; value });
        jcall!(imageCompleteUtil).waitForImage(touchButtonDownResourceImage.clone(), { let java_field = jcall!(touchButtonDownResource).as_TouchButtonResource_mut().RESOURCE.clone(); java_field }.clone());
        { let java_arguments = ({ let java_field = jcall!(touchButtonDownResource).as_TouchButtonResource_mut().RESOURCE.clone(); java_field }.clone(), OneRowSpriteIndexedAnimationFactory::createFactory(touchButtonDownResourceImage.clone(), AnimationBehaviorFactory::getInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface>>),); self.add(java_arguments.0, java_arguments.1) };
        { let java_arguments = ({ let java_field = jcall!(touchButtonDownResource).as_TouchButtonResource_mut().HINT.clone(); java_field }.clone(), nullAnimationFactory.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface>>),); self.add(java_arguments.0, java_arguments.1) };
        jcall!(progressCanvas).addPortion(portion.clone(), loadingString.clone(), { let value = index; index += 1; value });
        jcall!(imageCompleteUtil).waitForImage(touchButtonTurnLeftResourceImage.clone(), { let java_field = jcall!(touchButtonTurnLeftResource).as_TouchButtonResource_mut().RESOURCE.clone(); java_field }.clone());
        { let java_arguments = ({ let java_field = jcall!(touchButtonTurnLeftResource).as_TouchButtonResource_mut().RESOURCE.clone(); java_field }.clone(), OneRowSpriteIndexedAnimationFactory::createFactory(touchButtonTurnLeftResourceImage.clone(), AnimationBehaviorFactory::getInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface>>),); self.add(java_arguments.0, java_arguments.1) };
        { let java_arguments = ({ let java_field = jcall!(touchButtonTurnLeftResource).as_TouchButtonResource_mut().HINT.clone(); java_field }.clone(), nullAnimationFactory.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface>>),); self.add(java_arguments.0, java_arguments.1) };
        jcall!(progressCanvas).addPortion(portion.clone(), loadingString.clone(), { let value = index; index += 1; value });
        jcall!(imageCompleteUtil).waitForImage(touchButtonTurnRightResourceImage.clone(), { let java_field = jcall!(touchButtonTurnRightResource).as_TouchButtonResource_mut().RESOURCE.clone(); java_field }.clone());
        { let java_arguments = ({ let java_field = jcall!(touchButtonTurnRightResource).as_TouchButtonResource_mut().RESOURCE.clone(); java_field }.clone(), OneRowSpriteIndexedAnimationFactory::createFactory(touchButtonTurnRightResourceImage.clone(), AnimationBehaviorFactory::getInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface>>),); self.add(java_arguments.0, java_arguments.1) };
        { let java_arguments = ({ let java_field = jcall!(touchButtonTurnRightResource).as_TouchButtonResource_mut().HINT.clone(); java_field }.clone(), nullAnimationFactory.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface>>),); self.add(java_arguments.0, java_arguments.1) };
        jcall!(progressCanvas).addPortion(portion.clone(), loadingString.clone(), { let value = index; index += 1; value });
        jcall!(imageCompleteUtil).waitForImage(touchButtonStrafeLeftResourceImage.clone(), { let java_field = jcall!(touchButtonStrafeLeftResource).as_TouchButtonResource_mut().RESOURCE.clone(); java_field }.clone());
        { let java_arguments = ({ let java_field = jcall!(touchButtonStrafeLeftResource).as_TouchButtonResource_mut().RESOURCE.clone(); java_field }.clone(), OneRowSpriteIndexedAnimationFactory::createFactory(touchButtonStrafeLeftResourceImage.clone(), AnimationBehaviorFactory::getInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface>>),); self.add(java_arguments.0, java_arguments.1) };
        { let java_arguments = ({ let java_field = jcall!(touchButtonStrafeLeftResource).as_TouchButtonResource_mut().HINT.clone(); java_field }.clone(), nullAnimationFactory.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface>>),); self.add(java_arguments.0, java_arguments.1) };
        jcall!(progressCanvas).addPortion(portion.clone(), loadingString.clone(), { let value = index; index += 1; value });
        jcall!(imageCompleteUtil).waitForImage(touchButtonStrafeRightResourceImage.clone(), { let java_field = jcall!(touchButtonStrafeRightResource).as_TouchButtonResource_mut().RESOURCE.clone(); java_field }.clone());
        { let java_arguments = ({ let java_field = jcall!(touchButtonStrafeRightResource).as_TouchButtonResource_mut().RESOURCE.clone(); java_field }.clone(), OneRowSpriteIndexedAnimationFactory::createFactory(touchButtonStrafeRightResourceImage.clone(), AnimationBehaviorFactory::getInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface>>),); self.add(java_arguments.0, java_arguments.1) };
        { let java_arguments = ({ let java_field = jcall!(touchButtonStrafeRightResource).as_TouchButtonResource_mut().HINT.clone(); java_field }.clone(), nullAnimationFactory.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface>>),); self.add(java_arguments.0, java_arguments.1) };
        if jcall!(jcall!(OperatingSystemFactory::getInstance()).getOperatingSystemInstance()).as_GenericOperatingSystem_mut().isOverScan() {
            let mut touchButtonStartHintResource: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = jcall!(imageCache).getWithKey(crate::java_into_object({ let java_field = jcall!(TouchButtonStartResource::TouchButtonStartResource::getInstance()).as_TouchButtonResource_mut().HINT.clone(); java_field }.clone()));
            { let java_arguments = ({ let java_field = jcall!(touchButtonStartResource).as_TouchButtonResource_mut().HINT.clone(); java_field }.clone(), OneRowSpriteIndexedAnimationFactory::createFactory(touchButtonStartHintResource.clone(), AnimationBehaviorFactory::getInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface>>),); self.add(java_arguments.0, java_arguments.1) };
        } else {
            { let java_arguments = ({ let java_field = jcall!(touchButtonStartResource).as_TouchButtonResource_mut().HINT.clone(); java_field }.clone(), nullAnimationFactory.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface>>),); self.add(java_arguments.0, java_arguments.1) };
        }
        { let java_arguments = (level.clone(),); self.base.init(java_arguments.0) };
    }
    

    pub fn isLoadingLevel(&mut self, mut level: i32) -> bool {
        let mut resourceLoadingLevelFactory: Option<Rc<RefCell<ResourceLoadingLevelFactory>>> = ResourceLoadingLevelFactory::getInstance();
        if level == jcall!({ let java_field = jcall!(resourceLoadingLevelFactory).LOAD_TOUCH.clone(); java_field }).getLevel() {
            return true;
        } else {
            return { let java_arguments = (level.clone(),); self.base.isLoadingLevel(java_arguments.0) };
        }
    }
    

    pub fn isFeature(&mut self) -> bool {
        if !jcall!(Features::getInstance()).isDefault({ let java_field = jcall!(OpenGLFeatureFactory::getInstance()).OPENGL.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            return true;
        } else {
            return false;
        }
    }
}
impl std::ops::Deref for TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory {
    type Target = BaseResourceAnimationInterfaceFactoryInterfaceFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait TouchButtonResourceAnimationInterfaceFactoryInterfaceFactoryDyn: crate::org::allbinary::animation::resource::BaseResourceAnimationInterfaceFactoryInterfaceFactory::BaseResourceAnimationInterfaceFactoryInterfaceFactoryDyn {
    fn as_TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory(&self) -> &TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory;
    fn as_TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory_mut(&mut self) -> &mut TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory;
}
impl TouchButtonResourceAnimationInterfaceFactoryInterfaceFactoryDyn for TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory {
    fn as_TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory(&self) -> &TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory { self }
    fn as_TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory_mut(&mut self) -> &mut TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory { self }
}

impl crate::org::allbinary::animation::resource::BaseResourceAnimationInterfaceFactoryInterfaceFactory::BaseResourceAnimationInterfaceFactoryInterfaceFactoryDyn for TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory {
    fn as_BaseResourceAnimationInterfaceFactoryInterfaceFactory(&self) -> &crate::org::allbinary::animation::resource::BaseResourceAnimationInterfaceFactoryInterfaceFactory::BaseResourceAnimationInterfaceFactoryInterfaceFactory { &self.base }
    fn as_BaseResourceAnimationInterfaceFactoryInterfaceFactory_mut(&mut self) -> &mut crate::org::allbinary::animation::resource::BaseResourceAnimationInterfaceFactoryInterfaceFactory::BaseResourceAnimationInterfaceFactoryInterfaceFactory { &mut self.base }
    fn init(&mut self, mut level: i32) { Self::init(self, level) }
    fn initImageCache(&mut self, mut imageCache: Option<Rc<RefCell<dyn crate::org::allbinary::image::ImageCache::ImageCacheDyn>>>, mut level: i32) { Self::initImageCache(self, imageCache, level) }
    fn isFeature(&mut self) -> bool { Self::isFeature(self) }
    fn isLoadingLevel(&mut self, mut level: i32) -> bool { Self::isLoadingLevel(self, level) }
}


