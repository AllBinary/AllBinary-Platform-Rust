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
use crate::java::io::InputStream::InputStream;
use crate::javax::microedition::lcdui::Image::Image;
use crate::javax::microedition::lcdui::NullImage::NullImage;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::data::resource::ResourceUtil::ResourceUtil;
use crate::org::allbinary::game::gd::resource::GDResources::GDResources;
use crate::org::allbinary::logic::ABSystemWrapper::ABSystemWrapper;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::system::Memory::Memory;
use crate::java::lang::Thread::*;
use crate::java::lang::Exception::*;
use crate::java::lang::RuntimeException::*;
use super::ImageCacheBase::ImageCacheBase;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageCache {
    pub base: ImageCacheBase,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub systemWrapper: Option<Rc<RefCell<ABSystemWrapper>>>,
    pub tsUtil: Option<Rc<RefCell<TsUtil>>>,
}

pub static ImageCache_NULL_IMAGE_CACHE: crate::JavaStatic<Option<Rc<RefCell<dyn ImageCacheDyn>>>> = crate::JavaStatic::new(|| ImageCache::new().map(|reference| reference as Rc<RefCell<dyn ImageCacheDyn>>));

impl ImageCache {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: ImageCacheBase::new_struct(),
            logUtil: None,
            commonStrings: None,
            systemWrapper: None,
            tsUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.systemWrapper = ABSystemWrapper::getInstance();
        this.tsUtil = TsUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn get3(&mut self, mut caller: String, mut width: i32, mut height: i32) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        let mut foundIndex: i32 = self.getIndexWH(width.clone(), height.clone());
        let mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = self.getFromAvailable(foundIndex.clone(), width.clone(), height.clone());
        if match (&image, &crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.volume += width * height;
            if self.volume > 32000 {
                jcall!(self.tsUtil).gc();
                self.volume = 0;
            }
            image = self.createImage(caller.clone(), width.clone(), height.clone());
            if foundIndex == -1 {
                foundIndex = self.nextIndex.clone();
                { let java_index = (self.nextIndex) as usize; self.widths[java_index] = width.clone(); };
                { let java_index = (self.nextIndex) as usize; self.heights[java_index] = height.clone(); };
                self.nextIndex += 1;
            }
            jcall!(self.listOfList[(foundIndex) as usize]).add(crate::java_into_object(image.clone()));
        }
        return image.clone();
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn get(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        let mut resourceUtil: Option<Rc<RefCell<ResourceUtil>>> = ResourceUtil::getInstance();
        let mut resourceId: Option<i32> = jcall!(resourceUtil).getResourceId({ let java_object = (key).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() });
        let mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = self.getImage(resourceId.clone());
        if match (&image, &crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut inputStream: Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>> = None;
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (Memory::getInfo(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).GET.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                image = self.createImage(key.clone(), inputStream.clone());
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

                jcall!(self.logUtil).put(String::from("Exception: Trying Again After GC"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).GET.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("InputStream: ")).append(jcall!(inputStream).as_InputStream_mut().toString()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).GET.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                jcall!(self.tsUtil).gc();
                jcall!(self.tsUtil).gc();
                { let java_receiver = self.logUtil.clone(); let java_arguments = (Memory::getInfo(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).GET.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                std::thread::sleep(std::time::Duration::from_millis((100) as u64));
                image = self.createImage(key.clone(), inputStream.clone());
                }
            }
            jcall!(self.hashtable).put(crate::java_into_object((resourceId.clone()).expect("NullPointerException")), crate::java_into_object(image.clone()));
        }
        return image.clone();
    }
    

    pub fn getIndex(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> i32 {
        let mut gdResources: Option<Rc<RefCell<GDResources>>> = GDResources::getInstance();
        let mut resourceStringArray: Vec<String> = { let java_field = jcall!(gdResources).resourceStringArray.clone(); java_field }.clone();
        let mut size: i32 = (resourceStringArray.len() as i32).clone();
        for index in (0..size) {
            if crate::java_object_equals(&crate::java_into_object((resourceStringArray[(index) as usize]).clone()), &(key)) {
                return index.clone();
            }
        }
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("unable to find key: ")).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(key.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).RUN.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn createImage(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut inputStream: Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>>) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        return Image::createImageLater({ let java_object = (key).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }, -1, -1);
    }
    

    pub fn isLazy(&mut self) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
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
    fn get(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> { ImageCache::get(self.as_ImageCache_mut(), key) }
    fn getIndex(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> i32 { ImageCache::getIndex(self.as_ImageCache_mut(), key) }
    fn createImage(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut inputStream: Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>>) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> { ImageCache::createImage(self.as_ImageCache_mut(), key, inputStream) }
    fn isLazy(&mut self) -> bool { ImageCache::isLazy(self.as_ImageCache_mut()) }
}
impl ImageCacheDyn for ImageCache {
    fn as_ImageCache(&self) -> &ImageCache { self }
    fn as_ImageCache_mut(&mut self) -> &mut ImageCache { self }
}

impl crate::org::allbinary::image::ImageCacheBase::ImageCacheBaseDyn for ImageCache {
    fn as_ImageCacheBase(&self) -> &crate::org::allbinary::image::ImageCacheBase::ImageCacheBase { &self.base }
    fn as_ImageCacheBase_mut(&mut self) -> &mut crate::org::allbinary::image::ImageCacheBase::ImageCacheBase { &mut self.base }
    fn get(&mut self, mut caller: String, mut width: i32, mut height: i32) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> { Self::get3(self, caller, width, height) }
}


