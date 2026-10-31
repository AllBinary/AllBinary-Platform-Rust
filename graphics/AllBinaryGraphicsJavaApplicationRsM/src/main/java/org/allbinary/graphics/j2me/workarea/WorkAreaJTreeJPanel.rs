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
// package org.allbinary.graphics.j2me.workarea
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::swing::*;
// use crate::javax::swing::tree::DefaultMutableTreeNode::DefaultMutableTreeNode;
// use crate::javax::swing::tree::MutableTreeNode::MutableTreeNode;
// use crate::javax::swing::tree::TreePath::TreePath;
use crate::org::allbinary::graphics::j2me::workarea::canvas::CanvasTreeLabel::CanvasTreeLabel;
use crate::org::allbinary::graphics::j2me::workarea::canvas::event::MyCanvasEvent::MyCanvasEvent;
use crate::org::allbinary::graphics::j2me::workarea::canvas::event::MyCanvasEventService::MyCanvasEventService;
use crate::org::allbinary::graphics::j2me::workarea::canvas::event::MyCanvasEventSource::MyCanvasEventSource;
use crate::org::allbinary::graphics::j2me::workarea::tools::event::MyGraphicItemEvent::MyGraphicItemEvent;
use crate::org::allbinary::graphics::j2me::workarea::tools::event::MyGraphicItemEventService::MyGraphicItemEventService;
use crate::org::allbinary::graphics::j2me::workarea::tools::event::MyGraphicItemEventSource::MyGraphicItemEventSource;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WorkAreaJTreeJPanel {
    pub base: JPanel,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub rootTreeNode: Option<Rc<RefCell<DefaultMutableTreeNode>>>,
    pub workAreaJTree: Option<Rc<RefCell<JTree>>>,
    pub highlightedBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub copyJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub deleteJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub itemJPopupMenu: Option<Rc<RefCell<JPopupMenu>>>,
    pub itemJScrollPane1: Option<Rc<RefCell<JScrollPane>>>,
    pub rotateJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub workAreaPropertiesJPanel: Option<Rc<RefCell<JPanel>>>,
}


impl WorkAreaJTreeJPanel {
    

    pub fn new_struct(mut workAreaName: String) -> Self {
        let mut this = Self {
            base: JPanel::new_struct(),
            logUtil: None,
            commonStrings: None,
            rootTreeNode: None,
            workAreaJTree: None,
            highlightedBasicArrayList: None,
            copyJMenuItem: None,
            deleteJMenuItem: None,
            itemJPopupMenu: None,
            itemJScrollPane1: None,
            rotateJMenuItem: None,
            workAreaPropertiesJPanel: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.initComponents();
        this.rootTreeNode = DefaultMutableTreeNode::new(workAreaName.clone());
        jcall!(this.workAreaPropertiesJPanel).removeAll();
        this.updateTree();
        this.highlightedBasicArrayList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new(mut workAreaName: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(workAreaName))))
    }
    

    pub fn add(&mut self, mut treeNode: Option<Rc<RefCell<MutableTreeNode>>>) {
        jcall!(self.rootTreeNode).add(treeNode.clone());
    }
    

    pub fn updateTree(&mut self) {
        self.workAreaJTree = JTree::new(self.rootTreeNode.clone());
        { let java_receiver = self.workAreaJTree.clone(); let java_arguments = (MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.workAreaJTreeMousePressed(evt.clone());
            }
*/,); jcall!(java_receiver).addMouseListener(java_arguments.0); };
        jcall!(self.workAreaPropertiesJPanel).removeAll();
        jcall!(self.workAreaPropertiesJPanel).add(self.workAreaJTree.clone());
    }
    

    pub fn getNode(&mut self, mut treePathArray: Vec<Option<Rc<RefCell<TreePath>>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut basicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        if treePathArray.is_some() {
            for index in (0..(treePathArray.len() as i32)) {
                jcall!(basicArrayList).add(crate::java_into_object(self.getNode(treePathArray[(index) as usize].clone())));
            }
        }
        return basicArrayList.clone();
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getNode(&mut self, mut treePath: Option<Rc<RefCell<TreePath>>>) -> Option<Rc<RefCell<DefaultMutableTreeNode>>> {
        if treePath.is_some() {
            let mut obj: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(treePath).getPath();
            if obj.is_some() {
                let mut node: Option<Rc<RefCell<DefaultMutableTreeNode>>> = (obj[((obj.len() as i32) - 1) as usize]).clone() /* as DefaultMutableTreeNode */;
                return node.clone();
            }
        }
        return None;
    }
    

    // throws Exception
    pub fn deselectAll(&mut self) {
        let mut size: i32 = jcall!(self.highlightedBasicArrayList).size();
        jcall!(self.logUtil).putF(format!("{}{}", String::from("size: "), size), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("deselectAll"));
        for index in (0..size) {
            let mut node: Option<Rc<RefCell<DefaultMutableTreeNode>>> = (jcall!(self.highlightedBasicArrayList).get(index.clone())).clone() /* as DefaultMutableTreeNode */;
            MyGraphicItemEventService::fire(MyGraphicItemEvent::new((MyGraphicItemEventSource::new2(unsafe { crate::org::allbinary::graphics::j2me::workarea::tools::event::MyGraphicItemEventService::MyGraphicItemEventService_DESELECT.clone() }.clone(), node.clone())).clone() /* as Any */));
        }
        jcall!(self.highlightedBasicArrayList).clear();
    }
    

    // throws Exception
    pub fn selectGraphicItem(&mut self, mut node: Option<Rc<RefCell<DefaultMutableTreeNode>>>) {
        jcall!(self.highlightedBasicArrayList).add(crate::java_into_object(node.clone()));
        MyGraphicItemEventService::fire(MyGraphicItemEvent::new((MyGraphicItemEventSource::new2(unsafe { crate::org::allbinary::graphics::j2me::workarea::tools::event::MyGraphicItemEventService::MyGraphicItemEventService_SELECT.clone() }.clone(), node.clone())).clone() /* as Any */));
    }
    

    // throws Exception
    pub fn removeGraphicItem(&mut self, mut node: Option<Rc<RefCell<DefaultMutableTreeNode>>>) {
        MyGraphicItemEventService::fire(MyGraphicItemEvent::new((MyGraphicItemEventSource::new2(unsafe { crate::org::allbinary::graphics::j2me::workarea::tools::event::MyGraphicItemEventService::MyGraphicItemEventService_DELETE.clone() }.clone(), node.clone())).clone() /* as Any */));
    }
    

    // throws Exception
    pub fn removeCanvas(&mut self, mut node: Option<Rc<RefCell<DefaultMutableTreeNode>>>) {
        MyCanvasEventService::fire(MyCanvasEvent::new((MyCanvasEventSource::new2(unsafe { crate::org::allbinary::graphics::j2me::workarea::canvas::event::MyCanvasEventService::MyCanvasEventService_DELETE.clone() }.clone(), node.clone())).clone() /* as Any */));
    }
    

    // throws Exception
    pub fn remove(&mut self, mut treePath: Option<Rc<RefCell<TreePath>>>) {
        let mut node: Option<Rc<RefCell<DefaultMutableTreeNode>>> = self.getNode(treePath.clone());
        if node.is_some() {
            if (jcall!(node).getUserObject()).is_some() /* instanceof CanvasTreeLabel */ {
                self.removeCanvas(node.clone());
            } else {
                self.removeGraphicItem(node.clone());
            }
        }
        let mut parent: Option<Rc<RefCell<DefaultMutableTreeNode>>> = (jcall!(node).getParent()).clone() /* as DefaultMutableTreeNode */;
        jcall!(parent).remove(node.clone());
    }
    

    // throws Exception
    pub fn copy(&mut self) {
        let mut treePath: Option<Rc<RefCell<TreePath>>> = jcall!(self.workAreaJTree).getSelectionPath();
        self.copy1(treePath.clone());
        self.updateTree();
        self.expand();
        self.repaint();
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn copy1(&mut self, mut treePath: Option<Rc<RefCell<TreePath>>>) {
        let mut node: Option<Rc<RefCell<DefaultMutableTreeNode>>> = self.getNode(treePath.clone());
        if node.is_some() {
            if (jcall!(node).getUserObject()).is_some() /* instanceof CanvasTreeLabel */ {
                MyCanvasEventService::fire(MyCanvasEvent::new((MyCanvasEventSource::new2(unsafe { crate::org::allbinary::graphics::j2me::workarea::canvas::event::MyCanvasEventService::MyCanvasEventService_DUPLICATE.clone() }.clone(), node.clone())).clone() /* as Any */));
            } else {
                MyGraphicItemEventService::fire(MyGraphicItemEvent::new((MyGraphicItemEventSource::new2(unsafe { crate::org::allbinary::graphics::j2me::workarea::tools::event::MyGraphicItemEventService::MyGraphicItemEventService_DUPLICATE.clone() }.clone(), node.clone())).clone() /* as Any */));
            }
        }
    }
    

    // throws Exception
    pub fn rotate(&mut self, mut treePath: Option<Rc<RefCell<TreePath>>>, mut angle: f64) {
        let mut node: Option<Rc<RefCell<DefaultMutableTreeNode>>> = self.getNode(treePath.clone());
        if node.is_some() {
            if (jcall!(node).getUserObject()).is_some() /* instanceof CanvasTreeLabel */ {
                MyCanvasEventService::fire(MyCanvasEvent::new((MyCanvasEventSource::new3(unsafe { crate::org::allbinary::graphics::j2me::workarea::canvas::event::MyCanvasEventService::MyCanvasEventService_ROTATE.clone() }.clone(), angle.clone(), node.clone())).clone() /* as Any */));
            } else {
                MyGraphicItemEventService::fire(MyGraphicItemEvent::new((MyGraphicItemEventSource::new3(unsafe { crate::org::allbinary::graphics::j2me::workarea::tools::event::MyGraphicItemEventService::MyGraphicItemEventService_ROTATE.clone() }.clone(), angle.clone(), node.clone())).clone() /* as Any */));
            }
        }
    }
    

    pub fn getRootTreeNode(&mut self) -> Option<Rc<RefCell<MutableTreeNode>>> {
        return self.rootTreeNode.clone();
    }
    

    pub fn expand(&mut self) {
        jcall!(self.workAreaJTree).expandRow(0);
        jcall!(self.workAreaJTree).expandRow(1);
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn expand1(&mut self, mut treePath: Option<Rc<RefCell<TreePath>>>) {
        jcall!(self.workAreaJTree).expandPath(treePath.clone());
    }
    

    pub fn initComponents(&mut self) {
        self.itemJPopupMenu = JPopupMenu::new();
        self.deleteJMenuItem = JMenuItem::new();
        self.copyJMenuItem = JMenuItem::new();
        self.rotateJMenuItem = JMenuItem::new();
        self.itemJScrollPane1 = JScrollPane::new();
        self.workAreaPropertiesJPanel = JPanel::new();
        jcall!(self.itemJPopupMenu).addPopupMenuListener(PopupMenuListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn popupMenuCanceled(&mut self, mut evt: Option<Rc<RefCell<PopupMenuEvent>>>) {
            }
            

            pub fn popupMenuWillBecomeInvisible(&mut self, mut evt: Option<Rc<RefCell<PopupMenuEvent>>>) {
                self.itemJPopupMenuPopupMenuWillBecomeInvisible(evt.clone());
            }
            

            pub fn popupMenuWillBecomeVisible(&mut self, mut evt: Option<Rc<RefCell<PopupMenuEvent>>>) {
                self.itemJPopupMenuPopupMenuWillBecomeVisible(evt.clone());
            }
*/);
        jcall!(self.deleteJMenuItem).setText(String::from("Delete"));
        { let java_receiver = self.deleteJMenuItem.clone(); let java_arguments = (MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.deleteJMenuItemMousePressed(evt.clone());
            }
*/,); jcall!(java_receiver).addMouseListener(java_arguments.0); };
        jcall!(self.itemJPopupMenu).add(self.deleteJMenuItem.clone());
        jcall!(self.copyJMenuItem).setText(String::from("Copy"));
        { let java_receiver = self.copyJMenuItem.clone(); let java_arguments = (MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.copyJMenuItemMousePressed(evt.clone());
            }
*/,); jcall!(java_receiver).addMouseListener(java_arguments.0); };
        jcall!(self.itemJPopupMenu).add(self.copyJMenuItem.clone());
        jcall!(self.rotateJMenuItem).setText(String::from("Rotate 45*"));
        { let java_receiver = self.rotateJMenuItem.clone(); let java_arguments = (MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.rotateJMenuItemMousePressed(evt.clone());
            }
*/,); jcall!(java_receiver).addMouseListener(java_arguments.0); };
        jcall!(self.itemJPopupMenu).add(self.rotateJMenuItem.clone());
        self.setLayout(GridLayout::new(1, 1));
        { let java_receiver = self.workAreaPropertiesJPanel.clone(); let java_arguments = (MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.workAreaPropertiesJPanelMousePressed(evt.clone());
            }
*/,); jcall!(java_receiver).addMouseListener(java_arguments.0); };
        jcall!(self.workAreaPropertiesJPanel).setLayout(GridLayout::new(1, 0));
        jcall!(self.itemJScrollPane1).setViewportView(self.workAreaPropertiesJPanel.clone());
        self.add(self.itemJScrollPane1.clone());
    }
    

    pub fn rotateJMenuItemMousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut treePath: Option<Rc<RefCell<TreePath>>> = jcall!(self.workAreaJTree).getSelectionPath();
            self.rotate(treePath.clone(), 45);
            self.updateTree();
            self.expand();
            self.repaint();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("rotateJMenuItemMousePressed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn workAreaJTreeMousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut selectedTreePathArray: Vec<Option<Rc<RefCell<TreePath>>>> = jcall!(self.workAreaJTree).getSelectionPaths();
            let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getNode(selectedTreePathArray.clone());
            if jcall!(list).size() > 0 {
                if match (&(jcall!(evt).getModifiers() & jcall!(evt).BUTTON3_MASK), &jcall!(evt).BUTTON3_MASK) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    for index in (0..(selectedTreePathArray.len() as i32)) {
                        let mut defaultMutableTreeNode: Option<Rc<RefCell<DefaultMutableTreeNode>>> = (jcall!(list).get(index.clone())).clone() /* as DefaultMutableTreeNode */;
                        if (jcall!(defaultMutableTreeNode).getUserObject()).is_some() /* instanceof CanvasTreeLabel */ {
                            { let java_receiver = self.itemJPopupMenu.clone(); let java_arguments = (None /* Java this: a &mut self method has no Rc of itself */, jcall!(evt).getX(), jcall!(evt).getY(),); jcall!(java_receiver).show(java_arguments.0, java_arguments.1, java_arguments.2); };
                        } else {
                            { let java_receiver = self.itemJPopupMenu.clone(); let java_arguments = (None /* Java this: a &mut self method has no Rc of itself */, jcall!(evt).getX(), jcall!(evt).getY(),); jcall!(java_receiver).show(java_arguments.0, java_arguments.1, java_arguments.2); };
                        }
                    }
                }
                if match (&(jcall!(evt).getModifiers() & jcall!(evt).BUTTON1_MASK), &jcall!(evt).BUTTON1_MASK) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    self.deselectAll();
                    for index in (0..(selectedTreePathArray.len() as i32)) {
                        let mut defaultMutableTreeNode: Option<Rc<RefCell<DefaultMutableTreeNode>>> = (jcall!(list).get(index.clone())).clone() /* as DefaultMutableTreeNode */;
                        if (jcall!(defaultMutableTreeNode).getUserObject()).is_some() /* instanceof CanvasTreeLabel */ {
                            MyCanvasEventService::fire(MyCanvasEvent::new((MyCanvasEventSource::new2(unsafe { crate::org::allbinary::graphics::j2me::workarea::canvas::event::MyCanvasEventService::MyCanvasEventService_SELECT.clone() }.clone(), defaultMutableTreeNode.clone())).clone() /* as Any */));
                        } else {
                            self.selectGraphicItem(defaultMutableTreeNode.clone());
                        }
                    }
                }
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("workAreaJTreeMousePressed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn itemJPopupMenuPopupMenuWillBecomeInvisible(&mut self, mut evt: Option<Rc<RefCell<PopupMenuEvent>>>) {
    }
    

    pub fn itemJPopupMenuPopupMenuWillBecomeVisible(&mut self, mut evt: Option<Rc<RefCell<PopupMenuEvent>>>) {
    }
    

    pub fn copyJMenuItemMousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.copy();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("copyJMenuItemMousePressed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn deleteJMenuItemMousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut selectedTreePathArray: Vec<Option<Rc<RefCell<TreePath>>>> = jcall!(self.workAreaJTree).getSelectionPaths();
            for index in (0..(selectedTreePathArray.len() as i32)) {
                let mut treePath: Option<Rc<RefCell<TreePath>>> = selectedTreePathArray[(index) as usize].clone();
                self.remove(treePath.clone());
                self.updateTree();
                self.expand();
                self.repaint();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("deleteJMenuItemMousePressed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn workAreaPropertiesJPanelMousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
    }
}
impl std::ops::Deref for WorkAreaJTreeJPanel {
    type Target = JPanel;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for WorkAreaJTreeJPanel {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



