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
use crate::org::allbinary::logic::util::cache::BasicArrayListResetablePool::BasicArrayListResetablePool;
use crate::org::allbinary::logic::util::cache::CacheableInterface::CacheableInterface;
use crate::org::allbinary::logic::util::cache::CacheableInterfaceFactoryInterface::CacheableInterfaceFactoryInterface;
use crate::org::allbinary::logic::util::cache::PoolInterface::PoolInterface;
use super::ImageArrayRotationAnimationCacheableInterfaceFactory::ImageArrayRotationAnimationCacheableInterfaceFactory;
use super::ImageArrayRotationAnimationInfo::ImageArrayRotationAnimationInfo;
use super::ImageArrayRotationAnimationCacheable::ImageArrayRotationAnimationCacheable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageArrayRotationAnimationPool {
    pub base: BasicArrayListResetablePool,
}

pub static mut ImageArrayRotationAnimationPool_IMAGE_ROTATION_ANIMATION_POOL: std::cell::LazyCell<Option<Rc<RefCell<dyn PoolInterface>>>> = std::cell::LazyCell::new(|| ImageArrayRotationAnimationPool::new(ImageArrayRotationAnimationCacheableInterfaceFactory::new()).map(|reference| reference as Rc<RefCell<dyn PoolInterface>>));
impl PoolInterface for ImageArrayRotationAnimationPool {
    fn remove(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<dyn CacheableInterface>>> { Self::remove(self, key) }
}

impl ImageArrayRotationAnimationPool {
    

    pub fn new_struct(mut cacheableInterfaceFactoryInterface: Option<Rc<RefCell<dyn CacheableInterfaceFactoryInterface>>>) -> Self {
        let mut this = Self {
            base: BasicArrayListResetablePool::new_struct(cacheableInterfaceFactoryInterface.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut cacheableInterfaceFactoryInterface: Option<Rc<RefCell<dyn CacheableInterfaceFactoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(cacheableInterfaceFactoryInterface))))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn PoolInterface>>> {
        return unsafe { ImageArrayRotationAnimationPool_IMAGE_ROTATION_ANIMATION_POOL.clone() }.clone();
    }
    

    // throws Exception
    pub fn remove(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<dyn CacheableInterface>>> {
        let mut cacheableInterface: Option<Rc<RefCell<dyn CacheableInterface>>> = self.base.remove(key.clone());
        let mut allBinaryImageRotationAnimationInfo: Option<Rc<RefCell<ImageArrayRotationAnimationInfo>>> = (key).clone() /* as ImageArrayRotationAnimationInfo */;
        let mut imageArrayRotationAnimationCacheable: Option<Rc<RefCell<ImageArrayRotationAnimationCacheable>>> = ((cacheableInterface).clone() /* as ImageArrayRotationAnimationCacheable */);
        jcall!(imageArrayRotationAnimationCacheable).setImageArray(jcall!(allBinaryImageRotationAnimationInfo).getImageArray());
        return cacheableInterface.clone();
    }
}
impl std::ops::Deref for ImageArrayRotationAnimationPool {
    type Target = BasicArrayListResetablePool;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImageArrayRotationAnimationPool {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::util::cache::AbstractArrayListPool::AbstractArrayListPoolDyn for ImageArrayRotationAnimationPool {
    fn as_AbstractArrayListPool(&self) -> &crate::org::allbinary::logic::util::cache::AbstractArrayListPool::AbstractArrayListPool { &self.base.base }
    fn as_AbstractArrayListPool_mut(&mut self) -> &mut crate::org::allbinary::logic::util::cache::AbstractArrayListPool::AbstractArrayListPool { &mut self.base.base }
    fn clear(&mut self) { crate::org::allbinary::logic::util::cache::BasicArrayListResetablePool::BasicArrayListResetablePool::clear(crate::org::allbinary::logic::util::cache::BasicArrayListResetablePool::BasicArrayListResetablePoolDyn::as_BasicArrayListResetablePool_mut(self)) }
    fn remove(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<dyn CacheableInterface>>> { Self::remove(self, key) }
}
impl crate::org::allbinary::logic::util::cache::BasicArrayListResetablePool::BasicArrayListResetablePoolDyn for ImageArrayRotationAnimationPool {
    fn as_BasicArrayListResetablePool(&self) -> &crate::org::allbinary::logic::util::cache::BasicArrayListResetablePool::BasicArrayListResetablePool { &self.base }
    fn as_BasicArrayListResetablePool_mut(&mut self) -> &mut crate::org::allbinary::logic::util::cache::BasicArrayListResetablePool::BasicArrayListResetablePool { &mut self.base }
}


