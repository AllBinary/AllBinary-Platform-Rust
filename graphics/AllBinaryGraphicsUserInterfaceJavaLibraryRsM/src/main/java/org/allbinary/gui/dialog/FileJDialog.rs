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
// package org.allbinary.gui.dialog
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::thread::ARunnable::ARunnable;
use crate::java::io::File::File;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::JDialog::JDialog;
use super::FileJDialogListenerInterface::FileJDialogListenerInterface;
use super::Frame::Frame;
use super::JFileChooser::JFileChooser;
use super::ActionListener::ActionListener;
use super::ActionEvent::ActionEvent;
use super::GroupLayout::GroupLayout;
use super::JFrame::JFrame;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FileJDialog {
    pub base: JDialog,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub finishedListenerInterface: Option<Rc<RefCell<dyn FileJDialogListenerInterface>>>,
    pub jFileChooser1: Option<Rc<RefCell<JFileChooser>>>,
}


impl FileJDialog {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: JDialog::new_struct(),
            logUtil: None,
            finishedListenerInterface: None,
            jFileChooser1: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this.initComponents();
        jcall!(this.getJFileChooser1()).setMultiSelectionEnabled(true);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct2(mut parent: Option<Rc<RefCell<Frame>>>, mut modal: bool) -> Self {
        let mut this = Self {
            base: JDialog::new_struct2(parent.clone(), modal.clone()),
            logUtil: None,
            finishedListenerInterface: None,
            jFileChooser1: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this.initComponents();
        jcall!(this.getJFileChooser1()).setMultiSelectionEnabled(true);
        this
    }

    pub fn new2(mut parent: Option<Rc<RefCell<Frame>>>, mut modal: bool) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(parent, modal))))
    }
    

    pub fn addFinishedListener(&mut self, mut finishedListenerInterface: Option<Rc<RefCell<dyn FileJDialogListenerInterface>>>) {
        self.finishedListenerInterface = finishedListenerInterface.clone();
    }
    

    pub fn initComponents(&mut self) {
        self.jFileChooser1 = JFileChooser::new();
        { let java_arguments = ({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).WindowConstants.clone(); java_field }).DISPOSE_ON_CLOSE.clone(); java_field }.clone(),); self.setDefaultCloseOperation(java_arguments.0) };
        { let java_receiver = self.jFileChooser1.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.jFileChooser1ActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        let mut layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(self.getContentPane());
        jcall!(self.getContentPane()).setLayout(layout.clone());
        jcall!(layout).setHorizontalGroup(jcall!(jcall!(layout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_jdesktop.clone() }).layout.clone(); java_field }).GroupLayout.clone(); java_field }).LEADING.clone(); java_field }.clone())).add({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_jdesktop.clone() }).layout.clone(); java_field }).GroupLayout.clone(); java_field }).TRAILING.clone(); java_field }.clone(), jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addContainerGap({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_jdesktop.clone() }).layout.clone(); java_field }).GroupLayout.clone(); java_field }).DEFAULT_SIZE.clone(); java_field }.clone(), i16::MAX.clone())).add(self.jFileChooser1.clone(), { let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_jdesktop.clone() }).layout.clone(); java_field }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_jdesktop.clone() }).layout.clone(); java_field }).GroupLayout.clone(); java_field }).DEFAULT_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_jdesktop.clone() }).layout.clone(); java_field }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone())).addContainerGap()));
        jcall!(layout).setVerticalGroup(jcall!(jcall!(layout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_jdesktop.clone() }).layout.clone(); java_field }).GroupLayout.clone(); java_field }).LEADING.clone(); java_field }.clone())).add(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addContainerGap()).add(self.jFileChooser1.clone(), { let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_jdesktop.clone() }).layout.clone(); java_field }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone(), 357, { let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_jdesktop.clone() }).layout.clone(); java_field }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone())).addContainerGap({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_jdesktop.clone() }).layout.clone(); java_field }).GroupLayout.clone(); java_field }).DEFAULT_SIZE.clone(); java_field }.clone(), i16::MAX.clone())));
        self.pack();
    }
    

    pub fn jFileChooser1ActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Starting Action Command: "), jcall!(evt).getActionCommand()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("ActionPerformed"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut files: Vec<Option<Rc<RefCell<File>>>> = jcall!(self.getJFileChooser1()).getSelectedFiles();
        if match (&{ let java_value = jcall!(evt).getActionCommand(); java_value }, &{ let java_field = jcall!(self.getJFileChooser1()).CANCEL_SELECTION.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.setVisible(false);
        }
        if match (&{ let java_value = jcall!(evt).getActionCommand(); java_value }, &{ let java_field = jcall!(self.getJFileChooser1()).APPROVE_SELECTION.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            if !files.is_empty() && (files.len() as i32) > 0 {
                jcall!(self.finishedListenerInterface).onFiles(files.clone());
            }
        }
    }
    

    pub fn main(mut args: Vec<String>) {
        jcall!({ let java_field = jcall!(unsafe { java_awt.clone() }).EventQueue.clone(); java_field }).invokeLater(ARunnable::new() /* Java anonymous class body: implement it as a named type
            

            pub fn run(&mut self) {
                jcall!(FileJDialog::new2(JFrame::new(), true)).setVisible(true);
            }
*/);
    }
    

    pub fn getJFileChooser1(&mut self) -> Option<Rc<RefCell<JFileChooser>>> {
        return self.jFileChooser1.clone();
    }
    

    pub fn setJFileChooser1(&mut self, mut jFileChooser1: Option<Rc<RefCell<JFileChooser>>>) {
        self.jFileChooser1 = jFileChooser1.clone();
    }
}
impl std::ops::Deref for FileJDialog {
    type Target = JDialog;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for FileJDialog {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



