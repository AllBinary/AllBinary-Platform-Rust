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
// package org.allbinary.data.tables.transform.info
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::init::db::UserDbInitInfo::UserDbInitInfo;
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean;
use crate::org::allbinary::logic::control::crypt::Encoder::Encoder;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::ServiceClientInformationInterfaceFactory::ServiceClientInformationInterfaceFactory;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoData::TransformInfoData;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoFactoryInterface::TransformInfoFactoryInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfigAndManipulatorFactoryBase::TransformInfoObjectConfigAndManipulatorFactoryBase;
use crate::org::allbinary::logic::visual::transform::info::objectConfig::generator::TransformInfoObjectConfigGeneratorFactoryInterface::TransformInfoObjectConfigGeneratorFactoryInterface;
use super::TransformInfoEntityInterface::TransformInfoEntityInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformInfoEntity {
    pub base: AbSqlBean,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub tableName: String,
    pub abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>,
    pub transformInfoFactoryInterface: Option<Rc<RefCell<dyn TransformInfoFactoryInterface>>>,
    pub transformInfoObjectConfigAndManipulatorFactoryInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfigAndManipulatorFactoryBase::TransformInfoObjectConfigAndManipulatorFactoryBaseDyn>>>,
    pub transformInfoObjectConfigGeneratorFactoryInterface: Option<Rc<RefCell<dyn TransformInfoObjectConfigGeneratorFactoryInterface>>>,
}

impl TransformInfoEntityInterface for TransformInfoEntity {
    fn get(&mut self, mut name: String, mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<dyn TransformInfoInterface>>> { Self::get(self, name, propertiesHashMap, pageContext) }
    fn getObjectConfigs(&mut self, mut storeName: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getObjectConfigs(self, storeName) }
    fn getNames(&mut self, mut storeName: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getNames(self, storeName) }
    fn delete(&mut self, mut value: String) { Self::delete(self, value) }
    fn insert(&mut self, mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::insert(self, values) }
    fn update(&mut self, mut updatedValues: Option<Rc<RefCell<HashMap>>>) { Self::update(self, updatedValues) }
}
impl crate::org::allbinary::data::tables::BasicDataTableInterface::BasicDataTableInterface for TransformInfoEntity {
    fn createTable(&mut self) -> String { Self::createTable(self) }
    fn dropTable(&mut self) -> String { Self::dropTable(self) }
}

impl TransformInfoEntity {
    

    pub fn new_struct(mut transformInfoObjectConfigGeneratorFactoryInterface: Option<Rc<RefCell<dyn TransformInfoObjectConfigGeneratorFactoryInterface>>>, mut transformInfoObjectConfigAndManipulatorFactoryInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfigAndManipulatorFactoryBase::TransformInfoObjectConfigAndManipulatorFactoryBaseDyn>>>, mut transformInfoFactoryInterface: Option<Rc<RefCell<dyn TransformInfoFactoryInterface>>>) -> Self {
        let mut this = Self {
            base: AbSqlBean::new_struct(UserDbInitInfo::new()),
            logUtil: None,
            tableName: String::new(),
            abeClientInformation: None,
            transformInfoFactoryInterface: None,
            transformInfoObjectConfigAndManipulatorFactoryInterface: None,
            transformInfoObjectConfigGeneratorFactoryInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.tableName = String::from("transforminfo");
        this.abeClientInformation = ServiceClientInformationInterfaceFactory::getInstance();
        // super(...) is in the struct literal above
        this.setTableName(this.tableName.clone());
        this.transformInfoObjectConfigGeneratorFactoryInterface = transformInfoObjectConfigGeneratorFactoryInterface.clone();
        this.transformInfoObjectConfigAndManipulatorFactoryInterface = transformInfoObjectConfigAndManipulatorFactoryInterface.clone();
        this.transformInfoFactoryInterface = transformInfoFactoryInterface.clone();
        this
    }

    pub fn new(mut transformInfoObjectConfigGeneratorFactoryInterface: Option<Rc<RefCell<dyn TransformInfoObjectConfigGeneratorFactoryInterface>>>, mut transformInfoObjectConfigAndManipulatorFactoryInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfigAndManipulatorFactoryBase::TransformInfoObjectConfigAndManipulatorFactoryBaseDyn>>>, mut transformInfoFactoryInterface: Option<Rc<RefCell<dyn TransformInfoFactoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoObjectConfigGeneratorFactoryInterface, transformInfoObjectConfigAndManipulatorFactoryInterface, transformInfoFactoryInterface))))
    }
    

    pub fn insert(&mut self, mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.base.insert(values.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).putF(jcall!(self.commonStrings).SUCCESS.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), INSERT.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), INSERT.clone(), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    pub fn delete(&mut self, mut value: String) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.base.deleteWhere2(jcall!(TransformInfoData::getInstance()).NAME.clone(), value.clone());
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
    

    // throws Exception
    pub fn get(&mut self, mut name: String, mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<dyn TransformInfoInterface>>> {
        let mut transformInfoData: Option<Rc<RefCell<TransformInfoData>>> = TransformInfoData::getInstance();
        let mut keysAndValues: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(keysAndValues).put(jcall!(transformInfoData).NAME.clone(), name.clone());
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = self.base.getRow(keysAndValues.clone());
        if hashMap.is_some() {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(hashMap).get(jcall!(transformInfoData).OBJECTCONFIG.clone());
            if anyType.is_some() {
                let mut string: String = (anyType).clone() /* as String */;
                jcall!(hashMap).put(jcall!(transformInfoData).OBJECTCONFIG.clone(), Encoder::decode(string.clone()).clone());
            }
            let mut objectData: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(hashMap).get(jcall!(transformInfoData).DATA.clone());
            if objectData.is_some() {
                let mut string: String = (objectData).clone() /* as String */;
                jcall!(hashMap).put(jcall!(transformInfoData).DATA.clone(), Encoder::decode(string.clone()).clone());
            }
            return jcall!(self.transformInfoFactoryInterface).getInstance(hashMap.clone(), propertiesHashMap.clone(), pageContext.clone());
        } else {
            return None;
        }
    }
    

    // throws Exception
    pub fn getObjectConfigs(&mut self, mut storeName: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut objectConfigVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut objectConfigColumnVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_arguments = (jcall!(TransformInfoData::getInstance()).OBJECTCONFIG.clone(), jcall!(StoreFrontData::getInstance()).NAME.clone(), storeName.clone(),); self.getColumnWhere(java_arguments.0, java_arguments.1, java_arguments.2) };
        let mut size: i32 = jcall!(objectConfigColumnVector).size();
        for i in (0..size) {
            let mut objectConfigString: String = (jcall!(objectConfigColumnVector).get(i.clone())).clone() /* as String */;
            objectConfigString = Encoder::decode(objectConfigString.clone()).clone();
            jcall!(objectConfigVector).add(crate::java_into_object({ let java_receiver = self.transformInfoObjectConfigAndManipulatorFactoryInterface.clone(); let java_arguments = (self.abeClientInformation.clone(), (self).clone() /* as TransformInfoInterface */, DomDocumentHelper::create1(objectConfigString.clone()),); let java_result = jcall!(java_receiver).as_TransformInfoObjectConfigAndManipulatorFactoryBase_mut().getInstance3(java_arguments.0, java_arguments.1, java_arguments.2); java_result }));
        }
        return objectConfigVector.clone();
    }
    

    // throws Exception
    pub fn getNames(&mut self, mut storeName: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut viewNameVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut columnVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_arguments = (jcall!(TransformInfoData::getInstance()).NAME.clone(), jcall!(StoreFrontData::getInstance()).NAME.clone(), storeName.clone(),); self.getColumnWhere(java_arguments.0, java_arguments.1, java_arguments.2) };
        let mut size: i32 = jcall!(columnVector).size();
        for i in (0..size) {
            let mut viewNameString: String = (jcall!(columnVector).get(i.clone())).clone() /* as String */;
            jcall!(viewNameVector).add(crate::java_into_object(viewNameString.clone()));
        }
        return viewNameVector.clone();
    }
    

    pub fn createTableStatement(&mut self) -> String {
        let mut transformInfoData: Option<Rc<RefCell<TransformInfoData>>> = TransformInfoData::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).CREATE_TABLE.clone());
        jcall!(stringBuffer).append(self.tableName.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).START.clone());
        jcall!(stringBuffer).append(jcall!(transformInfoData).NAME.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(StoreFrontData::getInstance()).NAME.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(transformInfoData).OBJECTFILENAME.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(transformInfoData).OBJECT.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).BLOB_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(transformInfoData).OBJECTCONFIGFILENAME.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(transformInfoData).OBJECTCONFIG.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).BLOB_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(transformInfoData).TEMPLATEFILENAME.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(transformInfoData).TEMPLATE.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).BLOB_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(transformInfoData).DATAFILENAME.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(transformInfoData).DATA.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).BLOB_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(EntryData::getInstance()).LASTMODIFIED.clone());
        jcall!(stringBuffer).append(jcall!(EntryData::getInstance()).TIMECREATED.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).PRIMARY_KEY.clone());
        jcall!(stringBuffer).append(jcall!(transformInfoData).NAME.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).END.clone());
        return jcall!(stringBuffer).toString();
    }
    

    pub fn createTable(&mut self) -> String {
        return self.base.createTable(self.createTableStatement());
    }
    

    pub fn update(&mut self, mut updatedValues: Option<Rc<RefCell<HashMap>>>) {
        self.base.updateWhere3(jcall!(TransformInfoData::getInstance()).NAME.clone(), (jcall!(updatedValues).get(jcall!(TransformInfoData::getInstance()).NAME.clone())).clone() /* as String */, updatedValues.clone());
    }
    

    pub fn dropTable(&mut self) -> String {
        return self.base.dropTable();
    }
}
impl std::ops::Deref for TransformInfoEntity {
    type Target = AbSqlBean;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TransformInfoEntity {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasicDyn for TransformInfoEntity {
    fn as_AbSqlBasic(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &self.base.base.base.base.base }
    fn as_AbSqlBasic_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &mut self.base.base.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBeanDyn for TransformInfoEntity {
    fn as_AbSqlBean(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &self.base }
    fn as_AbSqlBean_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &mut self.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumnDyn for TransformInfoEntity {
    fn as_AbSqlColumn(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &self.base.base.base }
    fn as_AbSqlColumn_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &mut self.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRowDyn for TransformInfoEntity {
    fn as_AbSqlRow(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &self.base.base }
    fn as_AbSqlRow_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &mut self.base.base }
    fn insert(&mut self, mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::insert(self, values) }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTableDyn for TransformInfoEntity {
    fn as_AbSqlTable(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &self.base.base.base.base }
    fn as_AbSqlTable_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &mut self.base.base.base.base }
    fn dropTable(&mut self) -> String { Self::dropTable(self) }
}


