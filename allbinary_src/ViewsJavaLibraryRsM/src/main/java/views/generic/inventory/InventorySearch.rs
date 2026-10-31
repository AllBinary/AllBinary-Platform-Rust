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
// package views.generic.inventory
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::user::commerce::inventory::item::BasicItemData::BasicItemData;
use crate::org::allbinary::data::tables::staticpages::StaticPagesEntity::StaticPagesEntity;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::search::SearchParams::SearchParams;
use crate::org::allbinary::logic::control::search::SearchRequest::SearchRequest;
use crate::org::allbinary::logic::io::InputOutputTypeData::InputOutputTypeData;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::string::regex::replace::Replace::Replace;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::ServiceClientInformationInterfaceFactory::ServiceClientInformationInterfaceFactory;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::views::admin::inventory::listings::InventoryViewSearchInterface::InventoryViewSearchInterface;
use super::InventorySearchUtil::InventorySearchUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InventorySearch {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub basicItemData: Option<Rc<RefCell<BasicItemData>>>,
    pub searchRequest: Option<Rc<RefCell<SearchRequest>>>,
}

impl InventoryViewSearchInterface for InventorySearch {
    fn searchSingleStaticPage(&mut self) -> String { Self::searchSingleStaticPage(self) }
    fn searchSingleDynamicPage(&mut self) -> String { Self::searchSingleDynamicPage(self) }
    fn search(&mut self) -> Vec<String> { Self::search(self) }
    fn getProduct(&mut self, mut product: String) -> String { Self::getProduct(self, product) }
}

impl InventorySearch {
    

    pub fn new_struct(mut searchRequest: Option<Rc<RefCell<SearchRequest>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            basicItemData: None,
            searchRequest: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.basicItemData = BasicItemData::getInstance();
        // super(...) is in the struct literal above
        this.searchRequest = searchRequest.clone();
        this
    }

    pub fn new(mut searchRequest: Option<Rc<RefCell<SearchRequest>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(searchRequest))))
    }
    

    // throws Exception
    pub fn searchSingleStaticPage(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut storeFront: Option<Rc<RefCell<dyn StoreFrontInterface>>> = jcall!(self.searchRequest).getStoreFront();
            let mut searchParams: Option<Rc<RefCell<SearchParams>>> = jcall!(self.searchRequest).getParams();
            let mut columnValueHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(searchParams).get();
            let mut file: String = jcall!(StaticPagesEntity::new()).getFile(jcall!(storeFront).getName(), jcall!(Replace::new2(String::from("-"), jcall!(CommonSeps::getInstance()).SPACE.clone())).all((jcall!(columnValueHashMap).get(jcall!(self.basicItemData).KEYWORDS.clone())).clone() /* as String */));
            if jcall!(StringValidationUtil::getInstance()).isEmpty(file.clone()) {
                return None;
            } else {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(jcall!(storeFront).getCurrentHostName());
                jcall!(stringBuffer).append(jcall!(storeFront).getCurrentHostNamePath());
                jcall!(stringBuffer).append(jcall!(storeFront).getStaticPath());
                jcall!(stringBuffer).append(file.clone());
                jcall!(stringBuffer).append(jcall!(jcall!(self.searchRequest).getParams()).getEndPage());
                jcall!(stringBuffer).append(jcall!(AbPathData::getInstance()).EXTENSION_SEP.clone());
                jcall!(stringBuffer).append(jcall!(InputOutputTypeData::getInstance()).DEFAULT.clone());
                let mut filePath: String = jcall!(Replace::new2(jcall!(CommonSeps::getInstance()).SPACE.clone(), String::from("%20"))).all(jcall!(stringBuffer).toString());
                return filePath.clone();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).PRODUCTSEARCHLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("searchSingleStaticPage"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Failed retrieve Single Product Page Static"))));
            }
        }
    }
    

    // throws Exception
    pub fn searchSingleDynamicPage(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut str: Vec<String> = self.search();
            return str[(((jcall!(jcall!(self.searchRequest).getParams()).getStartPageInt()).unwrap() as i32)) as usize].clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).PRODUCTSEARCHLOGGINGERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("searchSingleDynamicPage"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Failed retrieve Single Product Page Dynamically"))));
            }
        }
    }
    

    // throws Exception
    pub fn search(&mut self) -> Vec<String> {
        let mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>> = ServiceClientInformationInterfaceFactory::getInstance().map(|reference| reference as Rc<RefCell<dyn AbeClientInformationInterface>>);
        let mut inventorySearchUtil: Option<Rc<RefCell<InventorySearchUtil>>> = InventorySearchUtil::getInstance();
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(inventorySearchUtil).getBasicItemIdColumn(self.searchRequest.clone());
        return jcall!(inventorySearchUtil).search(abeClientInformation.clone(), self.searchRequest.clone(), vector.clone());
    }
    

    // throws Exception
    pub fn getProduct(&mut self, mut product: String) -> String {
        let mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>> = ServiceClientInformationInterfaceFactory::getInstance().map(|reference| reference as Rc<RefCell<dyn AbeClientInformationInterface>>);
        return jcall!(InventorySearchProductUtil::InventorySearchProductUtil::getInstance()).getProduct(abeClientInformation.clone(), self.searchRequest.clone(), product.clone());
    }
}
pub trait InventorySearchDyn {
    fn as_InventorySearch(&self) -> &InventorySearch;
    fn as_InventorySearch_mut(&mut self) -> &mut InventorySearch;
    fn searchSingleStaticPage(&mut self) -> String { InventorySearch::searchSingleStaticPage(self.as_InventorySearch_mut()) }
    fn searchSingleDynamicPage(&mut self) -> String { InventorySearch::searchSingleDynamicPage(self.as_InventorySearch_mut()) }
    fn search(&mut self) -> Vec<String> { InventorySearch::search(self.as_InventorySearch_mut()) }
    fn getProduct(&mut self, mut product: String) -> String { InventorySearch::getProduct(self.as_InventorySearch_mut(), product) }
}
impl InventorySearchDyn for InventorySearch {
    fn as_InventorySearch(&self) -> &InventorySearch { self }
    fn as_InventorySearch_mut(&mut self) -> &mut InventorySearch { self }
}


