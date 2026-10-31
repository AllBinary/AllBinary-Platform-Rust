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
// package org.allbinary.image
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::InputStream::InputStream;
// use crate::javax::microedition::lcdui::Image::Image;
// use crate::javax::microedition::lcdui::NullImage::NullImage;
use crate::org::allbinary::J2MEUtil::J2MEUtil;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::data::resource::ResourceUtil::ResourceUtil;
use crate::org::allbinary::game::gd::resource::GDResources::GDResources;
use crate::org::allbinary::logic::ABSystemWrapper::ABSystemWrapper;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::system::Memory::Memory;
use crate::java::lang::Thread::*;
use super::ImageCacheBase::ImageCacheBase;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageCache {
    pub base: ImageCacheBase,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub tsUtil: Option<Rc<RefCell<TsUtil>>>,
}

thread_local! { pub static ImageCache_NULL_IMAGE_CACHE: Option<Rc<RefCell<dyn ImageCacheDyn>>> = ImageCache::new().map(|reference| reference as Rc<RefCell<dyn ImageCacheDyn>>); }

impl ImageCache {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: ImageCacheBase::new_struct(),
            commonStrings: None,
            tsUtil: None,
        };
        this.commonStrings = CommonStrings::getInstance();
        this.tsUtil = TsUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn addListener(&mut self, mut renderer: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
    

    pub fn getIndex(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> i32 {
        let mut gdResources: Option<Rc<RefCell<GDResources>>> = GDResources::getInstance();
        let mut resourceStringArray: Vec<String> = jcall!(gdResources).resourceStringArray.clone();
        let mut size: i32 = (resourceStringArray.len() as i32).clone();
        for index in (0..size) {
            if match (&resourceStringArray[(index) as usize], &key) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                return index.clone();
            }
        }
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("unable to find key: ")).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(key.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn get(&mut self, mut caller: String, mut width: i32, mut height: i32) -> Option<Rc<RefCell<Image>>> {
        let mut foundIndex: i32 = self.getIndexWH(width.clone(), height.clone());
        let mut image: Option<Rc<RefCell<Image>>> = self.getFromAvailable(foundIndex.clone(), width.clone(), height.clone());
        if match (&image, &crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.volume += width * height;
            if self.volume > 32000 {
                jcall!(self.tsUtil).gc();
                self.volume = 0;
            }
            image = self.createImage(caller.clone(), width.clone(), height.clone());
            let mut widths: Vec<i32> = self.widths.clone();
            if self.nextIndex > (widths.len() as i32) - 1 {
                if foundIndex == -1 {
                    foundIndex = self.nextIndex.clone();
                    widths[(self.nextIndex) as usize] = width.clone();
                    self.heights[(self.nextIndex) as usize] = height.clone();
                    self.nextIndex += 1;
                }
                jcall!(self.listOfList[(foundIndex) as usize]).add(image.clone());
            }
        }
        return image.clone();
    }
    

    // throws Exception
    pub fn getWithKey(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Image>>> {
        let mut image: Option<Rc<RefCell<Image>>> = self.getImage(key.clone());
        if match (&image, &crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut resourceUtil: Option<Rc<RefCell<ResourceUtil>>> = ResourceUtil::getInstance();
            let mut resourceInputStream: Option<Rc<RefCell<InputStream>>> = jcall!(resourceUtil).getResourceAsStream((key).clone() /* as String */);
            if resourceInputStream.is_none() {
                panic!("{:?}", format!("{}: {:?}", "RuntimeException", (jcall!(StringMaker::new()).append(String::from("Image resource is not available for key: ")).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(key.clone()))).toString())));
            }
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                image = self.createImageFromInputStream(key.clone(), resourceInputStream.clone());
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                jcall!(self.logUtil).put(String::from("Exception: Trying Again After GC"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone(), crate::java_into_object(e.clone()));
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("InputStream: ")).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(resourceInputStream.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                jcall!(self.tsUtil).gc();
                jcall!(self.tsUtil).gc();
                { let java_receiver = self.logUtil.clone(); let java_arguments = (Memory::getInfo(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                std::thread::sleep(std::time::Duration::from_millis((100) as u64));
                image = self.createImageFromInputStream(key.clone(), resourceInputStream.clone());
                }
            }
            if J2MEUtil::isHTML() {
            } else {
                jcall!(resourceInputStream).close();
            }
            jcall!(self.hashtable).put(key.clone(), crate::java_into_object(image.clone()));
        }
        return image.clone();
    }
    

    // throws Exception
    pub fn loadImageForAnimation(&mut self) {
    }
    

    pub fn progressEnded(&mut self) {
    }
    

    pub fn runTask(&mut self) {
    }
    

    pub fn isLazy(&mut self) -> bool {
        return false;
    }
}
impl std::ops::Deref for ImageCache {
    type Target = ImageCacheBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImageCache {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ImageCacheDyn: crate::org::allbinary::image::ImageCacheBase::ImageCacheBaseDyn {
    fn as_ImageCache(&self) -> &ImageCache;
    fn as_ImageCache_mut(&mut self) -> &mut ImageCache;
    fn addListener(&mut self, mut renderer: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { ImageCache::addListener(self.as_ImageCache_mut(), renderer) }
    fn getIndex(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> i32 { ImageCache::getIndex(self.as_ImageCache_mut(), key) }
    fn loadImageForAnimation(&mut self) { ImageCache::loadImageForAnimation(self.as_ImageCache_mut()) }
    fn progressEnded(&mut self) { ImageCache::progressEnded(self.as_ImageCache_mut()) }
    fn runTask(&mut self) { ImageCache::runTask(self.as_ImageCache_mut()) }
    fn isLazy(&mut self) -> bool { ImageCache::isLazy(self.as_ImageCache_mut()) }
}
impl ImageCacheDyn for ImageCache {
    fn as_ImageCache(&self) -> &ImageCache { self }
    fn as_ImageCache_mut(&mut self) -> &mut ImageCache { self }
}

impl crate::org::allbinary::image::ImageCacheBase::ImageCacheBaseDyn for ImageCache {
    fn as_ImageCacheBase(&self) -> &crate::org::allbinary::image::ImageCacheBase::ImageCacheBase { &self.base }
    fn as_ImageCacheBase_mut(&mut self) -> &mut crate::org::allbinary::image::ImageCacheBase::ImageCacheBase { &mut self.base }
    fn get(&mut self, mut caller: String, mut width: i32, mut height: i32) -> Option<Rc<RefCell<Image>>> { Self::get(self, caller, width, height) }
    fn getWithKey(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Image>>> { Self::getWithKey(self, key) }
}


