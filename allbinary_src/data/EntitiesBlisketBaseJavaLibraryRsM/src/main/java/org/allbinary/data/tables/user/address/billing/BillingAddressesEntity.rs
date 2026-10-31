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
// package org.allbinary.data.tables.user.address.billing
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::data::tables::user::address::StreetAddressesEntity::StreetAddressesEntity;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BillingAddressesEntity {
    pub base: StreetAddressesEntity,
    pub TABLENAME: String,
}


impl BillingAddressesEntity {
    

    pub fn new_struct(mut userName: String) -> Self {
        let mut this = Self {
            base: StreetAddressesEntity::new_struct1(userName.clone()),
            TABLENAME: String::new(),
        };
        this.TABLENAME = String::from("billing");
        // super(...) is in the struct literal above
        this.setTableName(this.TABLENAME.clone());
        this
    }

    pub fn new(mut userName: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(userName))))
    }
}
impl std::ops::Deref for BillingAddressesEntity {
    type Target = StreetAddressesEntity;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BillingAddressesEntity {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::data::tables::user::address::StreetAddressesEntity::StreetAddressesEntityDyn for BillingAddressesEntity {
    fn as_StreetAddressesEntity(&self) -> &crate::org::allbinary::data::tables::user::address::StreetAddressesEntity::StreetAddressesEntity { &self.base }
    fn as_StreetAddressesEntity_mut(&mut self) -> &mut crate::org::allbinary::data::tables::user::address::StreetAddressesEntity::StreetAddressesEntity { &mut self.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasicDyn for BillingAddressesEntity {
    fn as_AbSqlBasic(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &self.base.base.base.base.base.base }
    fn as_AbSqlBasic_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &mut self.base.base.base.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBeanDyn for BillingAddressesEntity {
    fn as_AbSqlBean(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &self.base.base }
    fn as_AbSqlBean_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &mut self.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumnDyn for BillingAddressesEntity {
    fn as_AbSqlColumn(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &self.base.base.base.base }
    fn as_AbSqlColumn_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &mut self.base.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRowDyn for BillingAddressesEntity {
    fn as_AbSqlRow(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &self.base.base.base }
    fn as_AbSqlRow_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &mut self.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTableDyn for BillingAddressesEntity {
    fn as_AbSqlTable(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &self.base.base.base.base.base }
    fn as_AbSqlTable_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &mut self.base.base.base.base.base }
}


