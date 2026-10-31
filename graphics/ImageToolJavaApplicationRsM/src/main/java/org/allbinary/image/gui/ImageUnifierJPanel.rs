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
// use crate::java::awt::image::BufferedImage::BufferedImage;
// use crate::java::io::File::File;
// use crate::javax::swing::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::file::FileWrapperUtil::FileWrapperUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::media::image::ImagePersistanceUtil::ImagePersistanceUtil;
use crate::org::allbinary::media::image::ImageProcessorInput::ImageProcessorInput;
use crate::org::allbinary::media::image::ImageProcessorInputCompositeInterface::ImageProcessorInputCompositeInterface;
use crate::org::allbinary::media::image::ImageStrings::ImageStrings;
use crate::org::allbinary::media::image::ImageUnifierCell::ImageUnifierCell;
use crate::org::allbinary::media::image::ImageUnifierProperties::ImageUnifierProperties;
use crate::org::allbinary::media::image::ImageUnifierUtil::ImageUnifierUtil;
use crate::org::allbinary::media::image::ImageUtil::ImageUtil;
use crate::org::allbinary::media::image::ImagesRatioUtil::ImagesRatioUtil;
use crate::org::allbinary::string::CommonPhoneStrings::CommonPhoneStrings;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageUnifierJPanel {
    pub base: JPanel,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub imageStrings: Option<Rc<RefCell<ImageStrings>>>,
    pub imagesRatioUtil: Option<Rc<RefCell<ImagesRatioUtil>>>,
    pub imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>,
    pub icon: Vec<Option<Rc<RefCell<ImageIconUnique>>>>,
    pub imageUnifierProperties: Option<Rc<RefCell<ImageUnifierProperties>>>,
    pub result: Option<Rc<RefCell<BufferedImage>>>,
    pub isImageFillIn: bool,
    pub avgRatioJTextField: Option<Rc<RefCell<JTextField>>>,
    pub cellHeightJTextField: Option<Rc<RefCell<JTextField>>>,
    pub cellRatioJTextField: Option<Rc<RefCell<JTextField>>>,
    pub cellWidthJTextField: Option<Rc<RefCell<JTextField>>>,
    pub columnsJTextField: Option<Rc<RefCell<JTextField>>>,
    pub downJButton: Option<Rc<RefCell<JButton>>>,
    pub fudgeItJButton: Option<Rc<RefCell<JButton>>>,
    pub imageJList: Option<Rc<RefCell<JList>>>,
    pub jButton1: Option<Rc<RefCell<JButton>>>,
    pub jButton2: Option<Rc<RefCell<JButton>>>,
    pub jButtonOrder: Option<Rc<RefCell<JButton>>>,
    pub jLabel1: Option<Rc<RefCell<JLabel>>>,
    pub jLabel2: Option<Rc<RefCell<JLabel>>>,
    pub jLabel3: Option<Rc<RefCell<JLabel>>>,
    pub jLabel4: Option<Rc<RefCell<JLabel>>>,
    pub jLabel5: Option<Rc<RefCell<JLabel>>>,
    pub jLabel6: Option<Rc<RefCell<JLabel>>>,
    pub jPanel1: Option<Rc<RefCell<JPanel>>>,
    pub jScrollPane1: Option<Rc<RefCell<JScrollPane>>>,
    pub jScrollPane2: Option<Rc<RefCell<JScrollPane>>>,
    pub rowsJTextField: Option<Rc<RefCell<JTextField>>>,
    pub upJButton: Option<Rc<RefCell<JButton>>>,
}

impl ImageProcessorInputCompositeInterface for ImageUnifierJPanel {
    fn getImageProcessorInput(&mut self) -> Option<Rc<RefCell<ImageProcessorInput>>> { Self::getImageProcessorInput(self) }
    fn setImageProcessorInput(&mut self, mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>) { Self::setImageProcessorInput(self, imageProcessorInput) }
}

impl ImageUnifierJPanel {
    

    pub fn new_struct(mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>) -> Self {
        let mut this = Self {
            base: JPanel::new_struct(),
            logUtil: None,
            commonStrings: None,
            imageStrings: None,
            imagesRatioUtil: None,
            imageProcessorInput: None,
            icon: Vec::new(),
            imageUnifierProperties: None,
            result: None,
            isImageFillIn: false,
            avgRatioJTextField: None,
            cellHeightJTextField: None,
            cellRatioJTextField: None,
            cellWidthJTextField: None,
            columnsJTextField: None,
            downJButton: None,
            fudgeItJButton: None,
            imageJList: None,
            jButton1: None,
            jButton2: None,
            jButtonOrder: None,
            jLabel1: None,
            jLabel2: None,
            jLabel3: None,
            jLabel4: None,
            jLabel5: None,
            jLabel6: None,
            jPanel1: None,
            jScrollPane1: None,
            jScrollPane2: None,
            rowsJTextField: None,
            upJButton: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.imageStrings = ImageStrings::getInstance();
        this.imagesRatioUtil = ImagesRatioUtil::getInstance();
        this.isImageFillIn = true;
        this.initComponents();
        this.setImageProcessorInput(imageProcessorInput.clone());
        this
    }

    pub fn new(mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(imageProcessorInput))))
    }
    

    pub fn init(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>> = self.getImageProcessorInput();
            let mut bufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = jcall!(imageProcessorInput).getBufferedImageArray();
            self.icon = vec![None; ((bufferedImageArray.len() as i32)) as usize];
            let mut defaultListModel: Option<Rc<RefCell<DefaultListModel>>> = DefaultListModel::new();
            for index in (0..(bufferedImageArray.len() as i32)) {
                let mut width: f64 = jcall!(bufferedImageArray[(index) as usize]).getWidth();
                let mut height: f64 = jcall!(bufferedImageArray[(index) as usize]).getHeight();
                let mut newWidth: i32 = 52;
                let mut oldRatio: f64 = width / height;
                let mut newHeight: i32 = (((((newWidth) as f64) / oldRatio)) as i32);
                let mut iconBufferedImage: Option<Rc<RefCell<BufferedImage>>> = jcall!(ImageUtil::getInstance()).createBufferedImage(bufferedImageArray[(index) as usize].clone(), newWidth.clone(), newHeight.clone(), true);
                self.icon[(index) as usize] = ImageIconUnique::new(iconBufferedImage.clone(), index.clone());
                jcall!(defaultListModel).addElement(self.icon[(index) as usize].clone());
            }
            jcall!(self.imageJList).setModel(defaultListModel.clone());
            self.updateOnPropertiesChange();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn updateImage(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut bufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = jcall!(self.getImageProcessorInput()).getBufferedImageArray();
            let mut tempBufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = vec![None; ((bufferedImageArray.len() as i32)) as usize];
            for index in (0..(bufferedImageArray.len() as i32)) {
                let mut indexedImageIcon: Option<Rc<RefCell<ImageIconUnique>>> = (jcall!(jcall!(self.imageJList).getModel()).getElementAt(index.clone())).clone() /* as ImageIconUnique */;
                let mut bufferedImage: Option<Rc<RefCell<BufferedImage>>> = jcall!(ImageUtil::getInstance()).createBufferedImage(bufferedImageArray[(jcall!(indexedImageIcon).getId()) as usize].clone(), jcall!(jcall!(self.imageUnifierProperties).getImageUnifierCell()).getWidth(), jcall!(jcall!(self.imageUnifierProperties).getImageUnifierCell()).getHeight(), false);
                tempBufferedImageArray[(index) as usize] = bufferedImage.clone();
            }
            let mut totalImages: i32 = jcall!(self.imageUnifierProperties).getRows() * jcall!(self.imageUnifierProperties).getColumns();
            let mut averageRatio: f64 = jcall!(self.imagesRatioUtil).getAverage(tempBufferedImageArray.clone(), totalImages.clone());
            let mut averageRatioString: String = jcall!(Some((averageRatio.clone()) as f64)).toString();
            if (averageRatioString.chars().count() as i32) > 4 {
                averageRatioString = averageRatioString[(0) as usize..(4) as usize].to_string();
            }
            jcall!(self.avgRatioJTextField).setText(averageRatioString.clone());
            jcall!(self.fudgeItJButton).setEnabled(false);
            if !jcall!(self.imagesRatioUtil).isEqual(tempBufferedImageArray.clone(), totalImages.clone()) {
                if self.isImageFillIn {
                    jcall!(self.fudgeItJButton).setEnabled(true);
                }
            }
            self.updateImage1(tempBufferedImageArray.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("updateImage"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn updateImageWithFudgedImages(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut bufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = jcall!(self.getImageProcessorInput()).getBufferedImageArray();
            let mut tempBufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = vec![None; ((bufferedImageArray.len() as i32)) as usize];
            for index in (0..(bufferedImageArray.len() as i32)) {
                let mut indexedImageIcon: Option<Rc<RefCell<ImageIconUnique>>> = (jcall!(jcall!(self.imageJList).getModel()).getElementAt(index.clone())).clone() /* as ImageIconUnique */;
                let mut bufferedImage: Option<Rc<RefCell<BufferedImage>>> = jcall!(ImageUtil::getInstance()).createBufferedImage(bufferedImageArray[(jcall!(indexedImageIcon).getId()) as usize].clone(), jcall!(jcall!(self.imageUnifierProperties).getImageUnifierCell()).getWidth(), jcall!(jcall!(self.imageUnifierProperties).getImageUnifierCell()).getHeight(), false);
                tempBufferedImageArray[(index) as usize] = bufferedImage.clone();
            }
            let mut totalImages: i32 = jcall!(self.imageUnifierProperties).getRows() * jcall!(self.imageUnifierProperties).getColumns();
            let mut averageRatio: f64 = jcall!(self.imagesRatioUtil).getAverage(tempBufferedImageArray.clone(), totalImages.clone());
            let mut fudgedBufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = jcall!(self.imagesRatioUtil).fudge3(tempBufferedImageArray.clone(), totalImages.clone(), averageRatio.clone());
            self.updateImage1(fudgedBufferedImageArray.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("updateImageWithFudgedImages"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn updateImage1(&mut self, mut tempBufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>>) {
        self.result = jcall!(ImageUnifierUtil::getInstance()).getImage(tempBufferedImageArray.clone(), self.imageUnifierProperties.clone());
        let mut image: Option<Rc<RefCell<Icon>>> = ImageIcon::new(self.result.clone());
        let mut label: Option<Rc<RefCell<JLabel>>> = JLabel::new(image.clone());
        jcall!(jcall!(self.jScrollPane2).getViewport()).add(label.clone());
        self.updateUI();
    }
    

    pub fn updateOnPropertiesChange(&mut self) {
        if jcall!(jcall!(self.rowsJTextField).getText()).length() > 0 && jcall!(jcall!(self.columnsJTextField).getText()).length() > 0 && jcall!(jcall!(self.cellWidthJTextField).getText()).length() > 0 && jcall!(jcall!(self.cellHeightJTextField).getText()).length() > 0 {
            self.imageUnifierProperties = ImageUnifierProperties::new();
            { let java_receiver = self.imageUnifierProperties.clone(); let java_arguments = (Some((jcall!(self.rowsJTextField).getText()) as i32),); jcall!(java_receiver).setRows(java_arguments.0); };
            { let java_receiver = self.imageUnifierProperties.clone(); let java_arguments = (Some((jcall!(self.columnsJTextField).getText()) as i32),); jcall!(java_receiver).setColumns(java_arguments.0); };
            let mut imageUnifierCell: Option<Rc<RefCell<ImageUnifierCell>>> = ImageUnifierCell::new(Some((jcall!(self.cellWidthJTextField).getText()) as i32), Some((jcall!(self.cellHeightJTextField).getText()) as i32));
            jcall!(self.imageUnifierProperties).setImageUnifierCell(imageUnifierCell.clone());
            let mut cellRatio: f64 = (((jcall!(imageUnifierCell).getWidth()) as f64) / jcall!(imageUnifierCell).getHeight());
            let mut cellRatioString: String = jcall!(Some((cellRatio.clone()) as f64)).toString();
            if (cellRatioString.chars().count() as i32) > 4 {
                cellRatioString = cellRatioString[(0) as usize..(4) as usize].to_string();
            }
            jcall!(self.cellRatioJTextField).setText(cellRatioString.clone());
            self.updateImage();
        }
    }
    

    pub fn initComponents(&mut self) {
        let mut commonPhoneStrings: Option<Rc<RefCell<CommonPhoneStrings>>> = CommonPhoneStrings::getInstance();
        self.jScrollPane1 = JScrollPane::new();
        self.imageJList = JList::new();
        self.jScrollPane2 = JScrollPane::new();
        self.jPanel1 = JPanel::new();
        self.jButton1 = JButton::new();
        self.upJButton = JButton::new();
        self.downJButton = JButton::new();
        self.fudgeItJButton = JButton::new();
        self.jLabel2 = JLabel::new();
        self.columnsJTextField = JTextField::new();
        self.jLabel1 = JLabel::new();
        self.rowsJTextField = JTextField::new();
        self.jLabel3 = JLabel::new();
        self.cellWidthJTextField = JTextField::new();
        self.jLabel4 = JLabel::new();
        self.cellHeightJTextField = JTextField::new();
        self.jLabel5 = JLabel::new();
        self.cellRatioJTextField = JTextField::new();
        self.jLabel6 = JLabel::new();
        self.avgRatioJTextField = JTextField::new();
        self.jButton2 = JButton::new();
        self.jButtonOrder = JButton::new();
        self.setMinimumSize(Dimension::new(500, 375));
        self.setPreferredSize(Dimension::new(0, 0));
        jcall!(self.imageJList).setMinimumSize(Dimension::new(50, 0));
        jcall!(self.jScrollPane1).setViewportView(self.imageJList.clone());
        jcall!(self.jButton1).setText(String::from("Save"));
        { let java_receiver = self.jButton1.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.jButton1ActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.upJButton).setText(String::from("Up"));
        { let java_receiver = self.upJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.upJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.downJButton).setText(String::from("Down"));
        { let java_receiver = self.downJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.downJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.fudgeItJButton).setText(String::from("Fudge It!"));
        { let java_receiver = self.fudgeItJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.fudgeItJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jLabel2).setText(String::from("Columns:"));
        jcall!(self.columnsJTextField).setText(jcall!(commonPhoneStrings).TWO.clone());
        { let java_receiver = self.columnsJTextField.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.columnsJTextFieldActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        { let java_receiver = self.columnsJTextField.clone(); let java_arguments = (PropertyChangeListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn propertyChange(&mut self, mut evt: Option<Rc<RefCell<PropertyChangeEvent>>>) {
                self.columnsJTextFieldPropertyChange(evt.clone());
            }
*/,); jcall!(java_receiver).addPropertyChangeListener(java_arguments.0); };
        { let java_receiver = self.columnsJTextField.clone(); let java_arguments = (KeyAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn keyPressed(&mut self, mut evt: Option<Rc<RefCell<KeyEvent>>>) {
                self.columnsJTextFieldKeyPressed(evt.clone());
            }
            

            pub fn keyTyped(&mut self, mut evt: Option<Rc<RefCell<KeyEvent>>>) {
                self.columnsJTextFieldKeyTyped(evt.clone());
            }
*/,); jcall!(java_receiver).addKeyListener(java_arguments.0); };
        jcall!(self.jLabel1).setText(String::from("Rows:"));
        jcall!(self.rowsJTextField).setText(jcall!(commonPhoneStrings).TWO.clone());
        { let java_receiver = self.rowsJTextField.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.rowsJTextFieldActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jLabel3).setText(String::from("Cell Width:"));
        jcall!(self.cellWidthJTextField).setText(String::from("200"));
        { let java_receiver = self.cellWidthJTextField.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.cellWidthJTextFieldActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jLabel4).setText(String::from("Cell Height:"));
        jcall!(self.cellHeightJTextField).setText(String::from("150"));
        { let java_receiver = self.cellHeightJTextField.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.cellHeightJTextFieldActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jLabel5).setText(String::from("Cell Ratio:"));
        jcall!(self.cellRatioJTextField).setEditable(false);
        jcall!(self.jLabel6).setText(String::from("Avg Image Ratio:"));
        jcall!(self.avgRatioJTextField).setEditable(false);
        jcall!(self.jButton2).setText(jcall!(self.commonStrings).UPDATE.clone());
        { let java_receiver = self.jButton2.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.jButton2ActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jButtonOrder).setText(String::from("Order"));
        { let java_receiver = self.jButtonOrder.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.jButtonOrderActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        let mut jPanel1Layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(self.jPanel1.clone());
        jcall!(self.jPanel1).setLayout(jPanel1Layout.clone());
        jcall!(jPanel1Layout).setHorizontalGroup(jcall!(jcall!(jPanel1Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone(), jcall!(jcall!(jcall!(jcall!(jPanel1Layout).createSequentialGroup()).addContainerGap()).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jPanel1Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone())).addComponent(self.jButton1.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 151, i16::MAX.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jPanel1Layout).createSequentialGroup()).addComponent(self.upJButton.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 71, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone(), 10, i16::MAX.clone())).addComponent(self.downJButton.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 70, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addComponent(self.fudgeItJButton.clone(), jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 151, i16::MAX.clone())).addGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone(), jcall!(jcall!(jcall!(jPanel1Layout).createSequentialGroup()).addComponent(self.jLabel3.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone(), 99, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone(), jcall!(jcall!(jcall!(jPanel1Layout).createSequentialGroup()).addComponent(self.jLabel4.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone(), 96, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addComponent(self.avgRatioJTextField.clone(), jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 151, i16::MAX.clone())).addComponent(self.jLabel6.clone(), jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 151, i16::MAX.clone())).addComponent(self.cellRatioJTextField.clone(), jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 151, i16::MAX.clone())).addComponent(self.jLabel5.clone(), jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 151, i16::MAX.clone())).addComponent(self.cellHeightJTextField.clone(), jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 151, i16::MAX.clone())).addComponent(self.cellWidthJTextField.clone(), jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 151, i16::MAX.clone())).addComponent(self.jButton2.clone(), jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 151, i16::MAX.clone())).addGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone(), jcall!(jcall!(jcall!(jcall!(jPanel1Layout).createSequentialGroup()).addGroup(jcall!(jcall!(jcall!(jPanel1Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addComponent(self.jLabel2.clone())).addComponent(self.jLabel1.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(jPanel1Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addComponent(self.rowsJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 103, i16::MAX.clone())).addGroup(jcall!(jcall!(jcall!(jPanel1Layout).createSequentialGroup()).addComponent(self.columnsJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 102, i16::MAX.clone())).addGap(1, 1, 1))))).addComponent(self.jButtonOrder.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 151, i16::MAX.clone()))).addContainerGap()));
        jcall!(jPanel1Layout).setVerticalGroup(jcall!(jcall!(jPanel1Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jPanel1Layout).createSequentialGroup()).addContainerGap()).addComponent(self.jButton1.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(jPanel1Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.upJButton.clone())).addComponent(self.downJButton.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.jButtonOrder.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.fudgeItJButton.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(jPanel1Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.jLabel2.clone())).addComponent(self.columnsJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addGroup(jcall!(jcall!(jcall!(jPanel1Layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).BASELINE.clone())).addComponent(self.jLabel1.clone())).addComponent(self.rowsJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.jLabel3.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.cellWidthJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.jLabel4.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.cellHeightJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.jButton2.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.jLabel5.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone())).addComponent(self.cellRatioJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addGap(7, 7, 7)).addComponent(self.jLabel6.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.avgRatioJTextField.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addGap(66, 66, 66)));
        let mut layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(None /* Java this: a &mut self method has no Rc of itself */);
        self.setLayout(layout.clone());
        jcall!(layout).setHorizontalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(layout).createSequentialGroup()).addComponent(self.jScrollPane1.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 84, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.jPanel1.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).LayoutStyle).ComponentPlacement).RELATED.clone())).addComponent(self.jScrollPane2.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 527, i16::MAX.clone())));
        jcall!(layout).setVerticalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone(), jcall!(jcall!(jcall!(layout).createSequentialGroup()).addGroup(jcall!(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone())).addComponent(self.jPanel1.clone(), jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone(), 0, 445, i16::MAX.clone())).addComponent(self.jScrollPane2.clone(), jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 445, i16::MAX.clone())).addComponent(self.jScrollPane1.clone(), jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 445, i16::MAX.clone()))).addContainerGap()));
    }
    

    pub fn jButton2ActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        self.updateOnPropertiesChange();
    }
    

    pub fn columnsJTextFieldKeyPressed(&mut self, mut evt: Option<Rc<RefCell<KeyEvent>>>) {
    }
    

    pub fn columnsJTextFieldKeyTyped(&mut self, mut evt: Option<Rc<RefCell<KeyEvent>>>) {
    }
    

    pub fn columnsJTextFieldPropertyChange(&mut self, mut evt: Option<Rc<RefCell<PropertyChangeEvent>>>) {
    }
    

    pub fn fudgeItJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        self.updateImageWithFudgedImages();
    }
    

    pub fn cellHeightJTextFieldActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        self.updateOnPropertiesChange();
    }
    

    pub fn cellWidthJTextFieldActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        self.updateOnPropertiesChange();
    }
    

    pub fn columnsJTextFieldActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        self.updateOnPropertiesChange();
    }
    

    pub fn rowsJTextFieldActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        self.updateOnPropertiesChange();
    }
    

    pub fn jButton1ActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut fileArray: Vec<Option<Rc<RefCell<File>>>> = jcall!(self.imageProcessorInput).getFiles();
            let mut file: Option<Rc<RefCell<File>>> = fileArray[(0) as usize].clone();
            let mut filePath: String = jcall!(file).getAbsolutePath();
            let mut extensionIndex: i32 = filePath.find(&jcall!(self.imageStrings).PNG_EXTENSION.clone()).map(|index| index as i32).unwrap_or(-1);
            filePath = jcall!(StringMaker::new()).append(filePath[(0) as usize..(extensionIndex) as usize].to_string()).append(jcall!(CommonSeps::getInstance()).UNDERSCORE.clone()).appendint(((jcall!(self.imageUnifierProperties).getColumns()) as i32)).append(String::from("_By_")).appendint(((jcall!(self.imageUnifierProperties).getRows()) as i32)).append(String::from("_Unified")).append(jcall!(self.imageStrings).PNG_EXTENSION.clone()).toString();
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("New File Path: "), filePath), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            let mut outputFile: Option<Rc<RefCell<File>>> = File::new(filePath.clone());
            jcall!(ImagePersistanceUtil::getInstance()).saveWithBatik(FileWrapperUtil::wrapFile(outputFile.clone()), self.result.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("jButton1ActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn downJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        let mut index: i32 = jcall!(self.imageJList).getSelectedIndex();
        let mut defaultListModel: Option<Rc<RefCell<DefaultListModel>>> = (jcall!(self.imageJList).getModel()).clone() /* as DefaultListModel */;
        if index >= 0 && index + 1 < jcall!(defaultListModel).size() {
            let mut indexedImageIcon: Option<Rc<RefCell<ImageIconUnique>>> = (jcall!(defaultListModel).remove(index.clone())).clone() /* as ImageIconUnique */;
            jcall!(defaultListModel).insertElementAt(indexedImageIcon.clone(), index + 1);
            jcall!(self.imageJList).setSelectedIndex(index + 1);
            self.updateImage();
        }
    }
    

    pub fn upJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        let mut index: i32 = jcall!(self.imageJList).getSelectedIndex();
        if index > 0 {
            let mut defaultListModel: Option<Rc<RefCell<DefaultListModel>>> = (jcall!(self.imageJList).getModel()).clone() /* as DefaultListModel */;
            let mut indexedImageIcon: Option<Rc<RefCell<ImageIconUnique>>> = (jcall!(defaultListModel).remove(index.clone())).clone() /* as ImageIconUnique */;
            jcall!(defaultListModel).insertElementAt(indexedImageIcon.clone(), index - 1);
            jcall!(self.imageJList).setSelectedIndex(index - 1);
            self.updateImage();
        }
    }
    

    pub fn jButtonOrderActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        let mut defaultListModel: Option<Rc<RefCell<DefaultListModel>>> = (jcall!(self.imageJList).getModel()).clone() /* as DefaultListModel */;
        let mut size: i32 = jcall!(defaultListModel).size();
        let mut last: i32 = size - 1;
        for index in (0..=last).rev() {
            let mut indexedImageIcon: Option<Rc<RefCell<ImageIconUnique>>> = (jcall!(defaultListModel).remove(last.clone())).clone() /* as ImageIconUnique */;
            jcall!(defaultListModel).insertElementAt(indexedImageIcon.clone(), last - index);
        }
        if size > 0 {
            jcall!(self.imageJList).setSelectedIndex(0);
            self.updateImage();
        }
    }
    

    pub fn getImageProcessorInput(&mut self) -> Option<Rc<RefCell<ImageProcessorInput>>> {
        return self.imageProcessorInput.clone();
    }
    

    pub fn setImageProcessorInput(&mut self, mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>) {
        self.imageProcessorInput = imageProcessorInput.clone();
        self.init();
    }
}
impl std::ops::Deref for ImageUnifierJPanel {
    type Target = JPanel;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImageUnifierJPanel {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



