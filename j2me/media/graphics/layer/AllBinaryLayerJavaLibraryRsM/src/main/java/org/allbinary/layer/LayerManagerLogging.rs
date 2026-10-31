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
// package org.allbinary.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use super::LayerManagerLoggingBase::LayerManagerLoggingBase;
use super::AllBinaryLayer::AllBinaryLayer;
use super::LayerManager::LayerManager;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

fn java_string_hash(text: &str) -> i32 {
    text.chars().fold(0i32, |hash, character| hash.wrapping_mul(31).wrapping_add(character as i32))
}

pub struct LayerManagerLogging {
    pub base: LayerManagerLoggingBase,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub stringBuilder: Option<Rc<RefCell<StringMaker>>>,
    pub APPEND: String,
    pub APPEND_: String,
    pub _AT_: String,
    pub ATTEMPT_REMOVE_: String,
    pub REMOVE_: String,
    pub REMOVE: String,
    pub DID_NOT_REMOVE: String,
    pub CLEAR: String,
}

thread_local! { pub static LayerManagerLogging_instance: Option<Rc<RefCell<LayerManagerLogging>>> = LayerManagerLogging::new(); }
pub static mut LayerManagerLogging_removeFailed: std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| false);

impl LayerManagerLogging {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: LayerManagerLoggingBase::new_struct(),
            logUtil: None,
            stringBuilder: None,
            APPEND: String::new(),
            APPEND_: String::new(),
            _AT_: String::new(),
            ATTEMPT_REMOVE_: String::new(),
            REMOVE_: String::new(),
            REMOVE: String::new(),
            DID_NOT_REMOVE: String::new(),
            CLEAR: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.stringBuilder = StringMaker::new();
        this.APPEND = String::from("append");
        this.APPEND_ = String::from(" append: ");
        this._AT_ = String::from(" at: ");
        this.ATTEMPT_REMOVE_ = String::from(" Remove Attempt: ");
        this.REMOVE_ = String::from(" Remove: ");
        this.REMOVE = String::from("remove");
        this.DID_NOT_REMOVE = String::from(" Did not remove: ");
        this.CLEAR = String::from(" Clear List");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<LayerManagerLogging>>> {
        return LayerManagerLogging_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn append(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) {
        { let java_receiver = self.stringBuilder.clone(); let java_arguments = (0, jcall!(self.stringBuilder).length(),); jcall!(java_receiver).delete(java_arguments.0, java_arguments.1); };
        { let java_receiver = self.logUtil.clone(); let java_arguments = ({ let java_receiver = self.stringBuilder.clone(); let java_arguments = (jcall!(TsUtil::getInstance()).hashCode(None /* Java this: a &mut self method has no Rc of itself */),); let java_result = jcall!(java_receiver).appendint(java_arguments.0); java_result }.append(self.APPEND_.clone()).append(jcall!(layerInterface).getName()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.APPEND.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn appendAt(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut index: i32) {
        { let java_receiver = self.stringBuilder.clone(); let java_arguments = (0, jcall!(self.stringBuilder).length(),); jcall!(java_receiver).delete(java_arguments.0, java_arguments.1); };
        { let java_receiver = self.logUtil.clone(); let java_arguments = ({ let java_receiver = self.stringBuilder.clone(); let java_arguments = (jcall!(TsUtil::getInstance()).hashCode(None /* Java this: a &mut self method has no Rc of itself */),); let java_result = jcall!(java_receiver).appendint(java_arguments.0); java_result }.append(self.APPEND_.clone()).append(jcall!(layerInterface).getName()).append(self._AT_.clone()).appendint(index.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.APPEND.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn remove(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) {
        if layerInterface.is_none() {
            { let java_receiver = self.stringBuilder.clone(); let java_arguments = (0, jcall!(self.stringBuilder).length(),); jcall!(java_receiver).delete(java_arguments.0, java_arguments.1); };
            { let java_receiver = self.logUtil.clone(); let java_arguments = ({ let java_receiver = self.stringBuilder.clone(); let java_arguments = (jcall!(TsUtil::getInstance()).hashCode(None /* Java this: a &mut self method has no Rc of itself */),); let java_result = jcall!(java_receiver).appendint(java_arguments.0); java_result }.append(self.ATTEMPT_REMOVE_.clone()).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(layerInterface.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.REMOVE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        } else {
            { let java_receiver = self.stringBuilder.clone(); let java_arguments = (0, jcall!(self.stringBuilder).length(),); jcall!(java_receiver).delete(java_arguments.0, java_arguments.1); };
            { let java_receiver = self.logUtil.clone(); let java_arguments = ({ let java_receiver = self.stringBuilder.clone(); let java_arguments = (jcall!(TsUtil::getInstance()).hashCode(None /* Java this: a &mut self method has no Rc of itself */),); let java_result = jcall!(java_receiver).appendint(java_arguments.0); java_result }.append(self.ATTEMPT_REMOVE_.clone()).append(jcall!(layerInterface).getName()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.REMOVE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
    }
    

    pub fn removeResult(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerManager::LayerManagerDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut result: bool) {
        if layerInterface.is_none() {
            { let java_receiver = self.stringBuilder.clone(); let java_arguments = (0, jcall!(self.stringBuilder).length(),); jcall!(java_receiver).delete(java_arguments.0, java_arguments.1); };
            { let java_receiver = self.logUtil.clone(); let java_arguments = ({ let java_receiver = self.stringBuilder.clone(); let java_arguments = (jcall!(TsUtil::getInstance()).hashCode(None /* Java this: a &mut self method has no Rc of itself */),); let java_result = jcall!(java_receiver).appendint(java_arguments.0); java_result }.append(self.REMOVE_.clone()).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(layerInterface.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.REMOVE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        } else if result {
            if unsafe { LayerManagerLogging_removeFailed.clone() } {
                { let java_receiver = self.stringBuilder.clone(); let java_arguments = (0, jcall!(self.stringBuilder).length(),); jcall!(java_receiver).delete(java_arguments.0, java_arguments.1); };
                { let java_receiver = self.logUtil.clone(); let java_arguments = ({ let java_receiver = self.stringBuilder.clone(); let java_arguments = (jcall!(TsUtil::getInstance()).hashCode(None /* Java this: a &mut self method has no Rc of itself */),); let java_result = jcall!(java_receiver).appendint(java_arguments.0); java_result }.append(self.REMOVE_.clone()).append(jcall!(layerInterface).getName()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.REMOVE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
        } else {
            { let java_receiver = self.stringBuilder.clone(); let java_arguments = (0, jcall!(self.stringBuilder).length(),); jcall!(java_receiver).delete(java_arguments.0, java_arguments.1); };
            { let java_receiver = self.logUtil.clone(); let java_arguments = ({ let java_receiver = self.stringBuilder.clone(); let java_arguments = (jcall!(TsUtil::getInstance()).hashCode(None /* Java this: a &mut self method has no Rc of itself */),); let java_result = jcall!(java_receiver).appendint(java_arguments.0); java_result }.append(self.DID_NOT_REMOVE.clone()).append(jcall!(layerInterface).getName()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.REMOVE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            unsafe { *LayerManagerLogging_removeFailed = true };
        }
    }
    

    pub fn log(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerManager::LayerManagerDyn>>>) {
        { let java_receiver = self.stringBuilder.clone(); let java_arguments = (0, jcall!(self.stringBuilder).length(),); jcall!(java_receiver).delete(java_arguments.0, java_arguments.1); };
        let mut size: i32 = jcall!(layerManager).getSize();
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        { let java_receiver = self.stringBuilder.clone(); let java_arguments = (format!("{:x}", jcall!(TsUtil::getInstance()).hashCode(crate::java_into_object(layerManager.clone()))),); let java_result = jcall!(java_receiver).append(java_arguments.0); java_result }.append(jcall!(commonSeps).COLON_SEP.clone());
        jcall!(self.stringBuilder).appendint(size.clone()).append(jcall!(commonSeps).COLON_SEP.clone());
        let mut allBinaryLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = None;
        for index in (0..size) {
            allBinaryLayer = ((jcall!(layerManager).getLayerAt(index.clone())).clone() /* as AllBinaryLayer */);
            { let java_receiver = self.stringBuilder.clone(); let java_arguments = (jcall!(allBinaryLayer).getName(),); let java_result = jcall!(java_receiver).append(java_arguments.0); java_result }.append(jcall!(commonSeps).COMMA.clone());
        }
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(self.stringBuilder).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.REMOVE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn clear(&mut self) {
        { let java_receiver = self.stringBuilder.clone(); let java_arguments = (0, jcall!(self.stringBuilder).length(),); jcall!(java_receiver).delete(java_arguments.0, java_arguments.1); };
        { let java_receiver = self.logUtil.clone(); let java_arguments = ({ let java_receiver = self.stringBuilder.clone(); let java_arguments = (jcall!(TsUtil::getInstance()).hashCode(None /* Java this: a &mut self method has no Rc of itself */),); let java_result = jcall!(java_receiver).appendint(java_arguments.0); java_result }.append(self.CLEAR.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.CLEAR.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
}
impl std::ops::Deref for LayerManagerLogging {
    type Target = LayerManagerLoggingBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for LayerManagerLogging {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::layer::LayerManagerLoggingBase::LayerManagerLoggingBaseDyn for LayerManagerLogging {
    fn as_LayerManagerLoggingBase(&self) -> &crate::org::allbinary::layer::LayerManagerLoggingBase::LayerManagerLoggingBase { &self.base }
    fn as_LayerManagerLoggingBase_mut(&mut self) -> &mut crate::org::allbinary::layer::LayerManagerLoggingBase::LayerManagerLoggingBase { &mut self.base }
    fn append(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) { Self::append(self, layerInterface) }
    fn appendAt(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut index: i32) { Self::appendAt(self, layerInterface, index) }
    fn remove(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) { Self::remove(self, layerInterface) }
    fn removeResult(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerManager::LayerManagerDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut result: bool) { Self::removeResult(self, layerManager, layerInterface, result) }
    fn clear(&mut self) { Self::clear(self) }
}


