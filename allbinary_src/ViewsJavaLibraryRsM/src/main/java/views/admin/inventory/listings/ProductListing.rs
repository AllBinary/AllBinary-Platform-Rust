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
// package views.admin.inventory.listings
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::HashSet::HashSet;
// use crate::java::util::Set::Set;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::user::commerce::inventory::item::BasicItemData::BasicItemData;
use crate::org::allbinary::data::tables::context::module::storefronts::StoreFrontsEntity::StoreFrontsEntity;
use crate::org::allbinary::data::tables::staticpages::StaticPagesEntity::StaticPagesEntity;
use crate::org::allbinary::data::tables::user::commerce::inventory::item::InventoryEntity::InventoryEntity;
use crate::org::allbinary::globals::GLOBALS2::GLOBALS2;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::search::SearchParams::SearchParams;
use crate::org::allbinary::logic::control::search::SearchRequest::SearchRequest;
use crate::org::allbinary::logic::io::AbDataOutputStream::AbDataOutputStream;
use crate::org::allbinary::logic::io::DataOutputStreamFactory::DataOutputStreamFactory;
use crate::org::allbinary::logic::io::InputOutputTypeData::InputOutputTypeData;
use crate::org::allbinary::logic::io::StreamUtil::StreamUtil;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::directory::Directory::Directory;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::string::SpecialCharacterUtil::SpecialCharacterUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::regex::replace::Replace::Replace;
use crate::org::allbinary::logic::string::tokens::UniqueTokens::UniqueTokens;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::ServiceClientInformationInterfaceFactory::ServiceClientInformationInterfaceFactory;
use crate::org::allbinary::string::CommonPhoneStrings::CommonPhoneStrings;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::views::generic::inventory::InventoryColumnUtil::InventoryColumnUtil;
use crate::views::generic::inventory::InventorySearchUtil::InventorySearchUtil;
use super::ProductListingInterface::ProductListingInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ProductListing {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub directory: Option<Rc<RefCell<Directory>>>,
    pub basicItemData: Option<Rc<RefCell<BasicItemData>>>,
    pub storeFronts: Option<Rc<RefCell<StoreFrontsEntity>>>,
    pub staticPages: Option<Rc<RefCell<StaticPagesEntity>>>,
    pub inventory: Option<Rc<RefCell<InventoryEntity>>>,
    pub searchRequest: Option<Rc<RefCell<SearchRequest>>>,
    pub INVENTORY: String,
}

impl ProductListingInterface for ProductListing {
    fn generateAll(&mut self) -> String { Self::generateAll(self) }
    fn generateAll1(&mut self, mut storeName: String) -> String { Self::generateAll1(self, storeName) }
}

impl ProductListing {
    

    pub fn new_struct(mut searchRequest: Option<Rc<RefCell<SearchRequest>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            directory: None,
            basicItemData: None,
            storeFronts: None,
            staticPages: None,
            inventory: None,
            searchRequest: None,
            INVENTORY: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.directory = Directory::getInstance();
        this.basicItemData = BasicItemData::getInstance();
        this.INVENTORY = String::from(".InventoryView");
        this.searchRequest = searchRequest.clone();
        this.storeFronts = StoreFrontsEntity::new();
        this.staticPages = StaticPagesEntity::new();
        this.inventory = InventoryEntity::new();
        this
    }

    pub fn new(mut searchRequest: Option<Rc<RefCell<SearchRequest>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(searchRequest))))
    }
    

    // throws Exception
    pub fn getHashSet(&mut self, mut storeFront: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Option<Rc<RefCell<HashSet>>> {
        let mut inventoryColumnUtil: Option<Rc<RefCell<InventoryColumnUtil>>> = InventoryColumnUtil::getInstance();
        let mut keywords: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(inventoryColumnUtil).getColumnWhereLike(self.inventory.clone(), jcall!(storeFront).getCategoryPath(), jcall!(self.basicItemData).KEYWORDS.clone());
        let mut subStoreVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(storeFront).getSubStores();
        let mut size: i32 = jcall!(subStoreVector).size();
        for index in (0..size) {
            let mut subStore: String = (jcall!(subStoreVector).get(index.clone())).clone() /* as String */;
            let mut substoreKeywords: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(inventoryColumnUtil).getColumnWhereLike(self.inventory.clone(), format!("{}{}", jcall!(AbPathData::getInstance()).SEPARATOR, subStore), jcall!(self.basicItemData).CATEGORY.clone());
            jcall!(keywords).addAllList(substoreKeywords.clone());
        }
        let mut uniqueTokens: Option<Rc<RefCell<UniqueTokens>>> = UniqueTokens::new();
        let mut keywordHashSet: Option<Rc<RefCell<HashSet>>> = jcall!(uniqueTokens).getWhithoutDashesAndSkipNumberOnlyTokens(keywords.clone());
        return keywordHashSet.clone();
    }
    

    // throws Exception
    pub fn savePage(&mut self, mut file: String, mut data: String) {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).STATICPAGEGENERATIONLOGGING.clone()) {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("Creating File: "), file), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generateAll()"));
        }
        let mut newFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(file.clone());
        if jcall!(newFile).exists() {
            jcall!(newFile).delete();
        }
        jcall!(newFile).createNewFile();
        if jcall!(newFile).exists() {
            let mut idOutData: Option<Rc<RefCell<AbDataOutputStream>>> = jcall!(DataOutputStreamFactory::getInstance()).getInstanceForAbFile(newFile.clone());
            jcall!(idOutData).writeBytes(data.clone());
            jcall!(idOutData).flush();
            jcall!(StreamUtil::getInstance()).close(idOutData.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).STATICPAGEGENERATIONLOGGING.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Wrote Total Bytes: "), jcall!(newFile).length()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generateAll()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("Could Not Create: "), file))));
        }
    }
    

    // throws Exception
    pub fn create(&mut self, mut keywordData: String, mut keywordFilenameHashMap: Option<Rc<RefCell<HashMap>>>, mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut staticPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = SpecialCharacterUtil::getHashMap();
        jcall!(hashMap).put(jcall!(CommonSeps::getInstance()).SPACE.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone());
        let mut pageName: String = jcall!(Replace::new1(hashMap.clone())).all(keywordData.clone());
        let mut searchParams: Option<Rc<RefCell<SearchParams>>> = jcall!(self.searchRequest).getParams();
        jcall!(searchParams).add(jcall!(self.basicItemData).KEYWORDS.clone(), keywordData.clone());
        jcall!(searchParams).setStartPage(jcall!(CommonPhoneStrings::getInstance()).ZERO.clone());
        jcall!(self.searchRequest).setParams(searchParams.clone());
        jcall!(self.searchRequest).setFileBaseName(pageName.clone());
        let mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>> = ServiceClientInformationInterfaceFactory::getInstance().map(|reference| reference as Rc<RefCell<dyn AbeClientInformationInterface>>);
        let mut inventorySearchUtil: Option<Rc<RefCell<InventorySearchUtil>>> = InventorySearchUtil::getInstance();
        let mut productListingPages: Vec<String> = jcall!(inventorySearchUtil).search(abeClientInformation.clone(), self.searchRequest.clone(), vector.clone());
        for index in (0..(productListingPages.len() as i32)) {
            if productListingPages[(index) as usize].is_empty() {
                break;
            }
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).STATICPAGEGENERATIONLOGGING.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("Saving Listing: "), index), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generateAll()"));
            }
            let mut indexStr: String = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
            if index > 0 {
                indexStr = jcall!(Some((index.clone()) as i32)).toString();
            }
            jcall!(keywordFilenameHashMap).put(keywordData.clone(), format!("{}{}", pageName, indexStr));
            jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
            jcall!(stringBuffer).append(jcall!(staticPath).toString());
            jcall!(stringBuffer).append(pageName.clone());
            jcall!(stringBuffer).append(indexStr.clone());
            jcall!(stringBuffer).append(jcall!(AbPathData::getInstance()).EXTENSION_SEP.clone());
            jcall!(stringBuffer).append(jcall!(InputOutputTypeData::getInstance()).DEFAULT.clone());
            let mut file: String = jcall!(stringBuffer).toString();
            if !jcall!(self.directory).create(staticPath.clone()) {
                panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("Could Not Create Directory: "), staticPath))));
            }
            self.savePage(file.clone(), productListingPages[(index) as usize].clone());
        }
    }
    

    pub fn addStaticPageInfoToDatabase(&mut self, mut storeFront: Option<Rc<RefCell<dyn StoreFrontInterface>>>, mut keywordFilenameHashMap: Option<Rc<RefCell<HashMap>>>) {
        let mut keywordHashSet: Option<Rc<RefCell<dyn Set>>> = jcall!(keywordFilenameHashMap).keySet();
        let mut keywordArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(keywordHashSet).toArray();
        let mut size: i32 = (keywordArray.len() as i32).clone();
        for index in (0..size) {
            let mut insertVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            let mut keywordData: String = (keywordArray[(index) as usize]).clone() /* as String */;
            let mut fileName: String = (jcall!(keywordFilenameHashMap).get(keywordData.clone())).clone() /* as String */;
            jcall!(insertVector).add(crate::java_into_object(jcall!(storeFront).getName()));
            jcall!(insertVector).add(crate::java_into_object(keywordData.clone()));
            jcall!(insertVector).add(crate::java_into_object(fileName.clone()));
            jcall!(self.staticPages).insert(insertVector.clone());
        }
    }
    

    // throws Exception
    pub fn generateAll(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut storeFront: Option<Rc<RefCell<dyn StoreFrontInterface>>> = jcall!(self.searchRequest).getStoreFront();
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(URLGLOBALS::getWebappPath());
            jcall!(stringBuffer).append(jcall!(storeFront).getName());
            jcall!(stringBuffer).append(jcall!(AbPathData::getInstance()).SEPARATOR.clone());
            jcall!(stringBuffer).append(jcall!(storeFront).getStaticPath());
            let mut staticPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(jcall!(stringBuffer).toString(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).STATICPAGEGENERATIONLOGGING.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("Store Static Pages Path: "), staticPath), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generateAll()"));
            }
            let mut keywordHashSet: Option<Rc<RefCell<HashSet>>> = self.getHashSet(storeFront.clone());
            let mut keywordArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(keywordHashSet).toArray();
            let mut size: i32 = (keywordArray.len() as i32).clone();
            let mut keywordFilenameHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            if size == 0 {
                jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                jcall!(stringBuffer).append(String::from("Products For "));
                jcall!(stringBuffer).append(jcall!(storeFront).getName());
                jcall!(stringBuffer).append(String::from(" Store Not Found"));
                return jcall!(stringBuffer).toString();
            }
            let mut inventorySearchUtil: Option<Rc<RefCell<InventorySearchUtil>>> = InventorySearchUtil::getInstance();
            let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(inventorySearchUtil).getBasicItemIdColumn(self.searchRequest.clone());
            for index in (0..size) {
                let mut keywordData: String = (keywordArray[(index) as usize]).clone() /* as String */;
                if (keywordData.chars().count() as i32) > 1 {
                    self.create(keywordData.clone(), keywordFilenameHashMap.clone(), vector.clone(), staticPath.clone());
                }
            }
            self.addStaticPageInfoToDatabase(storeFront.clone(), keywordFilenameHashMap.clone());
            jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
            jcall!(stringBuffer).append(String::from("Static Files Generated Successfully For "));
            jcall!(stringBuffer).append(jcall!(storeFront).getName());
            jcall!(stringBuffer).append(String::from(" it used packages "));
            jcall!(stringBuffer).append(jcall!(storeFront).getPackageLocation());
            jcall!(stringBuffer).append(self.INVENTORY.clone());
            return jcall!(stringBuffer).toString();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn generateAll1(&mut self, mut storeName: String) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            if true {
                if (storeName.cmp(&crate::org::allbinary::globals::GLOBALS2::GLOBALS2_GENERATEALLSTORES.with(Clone::clone).clone()) as i32) != 0 {
                    { let java_receiver = self.searchRequest.clone(); let java_arguments = (jcall!(self.storeFronts).getStoreFrontInterface(storeName.clone()),); jcall!(java_receiver).setStoreFront(java_arguments.0); };
                    jcall!(stringBuffer).append(self.generateAll());
                    jcall!(stringBuffer).append(String::from("<br />"));
                } else {
                    let mut storeFrontVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(self.storeFronts).getStoreFrontNames();
                    let mut size: i32 = jcall!(storeFrontVector).size();
                    for index in (0..size) {
                        storeName = (jcall!(storeFrontVector).get(index.clone())).clone() /* as String */;
                        { let java_receiver = self.searchRequest.clone(); let java_arguments = (jcall!(self.storeFronts).getStoreFrontInterface(storeName.clone()),); jcall!(java_receiver).setStoreFront(java_arguments.0); };
                        jcall!(stringBuffer).append(self.generateAll());
                        jcall!(stringBuffer).append(String::from("<br />"));
                    }
                }
            } else {
                return String::from("Generation Failed No Store Specified<br/>");
            }
            jcall!(stringBuffer).append(String::from("All Static Pages Generated<br/>"));
            return jcall!(stringBuffer).toString();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
}

