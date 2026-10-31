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
// package org.allbinary.image.gui
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::media::image::analysis::ImageAnalysisResults::ImageAnalysisResults;
use crate::org::allbinary::media::image::analysis::ImageColorRangeResults::ImageColorRangeResults;
use crate::org::allbinary::media::image::analysis::ImageColorResults::ImageColorResults;
use crate::org::allbinary::string::CommonPhoneStrings::CommonPhoneStrings;
use super::JPanel::JPanel;
use super::swing::swing;
use super::javax::javax;
use super::JLabel::JLabel;
use super::JTextField::JTextField;
use super::JComboBox::JComboBox;
use super::DefaultComboBoxModel::DefaultComboBoxModel;
use super::GroupLayout::GroupLayout;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageAnalysisResultsJPanel {
    pub base: JPanel,
    pub avgBlueJTextField: Option<Rc<RefCell<JTextField>>>,
    pub avgGreenJTextField: Option<Rc<RefCell<JTextField>>>,
    pub avgRedJTextField: Option<Rc<RefCell<JTextField>>>,
    pub jComboBox1: Option<Rc<RefCell<JComboBox>>>,
    pub jLabel1: Option<Rc<RefCell<JLabel>>>,
    pub jLabel10: Option<Rc<RefCell<JLabel>>>,
    pub jLabel11: Option<Rc<RefCell<JLabel>>>,
    pub jLabel12: Option<Rc<RefCell<JLabel>>>,
    pub jLabel13: Option<Rc<RefCell<JLabel>>>,
    pub jLabel2: Option<Rc<RefCell<JLabel>>>,
    pub jLabel3: Option<Rc<RefCell<JLabel>>>,
    pub jLabel4: Option<Rc<RefCell<JLabel>>>,
    pub jLabel5: Option<Rc<RefCell<JLabel>>>,
    pub jLabel6: Option<Rc<RefCell<JLabel>>>,
    pub jLabel7: Option<Rc<RefCell<JLabel>>>,
    pub jLabel8: Option<Rc<RefCell<JLabel>>>,
    pub jLabel9: Option<Rc<RefCell<JLabel>>>,
    pub matchingPixelsJTextField: Option<Rc<RefCell<JTextField>>>,
    pub maxBlueJTextField: Option<Rc<RefCell<JTextField>>>,
    pub maxGreenJTextField: Option<Rc<RefCell<JTextField>>>,
    pub maxRedJTextField: Option<Rc<RefCell<JTextField>>>,
    pub minBlueJTextField: Option<Rc<RefCell<JTextField>>>,
    pub minGreenJTextField: Option<Rc<RefCell<JTextField>>>,
    pub minRedJTextField: Option<Rc<RefCell<JTextField>>>,
    pub percentMatchingJTextField: Option<Rc<RefCell<JTextField>>>,
    pub totalPixelsJTextField: Option<Rc<RefCell<JTextField>>>,
}


impl ImageAnalysisResultsJPanel {
    

    pub fn new_struct(mut imageAnalysisResultsArray: Vec<Option<Rc<RefCell<ImageAnalysisResults>>>>) -> Self {
        let mut this = Self {
            base: JPanel::new_struct(),
            avgBlueJTextField: None,
            avgGreenJTextField: None,
            avgRedJTextField: None,
            jComboBox1: None,
            jLabel1: None,
            jLabel10: None,
            jLabel11: None,
            jLabel12: None,
            jLabel13: None,
            jLabel2: None,
            jLabel3: None,
            jLabel4: None,
            jLabel5: None,
            jLabel6: None,
            jLabel7: None,
            jLabel8: None,
            jLabel9: None,
            matchingPixelsJTextField: None,
            maxBlueJTextField: None,
            maxGreenJTextField: None,
            maxRedJTextField: None,
            minBlueJTextField: None,
            minGreenJTextField: None,
            minRedJTextField: None,
            percentMatchingJTextField: None,
            totalPixelsJTextField: None,
        };
        this.initComponents();
        let mut imageAnalysisResults: Option<Rc<RefCell<ImageAnalysisResults>>> = imageAnalysisResultsArray[(0) as usize].clone();
        this.updateImageColorRangeResults(imageAnalysisResults.clone());
        this
    }

    pub fn new(mut imageAnalysisResultsArray: Vec<Option<Rc<RefCell<ImageAnalysisResults>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(imageAnalysisResultsArray))))
    }
    

    pub fn initComponents(&mut self) {
        self.jLabel1 = JLabel::new();
        self.jLabel2 = JLabel::new();
        self.jLabel3 = JLabel::new();
        self.percentMatchingJTextField = JTextField::new();
        self.totalPixelsJTextField = JTextField::new();
        self.matchingPixelsJTextField = JTextField::new();
        self.jComboBox1 = JComboBox::new();
        self.jLabel4 = JLabel::new();
        self.jLabel5 = JLabel::new();
        self.jLabel6 = JLabel::new();
        self.jLabel7 = JLabel::new();
        self.avgRedJTextField = JTextField::new();
        self.avgGreenJTextField = JTextField::new();
        self.avgBlueJTextField = JTextField::new();
        self.minGreenJTextField = JTextField::new();
        self.jLabel8 = JLabel::new();
        self.jLabel9 = JLabel::new();
        self.jLabel10 = JLabel::new();
        self.minRedJTextField = JTextField::new();
        self.minBlueJTextField = JTextField::new();
        self.jLabel11 = JLabel::new();
        self.jLabel12 = JLabel::new();
        self.jLabel13 = JLabel::new();
        self.maxRedJTextField = JTextField::new();
        self.maxGreenJTextField = JTextField::new();
        self.maxBlueJTextField = JTextField::new();
        jcall!(self.jLabel1).setText(String::from("Matching Pixels:"));
        jcall!(self.jLabel2).setText(String::from("Total Pixels:"));
        jcall!(self.jLabel3).setText(String::from("Percent Matching:"));
        jcall!(self.percentMatchingJTextField).setEditable(false);
        jcall!(self.totalPixelsJTextField).setEditable(false);
        jcall!(self.matchingPixelsJTextField).setEditable(false);
        { let java_receiver = self.jComboBox1.clone(); let java_arguments = (DefaultComboBoxModel::new(vec![jcall!(CommonPhoneStrings::getInstance()).ONE.clone()]),); jcall!(java_receiver).setModel(java_arguments.0); };
        jcall!(self.jComboBox1).setEnabled(false);
        jcall!(self.jLabel4).setText(String::from("Image:"));
        jcall!(self.jLabel5).setText(String::from("Average Red:"));
        jcall!(self.jLabel6).setText(String::from("Average Green:"));
        jcall!(self.jLabel7).setText(String::from("Average Blue:"));
        jcall!(self.avgRedJTextField).setEditable(false);
        jcall!(self.avgGreenJTextField).setEditable(false);
        jcall!(self.avgBlueJTextField).setEditable(false);
        jcall!(self.minGreenJTextField).setEditable(false);
        jcall!(self.jLabel8).setText(String::from("Min Red:"));
        jcall!(self.jLabel9).setText(String::from("Min Green:"));
        jcall!(self.jLabel10).setText(String::from("Min Blue:"));
        jcall!(self.minRedJTextField).setEditable(false);
        jcall!(self.minBlueJTextField).setEditable(false);
        jcall!(self.jLabel11).setText(String::from("Max Red:"));
        jcall!(self.jLabel12).setText(String::from("Max Green:"));
        jcall!(self.jLabel13).setText(String::from("Max Blue:"));
        jcall!(self.maxRedJTextField).setEditable(false);
        jcall!(self.maxGreenJTextField).setEditable(false);
        jcall!(self.maxBlueJTextField).setEditable(false);
        let mut layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.setLayout(layout.clone());
        jcall!(layout).setHorizontalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addContainerGap()).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addComponent(self.jLabel3.clone())).addComponent(self.jLabel2.clone())).addComponent(self.jLabel1.clone())).addComponent(self.jLabel5.clone())).addComponent(self.jLabel6.clone())).addComponent(self.jLabel7.clone())).addComponent(self.jLabel4.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addComponent(self.jComboBox1.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 81, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addComponent(self.avgBlueJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 81, i16::MAX.clone())).addComponent(self.avgGreenJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 81, i16::MAX.clone())).addComponent(self.avgRedJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 81, i16::MAX.clone())).addComponent(self.totalPixelsJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 81, i16::MAX.clone())).addComponent(self.percentMatchingJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 81, i16::MAX.clone())).addComponent(self.matchingPixelsJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 81, i16::MAX.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addComponent(self.jLabel12.clone())).addComponent(self.jLabel13.clone())).addComponent(self.jLabel8.clone())).addComponent(self.jLabel9.clone())).addComponent(self.jLabel10.clone())).addComponent(self.jLabel11.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addComponent(self.minRedJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 82, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addComponent(self.maxGreenJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 82, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addComponent(self.minGreenJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 82, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addComponent(self.minBlueJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 82, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addComponent(self.maxRedJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 82, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addComponent(self.maxBlueJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 82, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addContainerGap(18, i16::MAX.clone())));
        jcall!(layout).setVerticalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addGroup(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.jLabel4.clone())).addComponent(self.jComboBox1.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.jLabel1.clone())).addComponent(self.matchingPixelsJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addComponent(self.jLabel8.clone())).addComponent(self.minRedJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.jLabel2.clone())).addComponent(self.totalPixelsJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addComponent(self.jLabel9.clone())).addComponent(self.minGreenJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.jLabel3.clone())).addComponent(self.percentMatchingJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addComponent(self.jLabel10.clone())).addComponent(self.minBlueJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.jLabel5.clone())).addComponent(self.avgRedJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addComponent(self.jLabel11.clone())).addComponent(self.maxRedJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.jLabel6.clone())).addComponent(self.avgGreenJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addComponent(self.maxGreenJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addComponent(self.jLabel12.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.jLabel7.clone())).addComponent(self.avgBlueJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addComponent(self.jLabel13.clone())).addComponent(self.maxBlueJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))));
    }
    

    pub fn updateImageColorRangeResults(&mut self, mut imageAnalysisResults: Option<Rc<RefCell<ImageAnalysisResults>>>) {
        let mut imageColorRangeResults: Option<Rc<RefCell<ImageColorRangeResults>>> = jcall!(imageAnalysisResults).getImageColorRangeResults();
        { let java_receiver = self.matchingPixelsJTextField.clone(); let java_arguments = (jcall!(Some((jcall!(imageColorRangeResults).getMatchingPixelsChecked()) as i64)).toString(),); jcall!(java_receiver).setText(java_arguments.0); };
        { let java_receiver = self.totalPixelsJTextField.clone(); let java_arguments = (jcall!(Some((jcall!(imageColorRangeResults).getTotalPixelsChecked()) as i64)).toString(),); jcall!(java_receiver).setText(java_arguments.0); };
        { let java_receiver = self.percentMatchingJTextField.clone(); let java_arguments = (jcall!(Some((jcall!(imageColorRangeResults).getPercent()) as f32)).toString(),); jcall!(java_receiver).setText(java_arguments.0); };
        let mut imageColorResults: Option<Rc<RefCell<ImageColorResults>>> = jcall!(imageAnalysisResults).getImageColorResults();
        { let java_receiver = self.avgRedJTextField.clone(); let java_arguments = (jcall!(Some((jcall!(jcall!(imageColorResults).getColorAverage()).getAvgRed()) as f32)).toString(),); jcall!(java_receiver).setText(java_arguments.0); };
        { let java_receiver = self.avgGreenJTextField.clone(); let java_arguments = (jcall!(Some((jcall!(jcall!(imageColorResults).getColorAverage()).getAvgGreen()) as f32)).toString(),); jcall!(java_receiver).setText(java_arguments.0); };
        { let java_receiver = self.avgBlueJTextField.clone(); let java_arguments = (jcall!(Some((jcall!(jcall!(imageColorResults).getColorAverage()).getAvgBlue()) as f32)).toString(),); jcall!(java_receiver).setText(java_arguments.0); };
        { let java_receiver = self.minRedJTextField.clone(); let java_arguments = (jcall!(Some((jcall!(jcall!(imageColorResults).getColorRange()).getMinRed()) as i64)).toString(),); jcall!(java_receiver).setText(java_arguments.0); };
        { let java_receiver = self.minGreenJTextField.clone(); let java_arguments = (jcall!(Some((jcall!(jcall!(imageColorResults).getColorRange()).getMinGreen()) as i64)).toString(),); jcall!(java_receiver).setText(java_arguments.0); };
        { let java_receiver = self.minBlueJTextField.clone(); let java_arguments = (jcall!(Some((jcall!(jcall!(imageColorResults).getColorRange()).getMinBlue()) as i64)).toString(),); jcall!(java_receiver).setText(java_arguments.0); };
        { let java_receiver = self.maxRedJTextField.clone(); let java_arguments = (jcall!(Some((jcall!(jcall!(imageColorResults).getColorRange()).getMaxRed()) as i64)).toString(),); jcall!(java_receiver).setText(java_arguments.0); };
        { let java_receiver = self.maxGreenJTextField.clone(); let java_arguments = (jcall!(Some((jcall!(jcall!(imageColorResults).getColorRange()).getMaxGreen()) as i64)).toString(),); jcall!(java_receiver).setText(java_arguments.0); };
        { let java_receiver = self.maxBlueJTextField.clone(); let java_arguments = (jcall!(Some((jcall!(jcall!(imageColorResults).getColorRange()).getMaxBlue()) as i64)).toString(),); jcall!(java_receiver).setText(java_arguments.0); };
    }
}
impl std::ops::Deref for ImageAnalysisResultsJPanel {
    type Target = JPanel;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImageAnalysisResultsJPanel {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



