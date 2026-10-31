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
use crate::org::allbinary::AndroidUtil::AndroidUtil;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactory;
use crate::org::allbinary::animation::ProceduralAnimationInterfaceFactoryInterface::ProceduralAnimationInterfaceFactoryInterface;
use crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactory;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorUtil::BasicColorUtil;
use crate::org::allbinary::image::sprite::AnimationFactorySpriteScaleUtil::AnimationFactorySpriteScaleUtil;
use crate::org::allbinary::logic::math::PrimitiveIntUtil::PrimitiveIntUtil;
use super::SpriteIndexedAnimation::SpriteIndexedAnimation;
use super::AdjustedSpriteIndexedAnimation::AdjustedSpriteIndexedAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SpriteIndexedAnimationFactory {
    pub base: BaseImageAnimationFactory,
    pub animationFactorySpriteScaleUtil: Option<Rc<RefCell<AnimationFactorySpriteScaleUtil>>>,
    pub basicColorArray: Vec<Option<Rc<RefCell<BasicColor>>>>,
}

impl ProceduralAnimationInterfaceFactoryInterface for SpriteIndexedAnimationFactory {
    fn getInstanceAnimation(&mut self, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstanceAnimation(self, animationInterface) }
}
impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for SpriteIndexedAnimationFactory {
}

impl SpriteIndexedAnimationFactory {
    

    // throws Exception
    pub fn createCWHDXY(mut image: Option<Rc<RefCell<Image>>>, mut basicColorArray: Vec<Option<Rc<RefCell<BasicColor>>>>, mut width: i32, mut height: i32, mut dx: i32, mut dy: i32, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Option<Rc<RefCell<SpriteIndexedAnimationFactory>>> {
        return SpriteIndexedAnimationFactory::new(image.clone(), basicColorArray.clone(), PrimitiveIntUtil::getArrayInstance(), width.clone(), height.clone(), dx.clone(), dy.clone(), animationBehaviorFactory.clone());
    }
    

    // throws Exception
    pub fn createCWH(mut image: Option<Rc<RefCell<Image>>>, mut basicColorArray: Vec<Option<Rc<RefCell<BasicColor>>>>, mut width: i32, mut height: i32, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Option<Rc<RefCell<SpriteIndexedAnimationFactory>>> {
        let mut spriteIndexedAnimationFactory: Option<Rc<RefCell<SpriteIndexedAnimationFactory>>> = SpriteIndexedAnimationFactory::new(image.clone(), basicColorArray.clone(), PrimitiveIntUtil::getArrayInstance(), width.clone(), height.clone(), 0, 0, animationBehaviorFactory.clone());
        jcall!(spriteIndexedAnimationFactory).initHWH();
        return spriteIndexedAnimationFactory.clone();
    }
    

    // throws Exception
    pub fn createWH(mut image: Option<Rc<RefCell<Image>>>, mut width: i32, mut height: i32, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Option<Rc<RefCell<SpriteIndexedAnimationFactory>>> {
        return SpriteIndexedAnimationFactory::new(image.clone(), jcall!(BasicColorUtil::getInstance()).ZERO_ARRAY.clone(), PrimitiveIntUtil::getArrayInstance(), width.clone(), height.clone(), 0, 0, animationBehaviorFactory.clone());
    }
    

    pub fn new_struct(mut image: Option<Rc<RefCell<Image>>>, mut basicColorArray: Vec<Option<Rc<RefCell<BasicColor>>>>, mut sequenceArray: Vec<i32>, mut width: i32, mut height: i32, mut dx: i32, mut dy: i32, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Self {
        let mut this = Self {
            base: BaseImageAnimationFactory::new_struct(image.clone(), sequenceArray.clone(), width.clone(), height.clone(), dx.clone(), dy.clone(), animationBehaviorFactory.clone()),
            animationFactorySpriteScaleUtil: None,
            basicColorArray: Vec::new(),
        };
        this.animationFactorySpriteScaleUtil = AnimationFactorySpriteScaleUtil::getInstance();
        this.basicColorArray = jcall!(BasicColorUtil::getInstance()).ZERO_ARRAY.clone();
        // super(...) is in the struct literal above
        this.basicColorArray = basicColorArray.clone();
        this
    }

    pub fn new(mut image: Option<Rc<RefCell<Image>>>, mut basicColorArray: Vec<Option<Rc<RefCell<BasicColor>>>>, mut sequenceArray: Vec<i32>, mut width: i32, mut height: i32, mut dx: i32, mut dy: i32, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(image, basicColorArray, sequenceArray, width, height, dx, dy, animationBehaviorFactory))))
    }
    

    pub fn initHWH(&mut self) {
        jcall!(self.animationFactoryInitializationVisitor).dx = -(jcall!(self.animationFactoryInitializationVisitor).width >> 2);
        jcall!(self.animationFactoryInitializationVisitor).dy = -(jcall!(self.animationFactoryInitializationVisitor).height >> 2);
        if AndroidUtil::isAndroid() {
            jcall!(self.animationFactoryInitializationVisitor).dx += 3;
        }
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        let mut sprite: Option<Rc<RefCell<Sprite>>> = { let java_receiver = self.animationFactorySpriteScaleUtil.clone(); let java_arguments = (self.getImage(), jcall!(self.animationFactoryInitializationVisitor).width.clone(), jcall!(self.animationFactoryInitializationVisitor).height.clone(), jcall!(self.scaleProperties).scaleWidth.clone(), jcall!(self.scaleProperties).scaleHeight.clone(),); let java_result = jcall!(java_receiver).createImage(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4); java_result };
        if jcall!(self.animationFactoryInitializationVisitor).dx != 0 || jcall!(self.animationFactoryInitializationVisitor).dy != 0 {
            return AdjustedSpriteIndexedAnimation::new(sprite.clone(), self.getImage(), self.basicColorArray.clone(), jcall!(self.animationFactoryInitializationVisitor).dx.clone(), jcall!(self.animationFactoryInitializationVisitor).dy.clone(), jcall!(self.animationBehaviorFactory).getOrCreateInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
        } else {
            return SpriteIndexedAnimation::new(sprite.clone(), self.getImage(), self.basicColorArray.clone(), jcall!(self.animationBehaviorFactory).getOrCreateInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
        }
    }
    

    // throws Exception
    pub fn getInstanceAnimation(&mut self, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return self.getInstance(0);
    }
}
impl std::ops::Deref for SpriteIndexedAnimationFactory {
    type Target = BaseImageAnimationFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SpriteIndexedAnimationFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactoryDyn for SpriteIndexedAnimationFactory {
    fn as_BaseImageAnimationFactory(&self) -> &crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactory { &self.base }
    fn as_BaseImageAnimationFactory_mut(&mut self) -> &mut crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactory { &mut self.base }
    fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstance(self, instanceId) }
}


