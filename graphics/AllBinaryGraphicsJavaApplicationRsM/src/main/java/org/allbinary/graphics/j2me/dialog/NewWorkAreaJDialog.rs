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
// use crate::java::awt::*;
// use crate::java::awt::event::ActionEvent::ActionEvent;
// use crate::java::awt::event::ActionListener::ActionListener;
// use crate::javax::swing::*;
use crate::org::allbinary::graphics::j2me::GraphicsException::GraphicsException;
use crate::org::allbinary::graphics::j2me::MyFrame::MyFrame;
use crate::org::allbinary::graphics::j2me::workarea::WorkAreaJPanel::WorkAreaJPanel;
use crate::org::allbinary::graphics::j2me::workarea::WorkAreaJPanelInterface::WorkAreaJPanelInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NewWorkAreaJDialog {
    pub base: JDialog,
    pub parent: Option<Rc<RefCell<MyFrame>>>,
    pub dimension: Option<Rc<RefCell<Dimension>>>,
    pub xSizeJTextField: Option<Rc<RefCell<JTextField>>>,
    pub ySizeJTextField: Option<Rc<RefCell<JTextField>>>,
    pub nameJTextField: Option<Rc<RefCell<JTextField>>>,
}


impl NewWorkAreaJDialog {
    

    pub fn new_struct(mut parent: Option<Rc<RefCell<MyFrame>>>, mut modal: bool, mut dimension: Option<Rc<RefCell<Dimension>>>, mut newName: String) -> Self {
        let mut this = Self {
            base: JDialog::new_struct2((parent).clone() /* as Frame */, modal.clone()),
            parent: None,
            dimension: None,
            xSizeJTextField: None,
            ySizeJTextField: None,
            nameJTextField: None,
        };
        // super(...) is in the struct literal above
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            this.initComponents();
            this.parent = parent.clone();
            this.xSizeJTextField = JTextField::new(String::from("12"));
            this.ySizeJTextField = JTextField::new(String::from("12"));
            this.nameJTextField = JTextField::new(newName.clone());
            this.dimension = dimension.clone();
            let mut submitButton: Option<Rc<RefCell<JButton>>> = JButton::new(String::from("Ok"));
            jcall!(submitButton).addActionListener(ActionListener::new() /* Java anonymous class body: implement it as a named type
                

                pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        self.disposeNewDialog();
                    })) {
                        Ok(_) => {}
                        Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                        }
                    }
                }
*/);
            this.setSize(150, 100);
            jcall!(this.getContentPane()).setLayout(GridLayout::new(5, 2));
            jcall!(this.getContentPane()).add(JLabel::new(String::from("Please enter")));
            jcall!(this.getContentPane()).add(JLabel::new(String::from(" the size.")));
            jcall!(this.getContentPane()).add(JLabel::new(String::from("Name:")));
            jcall!(this.getContentPane()).add(this.nameJTextField.clone());
            jcall!(this.getContentPane()).add(JLabel::new(String::from("X: ")));
            jcall!(this.getContentPane()).add(this.xSizeJTextField.clone());
            jcall!(this.getContentPane()).add(JLabel::new(String::from("Y: ")));
            jcall!(this.getContentPane()).add(this.ySizeJTextField.clone());
            jcall!(this.getContentPane()).add(submitButton.clone());
            this.show();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
        this
    }

    pub fn new(mut parent: Option<Rc<RefCell<MyFrame>>>, mut modal: bool, mut dimension: Option<Rc<RefCell<Dimension>>>, mut newName: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(parent, modal, dimension, newName))))
    }
    

    // throws Exception
    pub fn disposeNewDialog(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut canvasWidth: Option<i32> = Some((jcall!(self.xSizeJTextField).getText()) as i32);
            let mut canvasHeight: Option<i32> = Some((jcall!(self.ySizeJTextField).getText()) as i32);
            let mut newName: String = jcall!(self.nameJTextField).getText();
            if false {
                panic!("{:?}", format!("{}: {:?}", "GraphicsException", (String::from("No Name"), None /* Java this: a &mut self method has no Rc of itself */, String::from("disposeNewDialog"))));
            }
            let mut workAreaJPanel: Option<Rc<RefCell<WorkAreaJPanel>>> = WorkAreaJPanel::new4(newName.clone(), self.dimension.clone(), ((canvasWidth).unwrap() as i32), ((canvasHeight).unwrap() as i32));
            jcall!(workAreaJPanel).setLayout(GridLayout::new(1, 1));
            jcall!(self.parent).add((workAreaJPanel).clone() /* as WorkAreaJPanelInterface */);
            self.dispose();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
    

    pub fn initComponents(&mut self) {
        { let java_arguments = (WindowAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn windowClosing(&mut self, mut evt: Option<Rc<RefCell<WindowEvent>>>) {
                self.closeDialog(evt.clone());
            }
*/,); self.addWindowListener(java_arguments.0) };
        self.pack();
    }
    

    pub fn closeDialog(&mut self, mut evt: Option<Rc<RefCell<WindowEvent>>>) {
        self.setVisible(false);
        self.dispose();
    }
    

    pub fn main(mut args: Vec<String>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(NewWorkAreaJDialog::new(MyFrame::new(), true, Dimension::new(100, 100), String::from("testing"))).show();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            }
        }
    }
}
impl std::ops::Deref for NewWorkAreaJDialog {
    type Target = JDialog;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NewWorkAreaJDialog {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



