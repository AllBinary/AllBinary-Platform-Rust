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
// package org.allbinary.business.user.commerce.inventory.item
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::user::commerce::money::Money::Money;
use crate::org::allbinary::data::tables::TableMappingInterface::TableMappingInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait ItemInterface: TableMappingInterface {
    

    fn setId(&mut self, itemId: String);
    

    fn setNumber(&mut self, number: String);
    

    fn setInBaskets(&mut self, value: String);
    

    fn setWeight(&mut self, value: String);
    

    fn setEnabled(&mut self, value: String);
    

    fn setNewOrUsed(&mut self, value: String);
    

    fn setSummary(&mut self, value: String);
    

    fn setDistributor(&mut self, value: String);
    

    fn setIdUsedByDistributor(&mut self, value: String);
    

    fn setProducedBy(&mut self, value: String);
    

    fn setProductionDate(&mut self, value: String);
    

    fn setStartProductionDate(&mut self, value: String);
    

    fn setDescription(&mut self, value: String);
    

    fn setKeywords(&mut self, value: String);
    

    fn setCategory(&mut self, value: String);
    

    fn setType(&mut self, value: String);
    

    fn setSmallImage(&mut self, value: String);
    

    fn setMediumImage(&mut self, value: String);
    

    fn setLargeImage(&mut self, value: String);
    

    fn setTimeEntered(&mut self, value: String);
    

    fn setLastModified(&mut self, value: String);
    

    fn setPrice(&mut self, value: Option<Rc<RefCell<Money>>>);
    

    fn setComment(&mut self, value: String);
    

    fn setCustoms(&mut self, value: String);
    

    fn setDownloads(&mut self, value: String);
    

    fn setGroups(&mut self, value: String);
    

    fn setOptions(&mut self, value: String);
    

    fn setPermissions(&mut self, value: String);
    

    fn setSpecials(&mut self, value: String);
    

    fn getId(&mut self) -> String;
    

    fn getNumber(&mut self) -> String;
    

    fn getInBaskets(&mut self) -> String;
    

    fn getWeight(&mut self) -> String;
    

    fn getEnabled(&mut self) -> String;
    

    fn getNewOrUsed(&mut self) -> String;
    

    fn getSummary(&mut self) -> String;
    

    fn getDistributor(&mut self) -> String;
    

    fn getIdUsedByDistributor(&mut self) -> String;
    

    fn getProducedBy(&mut self) -> String;
    

    fn getProductionDate(&mut self) -> String;
    

    fn getStartProductionDate(&mut self) -> String;
    

    fn getDescription(&mut self) -> String;
    

    fn getKeywords(&mut self) -> String;
    

    fn getCategory(&mut self) -> String;
    

    fn getType(&mut self) -> String;
    

    fn getSmallImage(&mut self) -> String;
    

    fn getMediumImage(&mut self) -> String;
    

    fn getLargeImage(&mut self) -> String;
    

    fn getTimeEntered(&mut self) -> String;
    

    fn getLastModified(&mut self) -> String;
    

    fn getPrice(&mut self) -> Option<Rc<RefCell<Money>>>;
    

    fn getComment(&mut self) -> String;
    

    fn getCustoms(&mut self) -> String;
    

    fn getDownloads(&mut self) -> String;
    

    fn getGroups(&mut self) -> String;
    

    fn getOptions(&mut self) -> String;
    

    fn getPermissions(&mut self) -> String;
    

    fn getSpecials(&mut self) -> String;
    

    fn getTotal(&mut self) -> Option<Rc<RefCell<Money>>>;
}


