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
// package org.allbinary.graphics.j2me.workarea.properties
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::swing::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PropertiesJPanel {
    pub base: JPanel,
    pub rotateJButton: Option<Rc<RefCell<JButton>>>,
    pub copyVerticalJButton: Option<Rc<RefCell<JButton>>>,
    pub leftJButton: Option<Rc<RefCell<JButton>>>,
    pub rotateJPanel: Option<Rc<RefCell<JPanel>>>,
    pub angleJTextField: Option<Rc<RefCell<JTextField>>>,
    pub workAreaPropertiesJPanel: Option<Rc<RefCell<JPanel>>>,
    pub modifierJPanel: Option<Rc<RefCell<JPanel>>>,
    pub upJButton: Option<Rc<RefCell<JButton>>>,
    pub jPanel8: Option<Rc<RefCell<JPanel>>>,
    pub downJButton: Option<Rc<RefCell<JButton>>>,
    pub jPanel7: Option<Rc<RefCell<JPanel>>>,
    pub rightJButton: Option<Rc<RefCell<JButton>>>,
    pub copyHorizontalJButton: Option<Rc<RefCell<JButton>>>,
}


impl PropertiesJPanel {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: JPanel::new_struct(),
            rotateJButton: None,
            copyVerticalJButton: None,
            leftJButton: None,
            rotateJPanel: None,
            angleJTextField: None,
            workAreaPropertiesJPanel: None,
            modifierJPanel: None,
            upJButton: None,
            jPanel8: None,
            downJButton: None,
            jPanel7: None,
            rightJButton: None,
            copyHorizontalJButton: None,
        };
        this.initComponents();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn set(&mut self, mut panel: Option<Rc<RefCell<JPanel>>>) {
        jcall!(self.workAreaPropertiesJPanel).add(panel.clone());
        self.repaint();
    }
    

    pub fn initComponents(&mut self) {
        self.workAreaPropertiesJPanel = JPanel::new();
        self.modifierJPanel = JPanel::new();
        self.copyVerticalJButton = JButton::new();
        self.copyHorizontalJButton = JButton::new();
        self.rotateJPanel = JPanel::new();
        self.rotateJButton = JButton::new();
        self.angleJTextField = JTextField::new();
        self.jPanel7 = JPanel::new();
        self.upJButton = JButton::new();
        self.downJButton = JButton::new();
        self.jPanel8 = JPanel::new();
        self.leftJButton = JButton::new();
        self.rightJButton = JButton::new();
        self.setLayout(GridLayout::new(2, 1));
        jcall!(self.workAreaPropertiesJPanel).setLayout(GridLayout::new());
        self.add(self.workAreaPropertiesJPanel.clone());
        jcall!(self.modifierJPanel).setLayout(GridLayout::new(6, 0));
        jcall!(self.copyVerticalJButton).setFont(Font::new(String::from("Dialog"), 0, 10));
        jcall!(self.copyVerticalJButton).setText(String::from("Copy Vertical"));
        jcall!(self.modifierJPanel).add(self.copyVerticalJButton.clone());
        jcall!(self.copyHorizontalJButton).setFont(Font::new(String::from("Dialog"), 0, 10));
        jcall!(self.copyHorizontalJButton).setText(String::from("Copy Horizontal"));
        jcall!(self.modifierJPanel).add(self.copyHorizontalJButton.clone());
        jcall!(self.rotateJPanel).setLayout(GridLayout::new(1, 2));
        jcall!(self.rotateJButton).setFont(Font::new(String::from("Dialog"), 0, 10));
        jcall!(self.rotateJButton).setText(String::from("Rotate"));
        jcall!(self.rotateJPanel).add(self.rotateJButton.clone());
        jcall!(self.angleJTextField).setText(String::from("Angle"));
        jcall!(self.rotateJPanel).add(self.angleJTextField.clone());
        jcall!(self.modifierJPanel).add(self.rotateJPanel.clone());
        jcall!(self.jPanel7).setLayout(GridLayout::new(1, 2));
        jcall!(self.upJButton).setFont(Font::new(String::from("Dialog"), 0, 10));
        jcall!(self.upJButton).setText(String::from("Up"));
        jcall!(self.jPanel7).add(self.upJButton.clone());
        jcall!(self.downJButton).setFont(Font::new(String::from("Dialog"), 0, 10));
        jcall!(self.downJButton).setText(String::from("Down"));
        jcall!(self.jPanel7).add(self.downJButton.clone());
        jcall!(self.modifierJPanel).add(self.jPanel7.clone());
        jcall!(self.jPanel8).setLayout(GridLayout::new(1, 2));
        jcall!(self.leftJButton).setFont(Font::new(String::from("Dialog"), 0, 10));
        jcall!(self.leftJButton).setText(String::from("Left"));
        jcall!(self.jPanel8).add(self.leftJButton.clone());
        jcall!(self.rightJButton).setFont(Font::new(String::from("Dialog"), 0, 10));
        jcall!(self.rightJButton).setText(String::from("Right"));
        jcall!(self.jPanel8).add(self.rightJButton.clone());
        jcall!(self.modifierJPanel).add(self.jPanel8.clone());
        self.add(self.modifierJPanel.clone());
    }
}
impl std::ops::Deref for PropertiesJPanel {
    type Target = JPanel;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PropertiesJPanel {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



