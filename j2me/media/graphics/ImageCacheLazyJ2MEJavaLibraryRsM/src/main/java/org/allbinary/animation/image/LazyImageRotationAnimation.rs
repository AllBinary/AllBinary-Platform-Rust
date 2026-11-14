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
// package org.allbinary.animation.image
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::khronos::opengles::GL::GL;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior;
use crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation;
use crate::org::allbinary::animation::NullRotationAnimationFactory::NullRotationAnimationFactory;
use crate::org::allbinary::animation::RotationAnimation::RotationAnimation;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::image::ImageCache::ImageCache;
use crate::org::allbinary::image::ImageCacheFactory::ImageCacheFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::math::AngleFactory::AngleFactory;
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::media::ScaleProperties::ScaleProperties;
use crate::org::allbinary::util::CircularIndexUtil::CircularIndexUtil;
use crate::java::lang::Exception::*;
use super::BaseImageAnimationFactory::BaseImageAnimationFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LazyImageRotationAnimation {
    pub base: RotationAnimation,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub layoutIndex: i32,
    pub instanceId: i32,
    pub animationInterfaceFactoryInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactoryDyn>>>,
    pub NULL_INDEX_ANIMATION: Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>>,
    pub animation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>>,
    pub scaleProperties: Option<Rc<RefCell<ScaleProperties>>>,
}

thread_local! { pub static LazyImageRotationAnimation_SET_REAL_ANIMATION: String = String::from("setRealAnimation"); }
impl crate::org::allbinary::animation::AnimationInterface::AnimationInterface for LazyImageRotationAnimation {
    fn nextFrame(&mut self) { Self::nextFrame(self) }
    fn paintThreedXYZ(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32, mut z: i32) { Self::paintThreedXYZ(self, graphics, x, y, z) }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for LazyImageRotationAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}
impl crate::org::allbinary::direction::DirectionalInterface::DirectionalInterface for LazyImageRotationAnimation {
    fn setFrameByDirection(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::direction::Direction::Direction>>>) { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::setFrameByDirection(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self), p1) }
    fn setFrameToAngle(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::setFrameToAngle(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self), p1) }
    fn adjustFrameToAngle(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::adjustFrameToAngle(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self), p1) }
    fn adjustFrame(&mut self, p1: i16) { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::adjustFrame(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self), p1) }
}

impl LazyImageRotationAnimation {
    

    pub fn new_struct(mut layoutIndex: i32, mut instanceId: i32, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>, mut animationInterfaceFactoryInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactoryDyn>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: RotationAnimation::new_struct(AngleInfo::getInstance({ let java_field = jcall!(AngleFactory::getInstance()).QUARTER_TOTAL_ANGLE.clone(); java_field }.clone()), CircularIndexUtil::createInstance(4), animationBehavior.clone()),
            logUtil: None,
            layoutIndex: 0,
            instanceId: 0,
            animationInterfaceFactoryInterface: None,
            NULL_INDEX_ANIMATION: None,
            animation: None,
            scaleProperties: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.scaleProperties = crate::org::allbinary::media::ScaleProperties::ScaleProperties_instance.with(Clone::clone).clone();
        // super(...) is in the struct literal above
        this.layoutIndex = layoutIndex.clone();
        this.instanceId = instanceId.clone();
        this.animationInterfaceFactoryInterface = animationInterfaceFactoryInterface.clone();
        let mut imageCache: Option<Rc<RefCell<dyn crate::org::allbinary::image::ImageCache::ImageCacheDyn>>> = ImageCacheFactory::getInstance();
        this
    }

    pub fn new(mut layoutIndex: i32, mut instanceId: i32, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>, mut animationInterfaceFactoryInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactoryDyn>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(layoutIndex.clone(), instanceId.clone(), scaleProperties.clone(), animationInterfaceFactoryInterface.clone(), animationBehavior.clone()))));
            let mut imageCache: Option<Rc<RefCell<dyn crate::org::allbinary::image::ImageCache::ImageCacheDyn>>> = ImageCacheFactory::getInstance();
            jcall!(imageCache).add(this.clone());
            jcall!(this).scaleProperties = scaleProperties.clone();
            jcall!(this).NULL_INDEX_ANIMATION = { let java_value = ((|| -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>> { panic!("ClassCastException: a Rust dyn Animation cannot be cast to IndexedAnimation") })()).clone(); java_value };
            /* Java local class: move it out of the function
pub struct ActualRotationAnimation {
    pub base: RotationAnimation,
    pub lazyImageRotationAnimation: Option<Rc<RefCell<LazyImageRotationAnimation>>>,
    pub index: i32,
}

impl crate::org::allbinary::animation::AnimationInterface::AnimationInterface for ActualRotationAnimation {
    fn nextFrame(&mut self) { crate::org::allbinary::animation::Animation::Animation::nextFrame(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self)) }
    fn paintThreedXYZ(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32, mut z: i32) { Self::paintThreedXYZ(self, graphics, x, y, z) }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for ActualRotationAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}
impl crate::org::allbinary::direction::DirectionalInterface::DirectionalInterface for ActualRotationAnimation {
    fn setFrameByDirection(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::direction::Direction::Direction>>>) { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::setFrameByDirection(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self), p1) }
    fn setFrameToAngle(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::setFrameToAngle(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self), p1) }
    fn adjustFrameToAngle(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::adjustFrameToAngle(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self), p1) }
    fn adjustFrame(&mut self, p1: i16) { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::adjustFrame(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self), p1) }
}

impl ActualRotationAnimation {
            

            pub fn new_struct(mut lazyImageRotationAnimation: Option<Rc<RefCell<LazyImageRotationAnimation>>>) -> Self {
                let mut this = Self {
            base: RotationAnimation::new_struct(AngleInfo::getInstance({ let java_field = jcall!(AngleFactory::getInstance()).QUARTER_TOTAL_ANGLE.clone(); java_field }.clone()), CircularIndexUtil::createInstance(4), jcall!(this).base.animationBehavior.clone()),
            lazyImageRotationAnimation: None,
            index: 0,
                };
                // super(...) is in the struct literal above
                jcall!(this).lazyImageRotationAnimation = lazyImageRotationAnimation.clone();
                this
            }

            pub fn new(mut lazyImageRotationAnimation: Option<Rc<RefCell<LazyImageRotationAnimation>>>) -> Option<Rc<RefCell<Self>>> {
                Some(Rc::new(RefCell::new(Self::new_struct(lazyImageRotationAnimation))))
            }
            

            pub fn setFrame(&mut self, mut index: i32) {
                self.index = index.clone();
            }
            

            pub fn getFrame(&mut self) -> i32 {
                return self.index.clone();
            }
            

            pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32) {
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    jcall!(ImageCacheFactory::getInstance()).insertFirst(self);
                    jcall!(self.lazyImageRotationAnimation).animation = { let java_field = jcall!(self.lazyImageRotationAnimation).NULL_INDEX_ANIMATION.clone(); java_field }.clone();
                })) {
                    Ok(_) => {}
                    Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

                    let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
                    jcall!(logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).PROCESS.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
                    }
                }
            }
            

            pub fn paintThreedXYZ(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32, mut z: i32) {
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    jcall!(ImageCacheFactory::getInstance()).insertFirst(self);
                    jcall!(self.lazyImageRotationAnimation).animation = { let java_field = jcall!(self.lazyImageRotationAnimation).NULL_INDEX_ANIMATION.clone(); java_field }.clone();
                })) {
                    Ok(_) => {}
                    Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

                    let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
                    jcall!(logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).PROCESS.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
                    }
                }
            }
}
impl std::ops::Deref for ActualRotationAnimation {
    type Target = RotationAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ActualRotationAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for ActualRotationAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
    fn paintThreedXYZ(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32, mut z: i32) { Self::paintThreedXYZ(self, graphics, x, y, z) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for ActualRotationAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base.base }
    fn setFrame(&mut self, mut index: i32) { Self::setFrame(self, index) }
    fn getFrame(&mut self) -> i32 { Self::getFrame(self) }
    fn getSize(&mut self) -> i32 { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::getSize(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self)) }
}
impl crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn for ActualRotationAnimation {
    fn as_RotationAnimation(&self) -> &crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &self.base }
    fn as_RotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &mut self.base }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for ActualRotationAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}

*/
            jcall!(this).animation = ActualRotationAnimation::new(this.clone());
        this
    }
    

    pub fn setRealAnimation(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut animation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>> = self.animation.clone();
            jcall!(self.animationInterfaceFactoryInterface).setInitialScale(self.scaleProperties.clone());
            self.animation = { let java_value = ((|| -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>> { panic!("ClassCastException: a Rust dyn Animation cannot be cast to IndexedAnimation") })()).clone(); java_value };
            jcall!(self.animation).setState(animation.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), LazyImageRotationAnimation_SET_REAL_ANIMATION.with(Clone::clone).clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn setScale(&mut self, mut scaleX: f32, mut scaleY: f32) {
        jcall!(self.animation).setScale(scaleX.clone(), scaleY.clone());
    }
    

    pub fn getAnimationBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>> {
        return jcall!(self.animation).getAnimationBehavior();
    }
    

    // throws Exception
    pub fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) {
        jcall!(self.animation).as_IndexedAnimation_mut().set(gl.clone());
    }
    

    pub fn setAlpha(&mut self, mut alpha: i32) {
        jcall!(self.animation).setAlpha(alpha.clone());
    }
    

    pub fn setDx(&mut self, mut dx: i32) {
        jcall!(self.animation).setDx(dx.clone());
    }
    

    pub fn setDy(&mut self, mut dy: i32) {
        jcall!(self.animation).setDy(dy.clone());
    }
    

    pub fn setMaxScale(&mut self, mut maxScaleX: f32, mut maxScaleY: f32) {
        jcall!(self.animation).setMaxScale(maxScaleX.clone(), maxScaleY.clone());
    }
    

    pub fn nextRotation(&mut self) {
        let mut rotationAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>> = ((|| -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>> { panic!("ClassCastException: a Rust dyn IndexedAnimation cannot be cast to RotationAnimation") })());
        jcall!(rotationAnimation).nextRotation();
    }
    

    pub fn previousRotation(&mut self) {
        let mut rotationAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>> = ((|| -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>> { panic!("ClassCastException: a Rust dyn IndexedAnimation cannot be cast to RotationAnimation") })());
        jcall!(rotationAnimation).previousRotation();
    }
    

    pub fn nextRotationX(&mut self) {
        let mut rotationAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>> = ((|| -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>> { panic!("ClassCastException: a Rust dyn IndexedAnimation cannot be cast to RotationAnimation") })());
        jcall!(rotationAnimation).nextRotationX();
    }
    

    pub fn previousRotationX(&mut self) {
        let mut rotationAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>> = ((|| -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>> { panic!("ClassCastException: a Rust dyn IndexedAnimation cannot be cast to RotationAnimation") })());
        jcall!(rotationAnimation).previousRotationX();
    }
    

    pub fn nextRotationZ(&mut self) {
        let mut rotationAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>> = ((|| -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>> { panic!("ClassCastException: a Rust dyn IndexedAnimation cannot be cast to RotationAnimation") })());
        jcall!(rotationAnimation).nextRotationZ();
    }
    

    pub fn previousRotationZ(&mut self) {
        let mut rotationAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>> = ((|| -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>> { panic!("ClassCastException: a Rust dyn IndexedAnimation cannot be cast to RotationAnimation") })());
        jcall!(rotationAnimation).previousRotationZ();
    }
    

    pub fn changeBasicColor(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        jcall!(self.animation).changeBasicColor(basicColor.clone());
    }
    

    pub fn getBasicColorP(&mut self) -> Option<Rc<RefCell<BasicColor>>> {
        return jcall!(self.animation).getBasicColorP();
    }
    

    pub fn getChangeBasicColor(&mut self) -> Option<Rc<RefCell<BasicColor>>> {
        return jcall!(self.animation).getChangeBasicColor();
    }
    

    pub fn getChangeColor(&mut self) -> i32 {
        return jcall!(self.animation).getChangeColor();
    }
    

    pub fn getColor(&mut self) -> i32 {
        return jcall!(self.animation).getColor();
    }
    

    pub fn getDx(&mut self) -> i32 {
        return jcall!(self.animation).getDx();
    }
    

    pub fn getDy(&mut self) -> i32 {
        return jcall!(self.animation).getDy();
    }
    

    pub fn isThreed(&mut self) -> bool {
        return jcall!(self.animation).isThreed();
    }
    

    // throws Exception
    pub fn nextFrame(&mut self) {
        jcall!(self.animation).nextFrame();
    }
    

    pub fn reset(&mut self) {
        jcall!(self.animation).reset();
    }
    

    pub fn setFrame(&mut self, mut index: i32) {
        jcall!(self.animation).setFrame(index.clone());
    }
    

    pub fn getFrame(&mut self) -> i32 {
        return jcall!(self.animation).getFrame();
    }
    

    // throws Exception
    pub fn getAnimationSize(&mut self) -> i32 {
        return jcall!(self.animation).getAnimationSize();
    }
    

    pub fn getSize(&mut self) -> i32 {
        return jcall!(self.animation).getSize();
    }
    

    pub fn previousFrame(&mut self) {
        jcall!(self.animation).previousFrame();
    }
    

    pub fn isLastFrame(&mut self) -> bool {
        return jcall!(self.animation).isLastFrame();
    }
    

    pub fn setSequence(&mut self, mut sequence: Vec<i32>) {
        jcall!(self.animation).setSequence(sequence.clone());
    }
    

    pub fn getSequence(&mut self) -> Vec<i32> {
        return jcall!(self.animation).getSequence();
    }
    

    pub fn getWidth(&mut self) -> i32 {
        return jcall!(self.animation).getWidth();
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.animation).paintXY(graphics.clone(), x.clone(), y.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).PROCESS.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn paintThreedXYZ(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32, mut z: i32) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.animation).paintThreedXYZ(graphics.clone(), x.clone(), y.clone(), z.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).PROCESS.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn toString(&mut self) -> String {
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        let mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = jcall!(self.animationInterfaceFactoryInterface).getImage();
        return jcall!(StringMaker::new()).append(self.base.toString()).append({ let java_field = jcall!(commonSeps).SPACE.clone(); java_field }.clone()).append(jcall!(image).getName()).append({ let java_field = jcall!(commonSeps).SPACE.clone(); java_field }.clone()).appendint(jcall!(image).getWidth()).append({ let java_field = jcall!(commonSeps).SPACE.clone(); java_field }.clone()).appendint(jcall!(image).getHeight()).toString();
    }
}
impl std::ops::Deref for LazyImageRotationAnimation {
    type Target = RotationAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for LazyImageRotationAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for LazyImageRotationAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base }
    fn setAlpha(&mut self, mut alpha: i32) { Self::setAlpha(self, alpha) }
    fn nextFrame(&mut self) { Self::nextFrame(self) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
    fn paintThreedXYZ(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32, mut z: i32) { Self::paintThreedXYZ(self, graphics, x, y, z) }
    fn isThreed(&mut self) -> bool { Self::isThreed(self) }
    fn getBasicColorP(&mut self) -> Option<Rc<RefCell<BasicColor>>> { Self::getBasicColorP(self) }
    fn getChangeBasicColor(&mut self) -> Option<Rc<RefCell<BasicColor>>> { Self::getChangeBasicColor(self) }
    fn changeBasicColor(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { Self::changeBasicColor(self, basicColor) }
    fn getChangeColor(&mut self) -> i32 { Self::getChangeColor(self) }
    fn getColor(&mut self) -> i32 { Self::getColor(self) }
    fn setScale(&mut self, mut scaleX: f32, mut scaleY: f32) { Self::setScale(self, scaleX, scaleY) }
    fn setMaxScale(&mut self, mut maxScaleX: f32, mut maxScaleY: f32) { Self::setMaxScale(self, maxScaleX, maxScaleY) }
    fn setDx(&mut self, mut dx: i32) { Self::setDx(self, dx) }
    fn getDx(&mut self) -> i32 { Self::getDx(self) }
    fn setDy(&mut self, mut dy: i32) { Self::setDy(self, dy) }
    fn getDy(&mut self) -> i32 { Self::getDy(self) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for LazyImageRotationAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base.base }
    fn getAnimationBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>> { Self::getAnimationBehavior(self) }
    fn reset(&mut self) { Self::reset(self) }
    fn setFrame(&mut self, mut index: i32) { Self::setFrame(self, index) }
    fn getFrame(&mut self) -> i32 { Self::getFrame(self) }
    fn getAnimationSize(&mut self) -> i32 { Self::getAnimationSize(self) }
    fn getSize(&mut self) -> i32 { Self::getSize(self) }
    fn previousFrame(&mut self) { Self::previousFrame(self) }
    fn isLastFrame(&mut self) -> bool { Self::isLastFrame(self) }
    fn setSequence(&mut self, mut sequence: Vec<i32>) { Self::setSequence(self, sequence) }
    fn getSequence(&mut self) -> Vec<i32> { Self::getSequence(self) }
    fn getWidth(&mut self) -> i32 { Self::getWidth(self) }
}
impl crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn for LazyImageRotationAnimation {
    fn as_RotationAnimation(&self) -> &crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &self.base }
    fn as_RotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &mut self.base }
    fn nextRotationX(&mut self) { Self::nextRotationX(self) }
    fn previousRotationX(&mut self) { Self::previousRotationX(self) }
    fn nextRotation(&mut self) { Self::nextRotation(self) }
    fn previousRotation(&mut self) { Self::previousRotation(self) }
    fn nextRotationZ(&mut self) { Self::nextRotationZ(self) }
    fn previousRotationZ(&mut self) { Self::previousRotationZ(self) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for LazyImageRotationAnimation {
    fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) { Self::set(self, gl) }
}


