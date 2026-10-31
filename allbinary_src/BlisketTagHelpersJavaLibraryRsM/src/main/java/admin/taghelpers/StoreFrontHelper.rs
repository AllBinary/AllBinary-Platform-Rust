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
// package admin.taghelpers
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::business::context::modules::storefront::StoreFront::StoreFront;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontFactory::StoreFrontFactory;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::regex::replace::Replace::Replace;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use super::TagHelper::TagHelper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StoreFrontHelper {
    pub base: TagHelper,
    pub storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>,
    pub currentLocation: String,
    pub currentHomeLocation: String,
}


impl StoreFrontHelper {
    

    pub fn new_struct(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            base: TagHelper::new_struct(),
            storeFrontInterface: None,
            currentLocation: String::new(),
            currentHomeLocation: String::new(),
        };
        let mut storeName: String = (jcall!(hashMap).get(jcall!(StoreFrontData::getInstance()).NAME.clone())).clone() /* as String */;
        if true {
            this.storeFrontInterface = StoreFrontFactory::getInstance(storeName.clone());
            this.currentLocation = jcall!(Replace::new2(jcall!(CommonSeps::getInstance()).SPACE.clone(), String::from("%20"))).all(jcall!(this.storeFrontInterface).getCurrentHostName() + jcall!(this.storeFrontInterface).getCurrentHostNamePath());
            this.currentHomeLocation = jcall!(Replace::new2(jcall!(CommonSeps::getInstance()).SPACE.clone(), String::from("%20"))).all(jcall!(this.storeFrontInterface).getCurrentHomeHostName() + jcall!(this.storeFrontInterface).getCurrentHomeHostNamePath());
        } else {
            let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
            this.storeFrontInterface = StoreFront::new().map(|reference| reference as Rc<RefCell<dyn StoreFrontInterface>>);
            this.currentLocation = jcall!(stringUtil).EMPTY_STRING.clone();
            this.currentHomeLocation = jcall!(stringUtil).EMPTY_STRING.clone();
        }
        this
    }

    pub fn new(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(hashMap, pageContext))))
    }
    

    pub fn getCurrentLocation(&mut self) -> String {
        return self.currentLocation.clone();
    }
    

    pub fn getCurrentHomeLocation(&mut self) -> String {
        return self.currentHomeLocation.clone();
    }
}
impl std::ops::Deref for StoreFrontHelper {
    type Target = TagHelper;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for StoreFrontHelper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::admin::taghelpers::TagHelper::TagHelperDyn for StoreFrontHelper {
    fn as_TagHelper(&self) -> &crate::admin::taghelpers::TagHelper::TagHelper { &self.base }
    fn as_TagHelper_mut(&mut self) -> &mut crate::admin::taghelpers::TagHelper::TagHelper { &mut self.base }
}
impl crate::admin::taghelpers::TagHelperInterface::TagHelperInterface for StoreFrontHelper {
}


