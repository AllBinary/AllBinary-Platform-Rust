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
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::user::commerce::inventory::InventoryData::InventoryData;
use crate::org::allbinary::business::user::commerce::inventory::basket::BasketData::BasketData;
use crate::org::allbinary::business::user::commerce::inventory::item::BasicItemData::BasicItemData;
use crate::org::allbinary::business::user::commerce::inventory::item::BasicItemView::BasicItemView;
use crate::org::allbinary::business::user::commerce::inventory::item::ItemInterface::ItemInterface;
use crate::org::allbinary::data::tables::user::commerce::inventory::item::InventoryEntity::InventoryEntity;
use crate::org::allbinary::data::tables::user::commerce::inventory::item::InventoryEntityFactory::InventoryEntityFactory;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::data::tree::dom::document::XmlDocumentHelper::XmlDocumentHelper;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::search::SearchData::SearchData;
use crate::org::allbinary::logic::control::search::SearchParams::SearchParams;
use crate::org::allbinary::logic::control::search::SearchRequest::SearchRequest;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::string::regex::replace::Replace::Replace;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::visual::transform::StoreTransformer::StoreTransformer;
use crate::org::allbinary::logic::visual::transform::data::TransformDocumentInterface::TransformDocumentInterface;
use crate::org::allbinary::logic::visual::transform::data::TransformStoreDocumentFactory::TransformStoreDocumentFactory;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpSearch::TransformInfoHttpSearch;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::string::CommonPhoneStrings::CommonPhoneStrings;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::InventoryColumnUtil::InventoryColumnUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InventorySearchUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub commonPhoneStrings: Option<Rc<RefCell<CommonPhoneStrings>>>,
    pub basicItemData: Option<Rc<RefCell<BasicItemData>>>,
}

thread_local! { pub static InventorySearchUtil_instance: Option<Rc<RefCell<InventorySearchUtil>>> = InventorySearchUtil::new(); }

impl InventorySearchUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            commonPhoneStrings: None,
            basicItemData: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.commonPhoneStrings = CommonPhoneStrings::getInstance();
        this.basicItemData = BasicItemData::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<InventorySearchUtil>>> {
        return InventorySearchUtil_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn getBasicItemIdColumn(&mut self, mut searchRequest: Option<Rc<RefCell<SearchRequest>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut inventoryEntityInterface: Option<Rc<RefCell<InventoryEntity>>> = jcall!(InventoryEntityFactory::getInstance()).getInventoryEntityInstance();
        let mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>> = jcall!(searchRequest).getStoreFront();
        let mut inventorySearchUtil: Option<Rc<RefCell<InventoryColumnUtil>>> = InventoryColumnUtil::getInstance();
        let mut column: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(inventorySearchUtil).getColumnWhereLike(inventoryEntityInterface.clone(), jcall!(storeFrontInterface).getCategoryPath(), jcall!(self.basicItemData).ID.clone());
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).PRODUCTSEARCHLOGGING.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Number Of Items Found: "), jcall!(column).size()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("search"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut subStoreVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(storeFrontInterface).getSubStores();
        let mut size: i32 = jcall!(subStoreVector).size();
        for index in (0..size) {
            let mut subStore: String = (jcall!(subStoreVector).get(index.clone())).clone() /* as String */;
            let mut substoreIdColumn: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(inventorySearchUtil).getColumnWhereLike(inventoryEntityInterface.clone(), subStore.clone(), jcall!(self.basicItemData).ID.clone());
            jcall!(column).addAllList(substoreIdColumn.clone());
        }
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).PRODUCTSEARCHLOGGING.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Number Of Items Found Including SubStores: "), jcall!(column).size()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("search"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        return column.clone();
    }
    

    // throws Exception
    pub fn getNoResults(&mut self, mut viewDocumentInterface: Option<Rc<RefCell<dyn TransformDocumentInterface>>>, mut inventoryNode: Option<Rc<RefCell<Node>>>) -> String {
        jcall!(inventoryNode).appendChild(ModDomHelper::createNameValueNodes(jcall!(viewDocumentInterface).getDoc(), crate::org::allbinary::logic::control::search::SearchData::SearchData_TOTAL_NUMBER_PAGES.with(Clone::clone).clone(), jcall!(self.commonPhoneStrings).ZERO.clone()));
        jcall!(inventoryNode).appendChild(ModDomHelper::createNameValueNodes(jcall!(viewDocumentInterface).getDoc(), crate::org::allbinary::logic::control::search::SearchData::SearchData_TOTAL_NUMBER_ITEMS.with(Clone::clone).clone(), jcall!(self.commonPhoneStrings).ZERO.clone()));
        let mut success: String = XmlDocumentHelper::toString(jcall!(viewDocumentInterface).getDoc());
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).PRODUCTSEARCHLOGGING.clone()) {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("No Results Xml: "), success), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("search"));
        }
        return success.clone();
    }
    

    pub fn search(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut searchRequest: Option<Rc<RefCell<SearchRequest>>>, mut column: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Vec<String> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut inventoryEntityInterface: Option<Rc<RefCell<InventoryEntity>>> = jcall!(InventoryEntityFactory::getInstance()).getInventoryEntityInstance();
            let mut searchParams: Option<Rc<RefCell<SearchParams>>> = jcall!(searchRequest).getParams();
            let mut startPage: i32 = ((jcall!(searchParams).getStartPageInt()).unwrap() as i32);
            let mut endPage: i32 = ((jcall!(searchParams).getEndPageInt()).unwrap() as i32);
            let mut pageLength: i32 = ((jcall!(searchParams).getLengthInt()).unwrap() as i32);
            let mut savedPagesInRange: i32 = 0;
            let mut columnValueHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(searchParams).get();
            let mut keyword: String = jcall!(Replace::new2(String::from("-"), jcall!(CommonSeps::getInstance()).SPACE.clone())).all((jcall!(columnValueHashMap).get(jcall!(self.basicItemData).KEYWORDS.clone())).clone() /* as String */);
            if jcall!(StringValidationUtil::getInstance()).isEmpty(keyword.clone()) {
                return None;
            }
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            if startPage >= 0 {
                if pageLength == 0 {
                    pageLength = crate::org::allbinary::logic::control::search::SearchData::SearchData_DEFAULT_PAGE_LENGTH.with(Clone::clone).clone();
                }
                let mut MAXPAGES: i32 = 100;
                let mut productListingPages: Vec<String> = vec![String::new(); (MAXPAGES) as usize];
                let mut documents: Vec<Option<Rc<RefCell<Document>>>> = vec![None; (MAXPAGES) as usize];
                let mut inventoryNodes: Vec<Option<Rc<RefCell<Node>>>> = vec![None; (MAXPAGES) as usize];
                keyword = keyword.to_uppercase();
                let mut lastPage: i32 = -1;
                let mut numberOfResultsOnCurrentPage: i32 = 0;
                let mut numberOfResults: i32 = 0;
                let mut viewDocumentInterface: Option<Rc<RefCell<dyn TransformDocumentInterface>>> = TransformStoreDocumentFactory::getInstance(searchRequest.clone());
                let mut inventoryNode: Option<Rc<RefCell<Node>>> = jcall!(jcall!(viewDocumentInterface).getDoc()).createElement(crate::org::allbinary::business::user::commerce::inventory::InventoryData::InventoryData_INVENTORY.with(Clone::clone).clone());
                jcall!(jcall!(viewDocumentInterface).getBaseNode()).appendChild(inventoryNode.clone());
                jcall!(inventoryNode).appendChild(ModDomHelper::createNameValueNodes(jcall!(viewDocumentInterface).getDoc(), crate::org::allbinary::logic::control::search::SearchData::SearchData_PAGE.with(Clone::clone).clone(), jcall!(Some((startPage.clone()) as i32)).toString()));
                if jcall!(searchRequest).getFileBaseName().is_some() {
                    jcall!(inventoryNode).appendChild(jcall!(searchRequest).getFileBaseNameNode(jcall!(viewDocumentInterface).getDoc()));
                }
                let mut currentPage: i32 = -1;
                let mut size: i32 = jcall!(column).size();
                for index in (0..size) {
                    let mut product: String = (jcall!(column).get(index.clone())).clone() /* as String */.clone();
                    let mut itemInterface: Option<Rc<RefCell<dyn ItemInterface>>> = jcall!(inventoryEntityInterface).getItem(product.clone());
                    let mut keywords: String = jcall!(itemInterface).getKeywords();
                    keywords = keywords.to_uppercase();
                    if keywords.find(&keyword.clone()).map(|index| index as i32).unwrap_or(-1) != -1 {
                        if currentPage + 1 >= startPage && currentPage + 1 <= endPage {
                            let mut itemNode: Option<Rc<RefCell<Node>>> = jcall!(BasicItemView::new(itemInterface.clone(), BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>))).toXmlNode(jcall!(viewDocumentInterface).getDoc());
                            jcall!(itemNode).appendChild(ModDomHelper::createNameValueNodes(jcall!(viewDocumentInterface).getDoc(), crate::org::allbinary::business::user::commerce::inventory::basket::BasketData::BasketData_ITEMTOTALINBASKET.with(Clone::clone).clone(), jcall!(self.commonPhoneStrings).ONE.clone()));
                            jcall!(inventoryNode).appendChild(jcall!(itemNode).cloneNode(true));
                        }
                        currentPage = ((numberOfResults + 1) / pageLength) - 1;
                        numberOfResultsOnCurrentPage += 1;
                        if lastPage < currentPage {
                            if currentPage >= startPage && currentPage <= endPage {
                                jcall!(inventoryNode).appendChild(ModDomHelper::createNameValueNodes(jcall!(viewDocumentInterface).getDoc(), crate::org::allbinary::logic::control::search::SearchData::SearchData_TOTAL_NUMBER_ITEMS_ON_THIS_PAGE.with(Clone::clone).clone(), jcall!(Some((numberOfResultsOnCurrentPage.clone()) as i32)).toString()));
                                documents[(currentPage) as usize] = jcall!(viewDocumentInterface).getDoc();
                                inventoryNodes[(currentPage) as usize] = inventoryNode.clone();
                                savedPagesInRange += 1;
                                viewDocumentInterface = TransformStoreDocumentFactory::getInstance(searchRequest.clone());
                                inventoryNode = jcall!(jcall!(viewDocumentInterface).getDoc()).createElement(crate::org::allbinary::business::user::commerce::inventory::InventoryData::InventoryData_INVENTORY.with(Clone::clone).clone());
                                jcall!(jcall!(viewDocumentInterface).getBaseNode()).appendChild(inventoryNode.clone());
                                jcall!(inventoryNode).appendChild(ModDomHelper::createNameValueNodes(jcall!(viewDocumentInterface).getDoc(), crate::org::allbinary::logic::control::search::SearchData::SearchData_PAGE.with(Clone::clone).clone(), jcall!(Some((currentPage + 1) as i32)).toString()));
                                if jcall!(searchRequest).getFileBaseName().is_some() {
                                    jcall!(inventoryNode).appendChild(jcall!(searchRequest).getFileBaseNameNode(jcall!(viewDocumentInterface).getDoc()));
                                }
                            }
                            lastPage = currentPage.clone();
                            numberOfResultsOnCurrentPage = 0;
                        }
                        numberOfResults += 1;
                        if lastPage >= MAXPAGES - 1 {
                            break;
                        }
                    }
                }
                if numberOfResultsOnCurrentPage > 0 && (currentPage + 1 >= startPage && currentPage + 1 <= endPage) {
                    jcall!(inventoryNode).appendChild(ModDomHelper::createNameValueNodes(jcall!(viewDocumentInterface).getDoc(), crate::org::allbinary::logic::control::search::SearchData::SearchData_TOTAL_NUMBER_ITEMS_ON_THIS_PAGE.with(Clone::clone).clone(), jcall!(Some((numberOfResultsOnCurrentPage - 1) as i32)).toString()));
                    documents[(lastPage + 1) as usize] = jcall!(viewDocumentInterface).getDoc();
                    inventoryNodes[(lastPage + 1) as usize] = inventoryNode.clone();
                    savedPagesInRange += 1;
                }
                if numberOfResultsOnCurrentPage > 0 {
                    lastPage += 1;
                }
                if endPage > MAXPAGES - 1 {
                    endPage = MAXPAGES - 1;
                }
                for index in (startPage..=endPage) {
                    if index <= lastPage {
                        let mut tempDocument: Option<Rc<RefCell<Document>>> = documents[(index) as usize].clone();
                        jcall!(inventoryNodes[(index) as usize]).appendChild(ModDomHelper::createNameValueNodes(tempDocument.clone(), crate::org::allbinary::logic::control::search::SearchData::SearchData_TOTAL_NUMBER_PAGES.with(Clone::clone).clone(), jcall!(Some((lastPage + 1) as i32)).toString()));
                        jcall!(inventoryNodes[(index) as usize]).appendChild(ModDomHelper::createNameValueNodes(tempDocument.clone(), crate::org::allbinary::logic::control::search::SearchData::SearchData_TOTAL_NUMBER_ITEMS.with(Clone::clone).clone(), jcall!(Some((numberOfResults.clone()) as i32)).toString()));
                        for pageIndex in (0..=lastPage) {
                            jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                            jcall!(stringBuffer).append(jcall!(searchRequest).getFileBaseName());
                            if jcall!(stringBuffer).length() > 0 {
                                if pageIndex != 0 {
                                    jcall!(stringBuffer).append((pageIndex.clone()).to_string());
                                }
                                jcall!(inventoryNodes[(index) as usize]).appendChild(ModDomHelper::createNameValueNodes4(tempDocument.clone(), crate::org::allbinary::logic::control::search::SearchData::SearchData_PAGE_INFO.with(Clone::clone).clone(), (pageIndex.clone()).to_string(), jcall!(stringBuffer).toString()));
                            } else {
                                jcall!(inventoryNodes[(index) as usize]).appendChild(ModDomHelper::createNameValueNodes4(tempDocument.clone(), crate::org::allbinary::logic::control::search::SearchData::SearchData_PAGE_INFO.with(Clone::clone).clone(), (pageIndex.clone()).to_string(), crate::org::allbinary::logic::control::search::SearchData::SearchData_SEARCH.with(Clone::clone).clone()));
                            }
                        }
                        let mut success: String = XmlDocumentHelper::toString(tempDocument.clone());
                        productListingPages[(index) as usize] = jcall!(StoreTransformer::new(abeClientInformation.clone(), (TransformInfoHttpSearch::new(searchRequest.clone())).clone() /* as TransformInfoInterface */)).translate(success.clone());
                    }
                }
                if savedPagesInRange == 0 {
                    let mut result: String = self.getNoResults(viewDocumentInterface.clone(), inventoryNode.clone());
                    productListingPages[(0) as usize] = jcall!(StoreTransformer::new(abeClientInformation.clone(), TransformInfoHttpSearch::new(searchRequest.clone()))).translate(result.clone());
                }
                return productListingPages.clone();
            } else {
                return None;
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).PRODUCTSEARCHLOGGINGERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("search"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
}

