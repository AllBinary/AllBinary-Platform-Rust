// Generated Code Do Not Modify
// package org.allbinary.android
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::android::app::Activity::Activity;
use crate::android::view::View::View;
use crate::android::view::ViewGroup::ViewGroup;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ViewSwapper {
    pub rootViewId: i32,
    pub activity: Option<Rc<RefCell<Activity>>>,
    pub currentMainViewId: i32,
    pub mainViewArray: Vec<Option<Rc<RefCell<View>>>>,
    pub topViewArray: Vec<Option<Rc<RefCell<View>>>>,
}

thread_local! { pub static ViewSwapper_NULL_VIEW_ARRAY: Vec<Option<Rc<RefCell<View>>>> = vec![None; (0) as usize]; }

impl ViewSwapper {
    

    pub fn new_struct(mut activity: Option<Rc<RefCell<Activity>>>, mut rootViewId: i32) -> Self {
        let mut this = Self {
            rootViewId: 0,
            activity: None,
            currentMainViewId: 0,
            mainViewArray: Vec::new(),
            topViewArray: Vec::new(),
        };
        this.currentMainViewId = -1;
        this.mainViewArray = ViewSwapper_NULL_VIEW_ARRAY.with(Clone::clone).clone();
        this.topViewArray = ViewSwapper_NULL_VIEW_ARRAY.with(Clone::clone).clone();
        this.activity = activity.clone();
        this.rootViewId = rootViewId.clone();
        this
    }

    pub fn new(mut activity: Option<Rc<RefCell<Activity>>>, mut rootViewId: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(activity, rootViewId))))
    }
    

    pub fn setMainViews(&mut self, mut viewArray: Vec<Option<Rc<RefCell<View>>>>) {
        self.mainViewArray = viewArray.clone();
    }
    

    pub fn setTopViewArray(&mut self, mut topViewArray: Vec<Option<Rc<RefCell<View>>>>) {
        self.topViewArray = topViewArray.clone();
    }
    

    pub fn setMainView(&mut self, mut id: i32) {
        let mut viewGroup: Option<Rc<RefCell<ViewGroup>>> = (jcall!(self.activity).findViewById(self.rootViewId.clone())).clone() /* as ViewGroup */;
        if id != self.currentMainViewId {
            for index in (0..(self.topViewArray.len() as i32)) {
                jcall!(viewGroup).removeView(self.topViewArray[(index) as usize].clone());
            }
            for index in (0..(self.mainViewArray.len() as i32)) {
                if id != jcall!(self.mainViewArray[(index) as usize]).getId() {
                    jcall!(viewGroup).removeView(self.mainViewArray[(index) as usize].clone());
                }
            }
            for index in (0..(self.mainViewArray.len() as i32)) {
                if id == jcall!(self.mainViewArray[(index) as usize]).getId() {
                    jcall!(viewGroup).addView(self.mainViewArray[(index) as usize].clone());
                    self.currentMainViewId = id.clone();
                }
            }
            for index in (0..(self.topViewArray.len() as i32)) {
                jcall!(viewGroup).addView(self.topViewArray[(index) as usize].clone());
            }
        }
    }
}

