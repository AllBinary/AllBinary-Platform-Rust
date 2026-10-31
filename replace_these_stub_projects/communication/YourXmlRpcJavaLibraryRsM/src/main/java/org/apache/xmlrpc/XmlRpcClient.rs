// Generated Code Do Not Modify
// package org.apache.xmlrpc
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::IOException::IOException;
// use crate::java::net::MalformedURLException::MalformedURLException;
// use crate::java::net::URL::URL;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::init::crypt::jcehelper::CryptInterface::CryptInterface;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use super::XmlRpcHandler::XmlRpcHandler;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct XmlRpcClient {
    pub url: Option<Rc<RefCell<URL>>>,
}

impl XmlRpcHandler for XmlRpcClient {
    fn execute(&mut self, mut method: String, mut params: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::execute(self, method, params) }
}

impl XmlRpcClient {
    

    pub fn new_struct(mut url: String) -> Self {
        let mut this = Self {
            url: None,
        };
        this.url = URL::new(url.clone());
        this
    }

    pub fn new(mut url: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(url))))
    }
    

    pub fn getURL(&mut self) -> Option<Rc<RefCell<URL>>> {
        return self.url.clone();
    }
    

    pub fn setBasicAuthentication(&mut self, mut user: String, mut password: String) {
    }
    

    // throws XmlRpcException, IOException
    pub fn execute(&mut self, mut method: String, mut params: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return jcall!(NullUtil::getInstance()).NULL_OBJECT.clone();
    }
    

    // throws XmlRpcException, IOException
    // Java overloaded method: Rust needs a different name
    pub fn execute3(&mut self, mut method: String, mut params: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut cryptInterface: Option<Rc<RefCell<dyn CryptInterface>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return jcall!(NullUtil::getInstance()).NULL_OBJECT.clone();
    }
}
pub trait XmlRpcClientDyn: crate::org::apache::xmlrpc::XmlRpcHandler::XmlRpcHandler {
    fn as_XmlRpcClient(&self) -> &XmlRpcClient;
    fn as_XmlRpcClient_mut(&mut self) -> &mut XmlRpcClient;
    fn getURL(&mut self) -> Option<Rc<RefCell<URL>>> { XmlRpcClient::getURL(self.as_XmlRpcClient_mut()) }
    fn setBasicAuthentication(&mut self, mut user: String, mut password: String) { XmlRpcClient::setBasicAuthentication(self.as_XmlRpcClient_mut(), user, password) }
    fn execute3(&mut self, mut method: String, mut params: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut cryptInterface: Option<Rc<RefCell<dyn CryptInterface>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { XmlRpcClient::execute3(self.as_XmlRpcClient_mut(), method, params, cryptInterface) }
}
impl XmlRpcClientDyn for XmlRpcClient {
    fn as_XmlRpcClient(&self) -> &XmlRpcClient { self }
    fn as_XmlRpcClient_mut(&mut self) -> &mut XmlRpcClient { self }
}


