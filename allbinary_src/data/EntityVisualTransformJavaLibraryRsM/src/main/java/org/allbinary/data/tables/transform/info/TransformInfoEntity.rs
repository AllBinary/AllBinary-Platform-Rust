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
use super::*;
use crate::java::util::HashMap::HashMap;
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
use crate::java::lang::Exception::*;
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
    fn get(&mut self, mut name: String, mut propertiesHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<dyn TransformInfoInterface>>> { Self::get(self, name, propertiesHashMap, pageContext) }
    fn getObjectConfigs(&mut self, mut storeName: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getObjectConfigs(self, storeName) }
    fn getNames(&mut self, mut storeName: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getNames(self, storeName) }
    fn delete(&mut self, mut value: String) { Self::delete(self, value) }
    fn insert(&mut self, mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::insert(self, values) }
    fn update(&mut self, mut updatedValues: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) { Self::update(self, updatedValues) }
}
impl crate::org::allbinary::data::tables::BasicDataTableInterface::BasicDataTableInterface for TransformInfoEntity {
    fn createTable(&mut self) -> String { Self::createTable(self) }
    fn dropTable(&mut self) -> String { Self::dropTable(self) }
}

impl TransformInfoEntity {
    

    pub fn new_struct(mut transformInfoObjectConfigGeneratorFactoryInterface: Option<Rc<RefCell<dyn TransformInfoObjectConfigGeneratorFactoryInterface>>>, mut transformInfoObjectConfigAndManipulatorFactoryInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::objectConfig::TransformInfoObjectConfigAndManipulatorFactoryBase::TransformInfoObjectConfigAndManipulatorFactoryBaseDyn>>>, mut transformInfoFactoryInterface: Option<Rc<RefCell<dyn TransformInfoFactoryInterface>>>) -> Self {
        let mut this = Self {
            base: AbSqlBean::new_struct(UserDbInitInfo::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn>>)),
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
        { let java_arguments = (this.tableName.clone(),); this.setTableName(java_arguments.0) };
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
            { let java_arguments = (values.clone(),); self.base.insert(java_arguments.0) };
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).SUCCESS.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), INSERT.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).FAILURE.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), INSERT.clone(), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    pub fn delete(&mut self, mut value: String) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let java_arguments = ({ let java_field = jcall!(TransformInfoData::getInstance()).NAME.clone(); java_field }.clone(), value.clone(),); self.base.deleteWhere2(java_arguments.0, java_arguments.1) };
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).SUCCESS.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).delete.clone(); java_field }.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLLOGGING.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).FAILURE.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).delete.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    // throws Exception
    pub fn get(&mut self, mut name: String, mut propertiesHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<dyn TransformInfoInterface>>> {
        let mut transformInfoData: Option<Rc<RefCell<TransformInfoData>>> = TransformInfoData::getInstance();
        let mut keysAndValues: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(keysAndValues).put(crate::java_into_object({ let java_field = jcall!(transformInfoData).NAME.clone(); java_field }.clone()), crate::java_into_object(name.clone()));
        let mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = { let java_arguments = (keysAndValues.clone(),); self.base.getRow(java_arguments.0) };
        if hashMap.is_some() {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(transformInfoData).OBJECTCONFIG.clone(); java_field }.clone()));
            if anyType.is_some() {
                let mut string: String = { let java_object = (anyType).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
                jcall!(hashMap).put(crate::java_into_object({ let java_field = jcall!(transformInfoData).OBJECTCONFIG.clone(); java_field }.clone()), crate::java_into_object(Encoder::decode(string.clone()).clone()));
            }
            let mut objectData: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(transformInfoData).DATA.clone(); java_field }.clone()));
            if objectData.is_some() {
                let mut string: String = { let java_object = (objectData).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
                jcall!(hashMap).put(crate::java_into_object({ let java_field = jcall!(transformInfoData).DATA.clone(); java_field }.clone()), crate::java_into_object(Encoder::decode(string.clone()).clone()));
            }
            return jcall!(self.transformInfoFactoryInterface).getInstance(hashMap.clone(), propertiesHashMap.clone(), pageContext.clone());
        } else {
            return None;
        }
    }
    

    // throws Exception
    pub fn getObjectConfigs(&mut self, mut storeName: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut objectConfigVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut objectConfigColumnVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_arguments = ({ let java_field = jcall!(TransformInfoData::getInstance()).OBJECTCONFIG.clone(); java_field }.clone(), { let java_field = jcall!(StoreFrontData::getInstance()).NAME.clone(); java_field }.clone(), storeName.clone(),); self.getColumnWhere(java_arguments.0, java_arguments.1, java_arguments.2) };
        let mut size: i32 = jcall!(objectConfigColumnVector).size();
        for i in (0..size) {
            let mut objectConfigString: String = { let java_object = (jcall!(objectConfigColumnVector).get(i.clone())).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
            objectConfigString = { let java_value = (Encoder::decode(objectConfigString.clone()).clone()).clone(); java_value };
            jcall!(objectConfigVector).add(crate::java_into_object({ let java_receiver = self.transformInfoObjectConfigAndManipulatorFactoryInterface.clone(); let java_arguments = (self.abeClientInformation.clone(), None /* Java this: a &mut self method has no Rc of itself */, DomDocumentHelper::create1_String(objectConfigString.clone()),); let java_result = jcall!(java_receiver).as_TransformInfoObjectConfigAndManipulatorFactoryBase_mut().getInstance3(java_arguments.0, java_arguments.1, java_arguments.2); java_result }));
        }
        return objectConfigVector.clone();
    }
    

    // throws Exception
    pub fn getNames(&mut self, mut storeName: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut viewNameVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut columnVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_arguments = ({ let java_field = jcall!(TransformInfoData::getInstance()).NAME.clone(); java_field }.clone(), { let java_field = jcall!(StoreFrontData::getInstance()).NAME.clone(); java_field }.clone(), storeName.clone(),); self.getColumnWhere(java_arguments.0, java_arguments.1, java_arguments.2) };
        let mut size: i32 = jcall!(columnVector).size();
        for i in (0..size) {
            let mut viewNameString: String = { let java_object = (jcall!(columnVector).get(i.clone())).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
            jcall!(viewNameVector).add(crate::java_into_object(viewNameString.clone()));
        }
        return viewNameVector.clone();
    }
    

    pub fn createTableStatement(&mut self) -> String {
        let mut transformInfoData: Option<Rc<RefCell<TransformInfoData>>> = TransformInfoData::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).CREATE_TABLE.clone(); java_field }.clone());
        jcall!(stringBuffer).append(self.tableName.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).START.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(transformInfoData).NAME.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(StoreFrontData::getInstance()).NAME.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(transformInfoData).OBJECTFILENAME.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(transformInfoData).OBJECT.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).BLOB_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(transformInfoData).OBJECTCONFIGFILENAME.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(transformInfoData).OBJECTCONFIG.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).BLOB_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(transformInfoData).TEMPLATEFILENAME.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(transformInfoData).TEMPLATE.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).BLOB_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(transformInfoData).DATAFILENAME.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(transformInfoData).DATA.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).BLOB_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(EntryData::getInstance()).LASTMODIFIED.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(EntryData::getInstance()).TIMECREATED.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).PRIMARY_KEY.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(transformInfoData).NAME.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(self.sqlStrings).END.clone(); java_field }.clone());
        return jcall!(stringBuffer).toString();
    }
    

    pub fn createTable(&mut self) -> String {
        return { let java_arguments = (self.createTableStatement(),); self.base.createTable(java_arguments.0) };
    }
    

    pub fn update(&mut self, mut updatedValues: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) {
        { let java_arguments = ({ let java_field = jcall!(TransformInfoData::getInstance()).NAME.clone(); java_field }.clone(), { let java_object = (jcall!(updatedValues).get(crate::java_into_object({ let java_field = jcall!(TransformInfoData::getInstance()).NAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }, updatedValues.clone(),); self.base.updateWhere3(java_arguments.0, java_arguments.1, java_arguments.2) };
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


