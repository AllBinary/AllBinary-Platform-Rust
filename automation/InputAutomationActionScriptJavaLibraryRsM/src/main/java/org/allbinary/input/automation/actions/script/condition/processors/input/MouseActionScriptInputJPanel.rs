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
// use crate::java::awt::event::InputEvent::InputEvent;
// use crate::javax::swing::*;
use crate::org::allbinary::input::automation::robot::InputRobot::InputRobot;
use crate::org::allbinary::input::automation::robot::InputRobotFactory::InputRobotFactory;
use crate::org::allbinary::input::automation::robot::TempInputRobotNames::TempInputRobotNames;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonPhoneStrings::CommonPhoneStrings;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MouseActionScriptInputJPanel {
    pub base: JPanel,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub mouseActionScriptInputInterface: Option<Rc<RefCell<dyn MouseActionScriptInputInterface>>>,
    pub button1JCheckBox: Option<Rc<RefCell<JCheckBox>>>,
    pub button2JCheckBox: Option<Rc<RefCell<JCheckBox>>>,
    pub inputAutomationTypeJComboBox: Option<Rc<RefCell<JComboBox>>>,
    pub inputTypeJLabel: Option<Rc<RefCell<JLabel>>>,
    pub jLabel1: Option<Rc<RefCell<JLabel>>>,
    pub jLabel2: Option<Rc<RefCell<JLabel>>>,
    pub jLabel3: Option<Rc<RefCell<JLabel>>>,
    pub jLabel4: Option<Rc<RefCell<JLabel>>>,
    pub jLabel5: Option<Rc<RefCell<JLabel>>>,
    pub jLabel6: Option<Rc<RefCell<JLabel>>>,
    pub mouseActionJButton: Option<Rc<RefCell<JButton>>>,
    pub mouseActionJDialog: Option<Rc<RefCell<JDialog>>>,
    pub mouseDelayJTextField: Option<Rc<RefCell<JTextField>>>,
    pub mouseXJTextField: Option<Rc<RefCell<JTextField>>>,
    pub mouseYJTextField: Option<Rc<RefCell<JTextField>>>,
    pub okJButton: Option<Rc<RefCell<JButton>>>,
    pub titleJLabel: Option<Rc<RefCell<JLabel>>>,
}


impl MouseActionScriptInputJPanel {
    

    pub fn new_struct(mut mouseActionScriptInputInterface: Option<Rc<RefCell<dyn MouseActionScriptInputInterface>>>) -> Self {
        let mut this = Self {
            base: JPanel::new_struct(),
            logUtil: None,
            commonStrings: None,
            mouseActionScriptInputInterface: None,
            button1JCheckBox: None,
            button2JCheckBox: None,
            inputAutomationTypeJComboBox: None,
            inputTypeJLabel: None,
            jLabel1: None,
            jLabel2: None,
            jLabel3: None,
            jLabel4: None,
            jLabel5: None,
            jLabel6: None,
            mouseActionJButton: None,
            mouseActionJDialog: None,
            mouseDelayJTextField: None,
            mouseXJTextField: None,
            mouseYJTextField: None,
            okJButton: None,
            titleJLabel: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", jcall!(CommonLabels::getInstance()).START, mouseActionScriptInputInterface), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        this.initComponents();
        this.mouseActionScriptInputInterface = mouseActionScriptInputInterface.clone();
        let mut defaultComboBoxModel: Option<Rc<RefCell<DefaultComboBoxModel>>> = DefaultComboBoxModel::new();
        jcall!(defaultComboBoxModel).addElement(crate::org::allbinary::input::automation::robot::TempInputRobotNames::TempInputRobotNames_LOW_NAME.with(Clone::clone).clone());
        jcall!(defaultComboBoxModel).addElement(crate::org::allbinary::input::automation::robot::TempInputRobotNames::TempInputRobotNames_SYS_NAME.with(Clone::clone).clone());
        jcall!(defaultComboBoxModel).addElement(crate::org::allbinary::input::automation::robot::InputRobot::InputRobot_NAME.with(Clone::clone).clone());
        jcall!(this.inputAutomationTypeJComboBox).setModel(defaultComboBoxModel.clone());
        this.set();
        this
    }

    pub fn new(mut mouseActionScriptInputInterface: Option<Rc<RefCell<dyn MouseActionScriptInputInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(mouseActionScriptInputInterface))))
    }
    

    pub fn set(&mut self) {
        let mut inputTypeComboBoxModel: Option<Rc<RefCell<ComboBoxModel>>> = jcall!(self.inputAutomationTypeJComboBox).getModel();
        jcall!(inputTypeComboBoxModel).setSelectedItem(jcall!(jcall!(self.mouseActionScriptInputInterface).getInputRobotInterface()).getName());
        if (jcall!(self.mouseActionScriptInputInterface).getButtonClicks() & unsafe { crate::java::awt::event::InputEvent::InputEvent_BUTTON1_MASK.clone() }) != 0 {
            jcall!(self.button1JCheckBox).setSelected(true);
        } else {
            jcall!(self.button1JCheckBox).setSelected(false);
        }
        if (jcall!(self.mouseActionScriptInputInterface).getButtonClicks() & unsafe { crate::java::awt::event::InputEvent::InputEvent_BUTTON2_MASK.clone() }) != 0 {
            jcall!(self.button2JCheckBox).setSelected(true);
        } else {
            jcall!(self.button2JCheckBox).setSelected(false);
        }
        { let java_receiver = self.mouseDelayJTextField.clone(); let java_arguments = ((jcall!(self.mouseActionScriptInputInterface).getTime()).to_string(),); jcall!(java_receiver).setText(java_arguments.0); };
        { let java_receiver = self.mouseXJTextField.clone(); let java_arguments = ((jcall!(jcall!(self.mouseActionScriptInputInterface).getPoint()).x.clone()).to_string(),); jcall!(java_receiver).setText(java_arguments.0); };
        { let java_receiver = self.mouseYJTextField.clone(); let java_arguments = ((jcall!(jcall!(self.mouseActionScriptInputInterface).getPoint()).y.clone()).to_string(),); jcall!(java_receiver).setText(java_arguments.0); };
        jcall!(self.mouseActionScriptInputInterface).log();
    }
    

    // throws Exception
    pub fn updateInputType(&mut self) {
        let mut comboBoxModel: Option<Rc<RefCell<ComboBoxModel>>> = jcall!(self.inputAutomationTypeJComboBox).getModel();
        let mut selectedItem: String = (jcall!(comboBoxModel).getSelectedItem()).clone() /* as String */;
        if !jcall!(StringValidationUtil::getInstance()).isEmpty(selectedItem.clone()) {
            { let java_receiver = self.mouseActionScriptInputInterface.clone(); let java_arguments = (jcall!(InputRobotFactory::getInstance()).get1(selectedItem.clone()),); jcall!(java_receiver).setInputRobotInterface(java_arguments.0); };
        }
    }
    

    // throws Exception
    pub fn update(&mut self) {
        self.updateInputType();
        { let java_receiver = self.mouseActionScriptInputInterface.clone(); let java_arguments = ((jcall!(self.mouseDelayJTextField).getText()).to_string(),); jcall!(java_receiver).setTime(java_arguments.0); };
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Button State: "), jcall!(self.mouseActionScriptInputInterface).getButtonClicks()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("update"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        if jcall!(self.button1JCheckBox).isSelected() {
            if (jcall!(self.mouseActionScriptInputInterface).getButtonClicks() & unsafe { crate::java::awt::event::InputEvent::InputEvent_BUTTON1_MASK.clone() }) == 0 {
                { let java_receiver = self.mouseActionScriptInputInterface.clone(); let java_arguments = (jcall!(self.mouseActionScriptInputInterface).getButtonClicks() | unsafe { crate::java::awt::event::InputEvent::InputEvent_BUTTON1_MASK.clone() },); jcall!(java_receiver).setButtonClicks(java_arguments.0); };
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Button 1 Selected: "), jcall!(self.mouseActionScriptInputInterface).getButtonClicks()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("update"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
        } else {
            if (jcall!(self.mouseActionScriptInputInterface).getButtonClicks() & unsafe { crate::java::awt::event::InputEvent::InputEvent_BUTTON1_MASK.clone() }) != 0 {
                { let java_receiver = self.mouseActionScriptInputInterface.clone(); let java_arguments = (jcall!(self.mouseActionScriptInputInterface).getButtonClicks() ^ unsafe { crate::java::awt::event::InputEvent::InputEvent_BUTTON1_MASK.clone() },); jcall!(java_receiver).setButtonClicks(java_arguments.0); };
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Button 1 Deselected: "), jcall!(self.mouseActionScriptInputInterface).getButtonClicks()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("update"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
        }
        if jcall!(self.button2JCheckBox).isSelected() {
            if (jcall!(self.mouseActionScriptInputInterface).getButtonClicks() & unsafe { crate::java::awt::event::InputEvent::InputEvent_BUTTON2_MASK.clone() }) == 0 {
                { let java_receiver = self.mouseActionScriptInputInterface.clone(); let java_arguments = (jcall!(self.mouseActionScriptInputInterface).getButtonClicks() | unsafe { crate::java::awt::event::InputEvent::InputEvent_BUTTON2_MASK.clone() },); jcall!(java_receiver).setButtonClicks(java_arguments.0); };
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Button 2 Selected: "), jcall!(self.mouseActionScriptInputInterface).getButtonClicks()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("update"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
        } else {
            if (jcall!(self.mouseActionScriptInputInterface).getButtonClicks() & unsafe { crate::java::awt::event::InputEvent::InputEvent_BUTTON2_MASK.clone() }) != 0 {
                { let java_receiver = self.mouseActionScriptInputInterface.clone(); let java_arguments = (jcall!(self.mouseActionScriptInputInterface).getButtonClicks() ^ unsafe { crate::java::awt::event::InputEvent::InputEvent_BUTTON2_MASK.clone() },); jcall!(java_receiver).setButtonClicks(java_arguments.0); };
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Button 2 Deselected: "), jcall!(self.mouseActionScriptInputInterface).getButtonClicks()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("update"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
        }
        let mut mouseXString: String = jcall!(self.mouseXJTextField).getText();
        if !jcall!(StringValidationUtil::getInstance()).isEmpty(mouseXString.clone()) && jcall!(StringValidationUtil::getInstance()).isNumber(mouseXString.clone()) {
            jcall!(jcall!(self.mouseActionScriptInputInterface).getPoint()).x = mouseXString.clone().parse::<i32>().unwrap();
        }
        let mut mouseYString: String = jcall!(self.mouseYJTextField).getText();
        if !jcall!(StringValidationUtil::getInstance()).isEmpty(mouseYString.clone()) && jcall!(StringValidationUtil::getInstance()).isNumber(mouseYString.clone()) {
            jcall!(jcall!(self.mouseActionScriptInputInterface).getPoint()).y = mouseYString.clone().parse::<i32>().unwrap();
        }
        jcall!(self.mouseActionScriptInputInterface).log();
    }
    

    pub fn getMouseActionJDialog(&mut self) -> Option<Rc<RefCell<JDialog>>> {
        return self.mouseActionJDialog.clone();
    }
    

    pub fn setMouseActionJDialog(&mut self, mut mouseActionJDialog: Option<Rc<RefCell<JDialog>>>) {
        self.mouseActionJDialog = mouseActionJDialog.clone();
    }
    

    pub fn initComponents(&mut self) {
        self.mouseActionJDialog = JDialog::new();
        self.titleJLabel = JLabel::new();
        self.okJButton = JButton::new();
        self.jLabel2 = JLabel::new();
        self.jLabel3 = JLabel::new();
        self.button1JCheckBox = JCheckBox::new();
        self.button2JCheckBox = JCheckBox::new();
        self.jLabel4 = JLabel::new();
        self.mouseXJTextField = JTextField::new();
        self.jLabel5 = JLabel::new();
        self.mouseYJTextField = JTextField::new();
        self.inputTypeJLabel = JLabel::new();
        self.inputAutomationTypeJComboBox = JComboBox::new();
        self.jLabel6 = JLabel::new();
        self.mouseDelayJTextField = JTextField::new();
        self.mouseActionJButton = JButton::new();
        self.jLabel1 = JLabel::new();
        jcall!(self.mouseActionJDialog).setMinimumSize(Dimension::new(275, 185));
        jcall!(self.titleJLabel).setText(String::from("Mouse Input Options"));
        jcall!(self.okJButton).setText(String::from("OK"));
        { let java_receiver = self.okJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.okJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jLabel2).setText(String::from("Buttons:"));
        jcall!(self.jLabel3).setText(String::from("Move To:"));
        { let java_receiver = self.button1JCheckBox.clone(); let java_arguments = (jcall!(CommonPhoneStrings::getInstance()).ONE.clone(),); jcall!(java_receiver).setText(java_arguments.0); };
        { let java_receiver = self.button1JCheckBox.clone(); let java_arguments = (jcall!(jcall!(unsafe { javax_swing.clone() }).BorderFactory).createEmptyBorder(0, 0, 0, 0),); jcall!(java_receiver).setBorder(java_arguments.0); };
        jcall!(self.button1JCheckBox).setMargin(Insets::new(0, 0, 0, 0));
        { let java_receiver = self.button2JCheckBox.clone(); let java_arguments = (jcall!(CommonPhoneStrings::getInstance()).TWO.clone(),); jcall!(java_receiver).setText(java_arguments.0); };
        { let java_receiver = self.button2JCheckBox.clone(); let java_arguments = (jcall!(jcall!(unsafe { javax_swing.clone() }).BorderFactory).createEmptyBorder(0, 0, 0, 0),); jcall!(java_receiver).setBorder(java_arguments.0); };
        jcall!(self.button2JCheckBox).setMargin(Insets::new(0, 0, 0, 0));
        { let java_receiver = self.button2JCheckBox.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.button2JCheckBoxActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jLabel4).setText(String::from("x:"));
        { let java_receiver = self.mouseXJTextField.clone(); let java_arguments = (jcall!(CommonPhoneStrings::getInstance()).ZERO.clone(),); jcall!(java_receiver).setText(java_arguments.0); };
        jcall!(self.jLabel5).setText(String::from("y:"));
        { let java_receiver = self.mouseYJTextField.clone(); let java_arguments = (jcall!(CommonPhoneStrings::getInstance()).ZERO.clone(),); jcall!(java_receiver).setText(java_arguments.0); };
        jcall!(self.inputTypeJLabel).setText(String::from("Input Type:"));
        jcall!(self.jLabel6).setText(String::from("Delay:"));
        jcall!(self.mouseDelayJTextField).setText(String::from("100"));
        let mut mouseActionJDialogLayout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(jcall!(self.mouseActionJDialog).getContentPane());
        jcall!(jcall!(self.mouseActionJDialog).getContentPane()).setLayout(mouseActionJDialogLayout.clone());
        jcall!(mouseActionJDialogLayout).setHorizontalGroup(jcall!(jcall!(mouseActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(mouseActionJDialogLayout).createSequentialGroup()).addContainerGap()).addGroup(jcall!(jcall!(jcall!(mouseActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addComponent(self.inputTypeJLabel.clone())).addGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone(), jcall!(jcall!(jcall!(mouseActionJDialogLayout).createSequentialGroup()).addGroup(jcall!(jcall!(jcall!(mouseActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone(), jcall!(jcall!(jcall!(mouseActionJDialogLayout).createSequentialGroup()).addComponent(self.jLabel2.clone())).addGap(25, 25, 25))).addGroup(jcall!(jcall!(jcall!(mouseActionJDialogLayout).createSequentialGroup()).addGroup(jcall!(jcall!(jcall!(mouseActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addComponent(self.jLabel3.clone())).addComponent(self.jLabel6.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())))).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(mouseActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addComponent(self.mouseDelayJTextField.clone(), jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 154, i16::MAX.clone())).addGroup(jcall!(jcall!(jcall!(mouseActionJDialogLayout).createSequentialGroup()).addComponent(self.titleJLabel.clone())).addGap(45, 45, 45))).addGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone(), jcall!(jcall!(jcall!(mouseActionJDialogLayout).createSequentialGroup()).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.inputAutomationTypeJComboBox.clone(), 0, 154, i16::MAX.clone()))).addGroup(jcall!(jcall!(jcall!(jcall!(mouseActionJDialogLayout).createSequentialGroup()).addComponent(self.button1JCheckBox.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.button2JCheckBox.clone()))).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(mouseActionJDialogLayout).createSequentialGroup()).addGroup(jcall!(jcall!(jcall!(mouseActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone())).addGroup(jcall!(jcall!(jcall!(mouseActionJDialogLayout).createSequentialGroup()).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.okJButton.clone()))).addGroup(jcall!(jcall!(jcall!(jcall!(mouseActionJDialogLayout).createSequentialGroup()).addComponent(self.jLabel4.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone(), 5, i16::MAX.clone())).addComponent(self.mouseXJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 59, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.jLabel5.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.mouseYJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 62, i16::MAX.clone())))))).addContainerGap()));
        jcall!(mouseActionJDialogLayout).setVerticalGroup(jcall!(jcall!(mouseActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(mouseActionJDialogLayout).createSequentialGroup()).addComponent(self.titleJLabel.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(mouseActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.inputTypeJLabel.clone())).addComponent(self.inputAutomationTypeJComboBox.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(mouseActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addComponent(self.jLabel2.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 15, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addGroup(jcall!(jcall!(jcall!(mouseActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.button1JCheckBox.clone())).addComponent(self.button2JCheckBox.clone())))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(mouseActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.jLabel6.clone())).addComponent(self.mouseDelayJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(mouseActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.jLabel3.clone())).addComponent(self.jLabel4.clone())).addComponent(self.jLabel5.clone())).addComponent(self.mouseYJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addComponent(self.mouseXJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone())).addComponent(self.okJButton.clone())).addContainerGap()));
        self.setMinimumSize(Dimension::new(100, 0));
        jcall!(self.mouseActionJButton).setText(String::from("Edit"));
        { let java_receiver = self.mouseActionJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.mouseActionJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jLabel1).setText(String::from("Mouse:"));
        let mut layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.setLayout(layout.clone());
        jcall!(layout).setHorizontalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addComponent(self.jLabel1.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone(), 36, i16::MAX.clone())).addComponent(self.mouseActionJButton.clone())));
        jcall!(layout).setVerticalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.jLabel1.clone())).addComponent(self.mouseActionJButton.clone())));
    }
    

    pub fn button2JCheckBoxActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
    }
    

    pub fn okJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.update();
            jcall!(self.getMouseActionJDialog()).setVisible(false);
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("okButtonActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn mouseActionJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        jcall!(self.getMouseActionJDialog()).setVisible(true);
    }
}
impl std::ops::Deref for MouseActionScriptInputJPanel {
    type Target = JPanel;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MouseActionScriptInputJPanel {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



