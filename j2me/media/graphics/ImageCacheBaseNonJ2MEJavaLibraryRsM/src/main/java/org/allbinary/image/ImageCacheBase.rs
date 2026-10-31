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
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::ImageFactory::ImageFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageCacheBase {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub imageFactory: Option<Rc<RefCell<ImageFactory>>>,
    pub hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>,
    pub SIZE: i32,
    pub widths: Vec<i32>,
    pub heights: Vec<i32>,
    pub listOfList: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>,
    pub availableListOfList: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>,
    pub volume: i32,
    pub nextIndex: i32,
}


impl ImageCacheBase {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            imageFactory: None,
            hashtable: None,
            SIZE: 0,
            widths: Vec::new(),
            heights: Vec::new(),
            listOfList: Vec::new(),
            availableListOfList: Vec::new(),
            volume: 0,
            nextIndex: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.imageFactory = ImageFactory::getInstance();
        this.hashtable = jcall!(StdUtil::getInstance()).createHashtable();
        this.SIZE = 128;
        this.widths = vec![0; (this.SIZE) as usize];
        this.heights = vec![0; (this.SIZE) as usize];
        this.listOfList = vec![None; (this.SIZE) as usize];
        this.availableListOfList = vec![None; (this.SIZE) as usize];
        this.volume = 0;
        this.nextIndex = 0;
        for index in (0..=(this.listOfList.len() as i32) - 1).rev() {
            this.listOfList[(index) as usize] = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>);
        }
        for index in (0..=(this.availableListOfList.len() as i32) - 1).rev() {
            this.availableListOfList[(index) as usize] = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>);
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getImage(&mut self, mut resourceId: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Image>>> {
        let mut imageCanBeNull: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.hashtable).get(resourceId.clone());
        if imageCanBeNull.is_none() {
            return crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone).clone();
        }
        return (imageCanBeNull).clone() /* as Image */;
    }
    

    pub fn releaseAll(&mut self) {
        for index in (0..=(self.listOfList.len() as i32) - 1).rev() {
            jcall!(self.availableListOfList[(index) as usize]).clear();
            jcall!(self.availableListOfList[(index) as usize]).addAllList(self.listOfList[(index) as usize].clone());
        }
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("ImageCache: ")).append(self.toString()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("releaseAll"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn getIndexWH(&mut self, mut width: i32, mut height: i32) -> i32 {
        let mut foundIndex: i32 = -1;
        let mut size: i32 = (self.widths.len() as i32).clone();
        for index in (0..size) {
            if self.widths[(index) as usize] == width && self.heights[(index) as usize] == height {
                foundIndex = index.clone();
                return foundIndex.clone();
            }
        }
        return foundIndex.clone();
    }
    

    pub fn getFromAvailable(&mut self, mut foundIndex: i32, mut width: i32, mut height: i32) -> Option<Rc<RefCell<Image>>> {
        if foundIndex != -1 {
            if jcall!(self.availableListOfList[(foundIndex) as usize]).size() > 0 {
                let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.availableListOfList[(foundIndex) as usize].clone();
                return (jcall!(list).removeAt(jcall!(list).size() - 1)).clone() /* as Image */;
            }
        }
        return crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn get(&mut self, mut caller: String, mut width: i32, mut height: i32) -> Option<Rc<RefCell<Image>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn getWithKey(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Image>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn createImage(&mut self, mut caller: String, mut width: i32, mut height: i32) -> Option<Rc<RefCell<Image>>> {
        return jcall!(self.imageFactory).createImage(caller.clone(), width.clone(), height.clone());
    }
    

    // throws Exception
    pub fn createImageFromInputStream(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut inputStream: Option<Rc<RefCell<InputStream>>>) -> Option<Rc<RefCell<Image>>> {
        return jcall!(self.imageFactory).createImageFromInputStream2(crate::java_into_object(key.clone()), inputStream.clone());
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        for index in (0..=self.nextIndex - 1).rev() {
            let mut width: i32 = self.widths[(index) as usize].clone();
            let mut height: i32 = self.heights[(index) as usize].clone();
            let mut total: i32 = jcall!(self.listOfList[(index) as usize]).size();
            let mut totalAvailable: i32 = jcall!(self.availableListOfList[(index) as usize]).size();
            jcall!(stringBuffer).append(String::from(" w: "));
            jcall!(stringBuffer).appendint(width.clone());
            jcall!(stringBuffer).append(String::from(" h: "));
            jcall!(stringBuffer).appendint(height.clone());
            jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).SPACE.clone());
            jcall!(stringBuffer).append(jcall!(CommonLabels::getInstance()).TOTAL_LABEL.clone());
            jcall!(stringBuffer).appendint(total.clone());
            jcall!(stringBuffer).append(String::from(" available: "));
            jcall!(stringBuffer).appendint(totalAvailable.clone());
        }
        return jcall!(stringBuffer).toString();
    }
    

    pub fn getHashtableP(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        return self.hashtable.clone();
    }
    

    pub fn init(&mut self, mut image: Option<Rc<RefCell<Image>>>) {
    }
    

    pub fn initProgress(&mut self) {
    }
}
pub trait ImageCacheBaseDyn {
    fn as_ImageCacheBase(&self) -> &ImageCacheBase;
    fn as_ImageCacheBase_mut(&mut self) -> &mut ImageCacheBase;
    fn getImage(&mut self, mut resourceId: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Image>>> { ImageCacheBase::getImage(self.as_ImageCacheBase_mut(), resourceId) }
    fn releaseAll(&mut self) { ImageCacheBase::releaseAll(self.as_ImageCacheBase_mut()) }
    fn getIndexWH(&mut self, mut width: i32, mut height: i32) -> i32 { ImageCacheBase::getIndexWH(self.as_ImageCacheBase_mut(), width, height) }
    fn getFromAvailable(&mut self, mut foundIndex: i32, mut width: i32, mut height: i32) -> Option<Rc<RefCell<Image>>> { ImageCacheBase::getFromAvailable(self.as_ImageCacheBase_mut(), foundIndex, width, height) }
    fn get(&mut self, mut caller: String, mut width: i32, mut height: i32) -> Option<Rc<RefCell<Image>>> { ImageCacheBase::get(self.as_ImageCacheBase_mut(), caller, width, height) }
    fn getWithKey(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Image>>> { ImageCacheBase::getWithKey(self.as_ImageCacheBase_mut(), key) }
    fn createImage(&mut self, mut caller: String, mut width: i32, mut height: i32) -> Option<Rc<RefCell<Image>>> { ImageCacheBase::createImage(self.as_ImageCacheBase_mut(), caller, width, height) }
    fn createImageFromInputStream(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut inputStream: Option<Rc<RefCell<InputStream>>>) -> Option<Rc<RefCell<Image>>> { ImageCacheBase::createImageFromInputStream(self.as_ImageCacheBase_mut(), key, inputStream) }
    fn toString(&mut self) -> String { ImageCacheBase::toString(self.as_ImageCacheBase_mut()) }
    fn getHashtableP(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> { ImageCacheBase::getHashtableP(self.as_ImageCacheBase_mut()) }
    fn init(&mut self, mut image: Option<Rc<RefCell<Image>>>) { ImageCacheBase::init(self.as_ImageCacheBase_mut(), image) }
    fn initProgress(&mut self) { ImageCacheBase::initProgress(self.as_ImageCacheBase_mut()) }
}
impl ImageCacheBaseDyn for ImageCacheBase {
    fn as_ImageCacheBase(&self) -> &ImageCacheBase { self }
    fn as_ImageCacheBase_mut(&mut self) -> &mut ImageCacheBase { self }
}


