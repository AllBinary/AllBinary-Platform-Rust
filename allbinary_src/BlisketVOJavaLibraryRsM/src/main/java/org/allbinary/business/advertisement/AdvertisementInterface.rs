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
use crate::org::allbinary::business::advertisement::image::AdvertisementImageInterface::AdvertisementImageInterface;
use crate::org::allbinary::business::advertisement::product::AdvertisementProductInterface::AdvertisementProductInterface;
use crate::org::allbinary::business::advertisement::thumbnail::AdvertisementThumbnailInterface::AdvertisementThumbnailInterface;
use crate::org::allbinary::business::user::commerce::money::Money::Money;
use crate::org::allbinary::logic::control::contraints::display::browser::DisplayInBrowserContraintsInterface::DisplayInBrowserContraintsInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait AdvertisementInterface {
    

    fn getArtist(&mut self) -> String;
    

    fn getArtistLink(&mut self) -> String;
    

    fn getTitle(&mut self) -> String;
    

    fn getDateAdded(&mut self) -> String;
    

    fn getDirectory(&mut self) -> String;
    

    fn getFramedLink(&mut self) -> String;
    

    fn getListPrice(&mut self) -> Option<Rc<RefCell<Money>>>;
    

    fn getOurPrice(&mut self) -> Option<Rc<RefCell<Money>>>;
    

    fn getRank(&mut self) -> i32;
    

    fn getImage(&mut self) -> Option<Rc<RefCell<dyn AdvertisementImageInterface>>>;
    

    fn getProduct(&mut self) -> Option<Rc<RefCell<dyn AdvertisementProductInterface>>>;
    

    fn getThumbnail(&mut self) -> Option<Rc<RefCell<dyn AdvertisementThumbnailInterface>>>;
    

    fn getDisplayInBrowserContraintsInterface(&mut self) -> Option<Rc<RefCell<dyn DisplayInBrowserContraintsInterface>>>;
    

    fn setArtist(&mut self, artist: String);
    

    fn setArtistLink(&mut self, artistLink: String);
    

    fn setTitle(&mut self, title: String);
    

    fn setDateAdded(&mut self, dateAdded: String);
    

    fn setDirectory(&mut self, directory: String);
    

    fn setFramedLink(&mut self, framedLink: String);
    

    fn setRank(&mut self, rank: i32);
    

    fn setImage(&mut self, AdvertisementImageInterface: Option<Rc<RefCell<dyn AdvertisementImageInterface>>>);
    

    fn setProduct(&mut self, AdvertisementProductInterface: Option<Rc<RefCell<dyn AdvertisementProductInterface>>>);
    

    fn setThumbnail(&mut self, AdvertisementThumbnailInterface: Option<Rc<RefCell<dyn AdvertisementThumbnailInterface>>>);

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


