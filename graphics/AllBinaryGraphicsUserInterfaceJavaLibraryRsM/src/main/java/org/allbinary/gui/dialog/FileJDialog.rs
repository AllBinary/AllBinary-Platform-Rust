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
// use crate::java::io::File::File;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::JDialog::JDialog;
use super::swing::swing;
use super::javax::javax;
use super::FileJDialogListenerInterface::FileJDialogListenerInterface;
use super::Frame::Frame;
use super::awt::awt;
use super::java::java;
use super::JFileChooser::JFileChooser;
use super::ActionListener::ActionListener;
use super::event::event;
use super::ActionEvent::ActionEvent;
use super::GroupLayout::GroupLayout;
use super::layout::layout;
use super::jdesktop::jdesktop;
use super::org::org;
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
        self.setDefaultCloseOperation(jcall!(jcall!(unsafe { javax_swing.clone() }).WindowConstants).DISPOSE_ON_CLOSE.clone());
        { let java_receiver = self.jFileChooser1.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.jFileChooser1ActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        let mut layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(self.getContentPane());
        jcall!(self.getContentPane()).setLayout(layout.clone());
        jcall!(layout).setHorizontalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone())).add(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).TRAILING.clone(), jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addContainerGap(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone())).add(self.jFileChooser1.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone())).addContainerGap()));
        jcall!(layout).setVerticalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone())).add(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addContainerGap()).add(self.jFileChooser1.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone(), 357, jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone())).addContainerGap(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone())));
        self.pack();
    }
    

    pub fn jFileChooser1ActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Starting Action Command: "), jcall!(evt).getActionCommand()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("ActionPerformed"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut files: Vec<Option<Rc<RefCell<File>>>> = jcall!(self.getJFileChooser1()).getSelectedFiles();
        if match (&jcall!(evt).getActionCommand(), &jcall!(self.getJFileChooser1()).CANCEL_SELECTION) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.setVisible(false);
        }
        if match (&jcall!(evt).getActionCommand(), &jcall!(self.getJFileChooser1()).APPROVE_SELECTION) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            if files.is_some() && (files.len() as i32) > 0 {
                jcall!(self.finishedListenerInterface).onFiles(files.clone());
            }
        }
    }
    

    pub fn main(mut args: Vec<String>) {
        jcall!(jcall!(unsafe { java_awt.clone() }).EventQueue).invokeLater(ARunnable::new() /* Java anonymous class body: implement it as a named type
            

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



