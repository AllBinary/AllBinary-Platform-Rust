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
// package org.allbinary.input.automation.module.generic
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::image::BufferedImage::BufferedImage;
// use crate::java::io::File::File;
// use crate::javax::imageio::ImageIO::ImageIO;
use crate::org::allbinary::input::automation::module::generic::configuration::profile::SavedCaptureGenericProfileDataWorkerType::SavedCaptureGenericProfileDataWorkerType;
use crate::org::allbinary::input::media::image::capture::CaptureWorkerInterface::CaptureWorkerInterface;
use crate::org::allbinary::input::media::image::capture::CapturedBufferedImagesCacheSingleton::CapturedBufferedImagesCacheSingleton;
use crate::org::allbinary::input::media::image::capture::CapturedImageWorkerResultsEvent::CapturedImageWorkerResultsEvent;
use crate::org::allbinary::input::media::image::capture::ProcessingFrameIndexFactory::ProcessingFrameIndexFactory;
use crate::org::allbinary::input::media::image::capture::ScreenScavangerRobot::ScreenScavangerRobot;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::java::number::LongUtil::LongUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler;
use crate::org::allbinary::logic::visual::media::MediaDataFactory::MediaDataFactory;
use crate::org::allbinary::media::image::cache::BufferedImageFrameCacheable::BufferedImageFrameCacheable;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::java::lang::Thread::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SavedCaptureImagesWorker {
    pub base: BasicEventHandler,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub running: bool,
    pub screenScavangerRobot: Option<Rc<RefCell<ScreenScavangerRobot>>>,
    pub savedCaptureGenericProfileDataWorkerType: Option<Rc<RefCell<SavedCaptureGenericProfileDataWorkerType>>>,
}

pub static mut SavedCaptureImagesWorker_index: std::cell::LazyCell<i64> = std::cell::LazyCell::new(|| 0_i64);
// impl CaptureWorkerInterface for SavedCaptureImagesWorker with the methods of the trait

impl SavedCaptureImagesWorker {
    

    pub fn new_struct(mut savedCaptureGenericProfileDataWorkerType: Option<Rc<RefCell<SavedCaptureGenericProfileDataWorkerType>>>) -> Self {
        let mut this = Self {
            base: BasicEventHandler::new_struct(),
            logUtil: None,
            commonStrings: None,
            running: false,
            screenScavangerRobot: None,
            savedCaptureGenericProfileDataWorkerType: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.savedCaptureGenericProfileDataWorkerType = savedCaptureGenericProfileDataWorkerType.clone();
        this.screenScavangerRobot = ScreenScavangerRobot::new();
        unsafe { *SavedCaptureImagesWorker_index = ProcessingFrameIndexFactory::next() };
        this
    }

    pub fn new(mut savedCaptureGenericProfileDataWorkerType: Option<Rc<RefCell<SavedCaptureGenericProfileDataWorkerType>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(savedCaptureGenericProfileDataWorkerType))))
    }
    

    // throws Exception
    pub fn setThread(&mut self, mut thread: Option<Rc<RefCell<Thread>>>) {
    }
    

    // synchronized
    pub fn isRunning(&mut self) -> bool {
        return self.running.clone();
    }
    

    // synchronized
    pub fn setRunning(&mut self, mut running: bool) {
        self.running = running.clone();
    }
    

    pub fn run(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
            self.setRunning(true);
            let mut timeHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>> = TimeDelayHelper::new(1000).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
            while self.isRunning() {
                jcall!(timeHelper).setStartTimeTNT();
                let mut frame: Option<i64> = Some((unsafe { SavedCaptureImagesWorker_index.clone() }.clone()) as i64);
                let mut filePathStringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(filePathStringBuffer).append(jcall!(self.savedCaptureGenericProfileDataWorkerType).getPath());
                jcall!(filePathStringBuffer).append(LongUtil::fillIn((frame).expect("NullPointerException").to_string()));
                jcall!(filePathStringBuffer).append(jcall!(jcall!(MediaDataFactory::getInstance()).JPG).getExtension());
                let mut filePath: String = jcall!(filePathStringBuffer).toString();
                jcall!(self.logUtil).putF(format!("{}{}", String::from("Loading Image File Path: "), filePath), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
                let mut file: Option<Rc<RefCell<File>>> = File::new(filePath.clone());
                if jcall!(file).isFile() {
                    let mut bufferedImage: Option<Rc<RefCell<BufferedImage>>> = ImageIO::read(file.clone());
                    unsafe { *SavedCaptureImagesWorker_index += 1 };
                    jcall!(CapturedBufferedImagesCacheSingleton::getInstance()).add(BufferedImageFrameCacheable::new(bufferedImage.clone(), frame.clone()));
                    let mut capturedImageEvent: Option<Rc<RefCell<CapturedImageWorkerResultsEvent>>> = CapturedImageWorkerResultsEvent::new(None /* Java this: a &mut self method has no Rc of itself */, frame.clone(), bufferedImage.clone());
                    self.fireEvent(capturedImageEvent.clone());
                } else {
                    jcall!(self.logUtil).putF(format!("{}{}", String::from("Could Not Load File: "), filePath), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
                }
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", jcall!(CommonLabels::getInstance()).ELAPSED, jcall!(timeHelper).getElapsedTNT()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                self.setRunning(false);
            }
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).END.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
}
impl std::ops::Deref for SavedCaptureImagesWorker {
    type Target = BasicEventHandler;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SavedCaptureImagesWorker {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn for SavedCaptureImagesWorker {
    fn as_BasicEventHandler(&self) -> &crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler { &self.base }
    fn as_BasicEventHandler_mut(&mut self) -> &mut crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler { &mut self.base }
}


