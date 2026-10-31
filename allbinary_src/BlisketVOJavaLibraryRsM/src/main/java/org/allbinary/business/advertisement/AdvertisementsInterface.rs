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
use super::AdvertisementInterface::AdvertisementInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait AdvertisementsInterface {
    

    fn add(&mut self, advertisementInterface: Option<Rc<RefCell<dyn AdvertisementInterface>>>);
    

    fn get(&mut self, index: i32) -> Option<Rc<RefCell<dyn AdvertisementInterface>>>;
    

    fn getCurrentPageNum(&mut self) -> i32;
    

    fn getErrorDescription(&mut self) -> String;
    

    fn getErrorNum(&mut self) -> String;
    

    fn getNumberOfPosters(&mut self) -> i32;
    

    fn getNumberOfProductsInPage(&mut self) -> i32;
    

    fn getStatusCode(&mut self) -> i32;
    

    fn getTotalNumberOfProducts(&mut self) -> i32;
    

    fn isSearchValid(&mut self) -> bool;
    

    fn setCurrentPageNum(&mut self, currentPageNum: i32);
    

    fn setErrorDescription(&mut self, errorDescription: String);
    

    fn setErrorNum(&mut self, errorNum: String);
    

    fn setNumberOfProductsInPage(&mut self, numberOfProductsInPage: i32);
    

    fn setStatusCode(&mut self, statusCode: i32);
    

    fn setTotalNumberOfProducts(&mut self, totalNumberOfProducts: i32);
    

    fn toString(&mut self) -> String;
}


