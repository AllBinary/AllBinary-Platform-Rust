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
use super::AdvertisementInterface::AdvertisementInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Advertisement {
    pub artist: String,
    pub artistLink: String,
    pub title: String,
    pub dateAdded: String,
    pub directory: String,
    pub framedLink: String,
    pub rank: i32,
    pub listPrice: Option<Rc<RefCell<Money>>>,
    pub ourPrice: Option<Rc<RefCell<Money>>>,
    pub image: Option<Rc<RefCell<dyn AdvertisementImageInterface>>>,
    pub product: Option<Rc<RefCell<dyn AdvertisementProductInterface>>>,
    pub thumbnail: Option<Rc<RefCell<dyn AdvertisementThumbnailInterface>>>,
    pub displayInBrowserContraintsInterface: Option<Rc<RefCell<dyn DisplayInBrowserContraintsInterface>>>,
}

impl AdvertisementInterface for Advertisement {
    fn getArtist(&mut self) -> String { Self::getArtist(self) }
    fn getArtistLink(&mut self) -> String { Self::getArtistLink(self) }
    fn getTitle(&mut self) -> String { Self::getTitle(self) }
    fn getDateAdded(&mut self) -> String { Self::getDateAdded(self) }
    fn getDirectory(&mut self) -> String { Self::getDirectory(self) }
    fn getFramedLink(&mut self) -> String { Self::getFramedLink(self) }
    fn getListPrice(&mut self) -> Option<Rc<RefCell<Money>>> { Self::getListPrice(self) }
    fn getOurPrice(&mut self) -> Option<Rc<RefCell<Money>>> { Self::getOurPrice(self) }
    fn getRank(&mut self) -> i32 { Self::getRank(self) }
    fn getImage(&mut self) -> Option<Rc<RefCell<dyn AdvertisementImageInterface>>> { Self::getImage(self) }
    fn getProduct(&mut self) -> Option<Rc<RefCell<dyn AdvertisementProductInterface>>> { Self::getProduct(self) }
    fn getThumbnail(&mut self) -> Option<Rc<RefCell<dyn AdvertisementThumbnailInterface>>> { Self::getThumbnail(self) }
    fn getDisplayInBrowserContraintsInterface(&mut self) -> Option<Rc<RefCell<dyn DisplayInBrowserContraintsInterface>>> { Self::getDisplayInBrowserContraintsInterface(self) }
    fn setArtist(&mut self, mut artist: String) { Self::setArtist(self, artist) }
    fn setArtistLink(&mut self, mut artistLink: String) { Self::setArtistLink(self, artistLink) }
    fn setTitle(&mut self, mut title: String) { Self::setTitle(self, title) }
    fn setDateAdded(&mut self, mut dateAdded: String) { Self::setDateAdded(self, dateAdded) }
    fn setDirectory(&mut self, mut directory: String) { Self::setDirectory(self, directory) }
    fn setFramedLink(&mut self, mut framedLink: String) { Self::setFramedLink(self, framedLink) }
    fn setRank(&mut self, mut rank: i32) { Self::setRank(self, rank) }
    fn setImage(&mut self, mut image: Option<Rc<RefCell<dyn AdvertisementImageInterface>>>) { Self::setImage(self, image) }
    fn setProduct(&mut self, mut product: Option<Rc<RefCell<dyn AdvertisementProductInterface>>>) { Self::setProduct(self, product) }
    fn setThumbnail(&mut self, mut thumbnail: Option<Rc<RefCell<dyn AdvertisementThumbnailInterface>>>) { Self::setThumbnail(self, thumbnail) }
}

impl Advertisement {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            artist: String::new(),
            artistLink: String::new(),
            title: String::new(),
            dateAdded: String::new(),
            directory: String::new(),
            framedLink: String::new(),
            rank: 0,
            listPrice: None,
            ourPrice: None,
            image: None,
            product: None,
            thumbnail: None,
            displayInBrowserContraintsInterface: None,
        };
        this.displayInBrowserContraintsInterface = None;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getArtist(&mut self) -> String {
        return self.artist.clone();
    }
    

    pub fn setArtist(&mut self, mut artist: String) {
        self.artist = artist.clone();
    }
    

    pub fn getArtistLink(&mut self) -> String {
        return self.artistLink.clone();
    }
    

    pub fn setArtistLink(&mut self, mut artistLink: String) {
        self.artistLink = artistLink.clone();
    }
    

    pub fn getTitle(&mut self) -> String {
        return self.title.clone();
    }
    

    pub fn setTitle(&mut self, mut title: String) {
        self.title = title.clone();
    }
    

    pub fn getDateAdded(&mut self) -> String {
        return self.dateAdded.clone();
    }
    

    pub fn setDateAdded(&mut self, mut dateAdded: String) {
        self.dateAdded = dateAdded.clone();
    }
    

    pub fn getDirectory(&mut self) -> String {
        return self.directory.clone();
    }
    

    pub fn setDirectory(&mut self, mut directory: String) {
        self.directory = directory.clone();
    }
    

    pub fn getFramedLink(&mut self) -> String {
        return self.framedLink.clone();
    }
    

    pub fn setFramedLink(&mut self, mut framedLink: String) {
        self.framedLink = framedLink.clone();
    }
    

    pub fn getListPrice(&mut self) -> Option<Rc<RefCell<Money>>> {
        return self.listPrice.clone();
    }
    

    pub fn setListPrice(&mut self, mut listPrice: Option<Rc<RefCell<Money>>>) {
        self.listPrice = listPrice.clone();
    }
    

    pub fn getOurPrice(&mut self) -> Option<Rc<RefCell<Money>>> {
        return self.ourPrice.clone();
    }
    

    pub fn setOurPrice(&mut self, mut ourPrice: Option<Rc<RefCell<Money>>>) {
        self.ourPrice = ourPrice.clone();
    }
    

    pub fn getRank(&mut self) -> i32 {
        return self.rank.clone();
    }
    

    pub fn setRank(&mut self, mut rank: i32) {
        self.rank = rank.clone();
    }
    

    pub fn getImage(&mut self) -> Option<Rc<RefCell<dyn AdvertisementImageInterface>>> {
        return self.image.clone();
    }
    

    pub fn setImage(&mut self, mut image: Option<Rc<RefCell<dyn AdvertisementImageInterface>>>) {
        self.image = image.clone();
    }
    

    pub fn getProduct(&mut self) -> Option<Rc<RefCell<dyn AdvertisementProductInterface>>> {
        return self.product.clone();
    }
    

    pub fn setProduct(&mut self, mut product: Option<Rc<RefCell<dyn AdvertisementProductInterface>>>) {
        self.product = product.clone();
    }
    

    pub fn getThumbnail(&mut self) -> Option<Rc<RefCell<dyn AdvertisementThumbnailInterface>>> {
        return self.thumbnail.clone();
    }
    

    pub fn setThumbnail(&mut self, mut thumbnail: Option<Rc<RefCell<dyn AdvertisementThumbnailInterface>>>) {
        self.thumbnail = thumbnail.clone();
    }
    

    pub fn getDisplayInBrowserContraintsInterface(&mut self) -> Option<Rc<RefCell<dyn DisplayInBrowserContraintsInterface>>> {
        return self.displayInBrowserContraintsInterface.clone();
    }
}

