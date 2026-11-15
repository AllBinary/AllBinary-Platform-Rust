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
use super::*;
use crate::java::util::HashMap::HashMap;
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
use crate::java::lang::Exception::*;
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

pub static InventorySearchUtil_instance: crate::JavaStatic<Option<Rc<RefCell<InventorySearchUtil>>>> = crate::JavaStatic::new(|| InventorySearchUtil::new());

impl InventorySearchUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

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
        let mut column: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(inventorySearchUtil).getColumnWhereLike(inventoryEntityInterface.clone(), jcall!(storeFrontInterface).getCategoryPath(), { let java_field = jcall!(self.basicItemData).ID.clone(); java_field }.clone());
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).PRODUCTSEARCHLOGGING.clone(); java_field }.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Number Of Items Found: "), jcall!(column).size()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("search"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut subStoreVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(storeFrontInterface).getSubStores();
        let mut size: i32 = jcall!(subStoreVector).size();
        for index in (0..size) {
            let mut subStore: String = { let java_object = (jcall!(subStoreVector).get(index.clone())).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
            let mut substoreIdColumn: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(inventorySearchUtil).getColumnWhereLike(inventoryEntityInterface.clone(), subStore.clone(), { let java_field = jcall!(self.basicItemData).ID.clone(); java_field }.clone());
            jcall!(column).addAllList(substoreIdColumn.clone());
        }
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).PRODUCTSEARCHLOGGING.clone(); java_field }.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Number Of Items Found Including SubStores: "), jcall!(column).size()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("search"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        return column.clone();
    }
    

    // throws Exception
    pub fn getNoResults(&mut self, mut viewDocumentInterface: Option<Rc<RefCell<dyn TransformDocumentInterface>>>, mut inventoryNode: Option<Rc<RefCell<Node>>>) -> String {
        jcall!(inventoryNode).appendChild(ModDomHelper::createNameValueNodes(jcall!(viewDocumentInterface).getDoc(), crate::org::allbinary::logic::control::search::SearchData::SearchData_TOTAL_NUMBER_PAGES.with(Clone::clone).clone(), { let java_field = jcall!(self.commonPhoneStrings).ZERO.clone(); java_field }.clone()));
        jcall!(inventoryNode).appendChild(ModDomHelper::createNameValueNodes(jcall!(viewDocumentInterface).getDoc(), crate::org::allbinary::logic::control::search::SearchData::SearchData_TOTAL_NUMBER_ITEMS.with(Clone::clone).clone(), { let java_field = jcall!(self.commonPhoneStrings).ZERO.clone(); java_field }.clone()));
        let mut success: String = XmlDocumentHelper::toString(jcall!(viewDocumentInterface).getDoc());
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).PRODUCTSEARCHLOGGING.clone(); java_field }.clone()) {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("No Results Xml: "), success), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("search"));
        }
        return success.clone();
    }
    

    pub fn search(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut searchRequest: Option<Rc<RefCell<SearchRequest>>>, mut column: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Vec<String> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut inventoryEntityInterface: Option<Rc<RefCell<InventoryEntity>>> = jcall!(InventoryEntityFactory::getInstance()).getInventoryEntityInstance();
            let mut searchParams: Option<Rc<RefCell<SearchParams>>> = jcall!(searchRequest).getParams();
            let mut startPage: i32 = ((jcall!(searchParams).getStartPageInt()).unwrap() as i32);
            let mut endPage: i32 = ((jcall!(searchParams).getEndPageInt()).unwrap() as i32);
            let mut pageLength: i32 = ((jcall!(searchParams).getLengthInt()).unwrap() as i32);
            let mut savedPagesInRange: i32 = 0;
            let mut columnValueHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(searchParams).get();
            let mut keyword: String = jcall!(Replace::new2(String::from("-"), { let java_field = jcall!(CommonSeps::getInstance()).SPACE.clone(); java_field }.clone())).all({ let java_object = (jcall!(columnValueHashMap).get(crate::java_into_object({ let java_field = jcall!(self.basicItemData).KEYWORDS.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() });
            if jcall!(StringValidationUtil::getInstance()).isEmpty(keyword.clone()) {
                return Some(None);
            }
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            if startPage >= 0 {
                if pageLength == 0 {
                    pageLength = { let java_value = (crate::org::allbinary::logic::control::search::SearchData::SearchData_DEFAULT_PAGE_LENGTH.with(Clone::clone)).clone(); java_value }.clone();
                }
                let mut MAXPAGES: i32 = 100;
                let mut productListingPages: Vec<String> = vec![String::new(); (MAXPAGES) as usize];
                let mut documents: Vec<Option<Rc<RefCell<Document>>>> = vec![None; (MAXPAGES) as usize];
                let mut inventoryNodes: Vec<Option<Rc<RefCell<Node>>>> = vec![None; (MAXPAGES) as usize];
                keyword = { let java_value = (keyword.to_uppercase()).clone(); java_value };
                let mut lastPage: i32 = -1;
                let mut numberOfResultsOnCurrentPage: i32 = 0;
                let mut numberOfResults: i32 = 0;
                let mut viewDocumentInterface: Option<Rc<RefCell<dyn TransformDocumentInterface>>> = TransformStoreDocumentFactory::getInstance(searchRequest.clone());
                let mut inventoryNode: Option<Rc<RefCell<Node>>> = jcall!(jcall!(viewDocumentInterface).getDoc()).createElement(crate::org::allbinary::business::user::commerce::inventory::InventoryData::InventoryData_INVENTORY.with(Clone::clone).clone());
                jcall!(jcall!(viewDocumentInterface).getBaseNode()).appendChild(inventoryNode.clone());
                jcall!(inventoryNode).appendChild(ModDomHelper::createNameValueNodes(jcall!(viewDocumentInterface).getDoc(), crate::org::allbinary::logic::control::search::SearchData::SearchData_PAGE.with(Clone::clone).clone(), (Some((startPage.clone()) as i32)).expect("NullPointerException").to_string()));
                if true {
                    jcall!(inventoryNode).appendChild(jcall!(searchRequest).getFileBaseNameNode(jcall!(viewDocumentInterface).getDoc()));
                }
                let mut currentPage: i32 = -1;
                let mut size: i32 = jcall!(column).size();
                for index in (0..size) {
                    let mut product: String = { let java_object = (jcall!(column).get(index.clone())).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }.clone();
                    let mut itemInterface: Option<Rc<RefCell<dyn ItemInterface>>> = jcall!(inventoryEntityInterface).getItem(product.clone());
                    let mut keywords: String = jcall!(itemInterface).getKeywords();
                    keywords = { let java_value = (keywords.to_uppercase()).clone(); java_value };
                    if keywords.find(&keyword.clone()).map(|index| index as i32).unwrap_or(-1) != -1 {
                        if currentPage + 1 >= startPage && currentPage + 1 <= endPage {
                            let mut itemNode: Option<Rc<RefCell<Node>>> = jcall!(BasicItemView::new(itemInterface.clone(), BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>))).toXmlNode(jcall!(viewDocumentInterface).getDoc());
                            jcall!(itemNode).appendChild(ModDomHelper::createNameValueNodes(jcall!(viewDocumentInterface).getDoc(), crate::org::allbinary::business::user::commerce::inventory::basket::BasketData::BasketData_ITEMTOTALINBASKET.with(Clone::clone).clone(), { let java_field = jcall!(self.commonPhoneStrings).ONE.clone(); java_field }.clone()));
                            jcall!(inventoryNode).appendChild(jcall!(itemNode).cloneNode(true));
                        }
                        currentPage = ((numberOfResults + 1) / pageLength) - 1;
                        numberOfResultsOnCurrentPage += 1;
                        if lastPage < currentPage {
                            if currentPage >= startPage && currentPage <= endPage {
                                jcall!(inventoryNode).appendChild(ModDomHelper::createNameValueNodes(jcall!(viewDocumentInterface).getDoc(), crate::org::allbinary::logic::control::search::SearchData::SearchData_TOTAL_NUMBER_ITEMS_ON_THIS_PAGE.with(Clone::clone).clone(), (Some((numberOfResultsOnCurrentPage.clone()) as i32)).expect("NullPointerException").to_string()));
                                documents[(currentPage) as usize] = jcall!(viewDocumentInterface).getDoc();
                                inventoryNodes[(currentPage) as usize] = inventoryNode.clone();
                                savedPagesInRange += 1;
                                viewDocumentInterface = { let java_value = (TransformStoreDocumentFactory::getInstance(searchRequest.clone())).clone(); java_value };
                                inventoryNode = { let java_value = (jcall!(jcall!(viewDocumentInterface).getDoc()).createElement(crate::org::allbinary::business::user::commerce::inventory::InventoryData::InventoryData_INVENTORY.with(Clone::clone).clone())).clone(); java_value };
                                jcall!(jcall!(viewDocumentInterface).getBaseNode()).appendChild(inventoryNode.clone());
                                jcall!(inventoryNode).appendChild(ModDomHelper::createNameValueNodes(jcall!(viewDocumentInterface).getDoc(), crate::org::allbinary::logic::control::search::SearchData::SearchData_PAGE.with(Clone::clone).clone(), (Some((currentPage + 1) as i32)).expect("NullPointerException").to_string()));
                                if true {
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
                    jcall!(inventoryNode).appendChild(ModDomHelper::createNameValueNodes(jcall!(viewDocumentInterface).getDoc(), crate::org::allbinary::logic::control::search::SearchData::SearchData_TOTAL_NUMBER_ITEMS_ON_THIS_PAGE.with(Clone::clone).clone(), (Some((numberOfResultsOnCurrentPage - 1) as i32)).expect("NullPointerException").to_string()));
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
                        { let java_receiver = inventoryNodes[(index) as usize].clone(); let java_arguments = (ModDomHelper::createNameValueNodes_Document_String_HashMap(tempDocument.clone(), crate::org::allbinary::logic::control::search::SearchData::SearchData_TOTAL_NUMBER_PAGES.with(Clone::clone).clone(), (Some((lastPage + 1) as i32)).expect("NullPointerException").to_string()),); jcall!(java_receiver).appendChild(java_arguments.0); };
                        { let java_receiver = inventoryNodes[(index) as usize].clone(); let java_arguments = (ModDomHelper::createNameValueNodes_Document_String_HashMap(tempDocument.clone(), crate::org::allbinary::logic::control::search::SearchData::SearchData_TOTAL_NUMBER_ITEMS.with(Clone::clone).clone(), (Some((numberOfResults.clone()) as i32)).expect("NullPointerException").to_string()),); jcall!(java_receiver).appendChild(java_arguments.0); };
                        for pageIndex in (0..=lastPage) {
                            jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                            jcall!(stringBuffer).append(jcall!(searchRequest).getFileBaseName());
                            if jcall!(stringBuffer).length() > 0 {
                                if pageIndex != 0 {
                                    jcall!(stringBuffer).append((pageIndex.clone()).to_string());
                                }
                                { let java_receiver = inventoryNodes[(index) as usize].clone(); let java_arguments = (ModDomHelper::createNameValueNodes4_Document_String_String_BasicArrayList(tempDocument.clone(), crate::org::allbinary::logic::control::search::SearchData::SearchData_PAGE_INFO.with(Clone::clone).clone(), (pageIndex.clone()).to_string(), jcall!(stringBuffer).toString()),); jcall!(java_receiver).appendChild(java_arguments.0); };
                            } else {
                                { let java_receiver = inventoryNodes[(index) as usize].clone(); let java_arguments = (ModDomHelper::createNameValueNodes4_Document_String_String_BasicArrayList(tempDocument.clone(), crate::org::allbinary::logic::control::search::SearchData::SearchData_PAGE_INFO.with(Clone::clone).clone(), (pageIndex.clone()).to_string(), crate::org::allbinary::logic::control::search::SearchData::SearchData_SEARCH.with(Clone::clone).clone()),); jcall!(java_receiver).appendChild(java_arguments.0); };
                            }
                        }
                        let mut success: String = XmlDocumentHelper::toString(tempDocument.clone());
                        productListingPages[(index) as usize] = jcall!(StoreTransformer::new(abeClientInformation.clone(), (TransformInfoHttpSearch::new(searchRequest.clone())).clone() /* as TransformInfoInterface */)).translate(success.clone());
                    }
                }
                if savedPagesInRange == 0 {
                    let mut result: String = self.getNoResults(viewDocumentInterface.clone(), inventoryNode.clone());
                    productListingPages[(0) as usize] = jcall!(StoreTransformer::new(abeClientInformation.clone(), TransformInfoHttpSearch::new(searchRequest.clone()).map(|reference| reference as Rc<RefCell<dyn TransformInfoInterface>>))).translate(result.clone());
                }
                return Some(productListingPages.clone());
            } else {
                return Some(None);
            }None
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).PRODUCTSEARCHLOGGINGERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).FAILURE.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("search"), crate::java_into_object(e.clone()));
            }
            return None;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
}

