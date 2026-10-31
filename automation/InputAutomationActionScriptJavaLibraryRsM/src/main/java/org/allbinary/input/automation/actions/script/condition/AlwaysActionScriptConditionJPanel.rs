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
// package org.allbinary.input.automation.actions.script.condition
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::JPanel::JPanel;
use super::swing::swing;
use super::javax::javax;
use super::AlwaysActionScriptConditionInterface::AlwaysActionScriptConditionInterface;
use super::JDialog::JDialog;
use super::JLabel::JLabel;
use super::JButton::JButton;
use super::JComboBox::JComboBox;
use super::Dimension::Dimension;
use super::awt::awt;
use super::java::java;
use super::ActionListener::ActionListener;
use super::event::event;
use super::ActionEvent::ActionEvent;
use super::DefaultComboBoxModel::DefaultComboBoxModel;
use super::GroupLayout::GroupLayout;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AlwaysActionScriptConditionJPanel {
    pub base: JPanel,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub alwaysActionScriptConditionInterface: Option<Rc<RefCell<dyn AlwaysActionScriptConditionInterface>>>,
    pub alwaysActionJButton: Option<Rc<RefCell<JButton>>>,
    pub alwaysActionJDialog: Option<Rc<RefCell<JDialog>>>,
    pub alwaysJComboBox: Option<Rc<RefCell<JComboBox>>>,
    pub jLabel1: Option<Rc<RefCell<JLabel>>>,
    pub jLabel2: Option<Rc<RefCell<JLabel>>>,
    pub okJButton: Option<Rc<RefCell<JButton>>>,
    pub titleJLabel: Option<Rc<RefCell<JLabel>>>,
}


impl AlwaysActionScriptConditionJPanel {
    

    pub fn new_struct(mut alwaysActionScriptConditionInterface: Option<Rc<RefCell<dyn AlwaysActionScriptConditionInterface>>>) -> Self {
        let mut this = Self {
            base: JPanel::new_struct(),
            logUtil: None,
            commonStrings: None,
            alwaysActionScriptConditionInterface: None,
            alwaysActionJButton: None,
            alwaysActionJDialog: None,
            alwaysJComboBox: None,
            jLabel1: None,
            jLabel2: None,
            okJButton: None,
            titleJLabel: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        this.initComponents();
        this.alwaysActionScriptConditionInterface = alwaysActionScriptConditionInterface.clone();
        this.set();
        this
    }

    pub fn new(mut alwaysActionScriptConditionInterface: Option<Rc<RefCell<dyn AlwaysActionScriptConditionInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(alwaysActionScriptConditionInterface))))
    }
    

    pub fn set(&mut self) {
        if jcall!(self.alwaysActionScriptConditionInterface).isIsOn() {
            jcall!(self.alwaysJComboBox).setSelectedIndex(0);
        } else {
            jcall!(self.alwaysJComboBox).setSelectedIndex(1);
        }
        jcall!(self.alwaysActionScriptConditionInterface).log();
    }
    

    pub fn update(&mut self) {
        if jcall!(self.alwaysJComboBox).getSelectedIndex() == 0 {
            jcall!(self.alwaysActionScriptConditionInterface).setIsOn(true);
        } else {
            jcall!(self.alwaysActionScriptConditionInterface).setIsOn(false);
        }
        jcall!(self.alwaysActionScriptConditionInterface).log();
    }
    

    pub fn getAlwaysActionJDialog(&mut self) -> Option<Rc<RefCell<JDialog>>> {
        return self.alwaysActionJDialog.clone();
    }
    

    pub fn setAlwaysActionJDialog(&mut self, mut alwaysActionJDialog: Option<Rc<RefCell<JDialog>>>) {
        self.alwaysActionJDialog = alwaysActionJDialog.clone();
    }
    

    pub fn initComponents(&mut self) {
        self.alwaysActionJDialog = JDialog::new();
        self.titleJLabel = JLabel::new();
        self.okJButton = JButton::new();
        self.jLabel2 = JLabel::new();
        self.alwaysJComboBox = JComboBox::new();
        self.alwaysActionJButton = JButton::new();
        self.jLabel1 = JLabel::new();
        jcall!(self.alwaysActionJDialog).setMinimumSize(Dimension::new(250, 150));
        jcall!(self.titleJLabel).setText(String::from("Always Condition Options"));
        jcall!(self.okJButton).setText(String::from("OK"));
        { let java_receiver = self.okJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.okJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jLabel2).setText(String::from("On:"));
        jcall!(self.alwaysJComboBox).setModel(DefaultComboBoxModel::new(vec![String::from("true"), String::from("false")]));
        let mut alwaysActionJDialogLayout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(jcall!(self.alwaysActionJDialog).getContentPane());
        jcall!(jcall!(self.alwaysActionJDialog).getContentPane()).setLayout(alwaysActionJDialogLayout.clone());
        jcall!(alwaysActionJDialogLayout).setHorizontalGroup(jcall!(jcall!(alwaysActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(alwaysActionJDialogLayout).createSequentialGroup()).addGroup(jcall!(jcall!(jcall!(alwaysActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(alwaysActionJDialogLayout).createSequentialGroup()).addGap(47, 47, 47)).addGroup(jcall!(jcall!(jcall!(alwaysActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone(), false)).addGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone(), jcall!(jcall!(jcall!(jcall!(alwaysActionJDialogLayout).createSequentialGroup()).addComponent(self.jLabel2.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone())).addComponent(self.alwaysJComboBox.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addComponent(self.titleJLabel.clone())))).addGroup(jcall!(jcall!(jcall!(alwaysActionJDialogLayout).createSequentialGroup()).addGap(79, 79, 79)).addComponent(self.okJButton.clone())))).addContainerGap(45, i16::MAX.clone())));
        jcall!(alwaysActionJDialogLayout).setVerticalGroup(jcall!(jcall!(alwaysActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(alwaysActionJDialogLayout).createSequentialGroup()).addContainerGap()).addComponent(self.titleJLabel.clone())).addGap(20, 20, 20)).addGroup(jcall!(jcall!(jcall!(alwaysActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.alwaysJComboBox.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addComponent(self.jLabel2.clone()))).addGap(15, 15, 15)).addComponent(self.okJButton.clone())).addContainerGap()));
        self.setMinimumSize(Dimension::new(100, 0));
        jcall!(self.alwaysActionJButton).setText(String::from("Edit"));
        { let java_receiver = self.alwaysActionJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.alwaysActionJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jLabel1).setText(String::from("Always:"));
        let mut layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.setLayout(layout.clone());
        jcall!(layout).setHorizontalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addComponent(self.jLabel1.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone(), 33, i16::MAX.clone())).addComponent(self.alwaysActionJButton.clone())));
        jcall!(layout).setVerticalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.jLabel1.clone())).addComponent(self.alwaysActionJButton.clone())));
    }
    

    pub fn okJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        self.update();
        jcall!(self.getAlwaysActionJDialog()).setVisible(false);
    }
    

    pub fn alwaysActionJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        jcall!(self.getAlwaysActionJDialog()).setVisible(true);
    }
}
impl std::ops::Deref for AlwaysActionScriptConditionJPanel {
    type Target = JPanel;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AlwaysActionScriptConditionJPanel {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



