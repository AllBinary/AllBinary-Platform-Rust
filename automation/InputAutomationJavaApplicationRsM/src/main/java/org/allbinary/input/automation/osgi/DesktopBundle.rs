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
// package org.allbinary.input.automation.osgi
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::swing::*;
use crate::org::osgi::framework::Bundle::Bundle;
use crate::org::osgi::framework::BundleContext::BundleContext;
use crate::bundle::input::automation::InputAutomationBundleActivator::InputAutomationBundleActivator;
use crate::org::allbinary::gui::dialog::BasicTextJDialog::BasicTextJDialog;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DesktopBundle {
    pub bundle: Option<Rc<RefCell<dyn Bundle>>>,
}

thread_local! { pub static DesktopBundle_DESKTOP_SYMBOLIC_NAME: String = String::from("org.knopflerfish.bundle.desktop"); }

impl DesktopBundle {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            bundle: None,
        };
        let mut bundleContext: Option<Rc<RefCell<dyn BundleContext>>> = InputAutomationBundleActivator::getBundleContext();
        if bundleContext.is_some() {
            let mut bundleArray: Vec<Option<Rc<RefCell<dyn Bundle>>>> = jcall!(bundleContext).getBundles();
            for index in (0..(bundleArray.len() as i32)) {
                let mut bundle: Option<Rc<RefCell<dyn Bundle>>> = bundleArray[(index) as usize].clone();
                let mut symbolicName: String = jcall!(bundle).getSymbolicName();
                if (symbolicName.cmp(&DesktopBundle_DESKTOP_SYMBOLIC_NAME.with(Clone::clone).clone()) as i32) == 0 {
                    this.bundle = bundle.clone();
                }
            }
        } else {
            let mut textJDialog: Option<Rc<RefCell<JDialog>>> = BasicTextJDialog::new1(String::from("BundlContext is Null")).map(|reference| Rc::new(RefCell::new(Rc::try_unwrap(reference).ok().expect("new object is not shared").into_inner().base)));
            jcall!(textJDialog).setVisible(true);
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn start(&mut self) {
        jcall!(self.bundle).start();
    }
}

