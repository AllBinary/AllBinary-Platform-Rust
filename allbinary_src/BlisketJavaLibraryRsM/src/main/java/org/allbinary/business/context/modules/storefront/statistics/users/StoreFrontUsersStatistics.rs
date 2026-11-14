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
// package org.allbinary.business.context.modules.storefront.statistics.users
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::util::HashMap::HashMap;
use crate::java::util::Set::Set;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::user::UserInterface::UserInterface;
use crate::org::allbinary::business::user::role::UserRole::UserRole;
use crate::org::allbinary::data::tables::user::UserEntityFactory::UserEntityFactory;
use crate::org::allbinary::data::tables::user::UserEntityInterface::UserEntityInterface;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::java::lang::Exception::*;
use super::StoreFrontUsersStatisticsInterface::StoreFrontUsersStatisticsInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StoreFrontUsersStatistics {
    pub totalNumberOfUsers: Option<i64>,
    pub totalUsersByRoleHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>,
}

impl StoreFrontUsersStatisticsInterface for StoreFrontUsersStatistics {
    fn getNumberOfUsers(&mut self) -> Option<i64> { Self::getNumberOfUsers(self) }
    fn getNumberOfUsersByRole(&mut self, mut role: String) -> Option<i64> { Self::getNumberOfUsersByRole(self, role) }
}
impl crate::org::allbinary::data::tables::TableMappingInterface::TableMappingInterface for StoreFrontUsersStatistics {
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getKey(self) }
    fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::toVector(self) }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> { Self::toHashMap(self) }
}

impl StoreFrontUsersStatistics {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct(mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Self {
        let mut this = Self {
            totalNumberOfUsers: None,
            totalUsersByRoleHashMap: None,
        };
        this.totalUsersByRoleHashMap = { let java_value = (jcall!(StdUtil::getInstance()).createHashMap()).clone(); java_value };
        let mut userEntityInterface: Option<Rc<RefCell<dyn UserEntityInterface>>> = UserEntityFactory::getInstance().map(|reference| reference as Rc<RefCell<dyn UserEntityInterface>>);
        let mut userVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(userEntityInterface).getCustomers();
        this.totalNumberOfUsers = { let java_value = (Some((jcall!(userVector).size()) as i64)).clone(); java_value };
        let mut size: i32 = jcall!(userVector).size();
        for index in (0..size) {
            let mut userInterface: Option<Rc<RefCell<dyn UserInterface>>> = { let java_object = (jcall!(userVector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn UserInterface>>>>().expect("ClassCastException").clone()) };
            let mut nextUserRole: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>> = jcall!(userInterface).getRole();
            let mut currentNumberOfUsersForRole: Option<i64> = this.getNewTotal(nextUserRole.clone());
            jcall!(this.totalUsersByRoleHashMap).put(crate::java_into_object(nextUserRole.clone()), crate::java_into_object((currentNumberOfUsersForRole.clone()).expect("NullPointerException")));
        }
        this
    }

    pub fn new(mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(storeFrontInterface))))
    }
    

    pub fn getNewTotal(&mut self, mut userRole: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>) -> Option<i64> {
        let mut numberOfUsersForRoleLong: Option<i64> = { let java_object = (jcall!(self.totalUsersByRoleHashMap).get(crate::java_into_object(userRole.clone()))).clone(); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<i64>().copied().or_else(|| object.downcast_ref::<Option<i64>>().copied().flatten()).expect("ClassCastException") }) };
        if numberOfUsersForRoleLong.is_none() {
            numberOfUsersForRoleLong = Some((0) as i64);
        }
        return Some((((numberOfUsersForRoleLong).unwrap() as i64) + 1_i64) as i64);
    }
    

    pub fn getNumberOfUsers(&mut self) -> Option<i64> {
        return self.totalNumberOfUsers.clone();
    }
    

    pub fn getNumberOfUsersByRole(&mut self, mut role: String) -> Option<i64> {
        let mut totalForRole: Option<i64> = { let java_object = (jcall!(self.totalUsersByRoleHashMap).get(crate::java_into_object(role.clone()))).clone(); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<i64>().copied().or_else(|| object.downcast_ref::<Option<i64>>().copied().flatten()).expect("ClassCastException") }) };
        return totalForRole.clone();
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(crate::java_into_object({ let java_field = jcall!(StoreFrontUsersStatisticsData::StoreFrontUsersStatisticsData::getInstance()).NUMBEROFUSERS.clone(); java_field }.clone()), { let java_argument = crate::java_into_object((self.getNumberOfUsers()).expect("NullPointerException").to_string()); java_argument });
        let mut setOfUserRoles: Option<Rc<RefCell<dyn Set>>> = jcall!(self.totalUsersByRoleHashMap).keySet();
        let mut nextUserRole: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>> = None;
        let mut userRoleArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(setOfUserRoles).toArray();
        let mut size: i32 = (userRoleArray.len() as i32).clone();
        for index in (0..size) {
            nextUserRole = { let java_object = (userRoleArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>>().expect("ClassCastException").clone()) };
            let mut totalForRole: Option<i64> = { let java_object = (jcall!(self.totalUsersByRoleHashMap).get(crate::java_into_object(nextUserRole.clone()))).clone(); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<i64>().copied().or_else(|| object.downcast_ref::<Option<i64>>().copied().flatten()).expect("ClassCastException") }) };
            jcall!(hashMap).put(crate::java_into_object(jcall!(nextUserRole).toString()), crate::java_into_object((totalForRole).expect("NullPointerException").to_string()));
        }
        return hashMap.clone();
    }
    

    pub fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return None;
    }
    

    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return None;
    }
}

