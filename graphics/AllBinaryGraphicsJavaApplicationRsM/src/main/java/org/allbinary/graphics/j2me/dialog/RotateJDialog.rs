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
use super::*;
use crate::org::allbinary::thread::ARunnable::ARunnable;
use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::gui::dialog::HashMapJDialogListenerInterface::HashMapJDialogListenerInterface;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;
use super::JDialog::JDialog;
use super::Frame::Frame;
use super::JTextField::JTextField;
use super::JLabel::JLabel;
use super::JPanel::JPanel;
use super::JButton::JButton;
use super::ActionListener::ActionListener;
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
    pub hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>,
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
        { let java_arguments = ({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).WindowConstants.clone(); java_field }).DISPOSE_ON_CLOSE.clone(); java_field }.clone(),); self.setDefaultCloseOperation(java_arguments.0) };
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
        jcall!(jPanel1Layout).setHorizontalGroup(jcall!(jcall!(jPanel1Layout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).LEADING.clone(); java_field }.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jPanel1Layout).createSequentialGroup()).addGap(62, 62, 62)).addComponent(self.jButton1.clone())).addContainerGap(62, i16::MAX.clone())));
        jcall!(jPanel1Layout).setVerticalGroup(jcall!(jcall!(jPanel1Layout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).LEADING.clone(); java_field }.clone())).addComponent(self.jButton1.clone()));
        let mut layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(self.getContentPane());
        jcall!(self.getContentPane()).setLayout(layout.clone());
        jcall!(layout).setHorizontalGroup(jcall!(jcall!(jcall!(layout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).LEADING.clone(); java_field }.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addContainerGap()).addGroup(jcall!(jcall!(jcall!(layout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).TRAILING.clone(); java_field }.clone())).addComponent(self.jLabel1.clone())).addComponent(self.jLabel2.clone()))).addPreferredGap({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).LayoutStyle.clone(); java_field }).ComponentPlacement.clone(); java_field }).UNRELATED.clone(); java_field }.clone())).addGroup(jcall!(jcall!(jcall!(layout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).LEADING.clone(); java_field }.clone())).addComponent(self.totalRotationJTextField.clone(), { let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).TRAILING.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).DEFAULT_SIZE.clone(); java_field }.clone(), 31, i16::MAX.clone())).addComponent(self.angleIncrementJTextField.clone(), { let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).TRAILING.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone(), 31, { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone()))).addContainerGap())).addComponent(self.jPanel1.clone(), { let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).TRAILING.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).DEFAULT_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).DEFAULT_SIZE.clone(); java_field }.clone(), i16::MAX.clone()));
        jcall!(layout).setVerticalGroup(jcall!(jcall!(layout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).LEADING.clone(); java_field }.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addContainerGap()).addGroup(jcall!(jcall!(jcall!(layout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).BASELINE.clone(); java_field }.clone())).addComponent(self.angleIncrementJTextField.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).DEFAULT_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone())).addComponent(self.jLabel1.clone()))).addPreferredGap({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).LayoutStyle.clone(); java_field }).ComponentPlacement.clone(); java_field }).RELATED.clone(); java_field }.clone())).addGroup(jcall!(jcall!(jcall!(layout).createParallelGroup({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).Alignment.clone(); java_field }).BASELINE.clone(); java_field }.clone())).addComponent(self.totalRotationJTextField.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).DEFAULT_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone())).addComponent(self.jLabel2.clone()))).addPreferredGap({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).LayoutStyle.clone(); java_field }).ComponentPlacement.clone(); java_field }).RELATED.clone(); java_field }.clone(), 16, i16::MAX.clone())).addComponent(self.jPanel1.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).DEFAULT_SIZE.clone(); java_field }.clone(), { let java_field = jcall!({ let java_field = jcall!(unsafe { javax_swing.clone() }).GroupLayout.clone(); java_field }).PREFERRED_SIZE.clone(); java_field }.clone())).addContainerGap()));
        self.pack();
    }
    

    pub fn jButton1ActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let java_receiver = self.hashMap.clone(); let java_arguments = (crate::java_into_object(Some((0) as i32)), crate::java_into_object(Some((jcall!(self.angleIncrementJTextField).getText()) as i32)),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1); };
            { let java_receiver = self.hashMap.clone(); let java_arguments = (crate::java_into_object(Some((1) as i32)), crate::java_into_object(Some((jcall!(self.totalRotationJTextField).getText()) as i32)),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1); };
            jcall!(((self.getParent()).clone())).onHashMap(self.hashMap.clone());
            self.setVisible(false);
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).putF({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("jButton1ActionPerformed"));
            }
        }
    }
    

    pub fn main(mut args: Vec<String>) {
        jcall!({ let java_field = jcall!(unsafe { java_awt.clone() }).EventQueue.clone(); java_field }).invokeLater(ARunnable::new() /* Java anonymous class body: implement it as a named type
            

            pub fn run(&mut self) {
                let mut dialog: Option<Rc<RefCell<RotateJDialog>>> = RotateJDialog::new(JFrame::new(), true);
                jcall!(dialog).addWindowListener(WindowAdapter::new() / * Java anonymous class body: implement it as a named type
                    

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



