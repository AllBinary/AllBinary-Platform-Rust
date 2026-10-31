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
// package org.allbinary.input.automation.module.actions.script.condition.processors.output
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
// use crate::javax::swing::*;
use crate::org::allbinary::input::media::image::InputImageType::InputImageType;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageActionScriptOutputJPanel {
    pub base: JPanel,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub capturedImageActionScriptOutputInterface: Option<Rc<RefCell<dyn ImageActionScriptOutputInterface>>>,
    pub capturedImageActionJButton: Option<Rc<RefCell<JButton>>>,
    pub capturedImageActionJDialog: Option<Rc<RefCell<JDialog>>>,
    pub displayJCheckBox: Option<Rc<RefCell<JCheckBox>>>,
    pub itemTypesJList: Option<Rc<RefCell<JList>>>,
    pub jLabel1: Option<Rc<RefCell<JLabel>>>,
    pub jScrollPane1: Option<Rc<RefCell<JScrollPane>>>,
    pub okJButton: Option<Rc<RefCell<JButton>>>,
    pub saveJCheckBox: Option<Rc<RefCell<JCheckBox>>>,
    pub titleJLabel: Option<Rc<RefCell<JLabel>>>,
}


impl ImageActionScriptOutputJPanel {
    

    pub fn new_struct(mut capturedImageActionScriptOutputInterface: Option<Rc<RefCell<dyn ImageActionScriptOutputInterface>>>) -> Self {
        let mut this = Self {
            base: JPanel::new_struct(),
            logUtil: None,
            commonStrings: None,
            capturedImageActionScriptOutputInterface: None,
            capturedImageActionJButton: None,
            capturedImageActionJDialog: None,
            displayJCheckBox: None,
            itemTypesJList: None,
            jLabel1: None,
            jScrollPane1: None,
            okJButton: None,
            saveJCheckBox: None,
            titleJLabel: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        this.initComponents();
        this.capturedImageActionScriptOutputInterface = capturedImageActionScriptOutputInterface.clone();
        this.set();
        this
    }

    pub fn new(mut capturedImageActionScriptOutputInterface: Option<Rc<RefCell<dyn ImageActionScriptOutputInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(capturedImageActionScriptOutputInterface))))
    }
    

    pub fn set(&mut self) {
        self.setImageTypeJlistModel();
        let mut imageTypes: Option<Rc<RefCell<ImageTypes>>> = jcall!(self.capturedImageActionScriptOutputInterface).getImageTypes();
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(imageTypes).getVector();
        let mut selectedArray: Vec<i32> = vec![0; (jcall!(vector).size()) as usize];
        let mut size: i32 = jcall!(vector).size();
        for index in (0..size) {
            let mut imageType: Option<Rc<RefCell<InputImageType>>> = { let java_object = (jcall!(vector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<InputImageType>>>>().expect("ClassCastException").clone()) };
            selectedArray[({ let value = index; index += 1; value }) as usize] = jcall!(imageType).getIndex();
        }
        jcall!(self.itemTypesJList).setSelectedIndices(selectedArray.clone());
        if jcall!(self.capturedImageActionScriptOutputInterface).isSaved() {
            jcall!(self.saveJCheckBox).setSelected(true);
        }
        if jcall!(self.capturedImageActionScriptOutputInterface).isSaved() {
            jcall!(self.displayJCheckBox).setSelected(true);
        }
        jcall!(self.capturedImageActionScriptOutputInterface).log();
    }
    

    pub fn update(&mut self) {
        let mut anyType: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(self.itemTypesJList).getSelectedValues();
        let mut imageTypes: Option<Rc<RefCell<ImageTypes>>> = ImageTypes::valueOf(anyType.clone());
        jcall!(self.capturedImageActionScriptOutputInterface).setImageTypes(imageTypes.clone());
        if jcall!(self.saveJCheckBox).isSelected() {
            jcall!(self.capturedImageActionScriptOutputInterface).setSaved(true);
        } else {
            jcall!(self.capturedImageActionScriptOutputInterface).setSaved(false);
        }
        if jcall!(self.displayJCheckBox).isSelected() {
            jcall!(self.capturedImageActionScriptOutputInterface).setDisplay(true);
        } else {
            jcall!(self.capturedImageActionScriptOutputInterface).setDisplay(false);
        }
        jcall!(self.capturedImageActionScriptOutputInterface).log();
    }
    

    pub fn setImageTypeJlistModel(&mut self) {
        let mut defaultListModel: Option<Rc<RefCell<DefaultListModel>>> = DefaultListModel::new();
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = InputImageType::getAllAsVector();
        let mut size: i32 = jcall!(vector).size();
        for index in (0..size) {
            let mut imageType: Option<Rc<RefCell<InputImageType>>> = { let java_object = (jcall!(vector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<InputImageType>>>>().expect("ClassCastException").clone()) };
            jcall!(defaultListModel).addElement(jcall!(imageType).getName());
        }
        jcall!(self.itemTypesJList).setModel(defaultListModel.clone());
    }
    

    pub fn getCapturedImageActionJDialog(&mut self) -> Option<Rc<RefCell<JDialog>>> {
        return self.capturedImageActionJDialog.clone();
    }
    

    pub fn setCapturedImageActionJDialog(&mut self, mut capturedImageActionJDialog: Option<Rc<RefCell<JDialog>>>) {
        self.capturedImageActionJDialog = capturedImageActionJDialog.clone();
    }
    

    pub fn initComponents(&mut self) {
        self.capturedImageActionJDialog = JDialog::new();
        self.titleJLabel = JLabel::new();
        self.okJButton = JButton::new();
        self.saveJCheckBox = JCheckBox::new();
        self.displayJCheckBox = JCheckBox::new();
        self.jScrollPane1 = JScrollPane::new();
        self.itemTypesJList = JList::new();
        self.capturedImageActionJButton = JButton::new();
        self.jLabel1 = JLabel::new();
        jcall!(self.capturedImageActionJDialog).setMinimumSize(Dimension::new(250, 250));
        jcall!(self.titleJLabel).setText(String::from("Image Output Options"));
        jcall!(self.okJButton).setText(String::from("OK"));
        { let java_receiver = self.okJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.okJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.saveJCheckBox).setText(String::from("Save To File"));
        { let java_receiver = self.saveJCheckBox.clone(); let java_arguments = (jcall!(jcall!(unsafe { javax_swing.clone() }).BorderFactory).createEmptyBorder(0, 0, 0, 0),); jcall!(java_receiver).setBorder(java_arguments.0); };
        jcall!(self.saveJCheckBox).setMargin(Insets::new(0, 0, 0, 0));
        jcall!(self.displayJCheckBox).setText(String::from("Display"));
        { let java_receiver = self.displayJCheckBox.clone(); let java_arguments = (jcall!(jcall!(unsafe { javax_swing.clone() }).BorderFactory).createEmptyBorder(0, 0, 0, 0),); jcall!(java_receiver).setBorder(java_arguments.0); };
        jcall!(self.displayJCheckBox).setMargin(Insets::new(0, 0, 0, 0));
        { let java_receiver = self.displayJCheckBox.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.displayJCheckBoxActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.itemTypesJList).setModel(AbstractListModel::new() /* Java anonymous class body: implement it as a named type
            strings: vec![String::from("Item 1"), String::from("Item 2"), String::from("Item 3"), String::from("Item 4"), String::from("Item 5")],
            

            pub fn getSize(&mut self) -> i32 {
                return (strings.len() as i32).clone();
            }
            

            pub fn getElementAt(&mut self, mut i: i32) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
                return strings[(i) as usize].clone();
            }
*/);
        jcall!(self.jScrollPane1).setViewportView(self.itemTypesJList.clone());
        let mut capturedImageActionJDialogLayout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(jcall!(self.capturedImageActionJDialog).getContentPane());
        jcall!(jcall!(self.capturedImageActionJDialog).getContentPane()).setLayout(capturedImageActionJDialogLayout.clone());
        jcall!(capturedImageActionJDialogLayout).setHorizontalGroup(jcall!(jcall!(capturedImageActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(capturedImageActionJDialogLayout).createSequentialGroup()).addGroup(jcall!(jcall!(jcall!(jcall!(capturedImageActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(capturedImageActionJDialogLayout).createSequentialGroup()).addGap(61, 61, 61)).addComponent(self.titleJLabel.clone()))).addGroup(jcall!(jcall!(jcall!(capturedImageActionJDialogLayout).createSequentialGroup()).addContainerGap()).addGroup(jcall!(jcall!(jcall!(capturedImageActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(capturedImageActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addComponent(self.displayJCheckBox.clone())).addComponent(self.saveJCheckBox.clone(), jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone()))).addGroup(jcall!(jcall!(jcall!(capturedImageActionJDialogLayout).createSequentialGroup()).addGap(84, 84, 84)).addComponent(self.okJButton.clone()))))).addGroup(jcall!(jcall!(jcall!(capturedImageActionJDialogLayout).createSequentialGroup()).addContainerGap()).addComponent(self.jScrollPane1.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 216, i16::MAX.clone())))).addContainerGap()));
        jcall!(capturedImageActionJDialogLayout).setVerticalGroup(jcall!(jcall!(capturedImageActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(capturedImageActionJDialogLayout).createSequentialGroup()).addComponent(self.titleJLabel.clone())).addGap(5, 5, 5)).addComponent(self.jScrollPane1.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 105, i16::MAX.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.saveJCheckBox.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.displayJCheckBox.clone())).addGap(9, 9, 9)).addComponent(self.okJButton.clone())).addContainerGap()));
        self.setMinimumSize(Dimension::new(100, 0));
        jcall!(self.capturedImageActionJButton).setText(String::from("Edit"));
        { let java_receiver = self.capturedImageActionJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.capturedImageActionJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jLabel1).setText(String::from("Capture Image:"));
        let mut layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.setLayout(layout.clone());
        jcall!(layout).setHorizontalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addComponent(self.jLabel1.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone())).addComponent(self.capturedImageActionJButton.clone())));
        jcall!(layout).setVerticalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.jLabel1.clone())).addComponent(self.capturedImageActionJButton.clone())));
    }
    

    pub fn displayJCheckBoxActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
    }
    

    pub fn okJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        self.update();
        jcall!(self.getCapturedImageActionJDialog()).setVisible(false);
    }
    

    pub fn capturedImageActionJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        jcall!(self.getCapturedImageActionJDialog()).setVisible(true);
    }
}
impl std::ops::Deref for ImageActionScriptOutputJPanel {
    type Target = JPanel;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImageActionScriptOutputJPanel {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



