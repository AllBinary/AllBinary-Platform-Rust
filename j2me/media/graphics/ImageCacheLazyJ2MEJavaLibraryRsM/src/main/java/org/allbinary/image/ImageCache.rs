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
use crate::org::allbinary::animation::image::LazyImageRotationAnimation::LazyImageRotationAnimation;
use crate::org::allbinary::canvas::GameGlobalsFactory::GameGlobalsFactory;
use crate::org::allbinary::canvas::Processor::Processor;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::data::resource::ResourceUtil::ResourceUtil;
use crate::org::allbinary::game::canvas::ABToGBUtil::ABToGBUtil;
use crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas;
use crate::org::allbinary::game::gd::resource::GDLazyResources::GDLazyResources;
use crate::org::allbinary::game::gd::resource::GDResources::GDResources;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvasFactory::ProgressCanvasFactory;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::system::Memory::Memory;
use crate::org::allbinary::thread::BaseImageLoadingProcessor::BaseImageLoadingProcessor;
use crate::org::allbinary::thread::ConcurrentImageLoadingProcessor::ConcurrentImageLoadingProcessor;
use crate::org::allbinary::thread::SynchObject::SynchObject;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::logic::ABSystemWrapper::ABSystemWrapper;
use crate::java::lang::Thread::*;
use super::ImageCacheBase::ImageCacheBase;
use super::NotHTMLEndProcessor::NotHTMLEndProcessor;
use super::FirstProcessor::FirstProcessor;
use super::NotHTMLProcessor::NotHTMLProcessor;
use super::HTMLEndProcessor::HTMLEndProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageCache {
    pub base: ImageCacheBase,
    pub tsUtil: Option<Rc<RefCell<TsUtil>>>,
    pub concurrentImageLoadingProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::thread::BaseImageLoadingProcessor::BaseImageLoadingProcessorDyn>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub commonSeps: Option<Rc<RefCell<CommonSeps>>>,
    pub resourceUtil: Option<Rc<RefCell<ResourceUtil>>>,
    pub gameGlobalsFactory: Option<Rc<RefCell<GameGlobalsFactory>>>,
    pub gdResources: Option<Rc<RefCell<GDResources>>>,
    pub loadNowList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub loadSoonList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub loadList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub loadAfterList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub lock: Option<Rc<RefCell<SynchObject>>>,
    pub firstTime: bool,
    pub totalLoaded: i32,
    pub progressEnded: bool,
    pub hasAnyLazyAnimationFactories: bool,
    pub processor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>,
    pub endProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>,
    pub LOAD_IMAGE_FOR_ANIMATION: String,
}

thread_local! { pub static ImageCache_NULL_IMAGE_CACHE: Option<Rc<RefCell<dyn ImageCacheDyn>>> = ImageCache::new().map(|reference| reference as Rc<RefCell<dyn ImageCacheDyn>>); }

impl ImageCache {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: ImageCacheBase::new_struct(),
            tsUtil: None,
            concurrentImageLoadingProcessor: None,
            commonStrings: None,
            commonSeps: None,
            resourceUtil: None,
            gameGlobalsFactory: None,
            gdResources: None,
            loadNowList: None,
            loadSoonList: None,
            loadList: None,
            loadAfterList: None,
            lock: None,
            firstTime: false,
            totalLoaded: 0,
            progressEnded: false,
            hasAnyLazyAnimationFactories: false,
            processor: None,
            endProcessor: None,
            LOAD_IMAGE_FOR_ANIMATION: String::new(),
        };
        this.tsUtil = TsUtil::getInstance();
        this.concurrentImageLoadingProcessor = ConcurrentImageLoadingProcessor::new(this).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::thread::BaseImageLoadingProcessor::BaseImageLoadingProcessorDyn>>);
        this.commonStrings = CommonStrings::getInstance();
        this.commonSeps = CommonSeps::getInstance();
        this.resourceUtil = ResourceUtil::getInstance();
        this.gameGlobalsFactory = GameGlobalsFactory::getInstance();
        this.gdResources = GDResources::getInstance();
        this.loadNowList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.loadSoonList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.loadList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.loadAfterList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.lock = SynchObject::new();
        this.firstTime = true;
        this.totalLoaded = 0;
        this.progressEnded = false;
        this.hasAnyLazyAnimationFactories = false;
        this.processor = FirstProcessor::new();
        this.endProcessor = Processor::getInstance();
        this.LOAD_IMAGE_FOR_ANIMATION = String::from("Load Image Animation");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn firstProcess(&mut self) {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        let mut isHTML: bool = J2MEUtil::isHTML();
        if isHTML {
            self.processor = Processor::getInstance();
            self.endProcessor = HTMLEndProcessor::new();
        } else {
            self.processor = NotHTMLProcessor::new();
            self.endProcessor = NotHTMLEndProcessor::new();
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                self.runTask();
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                jcall!(logUtil).putF(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).END_METHOD_NAME.clone());
                }
            }
        }
    }
    

    pub fn addListener(&mut self, mut renderer: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
    

    // throws Exception
    pub fn waitForLoadNow(&mut self) {
        if self.firstTime {
            let mut abToGBUtil: Option<Rc<RefCell<ABToGBUtil>>> = ABToGBUtil::getInstance();
            let mut abCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>> = (jcall!(abToGBUtil).abCanvas).clone() /* as AllBinaryGameCanvas */;
            while jcall!(self.loadNowList).isEmpty() && (!jcall!(abCanvas).isInitialized() || (jcall!(abCanvas).isInitialized() && self.hasAnyLazyAnimationFactories)) && !self.progressEnded {
                std::thread::sleep(std::time::Duration::from_millis((120) as u64));
            }
            self.firstTime = false;
        }
    }
    

    // throws Exception
    pub fn loadImageForAnimation(&mut self) {
        let mut lazyImageRotationAnimation: Option<Rc<RefCell<LazyImageRotationAnimation>>> = None;
        { // synchronized (self.lock)
            if jcall!(self.loadNowList).isEmpty() {
                jcall!(self.endProcessor).process();
                if jcall!(self.loadSoonList).isEmpty() {
                    if jcall!(self.loadAfterList).isEmpty() {
                        if self.firstTime {
                        } else if jcall!(self.gameGlobalsFactory).newCanvas {
                        } else {
                            self.loadNextImage();
                        }
                        return;
                    } else {
                        lazyImageRotationAnimation = { let java_object = (jcall!(self.loadAfterList).get(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<LazyImageRotationAnimation>>>>().expect("ClassCastException").clone()) };
                        if self.loadImageForLazyAnimation(lazyImageRotationAnimation.clone()) {
                            jcall!(self.loadAfterList).remove(crate::java_into_object(lazyImageRotationAnimation.clone()));
                        }
                    }
                } else {
                    lazyImageRotationAnimation = { let java_object = (jcall!(self.loadSoonList).get(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<LazyImageRotationAnimation>>>>().expect("ClassCastException").clone()) };
                    if self.loadImageForLazyAnimation(lazyImageRotationAnimation.clone()) {
                        jcall!(self.loadSoonList).remove(crate::java_into_object(lazyImageRotationAnimation.clone()));
                    }
                }
                return;
            }
            lazyImageRotationAnimation = { let java_object = (jcall!(self.loadNowList).get(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<LazyImageRotationAnimation>>>>().expect("ClassCastException").clone()) };
        }
        if self.loadImageForLazyAnimation(lazyImageRotationAnimation.clone()) {
            { // synchronized (self.lock)
                jcall!(self.loadNowList).remove(crate::java_into_object(lazyImageRotationAnimation.clone()));
            }
            if jcall!(lazyImageRotationAnimation).layoutIndex != 0 {
                let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getAssociated(lazyImageRotationAnimation.clone());
                { // synchronized (self.lock)
                    let mut size: i32 = jcall!(list).size();
                    if size > 0 {
                        jcall!(self.loadSoonList).addAllList(list.clone());
                    }
                }
            }
            let mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>> = ProgressCanvasFactory::getInstance();
            let mut isHTML: bool = J2MEUtil::isHTML();
            if jcall!(self.loadNowList).isEmpty() && (!isHTML || self.firstTime) {
                jcall!(progressCanvas).endFromInitialLazyLoadingComplete();
            } else {
                if self.totalLoaded % 10 == 0 {
                    jcall!(progressCanvas).addNormalPortion(1, self.LOAD_IMAGE_FOR_ANIMATION.clone());
                }
            }
        }
    }
    

    // throws Exception
    pub fn loadImages(&mut self) {
        while !jcall!(self.loadList).isEmpty() || !jcall!(self.loadNowList).isEmpty() {
            self.loadImageForAnimations();
            self.loadNextImage();
        }
    }
    

    // throws Exception
    pub fn loadImageForAnimations(&mut self) {
        while !jcall!(self.loadNowList).isEmpty() {
            self.loadImageForAnimation();
        }
    }
    

    // throws Exception
    pub fn loadRemainingAnimations(&mut self) {
        while !jcall!(self.loadAfterList).isEmpty() || !jcall!(self.loadNowList).isEmpty() {
            while !jcall!(self.loadNowList).isEmpty() {
                self.loadImageForAnimation();
            }
            let mut lazyImageRotationAnimation: Option<Rc<RefCell<LazyImageRotationAnimation>>> = None;
            { // synchronized (self.lock)
                if !jcall!(self.loadAfterList).isEmpty() {
                    lazyImageRotationAnimation = { let java_object = (jcall!(self.loadAfterList).removeAt(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<LazyImageRotationAnimation>>>>().expect("ClassCastException").clone()) };
                }
            }
            if lazyImageRotationAnimation.is_some() {
                self.loadImageForLazyAnimation(lazyImageRotationAnimation.clone());
            }
        }
    }
    

    // throws Exception
    pub fn loadImageForLazyAnimation(&mut self, mut lazyImageRotationAnimation: Option<Rc<RefCell<LazyImageRotationAnimation>>>) -> bool {
        let mut image: Option<Rc<RefCell<Image>>> = jcall!(jcall!(lazyImageRotationAnimation).animationInterfaceFactoryInterface).getImage();
        if self.loadImage(image.clone()) {
            jcall!(lazyImageRotationAnimation).setRealAnimation();
            return true;
        }
        return false;
    }
    

    // throws Exception
    pub fn loadNextImage(&mut self) {
        let mut image: Option<Rc<RefCell<Image>>> = None;
        { // synchronized (self.lock)
            if jcall!(self.loadList).size() == 0 {
                return;
            }
            image = (jcall!(self.loadList).removeAt(0)).clone() /* as Image */;
        }
        self.loadImage(image.clone());
    }
    

    // throws Exception
    pub fn loadImage(&mut self, mut image: Option<Rc<RefCell<Image>>>) -> bool {
        if jcall!(image).isReady() {
            return true;
        } else {
            if jcall!(image).getImage().is_some() {
                if jcall!(image).setReady() {
                    if jcall!(image).isReady() {
                        self.totalLoaded += 1;
                        return true;
                    }
                }
            } else {
                let mut key: String = jcall!(image).getName();
                let mut image2: Option<Rc<RefCell<Image>>> = self.creatImage(key.clone());
                if jcall!(image2).isReady() {
                    self.init(image.clone(), image2.clone());
                    return true;
                } else {
                    jcall!(image).setImage(jcall!(image2).getImage());
                }
            }
        }
        return false;
    }
    

    // throws Exception
    pub fn init(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut image2: Option<Rc<RefCell<Image>>>) {
        jcall!(image).init(jcall!(image2).getImage());
    }
    

    // throws Exception
    pub fn creatImage(&mut self, mut key: String) -> Option<Rc<RefCell<Image>>> {
        let mut inputStream: Option<Rc<RefCell<InputStream>>> = jcall!(self.resourceUtil).getResourceAsStream(key.clone());
        let mut image: Option<Rc<RefCell<Image>>> = jcall!(ImageFactory::ImageFactory::getInstance()).createImageFromInputStream(inputStream.clone());
        jcall!(image).setName(key.clone());
        return image.clone();
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
            if self.nextIndex > (self.widths.len() as i32) - 1 {
                if foundIndex == -1 {
                    foundIndex = self.nextIndex.clone();
                    self.widths[(self.nextIndex) as usize] = width.clone();
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
            let mut inputStream: Option<Rc<RefCell<InputStream>>> = None;
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                image = self.createImageFromInputStream(key.clone(), inputStream.clone());
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                jcall!(self.logUtil).put(String::from("Exception: Trying Again After GC"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone(), crate::java_into_object(e.clone()));
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("InputStream: ")).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(inputStream.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                jcall!(self.tsUtil).gc();
                jcall!(self.tsUtil).gc();
                { let java_receiver = self.logUtil.clone(); let java_arguments = (Memory::getInfo(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                std::thread::sleep(std::time::Duration::from_millis((100) as u64));
                image = self.createImageFromInputStream(key.clone(), inputStream.clone());
                }
            }
            jcall!(self.hashtable).put(crate::java_into_object(key.clone()), crate::java_into_object(image.clone()));
        }
        return image.clone();
    }
    

    pub fn getIndex(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> i32 {
        let mut gdResources: Option<Rc<RefCell<GDResources>>> = GDResources::getInstance();
        let mut resourceStringArray: Vec<String> = jcall!(gdResources).resourceStringArray.clone();
        let mut size: i32 = (resourceStringArray.len() as i32).clone();
        for index in (0..size) {
            if resourceStringArray[(index) as usize] == key {
                return index.clone();
            }
        }
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("unable to find key: ")).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(key.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn createImageFromInputStream(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut inputStream: Option<Rc<RefCell<InputStream>>>) -> Option<Rc<RefCell<Image>>> {
        let mut gdLazyResources: Option<Rc<RefCell<GDLazyResources>>> = GDLazyResources::getInstance();
        let mut resourceStringArray: Vec<String> = jcall!(gdLazyResources).requiredResourcesBeforeLoadingArray.clone();
        let mut size: i32 = (resourceStringArray.len() as i32).clone();
        for index in (0..size) {
            if key == resourceStringArray[(index) as usize] {
                return self.creatImage((key).clone() /* as String */);
            }
        }
        self.runTask();
        let mut index: i32 = self.getIndex(key.clone());
        let mut width: i32 = jcall!(gdLazyResources).imageResourceWidthArray[(index) as usize].clone();
        let mut height: i32 = jcall!(gdLazyResources).imageResourceHeightArray[(index) as usize].clone();
        let mut image: Option<Rc<RefCell<Image>>> = self.createImageLater((key).clone() /* as String */, width.clone(), height.clone());
        { // synchronized (self.lock)
            jcall!(self.loadList).add(crate::java_into_object(image.clone()));
        }
        return image.clone();
    }
    

    // throws Exception
    pub fn createImageLater(&mut self, mut key: String, mut width: i32, mut height: i32) -> Option<Rc<RefCell<Image>>> {
        return Image::createImageLater(key.clone(), width.clone(), height.clone());
    }
    

    pub fn getAssociated(&mut self, mut lazyImageRotationAnimation: Option<Rc<RefCell<LazyImageRotationAnimation>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        { // synchronized (self.lock)
            let mut lazyImageRotationAnimation2: Option<Rc<RefCell<LazyImageRotationAnimation>>> = None;
            let mut size: i32 = jcall!(self.loadAfterList).size();
            for index in (0..size) {
                lazyImageRotationAnimation2 = { let java_object = (jcall!(self.loadAfterList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<LazyImageRotationAnimation>>>>().expect("ClassCastException").clone()) };
                if jcall!(lazyImageRotationAnimation2).instanceId == jcall!(lazyImageRotationAnimation).instanceId {
                    jcall!(list).add(crate::java_into_object(lazyImageRotationAnimation2.clone()));
                }
            }
            let mut size2: i32 = jcall!(list).size();
            for index in (0..size2) {
                { let java_receiver = self.loadAfterList.clone(); let java_arguments = (crate::java_into_object(jcall!(list).get(index.clone())),); jcall!(java_receiver).remove(java_arguments.0); };
            }
        }
        return list.clone();
    }
    

    pub fn add(&mut self, mut lazyImageRotationAnimation: Option<Rc<RefCell<LazyImageRotationAnimation>>>) {
        { // synchronized (self.lock)
            jcall!(self.loadAfterList).add(crate::java_into_object(lazyImageRotationAnimation.clone()));
        }
    }
    

    // throws Exception
    pub fn insertFirst(&mut self, mut lazyImageRotationAnimation: Option<Rc<RefCell<LazyImageRotationAnimation>>>) {
        if jcall!(self.loadNowList).contains(crate::java_into_object(lazyImageRotationAnimation.clone())) {
        } else {
            { // synchronized (self.lock)
                jcall!(self.loadNowList).add(crate::java_into_object(lazyImageRotationAnimation.clone()));
                jcall!(self.loadAfterList).remove(crate::java_into_object(lazyImageRotationAnimation.clone()));
            }
        }
        self.runTask();
    }
    

    pub fn progressEnded(&mut self) {
        self.progressEnded = true;
    }
    

    // throws Exception
    pub fn runTask(&mut self) {
        jcall!(self.processor).process();
    }
    

    pub fn initProgress(&mut self) {
        if self.firstTime {
            self.firstTime = false;
        }
    }
    

    pub fn isLazy(&mut self) -> bool {
        return true;
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
    fn firstProcess(&mut self) { ImageCache::firstProcess(self.as_ImageCache_mut()) }
    fn addListener(&mut self, mut renderer: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { ImageCache::addListener(self.as_ImageCache_mut(), renderer) }
    fn waitForLoadNow(&mut self) { ImageCache::waitForLoadNow(self.as_ImageCache_mut()) }
    fn loadImageForAnimation(&mut self) { ImageCache::loadImageForAnimation(self.as_ImageCache_mut()) }
    fn loadImages(&mut self) { ImageCache::loadImages(self.as_ImageCache_mut()) }
    fn loadImageForAnimations(&mut self) { ImageCache::loadImageForAnimations(self.as_ImageCache_mut()) }
    fn loadRemainingAnimations(&mut self) { ImageCache::loadRemainingAnimations(self.as_ImageCache_mut()) }
    fn loadImageForLazyAnimation(&mut self, mut lazyImageRotationAnimation: Option<Rc<RefCell<LazyImageRotationAnimation>>>) -> bool { ImageCache::loadImageForLazyAnimation(self.as_ImageCache_mut(), lazyImageRotationAnimation) }
    fn loadNextImage(&mut self) { ImageCache::loadNextImage(self.as_ImageCache_mut()) }
    fn loadImage(&mut self, mut image: Option<Rc<RefCell<Image>>>) -> bool { ImageCache::loadImage(self.as_ImageCache_mut(), image) }
    fn init(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut image2: Option<Rc<RefCell<Image>>>) { ImageCache::init(self.as_ImageCache_mut(), image, image2) }
    fn creatImage(&mut self, mut key: String) -> Option<Rc<RefCell<Image>>> { ImageCache::creatImage(self.as_ImageCache_mut(), key) }
    fn getIndex(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> i32 { ImageCache::getIndex(self.as_ImageCache_mut(), key) }
    fn createImageLater(&mut self, mut key: String, mut width: i32, mut height: i32) -> Option<Rc<RefCell<Image>>> { ImageCache::createImageLater(self.as_ImageCache_mut(), key, width, height) }
    fn getAssociated(&mut self, mut lazyImageRotationAnimation: Option<Rc<RefCell<LazyImageRotationAnimation>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { ImageCache::getAssociated(self.as_ImageCache_mut(), lazyImageRotationAnimation) }
    fn add(&mut self, mut lazyImageRotationAnimation: Option<Rc<RefCell<LazyImageRotationAnimation>>>) { ImageCache::add(self.as_ImageCache_mut(), lazyImageRotationAnimation) }
    fn insertFirst(&mut self, mut lazyImageRotationAnimation: Option<Rc<RefCell<LazyImageRotationAnimation>>>) { ImageCache::insertFirst(self.as_ImageCache_mut(), lazyImageRotationAnimation) }
    fn progressEnded(&mut self) { ImageCache::progressEnded(self.as_ImageCache_mut()) }
    fn runTask(&mut self) { ImageCache::runTask(self.as_ImageCache_mut()) }
    fn isLazy(&mut self) -> bool { ImageCache::isLazy(self.as_ImageCache_mut()) }
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
    fn createImageFromInputStream(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut inputStream: Option<Rc<RefCell<InputStream>>>) -> Option<Rc<RefCell<Image>>> { Self::createImageFromInputStream(self, key, inputStream) }
    fn initProgress(&mut self) { Self::initProgress(self) }
}


pub struct NotHTMLProcessor {
    pub base: Processor,
}


impl NotHTMLProcessor {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Processor::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
        

        pub fn process(&mut self) {
            jcall!(self.concurrentImageLoadingProcessor).runTask();
        }
}
impl std::ops::Deref for NotHTMLProcessor {
    type Target = Processor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NotHTMLProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::canvas::Processor::ProcessorDyn for NotHTMLProcessor {
    fn as_Processor(&self) -> &crate::org::allbinary::canvas::Processor::Processor { &self }
    fn as_Processor_mut(&mut self) -> &mut crate::org::allbinary::canvas::Processor::Processor { &mut self }
    fn process(&mut self) { Self::process(self) }
}


pub struct NotHTMLEndProcessor {
    pub base: Processor,
}


impl NotHTMLEndProcessor {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Processor::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
        

        pub fn process(&mut self) {
            let mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>> = ProgressCanvasFactory::getInstance();
            jcall!(progressCanvas).endIfPaintedSinceStart();
        }
}
impl std::ops::Deref for NotHTMLEndProcessor {
    type Target = Processor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NotHTMLEndProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::canvas::Processor::ProcessorDyn for NotHTMLEndProcessor {
    fn as_Processor(&self) -> &crate::org::allbinary::canvas::Processor::Processor { &self }
    fn as_Processor_mut(&mut self) -> &mut crate::org::allbinary::canvas::Processor::Processor { &mut self }
    fn process(&mut self) { Self::process(self) }
}


pub struct HTMLEndProcessor {
    pub base: Processor,
}


impl HTMLEndProcessor {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Processor::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
        

        pub fn process(&mut self) {
            let mut size: i32 = jcall!(self.gdResources).currentLayoutRequiredTotal.clone();
            if size == 0 {
                let mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>> = ProgressCanvasFactory::getInstance();
                jcall!(progressCanvas).endIfPaintedSinceStart();
            } else if self.totalLoaded > size / 12 {
                let mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>> = ProgressCanvasFactory::getInstance();
                jcall!(progressCanvas).endIfPaintedSinceStart();
                self.endProcessor = NotHTMLEndProcessor::new();
            }
        }
}
impl std::ops::Deref for HTMLEndProcessor {
    type Target = Processor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HTMLEndProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::canvas::Processor::ProcessorDyn for HTMLEndProcessor {
    fn as_Processor(&self) -> &crate::org::allbinary::canvas::Processor::Processor { &self }
    fn as_Processor_mut(&mut self) -> &mut crate::org::allbinary::canvas::Processor::Processor { &mut self }
    fn process(&mut self) { Self::process(self) }
}


pub struct FirstProcessor {
    pub base: Processor,
}


impl FirstProcessor {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Processor::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
        

        pub fn process(&mut self) {
            self.firstProcess();
        }
}
impl std::ops::Deref for FirstProcessor {
    type Target = Processor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for FirstProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::canvas::Processor::ProcessorDyn for FirstProcessor {
    fn as_Processor(&self) -> &crate::org::allbinary::canvas::Processor::Processor { &self }
    fn as_Processor_mut(&mut self) -> &mut crate::org::allbinary::canvas::Processor::Processor { &mut self }
    fn process(&mut self) { Self::process(self) }
}


