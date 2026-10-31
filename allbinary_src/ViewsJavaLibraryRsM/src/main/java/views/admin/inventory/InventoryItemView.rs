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
// package views.admin.inventory
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::Set::Set;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontFactory::StoreFrontFactory;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem;
use crate::org::allbinary::business::user::commerce::inventory::item::BasicItemData::BasicItemData;
use crate::org::allbinary::business::user::commerce::inventory::item::BasicItemView::BasicItemView;
use crate::org::allbinary::business::user::commerce::inventory::item::ItemInterface::ItemInterface;
use crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadableItem::DownloadableItem;
use crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadableItemView::DownloadableItemView;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::http::file::upload::HttpFileUploadUtil::HttpFileUploadUtil;
use crate::org::allbinary::logic::communication::http::request::HttpRequestUtil::HttpRequestUtil;
use crate::org::allbinary::logic::communication::http::request::MultipartRequestParams::MultipartRequestParams;
use crate::org::allbinary::logic::communication::http::request::RequestMapInterface::RequestMapInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::io::path::PathUtil::PathUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::visual::media::MediaData::MediaData;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::apache::commons::fileupload::FileItem::FileItem;
use crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView;
use super::InventoryUploadMediaUtil::InventoryUploadMediaUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InventoryItemView {
    pub base: HttpStoreComponentView,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub basicItemData: Option<Rc<RefCell<BasicItemData>>>,
    pub request: Option<Rc<RefCell<HttpServletRequest>>>,
    pub imageFileName: String,
    pub mediaData: Option<Rc<RefCell<MediaData>>>,
    pub itemInterface: Option<Rc<RefCell<dyn ItemInterface>>>,
    pub downloadableItemVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub requestHashMap: Option<Rc<RefCell<HashMap>>>,
}

pub static mut InventoryItemView_TYPE_ID: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| 10);
impl RequestMapInterface for InventoryItemView {
    fn getRequestHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { Self::getRequestHashMap(self) }
}

impl InventoryItemView {
    

    pub fn new_struct1(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: HttpStoreComponentView::new_struct(transformInfoInterface.clone()),
            logUtil: None,
            basicItemData: None,
            request: None,
            imageFileName: String::new(),
            mediaData: None,
            itemInterface: None,
            downloadableItemVector: None,
            requestHashMap: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.basicItemData = BasicItemData::getInstance();
        // super(...) is in the struct literal above
        this.request = (jcall!(this.getPageContext()).getRequest()).clone() /* as HttpServletRequest */;
        this.getFormData();
        this
    }

    pub fn new1(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(transformInfoInterface))))
    }
    

    pub fn new_struct2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut empty: String) -> Self {
        let mut this = Self {
            base: HttpStoreComponentView::new_struct(transformInfoInterface.clone()),
            logUtil: None,
            basicItemData: None,
            request: None,
            imageFileName: String::new(),
            mediaData: None,
            itemInterface: None,
            downloadableItemVector: None,
            requestHashMap: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.basicItemData = BasicItemData::getInstance();
        // super(...) is in the struct literal above
        this.request = (jcall!(this.getPageContext()).getRequest()).clone() /* as HttpServletRequest */;
        this
    }

    pub fn new2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut empty: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(transformInfoInterface, empty))))
    }
    

    pub fn getTypeId(&mut self) -> i32 {
        return unsafe { InventoryItemView_TYPE_ID.clone() }.clone();
    }
    

    // throws Exception
    pub fn getFormData(&mut self) {
        { let java_arguments = (jcall!(MultipartRequestParams::new1(self.request.clone())).toHashMap(),); self.setRequestHashMap(java_arguments.0) };
        let mut imageFileItemObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.getRequestHashMap()).get(jcall!(self.basicItemData).IMAGE.clone());
        if jcall!(HttpFileUploadUtil::getInstance()).isValid(crate::java_into_object(imageFileItemObject.clone())) {
            let mut fileItem: Option<Rc<RefCell<FileItem>>> = (imageFileItemObject).clone() /* as FileItem */;
            if fileItem.is_some() && jcall!(fileItem).getSize() > 1 {
                self.imageFileName = jcall!(HttpRequestUtil::getInstance()).generateFileName(jcall!(fileItem).getName());
                let mut pathData: Option<Rc<RefCell<AbPathData>>> = AbPathData::getInstance();
                let mut pathUtil: Option<Rc<RefCell<PathUtil>>> = PathUtil::getInstance();
                self.mediaData = MediaData::get(jcall!(pathData).getExtension(self.imageFileName.clone()));
                self.imageFileName = jcall!(pathUtil).getWithoutExtension(self.imageFileName.clone());
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                    let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                    jcall!(stringBuffer).append(String::from("Uploaded File Data: "));
                    jcall!(stringBuffer).append(self.imageFileName.clone());
                    jcall!(stringBuffer).append(String::from(" Extension: "));
                    jcall!(stringBuffer).append(jcall!(self.mediaData).getName());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getFormData()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
            }
        }
        self.itemInterface = (BasicItem::new1(self.getRequestHashMap()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn>>)).clone() /* as ItemInterface */;
    }
    

    pub fn addDomNodeInterfaces(&mut self) {
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut downloadableItem: Option<Rc<RefCell<DownloadableItem>>> = None;
        let mut size: i32 = jcall!(self.downloadableItemVector).size();
        for index in (0..size) {
            downloadableItem = { let java_object = (jcall!(self.downloadableItemVector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<DownloadableItem>>>>().expect("ClassCastException").clone()) };
            jcall!(vector).add(crate::java_into_object(DownloadableItemView::new(downloadableItem.clone())));
        }
        self.addDomNodeInterface(BasicItemView::new(self.itemInterface.clone(), vector.clone()));
    }
    

    pub fn getItemInterface(&mut self) -> Option<Rc<RefCell<dyn ItemInterface>>> {
        return self.itemInterface.clone();
    }
    

    // throws Exception
    pub fn view(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.addDomNodeInterfaces();
            return self.base.view();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("view()"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    pub fn processImageFiles(&mut self) {
        let mut set: Option<Rc<RefCell<dyn Set>>> = jcall!(self.getRequestHashMap()).keySet();
        let mut fieldNameArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(set).toArray();
        let mut size: i32 = (fieldNameArray.len() as i32).clone();
        for index in (0..size) {
            let mut fieldName: String = (fieldNameArray[(index) as usize]).clone() /* as String */;
            if (fieldName.cmp(&jcall!(self.basicItemData).IMAGE.clone()) as i32) == 0 {
                let mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>> = StoreFrontFactory::getInstance(jcall!(self.getWeblisketSession()).getStoreName());
                let mut inventoryUploadMediaUtil: Option<Rc<RefCell<InventoryUploadMediaUtil>>> = InventoryUploadMediaUtil::new(storeFrontInterface.clone(), self.itemInterface.clone());
                let mut fileItem: Option<Rc<RefCell<FileItem>>> = (jcall!(self.getRequestHashMap()).get(jcall!(self.basicItemData).IMAGE.clone())).clone() /* as FileItem */;
                self.itemInterface = jcall!(inventoryUploadMediaUtil).saveFiles(jcall!(fileItem).get(), self.imageFileName.clone(), self.mediaData.clone());
            }
        }
    }
    

    pub fn setRequestHashMap(&mut self, mut requestHashMap: Option<Rc<RefCell<HashMap>>>) {
        self.requestHashMap = requestHashMap.clone();
    }
    

    pub fn getRequestHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        return self.requestHashMap.clone();
    }
}
impl std::ops::Deref for InventoryItemView {
    type Target = HttpStoreComponentView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for InventoryItemView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait InventoryItemViewDyn: crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn {
    fn as_InventoryItemView(&self) -> &InventoryItemView;
    fn as_InventoryItemView_mut(&mut self) -> &mut InventoryItemView;
    fn getFormData(&mut self) { InventoryItemView::getFormData(self.as_InventoryItemView_mut()) }
    fn addDomNodeInterfaces(&mut self) { InventoryItemView::addDomNodeInterfaces(self.as_InventoryItemView_mut()) }
    fn getItemInterface(&mut self) -> Option<Rc<RefCell<dyn ItemInterface>>> { InventoryItemView::getItemInterface(self.as_InventoryItemView_mut()) }
    fn processImageFiles(&mut self) { InventoryItemView::processImageFiles(self.as_InventoryItemView_mut()) }
    fn setRequestHashMap(&mut self, mut requestHashMap: Option<Rc<RefCell<HashMap>>>) { InventoryItemView::setRequestHashMap(self.as_InventoryItemView_mut(), requestHashMap) }
    fn getRequestHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { InventoryItemView::getRequestHashMap(self.as_InventoryItemView_mut()) }
}
impl InventoryItemViewDyn for InventoryItemView {
    fn as_InventoryItemView(&self) -> &InventoryItemView { self }
    fn as_InventoryItemView_mut(&mut self) -> &mut InventoryItemView { self }
}

impl crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn for InventoryItemView {
    fn as_HttpStoreComponentView(&self) -> &crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &self.base }
    fn as_HttpStoreComponentView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &mut self.base }
}
impl crate::views::HttpComponentView::HttpComponentViewDyn for InventoryItemView {
    fn as_HttpComponentView(&self) -> &crate::views::HttpComponentView::HttpComponentView { &self.base.base }
    fn as_HttpComponentView_mut(&mut self) -> &mut crate::views::HttpComponentView::HttpComponentView { &mut self.base.base }
    fn getTypeId(&mut self) -> i32 { Self::getTypeId(self) }
    fn view(&mut self) -> String { Self::view(self) }
}


