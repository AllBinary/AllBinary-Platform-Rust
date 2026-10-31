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
// package org.allbinary.logic.java.proxy
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::lang::reflect::Method::Method;
// use crate::java::lang::reflect::Proxy::Proxy;
use super::InvocationHandler::InvocationHandler;
use super::reflect::reflect;
use super::lang::lang;
use super::java::java;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InterfaceCastProxy {
    pub anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>,
}

// impl InvocationHandler for InterfaceCastProxy with the methods of the trait

impl InterfaceCastProxy {
    

    pub fn new_struct(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Self {
        let mut this = Self {
            anyType: None,
        };
        this.anyType = anyType.clone();
        this
    }

    pub fn new(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(anyType))))
    }
    

    pub fn newInstance(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return Proxy::newProxyInstance(jcall!(std::any::type_name_of_val(&anyType)).getClassLoader(), jcall!(std::any::type_name_of_val(&anyType)).getInterfaces(), InterfaceCastProxy::new(anyType.clone()));
    }
    

    // throws Throwable
    pub fn invoke(&mut self, mut proxyObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut proxyMethod: Option<Rc<RefCell<Method>>>, mut proxyArgs: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        let mut methodName: String = jcall!(proxyMethod).getName();
        let mut realMethod: Option<Rc<RefCell<Method>>> = jcall!(std::any::type_name_of_val(&self.anyType)).getMethod(methodName.clone(), jcall!(proxyMethod).getParameterTypes());
        if !jcall!(realMethod).isAccessible() {
            jcall!(realMethod).setAccessible(true);
        }
        return jcall!(proxyMethod).invoke(self.anyType.clone(), proxyArgs.clone());
    }
}

