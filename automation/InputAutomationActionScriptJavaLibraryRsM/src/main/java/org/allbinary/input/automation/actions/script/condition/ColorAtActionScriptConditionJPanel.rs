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
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::media::image::analysis::ColorRangeInterface::ColorRangeInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::JPanel::JPanel;
use super::swing::swing;
use super::javax::javax;
use super::ColorAtActionScriptConditionInterface::ColorAtActionScriptConditionInterface;
use super::JTextField::JTextField;
use super::JLabel::JLabel;
use super::JDialog::JDialog;
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

pub struct ColorAtActionScriptConditionJPanel {
    pub base: JPanel,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub colorAtActionScriptConditionInterface: Option<Rc<RefCell<dyn ColorAtActionScriptConditionInterface>>>,
    pub blueJLabel: Option<Rc<RefCell<JLabel>>>,
    pub colorAtActionJButton: Option<Rc<RefCell<JButton>>>,
    pub colorAtActionJDialog: Option<Rc<RefCell<JDialog>>>,
    pub colorAtXJTextField: Option<Rc<RefCell<JTextField>>>,
    pub colorAtYJTextField: Option<Rc<RefCell<JTextField>>>,
    pub colorRangeJLabel: Option<Rc<RefCell<JLabel>>>,
    pub greenJLabel: Option<Rc<RefCell<JLabel>>>,
    pub jLabel1: Option<Rc<RefCell<JLabel>>>,
    pub locationJLabel: Option<Rc<RefCell<JLabel>>>,
    pub maxBlueJTextField: Option<Rc<RefCell<JTextField>>>,
    pub maxGreenJTextField: Option<Rc<RefCell<JTextField>>>,
    pub maxJLabel: Option<Rc<RefCell<JLabel>>>,
    pub maxRedJTextField: Option<Rc<RefCell<JTextField>>>,
    pub minBlueJTextField: Option<Rc<RefCell<JTextField>>>,
    pub minGreenJTextField: Option<Rc<RefCell<JTextField>>>,
    pub minJLabel: Option<Rc<RefCell<JLabel>>>,
    pub minRedJTextField: Option<Rc<RefCell<JTextField>>>,
    pub okJButton: Option<Rc<RefCell<JButton>>>,
    pub redJLabel: Option<Rc<RefCell<JLabel>>>,
    pub titleJLabel: Option<Rc<RefCell<JLabel>>>,
    pub xJLabel: Option<Rc<RefCell<JLabel>>>,
    pub yJLabel: Option<Rc<RefCell<JLabel>>>,
}


impl ColorAtActionScriptConditionJPanel {
    

    pub fn new_struct(mut colorAtActionScriptConditionInterface: Option<Rc<RefCell<dyn ColorAtActionScriptConditionInterface>>>) -> Self {
        let mut this = Self {
            base: JPanel::new_struct(),
            logUtil: None,
            commonStrings: None,
            colorAtActionScriptConditionInterface: None,
            blueJLabel: None,
            colorAtActionJButton: None,
            colorAtActionJDialog: None,
            colorAtXJTextField: None,
            colorAtYJTextField: None,
            colorRangeJLabel: None,
            greenJLabel: None,
            jLabel1: None,
            locationJLabel: None,
            maxBlueJTextField: None,
            maxGreenJTextField: None,
            maxJLabel: None,
            maxRedJTextField: None,
            minBlueJTextField: None,
            minGreenJTextField: None,
            minJLabel: None,
            minRedJTextField: None,
            okJButton: None,
            redJLabel: None,
            titleJLabel: None,
            xJLabel: None,
            yJLabel: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        this.initComponents();
        this.colorAtActionScriptConditionInterface = colorAtActionScriptConditionInterface.clone();
        this.set();
        this
    }

    pub fn new(mut colorAtActionScriptConditionInterface: Option<Rc<RefCell<dyn ColorAtActionScriptConditionInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(colorAtActionScriptConditionInterface))))
    }
    

    pub fn set(&mut self) {
        let mut colorRangeInterface: Option<Rc<RefCell<dyn ColorRangeInterface>>> = jcall!(self.colorAtActionScriptConditionInterface).getColorRangeInterface();
        jcall!(self.getMinRedJTextField()).setText((jcall!(colorRangeInterface).getMinRed()).to_string());
        jcall!(self.getMaxRedJTextField()).setText((jcall!(colorRangeInterface).getMaxRed()).to_string());
        jcall!(self.getMinGreenJTextField()).setText((jcall!(colorRangeInterface).getMinGreen()).to_string());
        jcall!(self.getMaxGreenJTextField()).setText((jcall!(colorRangeInterface).getMaxGreen()).to_string());
        jcall!(self.getMinBlueJTextField()).setText((jcall!(colorRangeInterface).getMinBlue()).to_string());
        jcall!(self.getMaxBlueJTextField()).setText((jcall!(colorRangeInterface).getMaxBlue()).to_string());
        jcall!(self.getColorAtXJTextField()).setText((jcall!(jcall!(self.colorAtActionScriptConditionInterface).getPoint()).x.clone()).to_string());
        jcall!(self.getColorAtYJTextField()).setText((jcall!(jcall!(self.colorAtActionScriptConditionInterface).getPoint()).y.clone()).to_string());
        jcall!(self.colorAtActionScriptConditionInterface).log();
    }
    

    pub fn update(&mut self) {
        let mut colorRangeInterface: Option<Rc<RefCell<dyn ColorRangeInterface>>> = jcall!(self.colorAtActionScriptConditionInterface).getColorRangeInterface();
        jcall!(colorRangeInterface).setMinRed((((jcall!(self.getMinRedJTextField()).getText()).to_string()).unwrap() as i32));
        jcall!(colorRangeInterface).setMaxRed((((jcall!(self.getMaxRedJTextField()).getText()).to_string()).unwrap() as i32));
        jcall!(colorRangeInterface).setMinGreen((((jcall!(self.getMinGreenJTextField()).getText()).to_string()).unwrap() as i32));
        jcall!(colorRangeInterface).setMaxGreen((((jcall!(self.getMaxGreenJTextField()).getText()).to_string()).unwrap() as i32));
        jcall!(colorRangeInterface).setMinBlue((((jcall!(self.getMinBlueJTextField()).getText()).to_string()).unwrap() as i32));
        jcall!(colorRangeInterface).setMaxBlue((((jcall!(self.getMaxBlueJTextField()).getText()).to_string()).unwrap() as i32));
        let mut xString: String = jcall!(self.getColorAtXJTextField()).getText();
        if !jcall!(StringValidationUtil::getInstance()).isEmpty(xString.clone()) && jcall!(StringValidationUtil::getInstance()).isNumber(xString.clone()) {
            jcall!(jcall!(self.colorAtActionScriptConditionInterface).getPoint()).x = xString.clone().parse::<i32>().unwrap();
        }
        let mut yString: String = jcall!(self.getColorAtYJTextField()).getText();
        if !jcall!(StringValidationUtil::getInstance()).isEmpty(yString.clone()) && jcall!(StringValidationUtil::getInstance()).isNumber(yString.clone()) {
            jcall!(jcall!(self.colorAtActionScriptConditionInterface).getPoint()).y = xString.clone().parse::<i32>().unwrap();
        }
        jcall!(self.colorAtActionScriptConditionInterface).log();
    }
    

    pub fn getColorAtXJTextField(&mut self) -> Option<Rc<RefCell<JTextField>>> {
        return self.colorAtXJTextField.clone();
    }
    

    pub fn setColorAtXJTextField(&mut self, mut colorAtXJTextField: Option<Rc<RefCell<JTextField>>>) {
        self.colorAtXJTextField = colorAtXJTextField.clone();
    }
    

    pub fn getColorAtYJTextField(&mut self) -> Option<Rc<RefCell<JTextField>>> {
        return self.colorAtYJTextField.clone();
    }
    

    pub fn setColorAtYJTextField(&mut self, mut colorAtYJTextField: Option<Rc<RefCell<JTextField>>>) {
        self.colorAtYJTextField = colorAtYJTextField.clone();
    }
    

    pub fn getMaxBlueJTextField(&mut self) -> Option<Rc<RefCell<JTextField>>> {
        return self.maxBlueJTextField.clone();
    }
    

    pub fn setMaxBlueJTextField(&mut self, mut maxBlueJTextField: Option<Rc<RefCell<JTextField>>>) {
        self.maxBlueJTextField = maxBlueJTextField.clone();
    }
    

    pub fn getMaxGreenJTextField(&mut self) -> Option<Rc<RefCell<JTextField>>> {
        return self.maxGreenJTextField.clone();
    }
    

    pub fn setMaxGreenJTextField(&mut self, mut maxGreenJTextField: Option<Rc<RefCell<JTextField>>>) {
        self.maxGreenJTextField = maxGreenJTextField.clone();
    }
    

    pub fn getMaxRedJTextField(&mut self) -> Option<Rc<RefCell<JTextField>>> {
        return self.maxRedJTextField.clone();
    }
    

    pub fn setMaxRedJTextField(&mut self, mut maxRedJTextField: Option<Rc<RefCell<JTextField>>>) {
        self.maxRedJTextField = maxRedJTextField.clone();
    }
    

    pub fn getMinBlueJTextField(&mut self) -> Option<Rc<RefCell<JTextField>>> {
        return self.minBlueJTextField.clone();
    }
    

    pub fn setMinBlueJTextField(&mut self, mut minBlueJTextField: Option<Rc<RefCell<JTextField>>>) {
        self.minBlueJTextField = minBlueJTextField.clone();
    }
    

    pub fn getMinGreenJTextField(&mut self) -> Option<Rc<RefCell<JTextField>>> {
        return self.minGreenJTextField.clone();
    }
    

    pub fn setMinGreenJTextField(&mut self, mut minGreenJTextField: Option<Rc<RefCell<JTextField>>>) {
        self.minGreenJTextField = minGreenJTextField.clone();
    }
    

    pub fn getMinRedJTextField(&mut self) -> Option<Rc<RefCell<JTextField>>> {
        return self.minRedJTextField.clone();
    }
    

    pub fn setMinRedJTextField(&mut self, mut minRedJTextField: Option<Rc<RefCell<JTextField>>>) {
        self.minRedJTextField = minRedJTextField.clone();
    }
    

    pub fn getYJLabel(&mut self) -> Option<Rc<RefCell<JLabel>>> {
        return self.yJLabel.clone();
    }
    

    pub fn setYJLabel(&mut self, mut yJLabel: Option<Rc<RefCell<JLabel>>>) {
        self.yJLabel = yJLabel.clone();
    }
    

    pub fn getColorAtActionJDialog(&mut self) -> Option<Rc<RefCell<JDialog>>> {
        return self.colorAtActionJDialog.clone();
    }
    

    pub fn setColorAtActionJDialog(&mut self, mut colorAtActionJDialog: Option<Rc<RefCell<JDialog>>>) {
        self.colorAtActionJDialog = colorAtActionJDialog.clone();
    }
    

    pub fn initComponents(&mut self) {
        self.colorAtActionJDialog = JDialog::new();
        self.titleJLabel = JLabel::new();
        self.locationJLabel = JLabel::new();
        self.colorAtXJTextField = JTextField::new();
        self.xJLabel = JLabel::new();
        self.yJLabel = JLabel::new();
        self.colorAtYJTextField = JTextField::new();
        self.colorRangeJLabel = JLabel::new();
        self.redJLabel = JLabel::new();
        self.minJLabel = JLabel::new();
        self.maxJLabel = JLabel::new();
        self.greenJLabel = JLabel::new();
        self.blueJLabel = JLabel::new();
        self.minRedJTextField = JTextField::new();
        self.maxRedJTextField = JTextField::new();
        self.maxGreenJTextField = JTextField::new();
        self.minBlueJTextField = JTextField::new();
        self.maxBlueJTextField = JTextField::new();
        self.okJButton = JButton::new();
        self.minGreenJTextField = JTextField::new();
        self.colorAtActionJButton = JButton::new();
        self.jLabel1 = JLabel::new();
        jcall!(self.colorAtActionJDialog).setMinimumSize(Dimension::new(350, 250));
        jcall!(self.titleJLabel).setText(String::from("              Color At Action Options            "));
        jcall!(self.locationJLabel).setText(String::from("Location:"));
        jcall!(self.xJLabel).setText(String::from("X:"));
        jcall!(self.yJLabel).setText(String::from("Y:"));
        jcall!(self.colorRangeJLabel).setText(String::from("Color Range"));
        jcall!(self.redJLabel).setText(String::from("Red:"));
        jcall!(self.minJLabel).setText(String::from("Minimum"));
        jcall!(self.maxJLabel).setText(String::from("Maximum"));
        jcall!(self.greenJLabel).setText(String::from("Green:"));
        jcall!(self.blueJLabel).setText(String::from("Blue:"));
        jcall!(self.okJButton).setText(String::from("OK"));
        { let java_receiver = self.okJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.okJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        let mut colorAtActionJDialogLayout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(jcall!(self.colorAtActionJDialog).getContentPane());
        jcall!(jcall!(self.colorAtActionJDialog).getContentPane()).setLayout(colorAtActionJDialogLayout.clone());
        jcall!(colorAtActionJDialogLayout).setHorizontalGroup(jcall!(jcall!(colorAtActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(colorAtActionJDialogLayout).createSequentialGroup()).addContainerGap()).addGroup(jcall!(jcall!(jcall!(colorAtActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(colorAtActionJDialogLayout).createSequentialGroup()).addGroup(jcall!(jcall!(jcall!(colorAtActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone())).addComponent(self.okJButton.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(colorAtActionJDialogLayout).createSequentialGroup()).addGroup(jcall!(jcall!(jcall!(colorAtActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone(), false)).addGroup(jcall!(jcall!(jcall!(colorAtActionJDialogLayout).createSequentialGroup()).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(colorAtActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addComponent(self.blueJLabel.clone())).addComponent(self.greenJLabel.clone())).addComponent(self.redJLabel.clone())).addComponent(self.colorRangeJLabel.clone()))).addGap(29, 29, 29))).addGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone(), jcall!(jcall!(jcall!(jcall!(jcall!(colorAtActionJDialogLayout).createSequentialGroup()).addComponent(self.locationJLabel.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone())).addComponent(self.xJLabel.clone())).addGap(18, 18, 18)))).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(colorAtActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addComponent(self.minBlueJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 40, i16::MAX.clone())).addComponent(self.minGreenJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 40, i16::MAX.clone())).addComponent(self.minJLabel.clone(), jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone())).addComponent(self.minRedJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 40, i16::MAX.clone())).addComponent(self.colorAtXJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 40, i16::MAX.clone()))).addGap(16, 16, 16)))).addGap(6, 6, 6)).addComponent(self.yJLabel.clone())).addGap(24, 24, 24)).addGroup(jcall!(jcall!(jcall!(jcall!(colorAtActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone())).addComponent(self.maxJLabel.clone(), jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone())).addGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone(), jcall!(jcall!(jcall!(colorAtActionJDialogLayout).createSequentialGroup()).addGap(1, 1, 1)).addGroup(jcall!(jcall!(jcall!(jcall!(colorAtActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone(), false)).addComponent(self.maxBlueJTextField.clone(), jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone())).addComponent(self.maxRedJTextField.clone(), jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 43, i16::MAX.clone())).addComponent(self.maxGreenJTextField.clone(), jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 43, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())))).addComponent(self.colorAtYJTextField.clone(), jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 44, i16::MAX.clone()))).addGap(22, 22, 22))).addGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone(), jcall!(jcall!(jcall!(colorAtActionJDialogLayout).createSequentialGroup()).addComponent(self.titleJLabel.clone())).addGap(33, 33, 33)))));
        jcall!(colorAtActionJDialogLayout).setVerticalGroup(jcall!(jcall!(colorAtActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(colorAtActionJDialogLayout).createSequentialGroup()).addContainerGap()).addComponent(self.titleJLabel.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(colorAtActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(colorAtActionJDialogLayout).createSequentialGroup()).addGap(15, 15, 15)).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(colorAtActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.xJLabel.clone())).addComponent(self.locationJLabel.clone())).addComponent(self.colorAtXJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addComponent(self.yJLabel.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(colorAtActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.minJLabel.clone())).addComponent(self.colorRangeJLabel.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(colorAtActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.minRedJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addComponent(self.redJLabel.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(colorAtActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.greenJLabel.clone())).addComponent(self.minGreenJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(colorAtActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.blueJLabel.clone())).addComponent(self.minBlueJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone(), 17, i16::MAX.clone())).addComponent(self.okJButton.clone()))).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(colorAtActionJDialogLayout).createSequentialGroup()).addGap(14, 14, 14)).addComponent(self.colorAtYJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.maxJLabel.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.maxRedJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.maxGreenJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.maxBlueJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())))).addContainerGap()));
        self.setPreferredSize(Dimension::new(100, 0));
        jcall!(self.colorAtActionJButton).setText(String::from("Edit"));
        { let java_receiver = self.colorAtActionJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.colorAtActionJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jLabel1).setText(String::from("Color At:"));
        let mut layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.setLayout(layout.clone());
        jcall!(layout).setHorizontalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addComponent(self.jLabel1.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone(), 15, i16::MAX.clone())).addComponent(self.colorAtActionJButton.clone())));
        jcall!(layout).setVerticalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.jLabel1.clone())).addComponent(self.colorAtActionJButton.clone())));
    }
    

    pub fn okJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        self.update();
        jcall!(self.getColorAtActionJDialog()).setVisible(false);
    }
    

    pub fn colorAtActionJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        jcall!(self.getColorAtActionJDialog()).setVisible(true);
    }
}
impl std::ops::Deref for ColorAtActionScriptConditionJPanel {
    type Target = JPanel;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ColorAtActionScriptConditionJPanel {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



