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
use super::TimeIntervalActionScriptConditionInterface::TimeIntervalActionScriptConditionInterface;
use super::JTextField::JTextField;
use super::JDialog::JDialog;
use super::JLabel::JLabel;
use super::JButton::JButton;
use super::Dimension::Dimension;
use super::awt::awt;
use super::java::java;
use super::ActionListener::ActionListener;
use super::event::event;
use super::ActionEvent::ActionEvent;
use super::GroupLayout::GroupLayout;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TimeIntervalActionScriptConditionJPanel {
    pub base: JPanel,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub timeIntervalActionScriptConditionInterface: Option<Rc<RefCell<dyn TimeIntervalActionScriptConditionInterface>>>,
    pub jLabel1: Option<Rc<RefCell<JLabel>>>,
    pub jLabel2: Option<Rc<RefCell<JLabel>>>,
    pub okJButton: Option<Rc<RefCell<JButton>>>,
    pub timeIntervalActionJButton: Option<Rc<RefCell<JButton>>>,
    pub timeIntervalActionJDialog: Option<Rc<RefCell<JDialog>>>,
    pub timeIntervalJTextField: Option<Rc<RefCell<JTextField>>>,
    pub titleJLabel: Option<Rc<RefCell<JLabel>>>,
}


impl TimeIntervalActionScriptConditionJPanel {
    

    pub fn new_struct(mut timeIntervalActionScriptConditionInterface: Option<Rc<RefCell<dyn TimeIntervalActionScriptConditionInterface>>>) -> Self {
        let mut this = Self {
            base: JPanel::new_struct(),
            logUtil: None,
            commonStrings: None,
            timeIntervalActionScriptConditionInterface: None,
            jLabel1: None,
            jLabel2: None,
            okJButton: None,
            timeIntervalActionJButton: None,
            timeIntervalActionJDialog: None,
            timeIntervalJTextField: None,
            titleJLabel: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        this.initComponents();
        this.timeIntervalActionScriptConditionInterface = timeIntervalActionScriptConditionInterface.clone();
        this.set();
        this
    }

    pub fn new(mut timeIntervalActionScriptConditionInterface: Option<Rc<RefCell<dyn TimeIntervalActionScriptConditionInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(timeIntervalActionScriptConditionInterface))))
    }
    

    pub fn set(&mut self) {
        jcall!(self.getTimeIntervalJTextField()).setText((jcall!(jcall!(self.timeIntervalActionScriptConditionInterface).getTimeDelayHelper()).delay.clone()).to_string());
        jcall!(self.timeIntervalActionScriptConditionInterface).log();
    }
    

    pub fn update(&mut self) {
        jcall!(jcall!(self.timeIntervalActionScriptConditionInterface).getTimeDelayHelper()).delay = (jcall!(self.getTimeIntervalJTextField()).getText()).to_string();
        jcall!(self.timeIntervalActionScriptConditionInterface).log();
    }
    

    pub fn getTimeIntervalJTextField(&mut self) -> Option<Rc<RefCell<JTextField>>> {
        return self.timeIntervalJTextField.clone();
    }
    

    pub fn setTimeIntervalJTextField(&mut self, mut timeIntervalJTextField: Option<Rc<RefCell<JTextField>>>) {
        self.timeIntervalJTextField = timeIntervalJTextField.clone();
    }
    

    pub fn getTimeIntervalActionJDialog(&mut self) -> Option<Rc<RefCell<JDialog>>> {
        return self.timeIntervalActionJDialog.clone();
    }
    

    pub fn setTimeIntervalActionJDialog(&mut self, mut timeIntervalActionJDialog: Option<Rc<RefCell<JDialog>>>) {
        self.timeIntervalActionJDialog = timeIntervalActionJDialog.clone();
    }
    

    pub fn initComponents(&mut self) {
        self.timeIntervalActionJDialog = JDialog::new();
        self.titleJLabel = JLabel::new();
        self.okJButton = JButton::new();
        self.jLabel2 = JLabel::new();
        self.timeIntervalJTextField = JTextField::new();
        self.timeIntervalActionJButton = JButton::new();
        self.jLabel1 = JLabel::new();
        jcall!(self.timeIntervalActionJDialog).setMinimumSize(Dimension::new(250, 150));
        jcall!(self.titleJLabel).setText(String::from("Time Interval Condition Options"));
        jcall!(self.okJButton).setText(String::from("OK"));
        { let java_receiver = self.okJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.okJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jLabel2).setText(String::from("Time (ms):"));
        let mut timeIntervalActionJDialogLayout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(jcall!(self.timeIntervalActionJDialog).getContentPane());
        jcall!(jcall!(self.timeIntervalActionJDialog).getContentPane()).setLayout(timeIntervalActionJDialogLayout.clone());
        jcall!(timeIntervalActionJDialogLayout).setHorizontalGroup(jcall!(jcall!(timeIntervalActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(timeIntervalActionJDialogLayout).createSequentialGroup()).addGroup(jcall!(jcall!(jcall!(timeIntervalActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone())).addGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone(), jcall!(jcall!(jcall!(timeIntervalActionJDialogLayout).createSequentialGroup()).addGap(82, 82, 82)).addComponent(self.okJButton.clone()))).addGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone(), jcall!(jcall!(jcall!(timeIntervalActionJDialogLayout).createSequentialGroup()).addGap(32, 32, 32)).addGroup(jcall!(jcall!(jcall!(timeIntervalActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(timeIntervalActionJDialogLayout).createSequentialGroup()).addComponent(self.jLabel2.clone())).addGap(16, 16, 16)).addComponent(self.timeIntervalJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 85, i16::MAX.clone()))).addComponent(self.titleJLabel.clone()))))).addContainerGap(36, i16::MAX.clone())));
        jcall!(timeIntervalActionJDialogLayout).setVerticalGroup(jcall!(jcall!(timeIntervalActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(timeIntervalActionJDialogLayout).createSequentialGroup()).addContainerGap()).addComponent(self.titleJLabel.clone())).addGap(20, 20, 20)).addGroup(jcall!(jcall!(jcall!(timeIntervalActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.jLabel2.clone())).addComponent(self.timeIntervalJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addGap(15, 15, 15)).addComponent(self.okJButton.clone())).addContainerGap()));
        self.setMinimumSize(Dimension::new(100, 0));
        jcall!(self.timeIntervalActionJButton).setText(String::from("Edit"));
        { let java_receiver = self.timeIntervalActionJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.timeIntervalActionJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jLabel1).setText(String::from("Time Interval:"));
        let mut layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.setLayout(layout.clone());
        jcall!(layout).setHorizontalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addComponent(self.jLabel1.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone())).addComponent(self.timeIntervalActionJButton.clone())));
        jcall!(layout).setVerticalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.jLabel1.clone())).addComponent(self.timeIntervalActionJButton.clone())));
    }
    

    pub fn okJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        self.update();
        jcall!(self.getTimeIntervalActionJDialog()).setVisible(false);
    }
    

    pub fn timeIntervalActionJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        jcall!(self.getTimeIntervalActionJDialog()).setVisible(true);
    }
}
impl std::ops::Deref for TimeIntervalActionScriptConditionJPanel {
    type Target = JPanel;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TimeIntervalActionScriptConditionJPanel {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



