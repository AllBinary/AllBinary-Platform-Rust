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
// package org.allbinary.input.automation.module.generic.configuration.profile.actions.script
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::event::MouseEvent::MouseEvent;
// use crate::javax::swing::*;
// use crate::javax::swing::tree::DefaultMutableTreeNode::DefaultMutableTreeNode;
// use crate::javax::swing::tree::DefaultTreeModel::DefaultTreeModel;
// use crate::javax::swing::tree::TreePath::TreePath;
use crate::org::allbinary::input::automation::actions::script::JTreeInterface::JTreeInterface;
use crate::org::allbinary::input::automation::actions::script::condition::CustomTreeNodeInterface::CustomTreeNodeInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GenericProfileActionScriptJPanel {
    pub base: JPanel,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub defaultMutableTreeNode: Option<Rc<RefCell<DefaultMutableTreeNode>>>,
    pub genericProfileActionScript: Option<Rc<RefCell<GenericProfileActionScript>>>,
    pub actionScriptJTree: Option<Rc<RefCell<JTree>>>,
    pub jScrollPane1: Option<Rc<RefCell<JScrollPane>>>,
}

impl JTreeInterface for GenericProfileActionScriptJPanel {
    fn updateJTree(&mut self) { Self::updateJTree(self) }
    fn getActionScriptJTree(&mut self) -> Option<Rc<RefCell<JTree>>> { Self::getActionScriptJTree(self) }
}

impl GenericProfileActionScriptJPanel {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: JPanel::new_struct(),
            logUtil: None,
            commonStrings: None,
            defaultMutableTreeNode: None,
            genericProfileActionScript: None,
            actionScriptJTree: None,
            jScrollPane1: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.defaultMutableTreeNode = DefaultMutableTreeNode::new(String::from("Root"));
        jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        this.initComponents();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn updateJTree(&mut self) {
        let mut defaultTreeModel: Option<Rc<RefCell<DefaultTreeModel>>> = DefaultTreeModel::new(self.getGenericProfileActionScript());
        jcall!(self.getActionScriptJTree()).setModel(defaultTreeModel.clone());
        self.repaint();
        jcall!(self.logUtil).putF(String::from("Updated UI"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("updateJTree"));
    }
    

    pub fn getGenericProfileActionScript(&mut self) -> Option<Rc<RefCell<GenericProfileActionScript>>> {
        return self.genericProfileActionScript.clone();
    }
    

    pub fn setGenericProfileActionScript(&mut self, mut genericProfileActionScript: Option<Rc<RefCell<GenericProfileActionScript>>>) {
        self.genericProfileActionScript = genericProfileActionScript.clone();
    }
    

    pub fn getActionScriptJTree(&mut self) -> Option<Rc<RefCell<JTree>>> {
        return self.actionScriptJTree.clone();
    }
    

    pub fn setActionScriptJTree(&mut self, mut actionScriptJTree: Option<Rc<RefCell<JTree>>>) {
        self.actionScriptJTree = actionScriptJTree.clone();
    }
    

    pub fn initComponents(&mut self) {
        self.jScrollPane1 = JScrollPane::new();
        self.actionScriptJTree = JTree::new();
        self.setLayout(GridLayout::new(1, 0));
        jcall!(self.actionScriptJTree).setPreferredSize(Dimension::new(500, 500));
        { let java_receiver = self.actionScriptJTree.clone(); let java_arguments = (MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mouseClicked(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.actionScriptJTreeMouseClicked(evt.clone());
            }
            

            pub fn mouseReleased(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.actionScriptJTreeMouseReleased(evt.clone());
            }
*/,); jcall!(java_receiver).addMouseListener(java_arguments.0); };
        jcall!(self.jScrollPane1).setViewportView(self.actionScriptJTree.clone());
        self.add(self.jScrollPane1.clone());
    }
    

    pub fn actionScriptJTreeMouseReleased(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
        let mut button: i32 = jcall!(evt).getButton();
        if button == unsafe { crate::java::awt::event::MouseEvent::MouseEvent_BUTTON3.clone() } {
            let mut treePath: Option<Rc<RefCell<TreePath>>> = jcall!(self.getActionScriptJTree()).getSelectionPath();
            if treePath.is_some() {
                let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(treePath).getLastPathComponent();
                if (anyType).is_some() /* instanceof CustomTreeNodeInterface */ {
                    let mut customTreeNodeInterface: Option<Rc<RefCell<dyn CustomTreeNodeInterface>>> = (anyType).clone() /* as CustomTreeNodeInterface */;
                    let mut jPopupMenu: Option<Rc<RefCell<JPopupMenu>>> = jcall!(customTreeNodeInterface).getJPopupMenu();
                    jcall!(jPopupMenu).show((jcall!(evt).getSource()).clone() /* as JComponent */, jcall!(evt).getX(), jcall!(evt).getY());
                }
            }
        }
    }
    

    pub fn actionScriptJTreeMouseClicked(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
    }
}
impl std::ops::Deref for GenericProfileActionScriptJPanel {
    type Target = JPanel;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GenericProfileActionScriptJPanel {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



