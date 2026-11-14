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
// package views.admin.inventory.download
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::ByteArrayInputStream::ByteArrayInputStream;
use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontFactory::StoreFrontFactory;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::user::commerce::inventory::item::BasicItemData::BasicItemData;
use crate::org::allbinary::business::user::commerce::inventory::item::BasicItemView::BasicItemView;
use crate::org::allbinary::business::user::commerce::inventory::item::ItemInterface::ItemInterface;
use crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadableItem::DownloadableItem;
use crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadableItemView::DownloadableItemView;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::http::request::MultipartRequestParams::MultipartRequestParams;
use crate::org::allbinary::logic::communication::http::request::RequestMapInterface::RequestMapInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::FilePathData::FilePathData;
use crate::org::allbinary::logic::io::file::FileUtil::FileUtil;
use crate::org::allbinary::logic::io::file::zip::ZipFileUtil::ZipFileUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::apache::commons::fileupload::FileItem::FileItem;
use crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DownloadableInventoryItemView {
    pub base: HttpStoreComponentView,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub basicItemData: Option<Rc<RefCell<BasicItemData>>>,
    pub request: Option<Rc<RefCell<HttpServletRequest>>>,
    pub id: String,
    pub itemInterface: Option<Rc<RefCell<dyn ItemInterface>>>,
    pub downloadableItem: Option<Rc<RefCell<DownloadableItem>>>,
    pub requestHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>,
}

pub static mut DownloadableInventoryItemView_TYPE_ID: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| 11);
impl RequestMapInterface for DownloadableInventoryItemView {
    fn getRequestHashMap(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> { Self::getRequestHashMap(self) }
}
impl crate::org::allbinary::logic::visual::transform::info::CompositeTransformInfoInterface::CompositeTransformInfoInterface for DownloadableInventoryItemView {
    fn getTransformInfoInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface>>> { crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite::getTransformInfoInterface(crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpCompositeDyn::as_TransformInfoHttpComposite_mut(self)) }
}

impl DownloadableInventoryItemView {
    

    pub fn new_struct1(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: HttpStoreComponentView::new_struct(transformInfoInterface.clone()),
            logUtil: None,
            basicItemData: None,
            request: None,
            id: String::new(),
            itemInterface: None,
            downloadableItem: None,
            requestHashMap: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.basicItemData = BasicItemData::getInstance();
        // super(...) is in the struct literal above
        this.request = { let java_value = ((jcall!(this.getPageContext()).getRequest()).clone() /* as HttpServletRequest */).clone(); java_value };
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
            id: String::new(),
            itemInterface: None,
            downloadableItem: None,
            requestHashMap: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.basicItemData = BasicItemData::getInstance();
        // super(...) is in the struct literal above
        this.request = { let java_value = ((jcall!(this.getPageContext()).getRequest()).clone() /* as HttpServletRequest */).clone(); java_value };
        this
    }

    pub fn new2(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut empty: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(transformInfoInterface, empty))))
    }
    

    pub fn getTypeId(&mut self) -> i32 {
        return unsafe { DownloadableInventoryItemView_TYPE_ID.clone() }.clone();
    }
    

    // throws Exception
    pub fn getFormData(&mut self) {
        { let java_arguments = (jcall!(MultipartRequestParams::new1(self.request.clone())).toHashMap(),); self.setRequestHashMap(java_arguments.0) };
        self.id = { let java_value = ({ let java_object = (jcall!(self.getRequestHashMap()).get(crate::java_into_object({ let java_field = jcall!(self.basicItemData).ID.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
    }
    

    pub fn addDomNodeInterfaces(&mut self) {
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        if self.downloadableItem.is_some() {
            jcall!(vector).add(crate::java_into_object(DownloadableItemView::new(self.downloadableItem.clone())));
        }
        { let java_arguments = (BasicItemView::new(self.itemInterface.clone(), vector.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface>>),); self.addDomNodeInterface(java_arguments.0) };
    }
    

    // throws Exception
    pub fn view(&mut self) -> String {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.addDomNodeInterfaces();
            return Some(self.base.view());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEWERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("view()"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // throws Exception
    pub fn processFile(&mut self, mut fileItem: Option<Rc<RefCell<FileItem>>>) {
        let mut fileName: String = jcall!(fileItem).getName();
        let mut fullPath: String = self.getItemFilePath();
        let mut itemResourceFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(fullPath.clone());
        jcall!(itemResourceFile).mkdir();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(fullPath.clone());
        jcall!(stringBuffer).append(fileName.clone());
        jcall!(self.logUtil).putF(format!("{}{}", String::from("FileName: "), fileName), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("processFile()"));
        let mut file: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(jcall!(stringBuffer).toString());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(file).getPath(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("processFiles()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        jcall!(file).createNewFile();
        let mut byteArray: Vec<i8> = jcall!(fileItem).get();
        jcall!(FileUtil::getInstance()).write(crate::java::io::InputStream::InputStream::new1(byteArray.clone()).map(|reference| reference as Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>), file.clone());
    }
    

    // throws Exception
    pub fn unzip(&mut self, mut fileItem: Option<Rc<RefCell<FileItem>>>) {
        let mut fileName: String = jcall!(fileItem).getName();
        let mut fullPath: String = self.getItemFilePath();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(fullPath.clone());
        jcall!(stringBuffer).append(fileName.clone());
        jcall!(self.logUtil).putF(format!("{}{}", String::from("FileName: "), fileName), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("unzip()"));
        let mut file: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(jcall!(stringBuffer).toString());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(file).getPath(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("unzip()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        jcall!(ZipFileUtil::getInstance()).unzip(fullPath.clone(), file.clone(), fileName.clone());
    }
    

    pub fn getItemFilePath(&mut self) -> String {
        let mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>> = StoreFrontFactory::getInstance({ let java_argument = jcall!(self.getWeblisketSession()).getStoreName(); java_argument }).map(|reference| reference as Rc<RefCell<dyn StoreFrontInterface>>);
        let mut filePathData: Option<Rc<RefCell<FilePathData>>> = FilePathData::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(URLGLOBALS::getWebappPath());
        jcall!(stringBuffer).append(jcall!(storeFrontInterface).getCurrentHostNamePath());
        jcall!(stringBuffer).append(jcall!(self.itemInterface).getCategory());
        jcall!(stringBuffer).appendchar({ let java_field = jcall!(filePathData).SEPARATORCHAR.clone(); java_field }.clone());
        jcall!(stringBuffer).append(jcall!(self.itemInterface).getId());
        jcall!(stringBuffer).appendchar({ let java_field = jcall!(filePathData).SEPARATORCHAR.clone(); java_field }.clone());
        jcall!(stringBuffer).append(jcall!(self.downloadableItem).getId());
        jcall!(stringBuffer).appendchar({ let java_field = jcall!(filePathData).SEPARATORCHAR.clone(); java_field }.clone());
        return jcall!(stringBuffer).toString();
    }
    

    pub fn setRequestHashMap(&mut self, mut requestHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) {
        self.requestHashMap = requestHashMap.clone();
    }
    

    pub fn getRequestHashMap(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> {
        return self.requestHashMap.clone();
    }
    

    pub fn getDownloadableItem(&mut self) -> Option<Rc<RefCell<DownloadableItem>>> {
        return self.downloadableItem.clone();
    }
}
impl std::ops::Deref for DownloadableInventoryItemView {
    type Target = HttpStoreComponentView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for DownloadableInventoryItemView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait DownloadableInventoryItemViewDyn: crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn {
    fn as_DownloadableInventoryItemView(&self) -> &DownloadableInventoryItemView;
    fn as_DownloadableInventoryItemView_mut(&mut self) -> &mut DownloadableInventoryItemView;
    fn getFormData(&mut self) { DownloadableInventoryItemView::getFormData(self.as_DownloadableInventoryItemView_mut()) }
    fn addDomNodeInterfaces(&mut self) { DownloadableInventoryItemView::addDomNodeInterfaces(self.as_DownloadableInventoryItemView_mut()) }
    fn processFile(&mut self, mut fileItem: Option<Rc<RefCell<FileItem>>>) { DownloadableInventoryItemView::processFile(self.as_DownloadableInventoryItemView_mut(), fileItem) }
    fn unzip(&mut self, mut fileItem: Option<Rc<RefCell<FileItem>>>) { DownloadableInventoryItemView::unzip(self.as_DownloadableInventoryItemView_mut(), fileItem) }
    fn getItemFilePath(&mut self) -> String { DownloadableInventoryItemView::getItemFilePath(self.as_DownloadableInventoryItemView_mut()) }
    fn setRequestHashMap(&mut self, mut requestHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) { DownloadableInventoryItemView::setRequestHashMap(self.as_DownloadableInventoryItemView_mut(), requestHashMap) }
    fn getRequestHashMap(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> { DownloadableInventoryItemView::getRequestHashMap(self.as_DownloadableInventoryItemView_mut()) }
    fn getDownloadableItem(&mut self) -> Option<Rc<RefCell<DownloadableItem>>> { DownloadableInventoryItemView::getDownloadableItem(self.as_DownloadableInventoryItemView_mut()) }
}
impl DownloadableInventoryItemViewDyn for DownloadableInventoryItemView {
    fn as_DownloadableInventoryItemView(&self) -> &DownloadableInventoryItemView { self }
    fn as_DownloadableInventoryItemView_mut(&mut self) -> &mut DownloadableInventoryItemView { self }
}

impl crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpCompositeDyn for DownloadableInventoryItemView {
    fn as_TransformInfoHttpComposite(&self) -> &crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &self.base.base.base }
    fn as_TransformInfoHttpComposite_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &mut self.base.base.base }
}
impl crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn for DownloadableInventoryItemView {
    fn as_HttpStoreComponentView(&self) -> &crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &self.base }
    fn as_HttpStoreComponentView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &mut self.base }
}
impl crate::views::HttpComponentView::HttpComponentViewDyn for DownloadableInventoryItemView {
    fn as_HttpComponentView(&self) -> &crate::views::HttpComponentView::HttpComponentView { &self.base.base }
    fn as_HttpComponentView_mut(&mut self) -> &mut crate::views::HttpComponentView::HttpComponentView { &mut self.base.base }
    fn getTypeId(&mut self) -> i32 { Self::getTypeId(self) }
    fn view(&mut self) -> String { Self::view(self) }
}


