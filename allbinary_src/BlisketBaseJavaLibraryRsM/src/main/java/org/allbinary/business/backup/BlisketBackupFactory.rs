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
// package org.allbinary.business.backup
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::data::tables::context::module::storefronts::StoreFrontsEntity::StoreFrontsEntity;
use crate::org::allbinary::data::tables::context::module::storefronts::StoreFrontsEntityFactory::StoreFrontsEntityFactory;
use crate::org::allbinary::globals::PATH_GLOBALS::PATH_GLOBALS;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::directory::Directory::Directory;
use crate::org::allbinary::logic::io::file::zip::ZipFileUtil::ZipFileUtil;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BlisketBackupFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub stringUtil: Option<Rc<RefCell<StringUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub directory: Option<Rc<RefCell<Directory>>>,
}

thread_local! { pub static BlisketBackupFactory_instance: Option<Rc<RefCell<BlisketBackupFactory>>> = BlisketBackupFactory::new(); }

impl BlisketBackupFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            stringUtil: None,
            commonStrings: None,
            directory: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.stringUtil = StringUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.directory = Directory::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<BlisketBackupFactory>>> {
        return BlisketBackupFactory_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn getFileBasicArrayList(&mut self, mut pathString: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut path: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(pathString.clone(), jcall!(self.stringUtil).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        let mut file: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFileFromAbPath(path.clone());
        return jcall!(self.directory).search2(file.clone(), true);
    }
    

    pub fn backup(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("backup()"));
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(URLGLOBALS::getWebappPath());
            jcall!(stringBuffer).append(jcall!(PATH_GLOBALS::getInstance()).BACKUP_PATH.clone());
            let mut backupPath: String = jcall!(stringBuffer).toString();
            let mut path: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(backupPath.clone(), jcall!(self.stringUtil).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
            let mut fileBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getFileBasicArrayList(backupPath.clone());
            { let java_arguments = (fileBasicArrayList.clone(), format!("{}{}", jcall!(path).toFileSystemString(), String::from("backup.zip")),); self.backup2(java_arguments.0, java_arguments.1) };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("backup()"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn backupViews(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("backupViews()"));
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(URLGLOBALS::getWebappPath());
            jcall!(stringBuffer).append(jcall!(PATH_GLOBALS::getInstance()).BACKUP_PATH.clone());
            let mut backupPath: String = jcall!(stringBuffer).toString();
            let mut path: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(backupPath.clone(), jcall!(self.stringUtil).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
            let mut fileBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            let mut storeFrontsEntity: Option<Rc<RefCell<StoreFrontsEntity>>> = jcall!(StoreFrontsEntityFactory::getInstance()).getStoreFrontsEntityInstance();
            let mut storeFrontNamesBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(storeFrontsEntity).getStoreFrontNames();
            let mut size: i32 = jcall!(storeFrontNamesBasicArrayList).size();
            let mut nextStore: String = String::new();
            let mut viewsPath: String = String::new();
            for index in (0..size) {
                nextStore = (jcall!(storeFrontNamesBasicArrayList).get(index.clone())).clone() /* as String */;
                jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                jcall!(stringBuffer).append(URLGLOBALS::getWebappPath());
                jcall!(stringBuffer).append(jcall!(PATH_GLOBALS::getInstance()).VIEWS_PATH.clone());
                jcall!(stringBuffer).append(nextStore.clone());
                viewsPath = jcall!(stringBuffer).toString();
                jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                jcall!(stringBuffer).append(String::from("Backup Store Views: "));
                jcall!(stringBuffer).append(nextStore.clone());
                jcall!(stringBuffer).append(String::from(" from: "));
                jcall!(stringBuffer).append(viewsPath.clone());
                PreLogUtil::put(jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("backupViews()"));
                jcall!(fileBasicArrayList).addAllList(self.getFileBasicArrayList(viewsPath.clone()));
            }
            { let java_arguments = (fileBasicArrayList.clone(), format!("{}{}", jcall!(path).toFileSystemString(), String::from("backupViews.zip")),); self.backup2(java_arguments.0, java_arguments.1) };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("backupViews()"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn backupResources(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("backupResources()"));
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(URLGLOBALS::getWebappPath());
            jcall!(stringBuffer).append(jcall!(PATH_GLOBALS::getInstance()).BACKUP_PATH.clone());
            let mut backupPath: String = jcall!(stringBuffer).toString();
            let mut path: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(backupPath.clone(), jcall!(self.stringUtil).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
            let mut fileBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            let mut storeFrontsEntity: Option<Rc<RefCell<StoreFrontsEntity>>> = jcall!(StoreFrontsEntityFactory::getInstance()).getStoreFrontsEntityInstance();
            let mut storeFrontNamesBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(storeFrontsEntity).getStoreFrontNames();
            let mut size: i32 = jcall!(storeFrontNamesBasicArrayList).size();
            let mut nextStore: String = String::new();
            let mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>> = None;
            let mut resourcesPath: String = String::new();
            for index in (0..size) {
                nextStore = (jcall!(storeFrontNamesBasicArrayList).get(index.clone())).clone() /* as String */;
                storeFrontInterface = jcall!(storeFrontsEntity).getStoreFrontInterface(nextStore.clone());
                jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                jcall!(stringBuffer).append(URLGLOBALS::getWebappPath());
                jcall!(stringBuffer).append(jcall!(storeFrontInterface).getCurrentHostNamePath());
                jcall!(stringBuffer).append(jcall!(storeFrontInterface).getCategoryPath());
                resourcesPath = jcall!(stringBuffer).toString();
                jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                jcall!(stringBuffer).append(String::from("Backup Store Resrouces: "));
                jcall!(stringBuffer).append(nextStore.clone());
                jcall!(stringBuffer).append(String::from(" from: "));
                jcall!(stringBuffer).append(resourcesPath.clone());
                PreLogUtil::put(jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("backupResources()"));
                jcall!(fileBasicArrayList).addAllList(self.getFileBasicArrayList(resourcesPath.clone()));
            }
            { let java_arguments = (fileBasicArrayList.clone(), format!("{}{}", jcall!(path).toFileSystemString(), String::from("backupResources.zip")),); self.backup2(java_arguments.0, java_arguments.1) };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("backupResources()"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn backupJsps(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("backupJsps()"));
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(URLGLOBALS::getWebappPath());
            jcall!(stringBuffer).append(jcall!(PATH_GLOBALS::getInstance()).BACKUP_PATH.clone());
            let mut backupPath: String = jcall!(stringBuffer).toString();
            let mut path: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(backupPath.clone(), jcall!(self.stringUtil).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
            let mut fileBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            let mut storeFrontsEntity: Option<Rc<RefCell<StoreFrontsEntity>>> = jcall!(StoreFrontsEntityFactory::getInstance()).getStoreFrontsEntityInstance();
            let mut storeFrontNamesBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(storeFrontsEntity).getStoreFrontNames();
            let mut size: i32 = jcall!(storeFrontNamesBasicArrayList).size();
            let mut nextStore: String = String::new();
            for index in (0..size) {
                nextStore = (jcall!(storeFrontNamesBasicArrayList).get(index.clone())).clone() /* as String */;
                PreLogUtil::put(format!("{}{}", String::from("Backup Store Jsps: "), nextStore), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("backupJsps()"));
                jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                jcall!(stringBuffer).append(URLGLOBALS::getWebappPath());
                jcall!(stringBuffer).append(nextStore.clone());
                jcall!(fileBasicArrayList).addAllList({ let java_arguments = (jcall!(stringBuffer).toString(),); self.getFileBasicArrayList(java_arguments.0) });
            }
            { let java_arguments = (fileBasicArrayList.clone(), format!("{}{}", jcall!(path).toFileSystemString(), String::from("backupJsps.zip")),); self.backup2(java_arguments.0, java_arguments.1) };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("backupJsps()"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn backup2(&mut self, mut fileBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut zipFile: String) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
            jcall!(stringBuffer).append(String::from("ZipFile: "));
            jcall!(stringBuffer).append(zipFile.clone());
            jcall!(stringBuffer).append(String::from(" BasicArrayList: "));
            jcall!(stringBuffer).appendint(jcall!(fileBasicArrayList).size());
            PreLogUtil::put(format!("{}{}", String::from("Creating Backup Zip File: "), jcall!(stringBuffer).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("backup()"));
            jcall!(ZipFileUtil::getInstance()).create(zipFile.clone(), fileBasicArrayList.clone());
            jcall!(self.logUtil).putF(String::from("Created Backup Zip File"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("backup()"));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("backup()"), crate::java_into_object(e.clone()));
            }
        }
    }
}

