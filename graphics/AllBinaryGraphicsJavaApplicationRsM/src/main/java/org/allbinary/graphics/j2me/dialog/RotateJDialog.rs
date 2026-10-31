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
// package org.allbinary.graphics.j2me.dialog
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
use crate::org::allbinary::thread::ARunnable::ARunnable;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::gui::dialog::HashMapJDialogListenerInterface::HashMapJDialogListenerInterface;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::JDialog::JDialog;
use super::swing::swing;
use super::javax::javax;
use super::Frame::Frame;
use super::awt::awt;
use super::java::java;
use super::JTextField::JTextField;
use super::JLabel::JLabel;
use super::JPanel::JPanel;
use super::JButton::JButton;
use super::ActionListener::ActionListener;
use super::event::event;
use super::ActionEvent::ActionEvent;
use super::GroupLayout::GroupLayout;
use super::JFrame::JFrame;
use super::WindowAdapter::WindowAdapter;
use super::WindowEvent::WindowEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RotateJDialog {
    pub base: JDialog,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub hashMap: Option<Rc<RefCell<HashMap>>>,
    pub angleIncrementJTextField: Option<Rc<RefCell<JTextField>>>,
    pub jButton1: Option<Rc<RefCell<JButton>>>,
    pub jLabel1: Option<Rc<RefCell<JLabel>>>,
    pub jLabel2: Option<Rc<RefCell<JLabel>>>,
    pub jPanel1: Option<Rc<RefCell<JPanel>>>,
    pub totalRotationJTextField: Option<Rc<RefCell<JTextField>>>,
}


impl RotateJDialog {
    

    pub fn new_struct(mut parent: Option<Rc<RefCell<Frame>>>, mut modal: bool) -> Self {
        let mut this = Self {
            base: JDialog::new_struct2(parent.clone(), modal.clone()),
            logUtil: None,
            hashMap: None,
            angleIncrementJTextField: None,
            jButton1: None,
            jLabel1: None,
            jLabel2: None,
            jPanel1: None,
            totalRotationJTextField: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.hashMap = jcall!(StdUtil::getInstance()).createHashMap();
        // super(...) is in the struct literal above
        this.initComponents();
        this
    }

    pub fn new(mut parent: Option<Rc<RefCell<Frame>>>, mut modal: bool) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(parent, modal))))
    }
    

    pub fn initComponents(&mut self) {
        self.angleIncrementJTextField = JTextField::new();
        self.jLabel1 = JLabel::new();
        self.totalRotationJTextField = JTextField::new();
        self.jLabel2 = JLabel::new();
        self.jPanel1 = JPanel::new();
        self.jButton1 = JButton::new();
        self.setDefaultCloseOperation(jcall!(jcall!(unsafe { javax_swing.clone() }).WindowConstants).DISPOSE_ON_CLOSE.clone());
        jcall!(self.angleIncrementJTextField).setText(String::from("36"));
        jcall!(self.jLabel1).setText(String::from("Angle Increment:"));
        jcall!(self.totalRotationJTextField).setText(String::from("360"));
        jcall!(self.jLabel2).setText(String::from("Total Angular Rotation:"));
        jcall!(self.jButton1).setText(String::from("OK"));
        { let java_receiver = self.jButton1.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.jButton1ActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        let mut jPanel1Layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(self.jPanel1.clone());
        jcall!(self.jPanel1).setLayout(jPanel1Layout.clone());
        jcall!(jPanel1Layout).setHorizontalGroup(jcall!(jcall!(jPanel1Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jPanel1Layout).createSequentialGroup()).addGap(62, 62, 62)).addComponent(self.jButton1.clone())).addContainerGap(62, i16::MAX.clone())));
        jcall!(jPanel1Layout).setVerticalGroup(jcall!(jcall!(jPanel1Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addComponent(self.jButton1.clone()));
        let mut layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(self.getContentPane());
        jcall!(self.getContentPane()).setLayout(layout.clone());
        jcall!(layout).setHorizontalGroup(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addContainerGap()).addGroup(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone())).addComponent(self.jLabel1.clone())).addComponent(self.jLabel2.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).UNRELATED.clone())).addGroup(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addComponent(self.totalRotationJTextField.clone(), jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 31, i16::MAX.clone())).addComponent(self.angleIncrementJTextField.clone(), jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 31, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addContainerGap())).addComponent(self.jPanel1.clone(), jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone()));
        jcall!(layout).setVerticalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addContainerGap()).addGroup(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.angleIncrementJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addComponent(self.jLabel1.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.totalRotationJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addComponent(self.jLabel2.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone(), 16, i16::MAX.clone())).addComponent(self.jPanel1.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addContainerGap()));
        self.pack();
    }
    

    pub fn jButton1ActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let java_receiver = self.hashMap.clone(); let java_arguments = ((0).to_string(), (jcall!(self.angleIncrementJTextField).getText()).to_string(),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1); };
            { let java_receiver = self.hashMap.clone(); let java_arguments = ((1).to_string(), (jcall!(self.totalRotationJTextField).getText()).to_string(),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1); };
            jcall!(((self.getParent()).clone() /* as HashMapJDialogListenerInterface */)).onHashMap(self.hashMap.clone());
            self.setVisible(false);
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).putF(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("jButton1ActionPerformed"));
            }
        }
    }
    

    pub fn main(mut args: Vec<String>) {
        jcall!(jcall!(unsafe { java_awt.clone() }).EventQueue).invokeLater(ARunnable::new() /* Java anonymous class body: implement it as a named type
            

            pub fn run(&mut self) {
                let mut dialog: Option<Rc<RefCell<RotateJDialog>>> = RotateJDialog::new(JFrame::new(), true);
                jcall!(dialog).addWindowListener(WindowAdapter::new() /* Java anonymous class body: implement it as a named type
                    

                    pub fn windowClosing(&mut self, mut e: Option<Rc<RefCell<WindowEvent>>>) {
                        std::process::exit(0);
                    }
* /);
                jcall!(dialog).setVisible(true);
            }
*/);
    }
}
impl std::ops::Deref for RotateJDialog {
    type Target = JDialog;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RotateJDialog {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



