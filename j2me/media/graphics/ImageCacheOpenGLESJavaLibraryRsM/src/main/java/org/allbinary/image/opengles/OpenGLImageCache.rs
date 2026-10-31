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
// package org.allbinary.image.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::InputStream::InputStream;
// use crate::javax::microedition::khronos::opengles::GL10::GL10;
// use crate::javax::microedition::lcdui::Image::Image;
// use crate::javax::microedition::lcdui::NullImage::NullImage;
use crate::org::allbinary::device::NullGL10::NullGL10;
use crate::org::allbinary::graphics::opengles::renderer::AllBinaryRendererBase3::AllBinaryRendererBase3;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::image::ImageCache::ImageCache;
use crate::org::allbinary::image::ImageCacheFactory::ImageCacheFactory;
use crate::org::allbinary::image::PreResourceImageUtil::PreResourceImageUtil;
use crate::org::allbinary::thread::SynchObject::SynchObject;
use super::OpenGLESImage::OpenGLESImage;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//ImageCacheOpenGLESJavaLibrary
pub struct OpenGLImageCache {
    pub base: ImageCache,
    pub imageCache: Option<Rc<RefCell<dyn crate::org::allbinary::image::ImageCache::ImageCacheDyn>>>,
    pub preResourceImageUtil: Option<Rc<RefCell<PreResourceImageUtil>>>,
    pub gl: Option<Rc<RefCell<dyn GL10>>>,
    pub lock: Option<Rc<RefCell<SynchObject>>>,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub renderer: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::renderer::AllBinaryRendererBase3::AllBinaryRendererBase3Dyn>>>,
}


impl OpenGLImageCache {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: ImageCache::new_struct(),
            imageCache: None,
            preResourceImageUtil: None,
            gl: None,
            lock: None,
            list: None,
            renderer: None,
        };
        this.imageCache = ImageCacheFactory::getInstance();
        this.preResourceImageUtil = PreResourceImageUtil::getInstance();
        this.gl = crate::org::allbinary::device::NullGL10::NullGL10_NULL_GL10.with(Clone::clone).clone();
        this.lock = SynchObject::new();
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.renderer = AllBinaryRendererBase3::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::opengles::renderer::AllBinaryRendererBase3::AllBinaryRendererBase3Dyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn addListener(&mut self, mut renderer: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
        self.renderer = (renderer).clone() /* as AllBinaryRendererBase3 */;
    }
    

    // throws Exception
    pub fn update(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) {
        self.gl = gl.clone();
        { // synchronized (self.lock)
            for index in (0..=jcall!(self.list).size() - 1).rev() {
                let mut openGLESImage: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>> = ({ let java_object = (jcall!(self.list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>>>().expect("ClassCastException").clone()) });
                if !match (&openGLESImage, &OpenGLESImage_NULL_OPENGL_IMAGE.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    jcall!(openGLESImage).set(gl.clone().map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>));
                }
            }
        }
    }
    

    // throws Exception
    pub fn createImage(&mut self, mut caller: String, mut width: i32, mut height: i32) -> Option<Rc<RefCell<Image>>> {
        let mut textureSize: i32 = width.clone();
        if height > width {
            textureSize = height.clone();
        }
        while (textureSize % 4) != 0 {
            textureSize += 1;
        }
        width = textureSize.clone();
        height = textureSize.clone();
        let mut image2: Option<Rc<RefCell<Image>>> = jcall!(self.imageCache).get(caller.clone(), width.clone(), height.clone());
        let mut image: Option<Rc<RefCell<Image>>> = jcall!(self.preResourceImageUtil).encapsulate(image2.clone());
        { // synchronized (self.lock)
            if !match (&image, &crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(self.list).add(crate::java_into_object(image.clone()));
            }
        }
        return image.clone();
    }
    

    // throws Exception
    pub fn createImageFromInputStream(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut inputStream: Option<Rc<RefCell<InputStream>>>) -> Option<Rc<RefCell<Image>>> {
        let mut cachedImage: Option<Rc<RefCell<Image>>> = jcall!(self.imageCache).getWithKey(crate::java_into_object(key.clone()));
        let mut image: Option<Rc<RefCell<Image>>> = jcall!(self.preResourceImageUtil).encapsulate(cachedImage.clone());
        { // synchronized (self.lock)
            if !match (&image, &crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(self.list).add(crate::java_into_object(image.clone()));
            }
        }
        return image.clone();
    }
    

    pub fn getGlP(&mut self) -> Option<Rc<RefCell<dyn GL10>>> {
        return self.gl.clone();
    }
    

    pub fn init(&mut self, mut image: Option<Rc<RefCell<Image>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { // synchronized (self.lock)
                if jcall!(self.list).contains(crate::java_into_object(image.clone())) {
                    panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
                }
                jcall!(self.list).add(crate::java_into_object(image.clone()));
            }
            jcall!(self.renderer).add(image.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), None /* Java this: a &mut self method has no Rc of itself */, jcall!(self.commonStrings).INIT.clone(), e.clone());
            }
        }
    }
}
impl std::ops::Deref for OpenGLImageCache {
    type Target = ImageCache;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for OpenGLImageCache {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::image::ImageCache::ImageCacheDyn for OpenGLImageCache {
    fn as_ImageCache(&self) -> &crate::org::allbinary::image::ImageCache::ImageCache { &self.base }
    fn as_ImageCache_mut(&mut self) -> &mut crate::org::allbinary::image::ImageCache::ImageCache { &mut self.base }
    fn addListener(&mut self, mut renderer: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { Self::addListener(self, renderer) }
}


