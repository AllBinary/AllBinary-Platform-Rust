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
// package org.allbinary.business.advertisement
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::rand::MyRandomFactory::MyRandomFactory;
use crate::org::allbinary::logic::system::security::licensing::LockedUtil::LockedUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::AdvertisementProcessorInterface::AdvertisementProcessorInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Advertisements {
    pub bannerList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub pageList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub myRandomFactory: Option<Rc<RefCell<MyRandomFactory>>>,
}


impl Advertisements {

    pub fn new_struct() -> Self {
        let mut this = Self {
            bannerList: None,
            pageList: None,
            myRandomFactory: None,
        };
        this.bannerList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.pageList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.myRandomFactory = MyRandomFactory::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn addPage(&mut self, mut advertisementProcessorInterface: Option<Rc<RefCell<dyn AdvertisementProcessorInterface>>>) {
        if !jcall!(self.pageList).contains(crate::java_into_object(advertisementProcessorInterface.clone())) {
            jcall!(self.pageList).add(crate::java_into_object(advertisementProcessorInterface.clone()));
        }
    }
    

    pub fn startRandomPage(&mut self) {
        let mut size: i32 = jcall!(self.pageList).size();
        if size > 0 {
            let mut random: i32 = jcall!(self.myRandomFactory).getAbsoluteNextInt(size.clone());
            let mut advertisementProcessorInterface: Option<Rc<RefCell<dyn AdvertisementProcessorInterface>>> = { let java_object = (jcall!(self.pageList).as_BasicArrayList_mut().objectArray[(random) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::business::advertisement::AdvertisementProcessorInterface::AdvertisementProcessorInterfaceDyn>>>>().expect("ClassCastException").clone()) };
            jcall!(advertisementProcessorInterface).start();
        }
    }
    

    pub fn processRandomPage(&mut self) {
        if jcall!(LockedUtil::getInstance()).isLocked() {
            let mut size: i32 = jcall!(self.pageList).size();
            if size > 0 {
                let mut random: i32 = jcall!(self.myRandomFactory).getAbsoluteNextInt(size.clone());
                let mut advertisementProcessorInterface: Option<Rc<RefCell<dyn AdvertisementProcessorInterface>>> = { let java_object = (jcall!(self.pageList).as_BasicArrayList_mut().objectArray[(random) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::business::advertisement::AdvertisementProcessorInterface::AdvertisementProcessorInterfaceDyn>>>>().expect("ClassCastException").clone()) };
                jcall!(advertisementProcessorInterface).process();
            }
        }
    }
    

    pub fn stopAllPage(&mut self) {
        {
            let mut index: i32 = jcall!(self.pageList).size();
            while { index -= 1; index } >= 0 {
            let mut advertisementProcessorInterface: Option<Rc<RefCell<dyn AdvertisementProcessorInterface>>> = { let java_object = (jcall!(self.pageList).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::business::advertisement::AdvertisementProcessorInterface::AdvertisementProcessorInterfaceDyn>>>>().expect("ClassCastException").clone()) };
            jcall!(advertisementProcessorInterface).stop();
            }
        }
    }
    

    pub fn addBanner(&mut self, mut advertisementProcessorInterface: Option<Rc<RefCell<dyn AdvertisementProcessorInterface>>>) {
        if !jcall!(self.bannerList).contains(crate::java_into_object(advertisementProcessorInterface.clone())) {
            jcall!(self.bannerList).add(crate::java_into_object(advertisementProcessorInterface.clone()));
        }
    }
    

    pub fn startAll(&mut self) {
        {
            let mut index: i32 = jcall!(self.bannerList).size();
            while { index -= 1; index } >= 0 {
            let mut advertisementProcessorInterface: Option<Rc<RefCell<dyn AdvertisementProcessorInterface>>> = { let java_object = (jcall!(self.bannerList).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::business::advertisement::AdvertisementProcessorInterface::AdvertisementProcessorInterfaceDyn>>>>().expect("ClassCastException").clone()) };
            jcall!(advertisementProcessorInterface).start();
            }
        }
    }
    

    pub fn stopAllBanner(&mut self) {
        {
            let mut index: i32 = jcall!(self.bannerList).size();
            while { index -= 1; index } >= 0 {
            let mut advertisementProcessorInterface: Option<Rc<RefCell<dyn AdvertisementProcessorInterface>>> = { let java_object = (jcall!(self.bannerList).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::business::advertisement::AdvertisementProcessorInterface::AdvertisementProcessorInterfaceDyn>>>>().expect("ClassCastException").clone()) };
            jcall!(advertisementProcessorInterface).stop();
            }
        }
    }
    

    pub fn getBanner(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn AdvertisementProcessorInterface>>> {
        let mut advertisementProcessorInterface: Option<Rc<RefCell<dyn AdvertisementProcessorInterface>>> = { let java_object = (jcall!(self.bannerList).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::business::advertisement::AdvertisementProcessorInterface::AdvertisementProcessorInterfaceDyn>>>>().expect("ClassCastException").clone()) };
        return advertisementProcessorInterface.clone();
    }
    

    pub fn stopAll(&mut self) {
        self.stopAllBanner();
        self.stopAllPage();
    }
}

