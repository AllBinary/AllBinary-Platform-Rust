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
// package org.allbinary.input.automation.actions.script.condition.processors.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::JPanel::JPanel;
use super::swing::swing;
use super::javax::javax;
use super::JComboBox::JComboBox;
use super::JLabel::JLabel;
use super::DefaultComboBoxModel::DefaultComboBoxModel;
use super::GroupLayout::GroupLayout;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputAutomationTypeJPanel {
    pub base: JPanel,
    pub inputAutomationTypeJComboBox: Option<Rc<RefCell<JComboBox>>>,
    pub jLabel1: Option<Rc<RefCell<JLabel>>>,
}


impl InputAutomationTypeJPanel {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: JPanel::new_struct(),
            inputAutomationTypeJComboBox: None,
            jLabel1: None,
        };
        this.initComponents();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInputAutomationTypeJComboBox(&mut self) -> Option<Rc<RefCell<JComboBox>>> {
        return self.inputAutomationTypeJComboBox.clone();
    }
    

    pub fn setInputAutomationTypeJComboBox(&mut self, mut inputAutomationTypeJComboBox: Option<Rc<RefCell<JComboBox>>>) {
        self.inputAutomationTypeJComboBox = inputAutomationTypeJComboBox.clone();
    }
    

    pub fn initComponents(&mut self) {
        self.inputAutomationTypeJComboBox = JComboBox::new();
        self.jLabel1 = JLabel::new();
        jcall!(self.inputAutomationTypeJComboBox).setModel(DefaultComboBoxModel::new(vec![String::from("DirectX"), String::from("DirectX 8"), String::from("Low Level"), String::from("Java Robot")]));
        jcall!(self.jLabel1).setText(String::from("Input Type:"));
        let mut layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.setLayout(layout.clone());
        jcall!(layout).setHorizontalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addComponent(self.jLabel1.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.inputAutomationTypeJComboBox.clone(), 0, 98, i16::MAX.clone())));
        jcall!(layout).setVerticalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.jLabel1.clone())).addComponent(self.inputAutomationTypeJComboBox.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())));
    }
}
impl std::ops::Deref for InputAutomationTypeJPanel {
    type Target = JPanel;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for InputAutomationTypeJPanel {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



