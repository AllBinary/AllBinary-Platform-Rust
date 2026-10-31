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
// package org.allbinary.logic.java.jar
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::IOException::IOException;
// use crate::java::net::URL::URL;
// use crate::java::net::URLConnection::URLConnection;
// use crate::java::util::jar::JarEntry::JarEntry;
// use crate::java::util::jar::JarInputStream::JarInputStream;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct JarUtil {
    pub jarURL: Option<Rc<RefCell<URL>>>,
}


impl JarUtil {
    

    pub fn new_struct(mut jarURL: Option<Rc<RefCell<URL>>>) -> Self {
        let mut this = Self {
            jarURL: None,
        };
        this.jarURL = jarURL.clone();
        this
    }

    pub fn new(mut jarURL: Option<Rc<RefCell<URL>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(jarURL))))
    }
    

    pub fn show(&mut self) {
        let mut cache: Vec<i8> = vec![0; (1024) as usize];
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut conn: Option<Rc<RefCell<URLConnection>>> = jcall!(self.jarURL).openConnection();
            let mut jis: Option<Rc<RefCell<JarInputStream>>> = JarInputStream::new(jcall!(conn).getInputStream());
            while true {
                let mut entry: Option<Rc<RefCell<JarEntry>>> = jcall!(jis).getNextJarEntry();
                if entry.is_some() {
                    if !jcall!(entry).isDirectory() {
                        let mut offset: i32 = 0;
                        let mut i: i32 = 0;
                        while ({ i = jcall!(jis).read(cache.clone(), offset.clone(), (cache.len() as i32) - offset); i.clone() }) != -1 {
                            offset += i.clone();
                            if offset >= (cache.len() as i32) {
                                let mut newcache: Vec<i8> = vec![0; ((cache.len() as i32) + 1024) as usize];
                                
                { let java_copy_source = cache[(0) as usize..(0 + (cache.len() as i32)) as usize].to_vec(); newcache[(0) as usize..(0 + (cache.len() as i32)) as usize].clone_from_slice(&java_copy_source); };
                                cache = newcache.clone();
                            }
                        }
                        let mut tmp: Vec<i8> = vec![0; (offset) as usize];
                        
                { let java_copy_source = cache[(0) as usize..(0 + offset) as usize].to_vec(); tmp[(0) as usize..(0 + offset) as usize].clone_from_slice(&java_copy_source); };
                        println!("{}", jcall!(entry).getName());
                    }
                } else {
                    break;
                }
            }
        })) {
            Ok(_) => {}
            Err(ex) => {
            let ex: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(ex as Box<dyn std::any::Any>)));

            eprintln!("{}", ex.clone());
            eprintln!("{:?}", ex);
            }
        }
    }
    

    // throws Exception
    pub fn main(mut args: Vec<String>) {
        jcall!(JarUtil::new(URL::new(String::from("file:///G:/mnt/bc/mydev/working/j2me/MiniSpaceWars/MiniSpaceWars/dist/MiniSpaceWars.jar")))).show();
    }
}

