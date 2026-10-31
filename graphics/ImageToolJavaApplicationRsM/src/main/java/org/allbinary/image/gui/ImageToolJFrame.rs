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
// use crate::java::util::Arrays::Arrays;
// use crate::java::util::Comparator::Comparator;
// use crate::javax::imageio::ImageIO::ImageIO;
use crate::org::allbinary::gui::dialog::FileJDialog::FileJDialog;
use crate::org::allbinary::gui::dialog::FileJDialogListenerInterface::FileJDialogListenerInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::media::image::ImageProcessorInput::ImageProcessorInput;
use crate::org::allbinary::media::image::ImageProcessorInputCompositeInterface::ImageProcessorInputCompositeInterface;
use crate::org::allbinary::media::image::analysis::ColorRange::ColorRange;
use crate::org::allbinary::media::image::analysis::ColorRangeInterface::ColorRangeInterface;
use crate::org::allbinary::media::image::analysis::ImageAnalysis::ImageAnalysis;
use crate::org::allbinary::media::image::analysis::ImageAnalysisResults::ImageAnalysisResults;
use crate::org::allbinary::string::CommonPhoneStrings::CommonPhoneStrings;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::JFrame::JFrame;
use super::swing::swing;
use super::javax::javax;
use super::JDialog::JDialog;
use super::JLabel::JLabel;
use super::JTextField::JTextField;
use super::JButton::JButton;
use super::JPanel::JPanel;
use super::JMenuBar::JMenuBar;
use super::JMenu::JMenu;
use super::JMenuItem::JMenuItem;
use super::Dimension::Dimension;
use super::awt::awt;
use super::java::java;
use super::ActionListener::ActionListener;
use super::event::event;
use super::ActionEvent::ActionEvent;
use super::GroupLayout::GroupLayout;
use super::layout::layout;
use super::jdesktop::jdesktop;
use super::org::org;
use super::GridLayout::GridLayout;
use super::ImageUnifierJPanel::ImageUnifierJPanel;
use super::ImageAnalysisResultsJPanel::ImageAnalysisResultsJPanel;
use super::RotationSpriteImageJPanel::RotationSpriteImageJPanel;
use super::SpriteSplitterImageJPanel::SpriteSplitterImageJPanel;
use super::RotationImageJPanel::RotationImageJPanel;
use super::ResizeImageJPanel::ResizeImageJPanel;
use super::CreateIconImagesJPanel::CreateIconImagesJPanel;
use super::CanvasImageJPanel::CanvasImageJPanel;
use super::MirrorSpriteImageJPanel::MirrorSpriteImageJPanel;
use super::MirrorImageJPanel::MirrorImageJPanel;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageToolJFrame {
    pub base: JFrame,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub fileJDialog: Option<Rc<RefCell<FileJDialog>>>,
    pub imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>,
    pub analysisActionJDialog: Option<Rc<RefCell<JDialog>>>,
    pub analyzeJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub blueJLabel: Option<Rc<RefCell<JLabel>>>,
    pub colorRangeJLabel: Option<Rc<RefCell<JLabel>>>,
    pub createStripImageJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub fileJMenu: Option<Rc<RefCell<JMenu>>>,
    pub generateSpriteSplitterMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub generateRotationMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub generateMirrorSpriteMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub generateRotationSpriteJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub greenJLabel: Option<Rc<RefCell<JLabel>>>,
    pub jPanel1: Option<Rc<RefCell<JPanel>>>,
    pub mainJMenuBar: Option<Rc<RefCell<JMenuBar>>>,
    pub maxBlueJTextField: Option<Rc<RefCell<JTextField>>>,
    pub maxGreenJTextField: Option<Rc<RefCell<JTextField>>>,
    pub maxJLabel: Option<Rc<RefCell<JLabel>>>,
    pub maxRedJTextField: Option<Rc<RefCell<JTextField>>>,
    pub minBlueJTextField: Option<Rc<RefCell<JTextField>>>,
    pub minGreenJTextField: Option<Rc<RefCell<JTextField>>>,
    pub minJLabel: Option<Rc<RefCell<JLabel>>>,
    pub minRedJTextField: Option<Rc<RefCell<JTextField>>>,
    pub mirrorJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub okJButton: Option<Rc<RefCell<JButton>>>,
    pub openImageJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub processingJMenu: Option<Rc<RefCell<JMenu>>>,
    pub redJLabel: Option<Rc<RefCell<JLabel>>>,
    pub scaleImagesJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub canvasImagesJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub createIconImagesJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub titleJLabel: Option<Rc<RefCell<JLabel>>>,
}

pub static mut ImageToolJFrame_imageProcessorInputCompositeInterface: std::cell::LazyCell<Option<Rc<RefCell<dyn ImageProcessorInputCompositeInterface>>>> = std::cell::LazyCell::new(|| None);
impl FileJDialogListenerInterface for ImageToolJFrame {
    fn onFiles(&mut self, mut files: Vec<Option<Rc<RefCell<File>>>>) { Self::onFiles(self, files) }
}

impl ImageToolJFrame {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: JFrame::new_struct(),
            logUtil: None,
            commonStrings: None,
            fileJDialog: None,
            imageProcessorInput: None,
            analysisActionJDialog: None,
            analyzeJMenuItem: None,
            blueJLabel: None,
            colorRangeJLabel: None,
            createStripImageJMenuItem: None,
            fileJMenu: None,
            generateSpriteSplitterMenuItem: None,
            generateRotationMenuItem: None,
            generateMirrorSpriteMenuItem: None,
            generateRotationSpriteJMenuItem: None,
            greenJLabel: None,
            jPanel1: None,
            mainJMenuBar: None,
            maxBlueJTextField: None,
            maxGreenJTextField: None,
            maxJLabel: None,
            maxRedJTextField: None,
            minBlueJTextField: None,
            minGreenJTextField: None,
            minJLabel: None,
            minRedJTextField: None,
            mirrorJMenuItem: None,
            okJButton: None,
            openImageJMenuItem: None,
            processingJMenu: None,
            redJLabel: None,
            scaleImagesJMenuItem: None,
            canvasImagesJMenuItem: None,
            createIconImagesJMenuItem: None,
            titleJLabel: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.initComponents();
        let mut commonPhoneStrings: Option<Rc<RefCell<CommonPhoneStrings>>> = CommonPhoneStrings::getInstance();
        this.setSize(640, 480);
        jcall!(this.getMaxRedJTextField()).setText(String::from("255"));
        jcall!(this.getMaxGreenJTextField()).setText(String::from("255"));
        jcall!(this.getMaxBlueJTextField()).setText(String::from("255"));
        jcall!(this.getMinRedJTextField()).setText(jcall!(commonPhoneStrings).ZERO.clone());
        jcall!(this.getMinGreenJTextField()).setText(jcall!(commonPhoneStrings).ZERO.clone());
        jcall!(this.getMinBlueJTextField()).setText(jcall!(commonPhoneStrings).ZERO.clone());
        this.fileJDialog = FileJDialog::new();
        jcall!(this.fileJDialog).addFinishedListener((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn initComponents(&mut self) {
        self.analysisActionJDialog = JDialog::new();
        self.titleJLabel = JLabel::new();
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
        self.jPanel1 = JPanel::new();
        self.mainJMenuBar = JMenuBar::new();
        self.fileJMenu = JMenu::new();
        self.openImageJMenuItem = JMenuItem::new();
        self.processingJMenu = JMenu::new();
        self.analyzeJMenuItem = JMenuItem::new();
        self.generateSpriteSplitterMenuItem = JMenuItem::new();
        self.generateRotationMenuItem = JMenuItem::new();
        self.generateMirrorSpriteMenuItem = JMenuItem::new();
        self.generateRotationSpriteJMenuItem = JMenuItem::new();
        self.createStripImageJMenuItem = JMenuItem::new();
        self.mirrorJMenuItem = JMenuItem::new();
        self.scaleImagesJMenuItem = JMenuItem::new();
        self.canvasImagesJMenuItem = JMenuItem::new();
        self.createIconImagesJMenuItem = JMenuItem::new();
        jcall!(self.analysisActionJDialog).setMinimumSize(Dimension::new(350, 250));
        jcall!(self.titleJLabel).setText(String::from("              Color At Action Options            "));
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
        let mut analysisActionJDialogLayout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(jcall!(self.analysisActionJDialog).getContentPane());
        jcall!(jcall!(self.analysisActionJDialog).getContentPane()).setLayout(analysisActionJDialogLayout.clone());
        jcall!(analysisActionJDialogLayout).setHorizontalGroup(jcall!(jcall!(analysisActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone())).add(jcall!(jcall!(jcall!(analysisActionJDialogLayout).createSequentialGroup()).addContainerGap()).add(jcall!(jcall!(jcall!(analysisActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone())).add(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(analysisActionJDialogLayout).createSequentialGroup()).add(jcall!(jcall!(jcall!(jcall!(jcall!(analysisActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone())).add(self.blueJLabel.clone())).add(self.greenJLabel.clone())).add(self.redJLabel.clone())).add(self.colorRangeJLabel.clone()))).add(29, 29, 29)).add(jcall!(jcall!(jcall!(analysisActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone())).add(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).TRAILING.clone(), self.minJLabel.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), 58, i16::MAX.clone())).add(jcall!(jcall!(jcall!(jcall!(jcall!(analysisActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).TRAILING.clone(), false)).add(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone(), self.minBlueJTextField.clone())).add(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone(), self.minGreenJTextField.clone())).add(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone(), self.minRedJTextField.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), 42, i16::MAX.clone())).add(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone(), self.okJButton.clone())))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).LayoutStyle).RELATED.clone())).add(jcall!(jcall!(jcall!(analysisActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).TRAILING.clone())).add(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone(), jcall!(jcall!(jcall!(analysisActionJDialogLayout).createSequentialGroup()).add(jcall!(jcall!(jcall!(analysisActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).TRAILING.clone(), false)).add(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone(), self.maxRedJTextField.clone())).add(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone(), jcall!(jcall!(jcall!(analysisActionJDialogLayout).createSequentialGroup()).add(1, 1, 1)).add(jcall!(jcall!(jcall!(analysisActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone())).add(self.maxBlueJTextField.clone())).add(self.maxGreenJTextField.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone(), 43, jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone()))))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).LayoutStyle).RELATED.clone(), 17, i16::MAX.clone()))).add(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone(), self.maxJLabel.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), 61, i16::MAX.clone()))).addContainerGap())).add(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).TRAILING.clone(), jcall!(jcall!(jcall!(analysisActionJDialogLayout).createSequentialGroup()).add(self.titleJLabel.clone())).add(25, 25, 25)))));
        jcall!(analysisActionJDialogLayout).setVerticalGroup(jcall!(jcall!(analysisActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).LEADING.clone())).add(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(analysisActionJDialogLayout).createSequentialGroup()).add(self.titleJLabel.clone())).addPreferredGap(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).LayoutStyle).RELATED.clone())).add(jcall!(jcall!(jcall!(jcall!(analysisActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).BASELINE.clone())).add(self.minJLabel.clone())).add(self.colorRangeJLabel.clone())).add(self.maxJLabel.clone()))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).LayoutStyle).RELATED.clone())).add(jcall!(jcall!(jcall!(analysisActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).TRAILING.clone())).add(self.redJLabel.clone())).add(jcall!(jcall!(jcall!(analysisActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).BASELINE.clone())).add(self.minRedJTextField.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone())).add(self.maxRedJTextField.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone())))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).LayoutStyle).RELATED.clone())).add(jcall!(jcall!(jcall!(analysisActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).TRAILING.clone())).add(self.greenJLabel.clone())).add(jcall!(jcall!(jcall!(analysisActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).BASELINE.clone())).add(self.minGreenJTextField.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone())).add(self.maxGreenJTextField.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone())))).addPreferredGap(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).LayoutStyle).RELATED.clone())).add(jcall!(jcall!(jcall!(analysisActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).TRAILING.clone())).add(self.blueJLabel.clone())).add(jcall!(jcall!(jcall!(analysisActionJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).BASELINE.clone())).add(self.minBlueJTextField.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone())).add(self.maxBlueJTextField.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).PREFERRED_SIZE.clone())))).add(16, 16, 16)).add(self.okJButton.clone())).addContainerGap(jcall!(jcall!(jcall!(unsafe { org_jdesktop.clone() }).layout).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone())));
        self.setDefaultCloseOperation(jcall!(jcall!(unsafe { javax_swing.clone() }).WindowConstants).EXIT_ON_CLOSE.clone());
        jcall!(self.getContentPane()).setLayout(GridLayout::new(1, 0));
        jcall!(self.jPanel1).setLayout(GridLayout::new(1, 0));
        jcall!(self.getContentPane()).add(self.jPanel1.clone());
        jcall!(self.fileJMenu).setText(String::from("File"));
        { let java_receiver = self.fileJMenu.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.fileJMenuActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.openImageJMenuItem).setText(String::from("Open"));
        { let java_receiver = self.openImageJMenuItem.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.openImageJMenuItemActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.fileJMenu).add(self.openImageJMenuItem.clone());
        jcall!(self.mainJMenuBar).add(self.fileJMenu.clone());
        jcall!(self.processingJMenu).setText(String::from("Processing"));
        { let java_receiver = self.processingJMenu.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.processingJMenuActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.analyzeJMenuItem).setText(String::from("Image Analyze"));
        { let java_receiver = self.analyzeJMenuItem.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.analyzeJMenuItemActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.processingJMenu).add(self.analyzeJMenuItem.clone());
        jcall!(self.generateMirrorSpriteMenuItem).setText(String::from("Mirror Sprite Generator"));
        { let java_receiver = self.generateMirrorSpriteMenuItem.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.generateMirrorSpriteMenuItemActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.processingJMenu).add(self.generateMirrorSpriteMenuItem.clone());
        jcall!(self.generateRotationSpriteJMenuItem).setText(String::from("Rotation Sprite Generator"));
        { let java_receiver = self.generateRotationSpriteJMenuItem.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.generateRotationSpriteJMenuItemActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.processingJMenu).add(self.generateRotationSpriteJMenuItem.clone());
        jcall!(self.generateSpriteSplitterMenuItem).setText(String::from("Sprite Splitter Generator"));
        { let java_receiver = self.generateSpriteSplitterMenuItem.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.generateSpriteSplitterMenuItemActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.processingJMenu).add(self.generateSpriteSplitterMenuItem.clone());
        jcall!(self.generateRotationMenuItem).setText(String::from("Rotate Image Generator"));
        { let java_receiver = self.generateRotationMenuItem.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.generateRotationMenuItemActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.processingJMenu).add(self.generateRotationMenuItem.clone());
        jcall!(self.createStripImageJMenuItem).setText(String::from("Multi Image Unifier"));
        { let java_receiver = self.createStripImageJMenuItem.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.createStripImageJMenuItemActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.processingJMenu).add(self.createStripImageJMenuItem.clone());
        jcall!(self.mirrorJMenuItem).setText(String::from("Mirror Image(s)"));
        { let java_receiver = self.mirrorJMenuItem.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.mirrorJMenuItemActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.processingJMenu).add(self.mirrorJMenuItem.clone());
        jcall!(self.scaleImagesJMenuItem).setText(String::from("Scale Image(s)"));
        { let java_receiver = self.scaleImagesJMenuItem.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.scaleImagesJMenuItemActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.processingJMenu).add(self.scaleImagesJMenuItem.clone());
        jcall!(self.canvasImagesJMenuItem).setText(String::from("Canvas Image(s)"));
        { let java_receiver = self.canvasImagesJMenuItem.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.canvasImagesJMenuItemActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.processingJMenu).add(self.canvasImagesJMenuItem.clone());
        jcall!(self.createIconImagesJMenuItem).setText(String::from("Create Icon Image(s)"));
        { let java_receiver = self.createIconImagesJMenuItem.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.createIconsJMenuItemActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.processingJMenu).add(self.createIconImagesJMenuItem.clone());
        jcall!(self.mainJMenuBar).add(self.processingJMenu.clone());
        self.setJMenuBar(self.mainJMenuBar.clone());
        self.pack();
    }
    

    pub fn createStripImageJMenuItemActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut imageUnifierJPanel: Option<Rc<RefCell<ImageUnifierJPanel>>> = ImageUnifierJPanel::new(self.imageProcessorInput.clone());
            jcall!(self.jPanel1).removeAll();
            jcall!(self.jPanel1).add(imageUnifierJPanel.clone());
            jcall!(self.jPanel1).updateUI();
            unsafe { *ImageToolJFrame_imageProcessorInputCompositeInterface = imageUnifierJPanel.clone() };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("createStripImageJMenuItemActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn okJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(String::from("Starting"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("okJButtonActionPerformed"));
            let mut imageAnalysisResultsArray: Vec<Option<Rc<RefCell<ImageAnalysisResults>>>> = jcall!(ImageAnalysis::getInstance()).process(jcall!(self.imageProcessorInput).getBufferedImageArray(), self.getColorRangeInterface());
            jcall!(self.jPanel1).removeAll();
            jcall!(self.jPanel1).add(ImageAnalysisResultsJPanel::new(imageAnalysisResultsArray.clone()));
            jcall!(self.jPanel1).updateUI();
            jcall!(self.analysisActionJDialog).setVisible(false);
            unsafe { *ImageToolJFrame_imageProcessorInputCompositeInterface = None };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("okJButtonActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn processingJMenuActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
    }
    

    pub fn openImageJMenuItemActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(String::from("Starting"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("openImageJMenuItemActionPerformed"));
            jcall!(self.fileJDialog).setVisible(true);
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("openImageJMenuItemActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn onFiles(&mut self, mut files: Vec<Option<Rc<RefCell<File>>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(format!("{}{}", format!("{}{}", String::from("Reading "), (files.len() as i32)), String::from(" files.")), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onFiles"));
            Arrays::sort(files.clone(), Comparator::new() /* Java anonymous class body: implement it as a named type
                

                pub fn compare(&mut self, mut file1: Option<Rc<RefCell<File>>>, mut file2: Option<Rc<RefCell<File>>>) -> i32 {
                    let mut fileNameIndexValue1: i32 = { let java_arguments = (jcall!(file1).getName(),); self.extractIndexNumberIfAnyFromFileName(java_arguments.0) };
                    let mut fileNameIndexValue2: i32 = { let java_arguments = (jcall!(file2).getName(),); self.extractIndexNumberIfAnyFromFileName(java_arguments.0) };
                    return fileNameIndexValue1 - fileNameIndexValue2;
                }
                

                pub fn extractIndexNumberIfAnyFromFileName(&mut self, mut name: String) -> i32 {
                    let mut i: i32 = 0;
                    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let mut start: i32 = name.rfind('_').map(|index| index as i32).unwrap_or(-1) + 1;
                        let mut end: i32 = name.rfind('.').map(|index| index as i32).unwrap_or(-1);
                        let mut number: String = name[(start) as usize..(end) as usize].to_string();
                        i = number.clone().parse::<i32>().unwrap();
                    })) {
                        Ok(_) => {}
                        Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                        i = 0;
                        }
                    }
                    return i.clone();
                }
*/);
            let mut bufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = vec![None; ((files.len() as i32)) as usize];
            for index in (0..(files.len() as i32)) {
                bufferedImageArray[(index) as usize] = ImageIO::read(files[(index) as usize].clone());
                if bufferedImageArray[(index) as usize].is_none() {
                    panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("Unable to read file: "), jcall!(files[(index) as usize]).getPath()))));
                }
            }
            self.imageProcessorInput = ImageProcessorInput::new(files.clone(), bufferedImageArray.clone());
            jcall!(self.fileJDialog).setVisible(false);
            if unsafe { ImageToolJFrame_imageProcessorInputCompositeInterface.clone() }.is_some() {
                jcall!(unsafe { ImageToolJFrame_imageProcessorInputCompositeInterface.clone() }).setImageProcessorInput(self.imageProcessorInput.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onFiles"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn fileJMenuActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
    }
    

    pub fn analyzeJMenuItemActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(String::from("Starting"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("analyzeJMenuItemActionPerformed"));
            jcall!(self.analysisActionJDialog).setVisible(true);
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("analyzeJMenuItemActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn generateRotationSpriteJMenuItemActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(String::from("Starting"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generateRotationSpriteJMenuItemActionPerformed"));
            let mut spriteImageJPanel: Option<Rc<RefCell<RotationSpriteImageJPanel>>> = RotationSpriteImageJPanel::new(self.imageProcessorInput.clone());
            jcall!(self.jPanel1).removeAll();
            jcall!(self.jPanel1).add(spriteImageJPanel.clone());
            jcall!(self.jPanel1).updateUI();
            unsafe { *ImageToolJFrame_imageProcessorInputCompositeInterface = spriteImageJPanel.clone() };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generateRotationSpriteJMenuItemActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn generateSpriteSplitterMenuItemActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(String::from("Starting"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generateRotationSpriteJMenuItemActionPerformed"));
            let mut imageJPanel: Option<Rc<RefCell<SpriteSplitterImageJPanel>>> = SpriteSplitterImageJPanel::new(self.imageProcessorInput.clone());
            jcall!(self.jPanel1).removeAll();
            jcall!(self.jPanel1).add(imageJPanel.clone());
            jcall!(self.jPanel1).updateUI();
            unsafe { *ImageToolJFrame_imageProcessorInputCompositeInterface = imageJPanel.clone() };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generateRotationMenuItemActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn generateRotationMenuItemActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(String::from("Starting"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generateRotationSpriteJMenuItemActionPerformed"));
            let mut rotationImageJPanel: Option<Rc<RefCell<RotationImageJPanel>>> = RotationImageJPanel::new(self.imageProcessorInput.clone());
            jcall!(self.jPanel1).removeAll();
            jcall!(self.jPanel1).add(rotationImageJPanel.clone());
            jcall!(self.jPanel1).updateUI();
            unsafe { *ImageToolJFrame_imageProcessorInputCompositeInterface = rotationImageJPanel.clone() };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generateRotationMenuItemActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn scaleImagesJMenuItemActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("scaleImagesJMenuItemActionPerformed"));
            let mut resizeImageJPanel: Option<Rc<RefCell<ResizeImageJPanel>>> = ResizeImageJPanel::new(self.imageProcessorInput.clone());
            jcall!(self.jPanel1).removeAll();
            jcall!(self.jPanel1).add(resizeImageJPanel.clone());
            jcall!(self.jPanel1).updateUI();
            unsafe { *ImageToolJFrame_imageProcessorInputCompositeInterface = resizeImageJPanel.clone() };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("scaleImagesJMenuItemActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn createIconsJMenuItemActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("createIconsJMenuItemActionPerformed"));
            let mut createIconImagesJPanel: Option<Rc<RefCell<CreateIconImagesJPanel>>> = CreateIconImagesJPanel::new(self.imageProcessorInput.clone());
            jcall!(self.jPanel1).removeAll();
            jcall!(self.jPanel1).add(createIconImagesJPanel.clone());
            jcall!(self.jPanel1).updateUI();
            unsafe { *ImageToolJFrame_imageProcessorInputCompositeInterface = createIconImagesJPanel.clone() };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("scaleImagesJMenuItemActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn canvasImagesJMenuItemActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("canvasImagesJMenuItemActionPerformed"));
            let mut resizeImageJPanel: Option<Rc<RefCell<CanvasImageJPanel>>> = CanvasImageJPanel::new(self.imageProcessorInput.clone());
            jcall!(self.jPanel1).removeAll();
            jcall!(self.jPanel1).add(resizeImageJPanel.clone());
            jcall!(self.jPanel1).updateUI();
            unsafe { *ImageToolJFrame_imageProcessorInputCompositeInterface = resizeImageJPanel.clone() };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("scaleImagesJMenuItemActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn generateMirrorSpriteMenuItemActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(String::from("Starting"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generateMirrorSpriteJMenuItemActionPerformed"));
            let mut spriteImageJPanel: Option<Rc<RefCell<MirrorSpriteImageJPanel>>> = MirrorSpriteImageJPanel::new(self.imageProcessorInput.clone());
            jcall!(self.jPanel1).removeAll();
            jcall!(self.jPanel1).add(spriteImageJPanel.clone());
            jcall!(self.jPanel1).updateUI();
            unsafe { *ImageToolJFrame_imageProcessorInputCompositeInterface = spriteImageJPanel.clone() };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generateMirrorSpriteJMenuItemActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn mirrorJMenuItemActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(String::from("Starting"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generateMirrorJMenuItemActionPerformed"));
            let mut mirrorImageJPanel: Option<Rc<RefCell<MirrorImageJPanel>>> = MirrorImageJPanel::new(self.imageProcessorInput.clone());
            jcall!(self.jPanel1).removeAll();
            jcall!(self.jPanel1).add(mirrorImageJPanel.clone());
            jcall!(self.jPanel1).updateUI();
            unsafe { *ImageToolJFrame_imageProcessorInputCompositeInterface = mirrorImageJPanel.clone() };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("generateMirrorJMenuItemActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn getColorRangeInterface(&mut self) -> Option<Rc<RefCell<dyn ColorRangeInterface>>> {
        let mut colorRangeInterface: Option<Rc<RefCell<dyn ColorRangeInterface>>> = (ColorRange::new()).clone() /* as ColorRangeInterface */;
        jcall!(colorRangeInterface).setMinRed(((Some((jcall!(self.getMinRedJTextField()).getText()) as i32)).unwrap() as i32));
        jcall!(colorRangeInterface).setMaxRed(((Some((jcall!(self.getMaxRedJTextField()).getText()) as i32)).unwrap() as i32));
        jcall!(colorRangeInterface).setMinGreen(((Some((jcall!(self.getMinGreenJTextField()).getText()) as i32)).unwrap() as i32));
        jcall!(colorRangeInterface).setMaxGreen(((Some((jcall!(self.getMaxGreenJTextField()).getText()) as i32)).unwrap() as i32));
        jcall!(colorRangeInterface).setMinBlue(((Some((jcall!(self.getMinBlueJTextField()).getText()) as i32)).unwrap() as i32));
        jcall!(colorRangeInterface).setMaxBlue(((Some((jcall!(self.getMaxBlueJTextField()).getText()) as i32)).unwrap() as i32));
        return colorRangeInterface.clone();
    }
    

    pub fn getMaxBlueJTextField(&mut self) -> Option<Rc<RefCell<JTextField>>> {
        return self.maxBlueJTextField.clone();
    }
    

    pub fn setMaxBlueJTextField(&mut self, mut maxBlueJTextField: Option<Rc<RefCell<JTextField>>>) {
        self.maxBlueJTextField = self.maxBlueJTextField.clone();
    }
    

    pub fn getMaxGreenJTextField(&mut self) -> Option<Rc<RefCell<JTextField>>> {
        return self.maxGreenJTextField.clone();
    }
    

    pub fn setMaxGreenJTextField(&mut self, mut maxGreenJTextField: Option<Rc<RefCell<JTextField>>>) {
        self.maxGreenJTextField = self.maxGreenJTextField.clone();
    }
    

    pub fn getMaxRedJTextField(&mut self) -> Option<Rc<RefCell<JTextField>>> {
        return self.maxRedJTextField.clone();
    }
    

    pub fn setMaxRedJTextField(&mut self, mut maxRedJTextField: Option<Rc<RefCell<JTextField>>>) {
        self.maxRedJTextField = self.maxRedJTextField.clone();
    }
    

    pub fn getMinBlueJTextField(&mut self) -> Option<Rc<RefCell<JTextField>>> {
        return self.minBlueJTextField.clone();
    }
    

    pub fn setMinBlueJTextField(&mut self, mut minBlueJTextField: Option<Rc<RefCell<JTextField>>>) {
        self.minBlueJTextField = self.minBlueJTextField.clone();
    }
    

    pub fn getMinGreenJTextField(&mut self) -> Option<Rc<RefCell<JTextField>>> {
        return self.minGreenJTextField.clone();
    }
    

    pub fn setMinGreenJTextField(&mut self, mut minGreenJTextField: Option<Rc<RefCell<JTextField>>>) {
        self.minGreenJTextField = self.minGreenJTextField.clone();
    }
    

    pub fn getMinRedJTextField(&mut self) -> Option<Rc<RefCell<JTextField>>> {
        return self.minRedJTextField.clone();
    }
    

    pub fn setMinRedJTextField(&mut self, mut minRedJTextField: Option<Rc<RefCell<JTextField>>>) {
        self.minRedJTextField = self.minRedJTextField.clone();
    }
}
impl std::ops::Deref for ImageToolJFrame {
    type Target = JFrame;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImageToolJFrame {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



