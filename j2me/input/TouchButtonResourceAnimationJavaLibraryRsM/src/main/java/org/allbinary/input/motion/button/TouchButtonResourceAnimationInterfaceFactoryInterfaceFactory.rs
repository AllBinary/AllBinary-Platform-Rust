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
// use crate::javax::microedition::lcdui::Image::Image;
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
        let mut touchButtonBlankResource: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>> = TouchButtonBlankResource::TouchButtonBlankResource::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>);
        let mut touchButtonGenericActionResource: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>> = TouchButtonGenericActionResource::TouchButtonGenericActionResource::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>);
        let mut touchButtonStartResource: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>> = TouchButtonStartResource::TouchButtonStartResource::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>);
        let mut touchButtonUpResource: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>> = TouchButtonUpResource::TouchButtonUpResource::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>);
        let mut touchButtonDownResource: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>> = TouchButtonDownResource::TouchButtonDownResource::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>);
        let mut touchButtonTurnLeftResource: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>> = TouchButtonTurnLeftResource::TouchButtonTurnLeftResource::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>);
        let mut touchButtonTurnRightResource: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>> = TouchButtonTurnRightResource::TouchButtonTurnRightResource::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>);
        let mut touchButtonStrafeLeftResource: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>> = TouchButtonStrafeLeftResource::TouchButtonStrafeLeftResource::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>);
        let mut touchButtonStrafeRightResource: Option<Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>> = TouchButtonStrafeRightResource::TouchButtonStrafeRightResource::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::input::motion::button::TouchButtonResource::TouchButtonResourceDyn>>);
        let mut touchButtonBlankResourceImage: Option<Rc<RefCell<Image>>> = jcall!(imageCache).getWithKey(crate::java_into_object(jcall!(touchButtonBlankResource).as_TouchButtonResource_mut().RESOURCE.clone()));
        let mut touchButtonGenericActionResourceImage: Option<Rc<RefCell<Image>>> = jcall!(imageCache).getWithKey(crate::java_into_object(jcall!(touchButtonGenericActionResource).as_TouchButtonResource_mut().RESOURCE.clone()));
        let mut touchButtonStartResourceImage: Option<Rc<RefCell<Image>>> = jcall!(imageCache).getWithKey(crate::java_into_object(jcall!(touchButtonStartResource).as_TouchButtonResource_mut().RESOURCE.clone()));
        let mut touchButtonUpResourceImage: Option<Rc<RefCell<Image>>> = jcall!(imageCache).getWithKey(crate::java_into_object(jcall!(touchButtonUpResource).as_TouchButtonResource_mut().RESOURCE.clone()));
        let mut touchButtonDownResourceImage: Option<Rc<RefCell<Image>>> = jcall!(imageCache).getWithKey(crate::java_into_object(jcall!(touchButtonDownResource).as_TouchButtonResource_mut().RESOURCE.clone()));
        let mut touchButtonTurnLeftResourceImage: Option<Rc<RefCell<Image>>> = jcall!(imageCache).getWithKey(crate::java_into_object(jcall!(touchButtonTurnLeftResource).as_TouchButtonResource_mut().RESOURCE.clone()));
        let mut touchButtonTurnRightResourceImage: Option<Rc<RefCell<Image>>> = jcall!(imageCache).getWithKey(crate::java_into_object(jcall!(touchButtonTurnRightResource).as_TouchButtonResource_mut().RESOURCE.clone()));
        let mut touchButtonStrafeLeftResourceImage: Option<Rc<RefCell<Image>>> = jcall!(imageCache).getWithKey(crate::java_into_object(jcall!(touchButtonStrafeLeftResource).as_TouchButtonResource_mut().RESOURCE.clone()));
        let mut touchButtonStrafeRightResourceImage: Option<Rc<RefCell<Image>>> = jcall!(imageCache).getWithKey(crate::java_into_object(jcall!(touchButtonStrafeRightResource).as_TouchButtonResource_mut().RESOURCE.clone()));
        let mut nullAnimationFactory: Option<Rc<RefCell<NullAnimationFactory>>> = NullAnimationFactory::getFactoryInstance();
        if jcall!(jcall!(OperatingSystemFactory::getInstance()).getOperatingSystemInstance()).isOverScan() {
            jcall!(imageCache).getWithKey(crate::java_into_object(jcall!(TouchButtonStartResource::TouchButtonStartResource::getInstance()).HINT.clone()));
        }
        jcall!(progressCanvas).addPortion(portion.clone(), loadingString.clone(), { let value = index; index += 1; value });
        let mut imageCompleteUtil: Option<Rc<RefCell<dyn crate::org::allbinary::media::image::ImageCompleteUtil::ImageCompleteUtilDyn>>> = ImageCompleteUtil::getInstance();
        jcall!(progressCanvas).addPortion(portion.clone(), loadingString.clone(), { let value = index; index += 1; value });
        jcall!(imageCompleteUtil).waitForImage(touchButtonBlankResourceImage.clone(), jcall!(touchButtonBlankResource).as_TouchButtonResource_mut().RESOURCE.clone());
        { let java_arguments = (jcall!(touchButtonBlankResource).as_TouchButtonResource_mut().RESOURCE.clone(), OneRowSpriteIndexedAnimationFactory::createFactory(touchButtonBlankResourceImage.clone(), AnimationBehaviorFactory::getInstance()),); self.add(java_arguments.0, java_arguments.1) };
        self.add(jcall!(touchButtonBlankResource).as_TouchButtonResource_mut().HINT.clone(), nullAnimationFactory.clone());
        jcall!(progressCanvas).addPortion(portion.clone(), loadingString.clone(), { let value = index; index += 1; value });
        jcall!(imageCompleteUtil).waitForImage(touchButtonGenericActionResourceImage.clone(), jcall!(touchButtonGenericActionResource).as_TouchButtonResource_mut().RESOURCE.clone());
        { let java_arguments = (jcall!(touchButtonGenericActionResource).as_TouchButtonResource_mut().RESOURCE.clone(), OneRowSpriteIndexedAnimationFactory::createFactory(touchButtonGenericActionResourceImage.clone(), AnimationBehaviorFactory::getInstance()),); self.add(java_arguments.0, java_arguments.1) };
        self.add(jcall!(touchButtonGenericActionResource).as_TouchButtonResource_mut().HINT.clone(), nullAnimationFactory.clone());
        jcall!(progressCanvas).addPortion(portion.clone(), loadingString.clone(), { let value = index; index += 1; value });
        jcall!(imageCompleteUtil).waitForImage(touchButtonStartResourceImage.clone(), jcall!(touchButtonStartResource).as_TouchButtonResource_mut().RESOURCE.clone());
        { let java_arguments = (jcall!(touchButtonStartResource).as_TouchButtonResource_mut().RESOURCE.clone(), OneRowSpriteIndexedAnimationFactory::createFactory(touchButtonStartResourceImage.clone(), AnimationBehaviorFactory::getInstance()),); self.add(java_arguments.0, java_arguments.1) };
        jcall!(progressCanvas).addPortion(portion.clone(), loadingString.clone(), { let value = index; index += 1; value });
        jcall!(imageCompleteUtil).waitForImage(touchButtonUpResourceImage.clone(), jcall!(touchButtonUpResource).as_TouchButtonResource_mut().RESOURCE.clone());
        { let java_arguments = (jcall!(touchButtonUpResource).as_TouchButtonResource_mut().RESOURCE.clone(), OneRowSpriteIndexedAnimationFactory::createFactory(touchButtonUpResourceImage.clone(), AnimationBehaviorFactory::getInstance()),); self.add(java_arguments.0, java_arguments.1) };
        self.add(jcall!(touchButtonUpResource).as_TouchButtonResource_mut().HINT.clone(), nullAnimationFactory.clone());
        jcall!(progressCanvas).addPortion(portion.clone(), loadingString.clone(), { let value = index; index += 1; value });
        jcall!(imageCompleteUtil).waitForImage(touchButtonDownResourceImage.clone(), jcall!(touchButtonDownResource).as_TouchButtonResource_mut().RESOURCE.clone());
        { let java_arguments = (jcall!(touchButtonDownResource).as_TouchButtonResource_mut().RESOURCE.clone(), OneRowSpriteIndexedAnimationFactory::createFactory(touchButtonDownResourceImage.clone(), AnimationBehaviorFactory::getInstance()),); self.add(java_arguments.0, java_arguments.1) };
        self.add(jcall!(touchButtonDownResource).as_TouchButtonResource_mut().HINT.clone(), nullAnimationFactory.clone());
        jcall!(progressCanvas).addPortion(portion.clone(), loadingString.clone(), { let value = index; index += 1; value });
        jcall!(imageCompleteUtil).waitForImage(touchButtonTurnLeftResourceImage.clone(), jcall!(touchButtonTurnLeftResource).as_TouchButtonResource_mut().RESOURCE.clone());
        { let java_arguments = (jcall!(touchButtonTurnLeftResource).as_TouchButtonResource_mut().RESOURCE.clone(), OneRowSpriteIndexedAnimationFactory::createFactory(touchButtonTurnLeftResourceImage.clone(), AnimationBehaviorFactory::getInstance()),); self.add(java_arguments.0, java_arguments.1) };
        self.add(jcall!(touchButtonTurnLeftResource).as_TouchButtonResource_mut().HINT.clone(), nullAnimationFactory.clone());
        jcall!(progressCanvas).addPortion(portion.clone(), loadingString.clone(), { let value = index; index += 1; value });
        jcall!(imageCompleteUtil).waitForImage(touchButtonTurnRightResourceImage.clone(), jcall!(touchButtonTurnRightResource).as_TouchButtonResource_mut().RESOURCE.clone());
        { let java_arguments = (jcall!(touchButtonTurnRightResource).as_TouchButtonResource_mut().RESOURCE.clone(), OneRowSpriteIndexedAnimationFactory::createFactory(touchButtonTurnRightResourceImage.clone(), AnimationBehaviorFactory::getInstance()),); self.add(java_arguments.0, java_arguments.1) };
        self.add(jcall!(touchButtonTurnRightResource).as_TouchButtonResource_mut().HINT.clone(), nullAnimationFactory.clone());
        jcall!(progressCanvas).addPortion(portion.clone(), loadingString.clone(), { let value = index; index += 1; value });
        jcall!(imageCompleteUtil).waitForImage(touchButtonStrafeLeftResourceImage.clone(), jcall!(touchButtonStrafeLeftResource).as_TouchButtonResource_mut().RESOURCE.clone());
        { let java_arguments = (jcall!(touchButtonStrafeLeftResource).as_TouchButtonResource_mut().RESOURCE.clone(), OneRowSpriteIndexedAnimationFactory::createFactory(touchButtonStrafeLeftResourceImage.clone(), AnimationBehaviorFactory::getInstance()),); self.add(java_arguments.0, java_arguments.1) };
        self.add(jcall!(touchButtonStrafeLeftResource).as_TouchButtonResource_mut().HINT.clone(), nullAnimationFactory.clone());
        jcall!(progressCanvas).addPortion(portion.clone(), loadingString.clone(), { let value = index; index += 1; value });
        jcall!(imageCompleteUtil).waitForImage(touchButtonStrafeRightResourceImage.clone(), jcall!(touchButtonStrafeRightResource).as_TouchButtonResource_mut().RESOURCE.clone());
        { let java_arguments = (jcall!(touchButtonStrafeRightResource).as_TouchButtonResource_mut().RESOURCE.clone(), OneRowSpriteIndexedAnimationFactory::createFactory(touchButtonStrafeRightResourceImage.clone(), AnimationBehaviorFactory::getInstance()),); self.add(java_arguments.0, java_arguments.1) };
        self.add(jcall!(touchButtonStrafeRightResource).as_TouchButtonResource_mut().HINT.clone(), nullAnimationFactory.clone());
        if jcall!(jcall!(OperatingSystemFactory::getInstance()).getOperatingSystemInstance()).isOverScan() {
            let mut touchButtonStartHintResource: Option<Rc<RefCell<Image>>> = jcall!(imageCache).getWithKey(crate::java_into_object(jcall!(TouchButtonStartResource::TouchButtonStartResource::getInstance()).HINT.clone()));
            { let java_arguments = (jcall!(touchButtonStartResource).as_TouchButtonResource_mut().HINT.clone(), OneRowSpriteIndexedAnimationFactory::createFactory(touchButtonStartHintResource.clone(), AnimationBehaviorFactory::getInstance()),); self.add(java_arguments.0, java_arguments.1) };
        } else {
            self.add(jcall!(touchButtonStartResource).as_TouchButtonResource_mut().HINT.clone(), nullAnimationFactory.clone());
        }
        self.base.init(level.clone());
    }
    

    pub fn isLoadingLevel(&mut self, mut level: i32) -> bool {
        let mut resourceLoadingLevelFactory: Option<Rc<RefCell<ResourceLoadingLevelFactory>>> = ResourceLoadingLevelFactory::getInstance();
        if level == jcall!(jcall!(resourceLoadingLevelFactory).LOAD_TOUCH).getLevel() {
            return true;
        } else {
            return self.base.isLoadingLevel(level.clone());
        }
    }
    

    pub fn isFeature(&mut self) -> bool {
        if !jcall!(Features::getInstance()).isDefault(jcall!(OpenGLFeatureFactory::getInstance()).OPENGL.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
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


