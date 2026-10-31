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
use super::JDialog::JDialog;
use super::swing::swing;
use super::javax::javax;
use super::CloseJDialogListenerInterface::CloseJDialogListenerInterface;
use super::Frame::Frame;
use super::awt::awt;
use super::java::java;
use super::JButton::JButton;
use super::JLabel::JLabel;
use super::ActionListener::ActionListener;
use super::event::event;
use super::ActionEvent::ActionEvent;
use super::GroupLayout::GroupLayout;
use super::JFrame::JFrame;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicTextJDialog {
    pub base: JDialog,
    pub closeListenerInterface: Option<Rc<RefCell<dyn CloseJDialogListenerInterface>>>,
    pub jButton1: Option<Rc<RefCell<JButton>>>,
    pub textJLabel: Option<Rc<RefCell<JLabel>>>,
}


impl BasicTextJDialog {
    

    pub fn new_struct3(mut parent: Option<Rc<RefCell<Frame>>>, mut modal: bool, mut text: String) -> Self {
        let mut this = Self {
            base: JDialog::new_struct2(parent.clone(), modal.clone()),
            closeListenerInterface: None,
            jButton1: None,
            textJLabel: None,
        };
        // super(...) is in the struct literal above
        this.initComponents();
        this.setText(text.clone());
        this
    }

    pub fn new3(mut parent: Option<Rc<RefCell<Frame>>>, mut modal: bool, mut text: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct3(parent, modal, text))))
    }
    

    pub fn setText(&mut self, mut text: String) {
        jcall!(self.textJLabel).setText(text.clone());
    }
    

    pub fn addCloseListener(&mut self, mut closeListenerInterface: Option<Rc<RefCell<dyn CloseJDialogListenerInterface>>>) {
        self.closeListenerInterface = closeListenerInterface.clone();
    }
    

    pub fn new_struct1(mut text: String) -> Self {
        let mut this = Self {
            base: JDialog::new_struct(),
            closeListenerInterface: None,
            jButton1: None,
            textJLabel: None,
        };
        this.initComponents();
        jcall!(this.textJLabel).setText(text.clone());
        this
    }

    pub fn new1(mut text: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(text))))
    }
    

    pub fn initComponents(&mut self) {
        self.jButton1 = JButton::new();
        self.textJLabel = JLabel::new();
        self.setDefaultCloseOperation(jcall!(jcall!(unsafe { javax_swing.clone() }).WindowConstants).DISPOSE_ON_CLOSE.clone());
        jcall!(self.jButton1).setText(String::from("OK"));
        jcall!(self.jButton1).addActionListener(ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.jButton1ActionPerformed(evt.clone());
            }
*/);
        jcall!(self.textJLabel).setText(String::from("No Text Set"));
        let mut layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(self.getContentPane());
        jcall!(self.getContentPane()).setLayout(layout.clone());
        jcall!(layout).setHorizontalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addGroup(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addGap(80, 80, 80)).addComponent(self.jButton1.clone()))).addGroup(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addContainerGap()).addComponent(self.textJLabel.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 184, i16::MAX.clone())))).addContainerGap()));
        jcall!(layout).setVerticalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addGap(20, 20, 20)).addComponent(self.textJLabel.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 20, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addGap(30, 30, 30)).addComponent(self.jButton1.clone())));
        self.pack();
    }
    

    pub fn jButton1ActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        self.setVisible(false);
        if self.closeListenerInterface.is_some() {
            jcall!(self.closeListenerInterface).onClose();
        }
    }
    

    pub fn main(mut args: Vec<String>) {
        jcall!(jcall!(unsafe { java_awt.clone() }).EventQueue).invokeLater(ARunnable::new() /* Java anonymous class body: implement it as a named type
            

            pub fn run(&mut self) {
                jcall!(BasicTextJDialog::new3(JFrame::new(), true, String::from("Test"))).setVisible(true);
            }
*/);
    }
}
impl std::ops::Deref for BasicTextJDialog {
    type Target = JDialog;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicTextJDialog {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



