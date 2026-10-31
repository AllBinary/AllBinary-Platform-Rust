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
// package org.allbinary.gui.swing.workers
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::swing::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct JListSwingWorker {
    pub base: SwingWorker,
    pub jList: Option<Rc<RefCell<JList>>>,
    pub listModel: Option<Rc<RefCell<ListModel>>>,
}


impl JListSwingWorker {
    

    pub fn new_struct(mut jList: Option<Rc<RefCell<JList>>>, mut listModel: Option<Rc<RefCell<ListModel>>>) -> Self {
        let mut this = Self {
            base: SwingWorker::new_struct(),
            jList: None,
            listModel: None,
        };
        this.setJList(jList.clone());
        this.setListModel(listModel.clone());
        this
    }

    pub fn new(mut jList: Option<Rc<RefCell<JList>>>, mut listModel: Option<Rc<RefCell<ListModel>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(jList, listModel))))
    }
    

    pub fn doInBackground(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return None;
    }
    

    pub fn done(&mut self) {
        jcall!(self.getJList()).setModel(self.getListModel());
    }
    

    pub fn getJList(&mut self) -> Option<Rc<RefCell<JList>>> {
        return self.jList.clone();
    }
    

    pub fn setJList(&mut self, mut jList: Option<Rc<RefCell<JList>>>) {
        self.jList = jList.clone();
    }
    

    pub fn getListModel(&mut self) -> Option<Rc<RefCell<ListModel>>> {
        return self.listModel.clone();
    }
    

    pub fn setListModel(&mut self, mut listModel: Option<Rc<RefCell<ListModel>>>) {
        self.listModel = listModel.clone();
    }
}
impl std::ops::Deref for JListSwingWorker {
    type Target = SwingWorker;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for JListSwingWorker {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



