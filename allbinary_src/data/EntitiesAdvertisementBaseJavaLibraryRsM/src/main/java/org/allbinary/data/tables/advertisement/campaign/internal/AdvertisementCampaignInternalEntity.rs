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
// package org.allbinary.data.tables.advertisement.campaign.internal
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::advertisement::campaign::AdvertisementCampaign::AdvertisementCampaign;
use crate::org::allbinary::business::advertisement::campaign::AdvertisementCampaignData::AdvertisementCampaignData;
use crate::org::allbinary::business::advertisement::campaign::AdvertisementCampaignInterface::AdvertisementCampaignInterface;
use crate::org::allbinary::business::advertisement::campaign::AdvertisementCampaigns::AdvertisementCampaigns;
use crate::org::allbinary::business::advertisement::campaign::AdvertisementCampaignsInterface::AdvertisementCampaignsInterface;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::init::db::UserDbInitInfo::UserDbInitInfo;
use crate::org::allbinary::business::time::TimeData::TimeData;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use super::AdvertisementCampaignInternalEntityInterface::AdvertisementCampaignInternalEntityInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AdvertisementCampaignInternalEntity {
    pub base: AbSqlBean,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub tableName: String,
}

// impl AdvertisementCampaignInternalEntityInterface for AdvertisementCampaignInternalEntity with the methods of the trait

impl AdvertisementCampaignInternalEntity {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AbSqlBean::new_struct(UserDbInitInfo::new()),
            logUtil: None,
            tableName: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.tableName = String::from("adCampaignInternal");
        // super(...) is in the struct literal above
        this.setTableName(this.tableName.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn delete(&mut self, mut value: String) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.base.deleteWhere2(jcall!(StoreFrontData::getInstance()).NAME.clone(), value.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).putF(jcall!(self.commonStrings).SUCCESS.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).delete.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).delete.clone(), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    pub fn getCampaignsInStore(&mut self, mut storeName: String) -> Option<Rc<RefCell<dyn AdvertisementCampaignsInterface>>> {
        let mut keysAndValues: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(keysAndValues).put(jcall!(StoreFrontData::getInstance()).NAME.clone(), storeName.clone());
        let mut hashMapVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.base.getRows(keysAndValues.clone());
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut size: i32 = jcall!(hashMapVector).size();
        for index in (0..size) {
            let mut hashMap: Option<Rc<RefCell<HashMap>>> = (jcall!(hashMapVector).get(index.clone())).clone() /* as HashMap */;
            if hashMap.is_some() {
                jcall!(vector).add(crate::java_into_object((AdvertisementCampaign::new(hashMap.clone())).clone() /* as AdvertisementCampaignInterface */));
            }
        }
        return (AdvertisementCampaigns::new(vector.clone())).clone() /* as AdvertisementCampaignsInterface */;
    }
    

    pub fn get(&mut self, mut storeName: String, mut name: String) -> Option<Rc<RefCell<dyn AdvertisementCampaignInterface>>> {
        let mut keysAndValues: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(keysAndValues).put(jcall!(StoreFrontData::getInstance()).NAME.clone(), storeName.clone());
        jcall!(keysAndValues).put(jcall!(AdvertisementCampaignData::getInstance()).NAME.clone(), name.clone());
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = self.base.getRow(keysAndValues.clone());
        if hashMap.is_some() {
            return (AdvertisementCampaign::new(hashMap.clone())).clone() /* as AdvertisementCampaignInterface */;
        } else {
            return None;
        }
    }
    

    pub fn update(&mut self, mut updatedValues: Option<Rc<RefCell<HashMap>>>) {
        self.base.updateWhere3(jcall!(jcall!(EntryData::getInstance()).getInstance()).ID.clone(), (jcall!(updatedValues).get(jcall!(jcall!(EntryData::getInstance()).getInstance()).ID.clone())).clone() /* as String */, updatedValues.clone());
    }
    

    pub fn createTableStatement(&mut self) -> String {
        let mut entryData: Option<Rc<RefCell<EntryData>>> = jcall!(EntryData::getInstance()).getInstance();
        let mut advertisementCampaignData: Option<Rc<RefCell<AdvertisementCampaignData>>> = AdvertisementCampaignData::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).CREATE_TABLE.clone());
        jcall!(stringBuffer).append(self.getTableName());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).START.clone());
        jcall!(stringBuffer).append(jcall!(entryData).ID.clone()).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone()).append(jcall!(StoreFrontData::getInstance()).NAME.clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(advertisementCampaignData).NAME.clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(advertisementCampaignData).DESCRIPTION.clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(jcall!(advertisementCampaignData).CONFIG.clone()).append(jcall!(self.sqlTypeStrings).BLOB_NOT_NULL.clone()).append(jcall!(TimeData::getInstance()).START.clone()).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone()).append(jcall!(TimeData::getInstance()).END.clone()).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone()).append(jcall!(entryData).TIMECREATED.clone()).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone()).append(jcall!(entryData).LASTMODIFIED.clone()).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone()).append(jcall!(self.sqlStrings).PRIMARY_KEY.clone()).append(jcall!(entryData).ID.clone()).append(jcall!(self.sqlStrings).END.clone());
        return jcall!(stringBuffer).toString();
    }
    

    pub fn createTable(&mut self) -> String {
        let mut returnStr: String = self.base.createTable(self.createTableStatement());
        return returnStr.clone();
    }
    

    pub fn dropTable(&mut self) -> String {
        return self.base.dropTable();
    }
}
impl std::ops::Deref for AdvertisementCampaignInternalEntity {
    type Target = AbSqlBean;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AdvertisementCampaignInternalEntity {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasicDyn for AdvertisementCampaignInternalEntity {
    fn as_AbSqlBasic(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &self.base.base.base.base.base }
    fn as_AbSqlBasic_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &mut self.base.base.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBeanDyn for AdvertisementCampaignInternalEntity {
    fn as_AbSqlBean(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &self.base }
    fn as_AbSqlBean_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &mut self.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumnDyn for AdvertisementCampaignInternalEntity {
    fn as_AbSqlColumn(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &self.base.base.base }
    fn as_AbSqlColumn_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &mut self.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRowDyn for AdvertisementCampaignInternalEntity {
    fn as_AbSqlRow(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &self.base.base }
    fn as_AbSqlRow_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &mut self.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTableDyn for AdvertisementCampaignInternalEntity {
    fn as_AbSqlTable(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &self.base.base.base.base }
    fn as_AbSqlTable_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &mut self.base.base.base.base }
    fn dropTable(&mut self) -> String { Self::dropTable(self) }
}


