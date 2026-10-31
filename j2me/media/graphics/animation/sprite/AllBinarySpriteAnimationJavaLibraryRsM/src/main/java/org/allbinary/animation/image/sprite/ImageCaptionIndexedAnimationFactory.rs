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
// package org.allbinary.animation.image.sprite
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Image::Image;
// use crate::javax::microedition::lcdui::game::Sprite::Sprite;
// use crate::javax::microedition::media::Player::Player;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactory;
use crate::org::allbinary::animation::AnimationInterfaceFactoryInterface::AnimationInterfaceFactoryInterface;
use crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation;
use crate::org::allbinary::animation::caption::CaptionIndexedAnimation::CaptionIndexedAnimation;
use crate::org::allbinary::animation::image::ImageAnimation::ImageAnimation;
use crate::org::allbinary::graphics::color::BasicColorUtil::BasicColorUtil;
use crate::org::allbinary::image::AnimationFactoryImageScaleUtil::AnimationFactoryImageScaleUtil;
use crate::org::allbinary::image::sprite::AnimationFactorySpriteScaleUtil::AnimationFactorySpriteScaleUtil;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::media::ScaleProperties::ScaleProperties;
use crate::org::allbinary::media::audio::Sound::Sound;
use super::SpriteIndexedAnimation::SpriteIndexedAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageCaptionIndexedAnimationFactory {
    pub animationFactoryImageScaleUtil: Option<Rc<RefCell<AnimationFactoryImageScaleUtil>>>,
    pub animationFactorySpriteScaleUtil: Option<Rc<RefCell<AnimationFactorySpriteScaleUtil>>>,
    pub captionImage: Option<Rc<RefCell<Image>>>,
    pub spriteMovieImage: Option<Rc<RefCell<Image>>>,
    pub frameWidth: i32,
    pub frameHeight: i32,
    pub captionDx: i32,
    pub captionDy: i32,
    pub dx: i32,
    pub dy: i32,
    pub time: i32,
    pub soundInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>,
    pub scaleProperties: Option<Rc<RefCell<ScaleProperties>>>,
    pub animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>,
}

impl AnimationInterfaceFactoryInterface for ImageCaptionIndexedAnimationFactory {
    fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstance(self, instanceId) }
    fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) { Self::setInitialScale(self, scaleProperties) }
}
impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for ImageCaptionIndexedAnimationFactory {
}

impl ImageCaptionIndexedAnimationFactory {
    

    pub fn new_struct(mut captionImage: Option<Rc<RefCell<Image>>>, mut spriteMovieImage: Option<Rc<RefCell<Image>>>, mut soundInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>, mut frameWidth: i32, mut frameHeight: i32, mut captionDx: i32, mut captionDy: i32, mut dx: i32, mut dy: i32, mut time: i32, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Self {
        let mut this = Self {
            animationFactoryImageScaleUtil: None,
            animationFactorySpriteScaleUtil: None,
            captionImage: None,
            spriteMovieImage: None,
            frameWidth: 0,
            frameHeight: 0,
            captionDx: 0,
            captionDy: 0,
            dx: 0,
            dy: 0,
            time: 0,
            soundInterface: None,
            scaleProperties: None,
            animationBehaviorFactory: None,
        };
        this.animationFactoryImageScaleUtil = AnimationFactoryImageScaleUtil::getInstance();
        this.animationFactorySpriteScaleUtil = AnimationFactorySpriteScaleUtil::getInstance();
        this.scaleProperties = crate::org::allbinary::media::ScaleProperties::ScaleProperties_instance.with(Clone::clone).clone();
        if match (&animationBehaviorFactory, &AnimationBehaviorFactory::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            ForcedLogUtil::log(String::from("Using default AnimationBehaviorFactory with IndexedAnimationFactory"), crate::java_into_object(String::from(std::any::type_name::<Self>())));
        }
        this.captionImage = captionImage.clone();
        this.spriteMovieImage = spriteMovieImage.clone();
        this.frameWidth = frameWidth.clone();
        this.frameHeight = frameHeight.clone();
        this.captionDx = captionDx.clone();
        this.captionDy = captionDy.clone();
        this.dx = dx.clone();
        this.dy = dy.clone();
        this.time = time.clone();
        this.soundInterface = soundInterface.clone();
        this.animationBehaviorFactory = animationBehaviorFactory.clone();
        this
    }

    pub fn new(mut captionImage: Option<Rc<RefCell<Image>>>, mut spriteMovieImage: Option<Rc<RefCell<Image>>>, mut soundInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>, mut frameWidth: i32, mut frameHeight: i32, mut captionDx: i32, mut captionDy: i32, mut dx: i32, mut dy: i32, mut time: i32, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(captionImage, spriteMovieImage, soundInterface, frameWidth, frameHeight, captionDx, captionDy, dx, dy, time, animationBehaviorFactory))))
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        let mut scaledImage: Option<Rc<RefCell<Image>>> = { let java_receiver = self.animationFactoryImageScaleUtil.clone(); let java_arguments = (self.captionImage.clone(), jcall!(self.captionImage).getWidth(), jcall!(self.captionImage).getHeight(), jcall!(self.scaleProperties).scaleWidth.clone(), jcall!(self.scaleProperties).scaleHeight.clone(),); let java_result = jcall!(java_receiver).createImage(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4); java_result };
        let mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> = ImageAnimation::new(scaledImage.clone(), jcall!(self.animationBehaviorFactory).getOrCreateInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
        let mut sprite: Option<Rc<RefCell<Sprite>>> = jcall!(self.animationFactorySpriteScaleUtil).createImage(self.spriteMovieImage.clone(), self.frameWidth.clone(), self.frameHeight.clone(), jcall!(self.scaleProperties).scaleWidth.clone(), jcall!(self.scaleProperties).scaleHeight.clone());
        let mut movieIndexedAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>> = SpriteIndexedAnimation::new(sprite.clone(), self.spriteMovieImage.clone(), jcall!(BasicColorUtil::getInstance()).ZERO_ARRAY.clone(), jcall!(self.animationBehaviorFactory).getOrCreateInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>);
        let mut player: Option<Rc<RefCell<Player>>> = jcall!(self.soundInterface).getPlayerP();
        if player.is_none() {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Sound Was not Initialized"))));
        }
        return CaptionIndexedAnimation::new(animationInterface.clone(), movieIndexedAnimationInterface.clone(), player.clone(), self.captionDx.clone(), self.captionDy.clone(), self.dx.clone(), self.dy.clone(), self.time.clone(), jcall!(self.animationBehaviorFactory).getOrCreateInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
    }
    

    pub fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) {
        self.scaleProperties = scaleProperties.clone();
    }
}

