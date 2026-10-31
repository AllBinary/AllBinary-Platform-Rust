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
// package org.allbinary.input.automation.actions.script
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::event::ActionEvent::ActionEvent;
// use crate::java::awt::event::ActionListener::ActionListener;
// use crate::javax::swing::*;
// use crate::javax::swing::tree::DefaultMutableTreeNode::DefaultMutableTreeNode;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Node::Node;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ProfileActionScriptItem {
    pub base: DefaultMutableTreeNode,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub jPopupMenu: Option<Rc<RefCell<JPopupMenu>>>,
}

thread_local! { pub static ProfileActionScriptItem_EDIT: String = String::from("Edit"); }
thread_local! { pub static ProfileActionScriptItem_DELETE: String = String::from("Delete"); }
thread_local! { pub static ProfileActionScriptItem_NEW_CONDITION: String = String::from("New Condition"); }
thread_local! { pub static ProfileActionScriptItem_NEW_INPUT: String = String::from("New Input"); }
thread_local! { pub static ProfileActionScriptItem_NEW_OUTPUT: String = String::from("New Output"); }
thread_local! { pub static ProfileActionScriptItem_NEW_COLOR_AT: String = String::from("Color At"); }
thread_local! { pub static ProfileActionScriptItem_NEW_TIME_INTERVAL: String = String::from("Time Interval"); }
thread_local! { pub static ProfileActionScriptItem_NEW_ALWAYS: String = String::from("Always On/Off"); }
thread_local! { pub static ProfileActionScriptItem_NEW_KEY_INPUT: String = String::from("Key"); }
thread_local! { pub static ProfileActionScriptItem_NEW_MOUSE_INPUT: String = String::from("Mouse"); }
thread_local! { pub static ProfileActionScriptItem_NEW_JOYSTICK_INPUT: String = String::from("Joystick"); }
thread_local! { pub static ProfileActionScriptItem_NEW_IMAGE_OUTPUT: String = String::from("Image"); }
impl ProfileActionScriptItemInterface for ProfileActionScriptItem {
    fn process(&mut self, mut frame: Option<i64>) { Self::process(self, frame) }
}
// impl ActionListener for ProfileActionScriptItem with the methods of the trait

impl ProfileActionScriptItem {
    

    pub fn new_struct2(mut label: String, mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            base: DefaultMutableTreeNode::new_struct1(label.clone()),
            logUtil: None,
            commonStrings: None,
            jPopupMenu: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        // super(...) is in the struct literal above
        this.init();
        jcall!(this.logUtil).putF(format!("{}{}", String::from("Label: "), label), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        this
    }

    pub fn new2(mut label: String, mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(label, node))))
    }
    

    pub fn new_struct1(mut label: String) -> Self {
        let mut this = Self {
            base: DefaultMutableTreeNode::new_struct1(label.clone()),
            logUtil: None,
            commonStrings: None,
            jPopupMenu: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        // super(...) is in the struct literal above
        this.init();
        jcall!(this.logUtil).putF(format!("{}{}", String::from("Label: "), label), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        this
    }

    pub fn new1(mut label: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(label))))
    }
    

    pub fn init(&mut self) {
        self.setJPopupMenu(JPopupMenu::new());
        let mut jMenuItemEdit: Option<Rc<RefCell<JMenuItem>>> = JMenuItem::new(ProfileActionScriptItem_EDIT.with(Clone::clone).clone());
        let mut jMenuItemDelete: Option<Rc<RefCell<JMenuItem>>> = JMenuItem::new(ProfileActionScriptItem_DELETE.with(Clone::clone).clone());
        jcall!(jMenuItemEdit).addActionListener(None /* Java this: a &mut self method has no Rc of itself */);
        jcall!(jMenuItemDelete).addActionListener(None /* Java this: a &mut self method has no Rc of itself */);
        jcall!(self.getJPopupMenu()).add(jMenuItemEdit.clone());
        jcall!(self.getJPopupMenu()).add(jMenuItemDelete.clone());
    }
    

    // throws Exception
    pub fn process(&mut self, mut frame: Option<i64>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getJPopupMenu(&mut self) -> Option<Rc<RefCell<JPopupMenu>>> {
        return self.jPopupMenu.clone();
    }
    

    pub fn showDialog(&mut self) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn actionPerformed(&mut self, mut actionEvent: Option<Rc<RefCell<ActionEvent>>>) {
        if jcall!(jcall!(actionEvent).getActionCommand()).compareTo(ProfileActionScriptItem_EDIT.with(Clone::clone).clone()) == 0 {
            self.showDialog();
        }
    }
    

    pub fn setJPopupMenu(&mut self, mut jPopupMenu: Option<Rc<RefCell<JPopupMenu>>>) {
        self.jPopupMenu = jPopupMenu.clone();
    }
}
impl std::ops::Deref for ProfileActionScriptItem {
    type Target = DefaultMutableTreeNode;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ProfileActionScriptItem {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ProfileActionScriptItemDyn {
    fn as_ProfileActionScriptItem(&self) -> &ProfileActionScriptItem;
    fn as_ProfileActionScriptItem_mut(&mut self) -> &mut ProfileActionScriptItem;
    fn init(&mut self) { ProfileActionScriptItem::init(self.as_ProfileActionScriptItem_mut()) }
    fn process(&mut self, mut frame: Option<i64>) { ProfileActionScriptItem::process(self.as_ProfileActionScriptItem_mut(), frame) }
    fn getJPopupMenu(&mut self) -> Option<Rc<RefCell<JPopupMenu>>> { ProfileActionScriptItem::getJPopupMenu(self.as_ProfileActionScriptItem_mut()) }
    fn showDialog(&mut self) { ProfileActionScriptItem::showDialog(self.as_ProfileActionScriptItem_mut()) }
    fn actionPerformed(&mut self, mut actionEvent: Option<Rc<RefCell<ActionEvent>>>) { ProfileActionScriptItem::actionPerformed(self.as_ProfileActionScriptItem_mut(), actionEvent) }
    fn setJPopupMenu(&mut self, mut jPopupMenu: Option<Rc<RefCell<JPopupMenu>>>) { ProfileActionScriptItem::setJPopupMenu(self.as_ProfileActionScriptItem_mut(), jPopupMenu) }
}
impl ProfileActionScriptItemDyn for ProfileActionScriptItem {
    fn as_ProfileActionScriptItem(&self) -> &ProfileActionScriptItem { self }
    fn as_ProfileActionScriptItem_mut(&mut self) -> &mut ProfileActionScriptItem { self }
}



