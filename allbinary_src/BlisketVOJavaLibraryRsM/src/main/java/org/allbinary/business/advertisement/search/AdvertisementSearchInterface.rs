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
// package org.allbinary.business.advertisement.search
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::business::advertisement::search::category::AdvertisementCategoryInterface::AdvertisementCategoryInterface;
use crate::org::allbinary::business::advertisement::search::website::AdvertisementWebsiteInterface::AdvertisementWebsiteInterface;
use crate::org::allbinary::logic::control::contraints::price::PriceConstraintInterface::PriceConstraintInterface;
use crate::org::allbinary::logic::control::contraints::size::TwoDimensionalConstraintInterface::TwoDimensionalConstraintInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait AdvertisementSearchInterface {
    

    fn getWebSite(&mut self) -> Option<Rc<RefCell<dyn AdvertisementWebsiteInterface>>>;
    

    fn getCategory(&mut self) -> Option<Rc<RefCell<dyn AdvertisementCategoryInterface>>>;
    

    fn getSizeConstraints(&mut self) -> Option<Rc<RefCell<dyn TwoDimensionalConstraintInterface>>>;
    

    fn getPricingConstraints(&mut self) -> Option<Rc<RefCell<dyn PriceConstraintInterface>>>;
    

    fn getPageNumber(&mut self) -> i32;
    

    fn getProductsPerPage(&mut self) -> i32;
    

    fn getSearchParams(&mut self) -> Option<Rc<RefCell<HashMap>>>;
    

    fn setCategory(&mut self, advertisementCategoryInterface: Option<Rc<RefCell<dyn AdvertisementCategoryInterface>>>);
    

    fn setSizeConstraints(&mut self, twoDimensionConstraintsInterface: Option<Rc<RefCell<dyn TwoDimensionalConstraintInterface>>>);
    

    fn setPricingConstraints(&mut self, priceConstraintInterface: Option<Rc<RefCell<dyn PriceConstraintInterface>>>);
    

    fn setPageNumber(&mut self, pageNumber: i32);
    

    fn setProductsPerPage(&mut self, productsPerPage: i32);
    

    fn setSearchParams(&mut self, columnAndKeywordHashMap: Option<Rc<RefCell<HashMap>>>);

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


