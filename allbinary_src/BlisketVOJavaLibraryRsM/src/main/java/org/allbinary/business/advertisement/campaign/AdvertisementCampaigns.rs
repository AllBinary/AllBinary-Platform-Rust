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
// package org.allbinary.business.advertisement.campaign
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::advertisement::AdvertisementInterface::AdvertisementInterface;
use crate::org::allbinary::business::advertisement::AdvertisementsInterface::AdvertisementsInterface;
use crate::org::allbinary::business::advertisement::search::AdvertisementSearchInterface::AdvertisementSearchInterface;
use super::AdvertisementCampaignsInterface::AdvertisementCampaignsInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AdvertisementCampaigns {
    pub advertisementsVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

impl AdvertisementCampaignsInterface for AdvertisementCampaigns {
    fn add(&mut self, mut advertisementInterface: Option<Rc<RefCell<dyn AdvertisementInterface>>>) { Self::add(self, advertisementInterface) }
    fn get(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn AdvertisementInterface>>> { Self::get(self, index) }
    fn size(&mut self) -> i32 { Self::size(self) }
    fn isEnabled(&mut self) -> bool { Self::isEnabled(self) }
}

impl AdvertisementCampaigns {
    

    pub fn new_struct(mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Self {
        let mut this = Self {
            advertisementsVector: None,
        };
        this.advertisementsVector = this.advertisementsVector.clone();
        this
    }

    pub fn new(mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(vector))))
    }
    

    pub fn add(&mut self, mut advertisementInterface: Option<Rc<RefCell<dyn AdvertisementInterface>>>) {
        jcall!(self.advertisementsVector).add(crate::java_into_object(advertisementInterface.clone()));
    }
    

    pub fn get(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn AdvertisementInterface>>> {
        return { let java_object = jcall!(self.advertisementsVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn AdvertisementInterface>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn size(&mut self) -> i32 {
        return jcall!(self.advertisementsVector).size();
    }
    

    pub fn isEnabled(&mut self) -> bool {
        return true;
    }
    

    pub fn search(&mut self, mut advertisementSearchInterface: Option<Rc<RefCell<dyn AdvertisementSearchInterface>>>) -> Option<Rc<RefCell<dyn AdvertisementsInterface>>> {
        return None;
    }
}

