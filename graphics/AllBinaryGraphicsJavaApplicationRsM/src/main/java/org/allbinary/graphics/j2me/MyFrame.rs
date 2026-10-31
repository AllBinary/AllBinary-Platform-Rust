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
// package org.allbinary.graphics.j2me
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Runnable;
use std::collections::HashMap;
use super::*;
// use crate::java::awt::*;
// use crate::java::io::File::File;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::swing::*;
use crate::org::allbinary::data::tree::dom::XslHelper::XslHelper;
use crate::org::allbinary::dom::DomHelper::DomHelper;
use crate::org::allbinary::graphics::j2me::dialog::NewWorkAreaJDialog::NewWorkAreaJDialog;
use crate::org::allbinary::graphics::j2me::dialog::RotateJDialog::RotateJDialog;
use crate::org::allbinary::graphics::j2me::workarea::WorkAreaDom::WorkAreaDom;
use crate::org::allbinary::graphics::j2me::workarea::WorkAreaJPanel::WorkAreaJPanel;
use crate::org::allbinary::graphics::j2me::workarea::WorkAreaJPanelInterface::WorkAreaJPanelInterface;
use crate::org::allbinary::graphics::j2me::workarea::canvas::event::MyCanvasEvent::MyCanvasEvent;
use crate::org::allbinary::graphics::j2me::workarea::canvas::event::MyCanvasEventService::MyCanvasEventService;
use crate::org::allbinary::graphics::j2me::workarea::canvas::event::MyCanvasEventSource::MyCanvasEventSource;
use crate::org::allbinary::gui::dialog::FileJDialog::FileJDialog;
use crate::org::allbinary::gui::dialog::FileJDialogListenerInterface::FileJDialogListenerInterface;
use crate::org::allbinary::gui::dialog::HashMapJDialogListenerInterface::HashMapJDialogListenerInterface;
use crate::org::allbinary::log::LOGGING::LOGGING;
use crate::org::allbinary::logic::communication::log::GuiLog::GuiLog;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::math::AngleFactory::AngleFactory;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::java::lang::Thread::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MyFrame {
    pub base: JFrame,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub guiLog: Option<Rc<RefCell<GuiLog>>>,
    pub xslHelper: Option<Rc<RefCell<XslHelper>>>,
    pub JAVA_TEMPLATE: String,
    pub SAVEEXTENSION: String,
    pub JAVAEXTENSION: String,
    pub fileJDialog: Option<Rc<RefCell<FileJDialog>>>,
    pub rotateJDialog: Option<Rc<RefCell<RotateJDialog>>>,
    pub animateJButton: Option<Rc<RefCell<JButton>>>,
    pub autoDamageJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub autoExplodeJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub autoGrowJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub autoRotateGenerationJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub autoSheildMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub autoShrinkJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub centerJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub copyJMenuItem1: Option<Rc<RefCell<JMenuItem>>>,
    pub exitJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub explodeAllJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub exportJMenuItem1: Option<Rc<RefCell<JMenuItem>>>,
    pub fileJMenu: Option<Rc<RefCell<JMenu>>>,
    pub frameJMenu: Option<Rc<RefCell<JMenu>>>,
    pub generateJMenu: Option<Rc<RefCell<JMenu>>>,
    pub insertAfterJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub insertBeforeJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub jButton1: Option<Rc<RefCell<JButton>>>,
    pub jButton2: Option<Rc<RefCell<JButton>>>,
    pub jButton3: Option<Rc<RefCell<JButton>>>,
    pub jButton4: Option<Rc<RefCell<JButton>>>,
    pub jButton41: Option<Rc<RefCell<JButton>>>,
    pub jButton5: Option<Rc<RefCell<JButton>>>,
    pub jMenu1: Option<Rc<RefCell<JMenu>>>,
    pub jMenuItem1: Option<Rc<RefCell<JMenuItem>>>,
    pub jPanel1: Option<Rc<RefCell<JPanel>>>,
    pub jTabbedPane: Option<Rc<RefCell<JTabbedPane>>>,
    pub jToolBar1: Option<Rc<RefCell<JToolBar>>>,
    pub mainJPanel: Option<Rc<RefCell<JPanel>>>,
    pub menuJMenuBar: Option<Rc<RefCell<JMenuBar>>>,
    pub newJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub nextFrameJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub objectJMenu: Option<Rc<RefCell<JMenu>>>,
    pub openJButton: Option<Rc<RefCell<JButton>>>,
    pub openJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub playJButton: Option<Rc<RefCell<JButton>>>,
    pub previousFrameJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub removeJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub saveAsJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub saveJButton: Option<Rc<RefCell<JButton>>>,
    pub saveJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub statusJPanel: Option<Rc<RefCell<JPanel>>>,
    pub stopJjButton: Option<Rc<RefCell<JButton>>>,
    pub viewJMenu: Option<Rc<RefCell<JMenu>>>,
    pub zoomInJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub zoomOutJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
}

thread_local! { pub static MyFrame_MAXWORKAREAS: i32 = 10; }
pub static mut MyFrame_numberOfWorkAreas: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| 0);
thread_local! { pub static MyFrame_PATH: String = String::from("G:/mnt/bc/mydev/work/graphics/AllBinaryGraphicsJavaApplication"); }
thread_local! { pub static MyFrame_SAVE_PATH: String = format!("{}{}", MyFrame_PATH.with(Clone::clone), String::from("/save")); }
thread_local! { pub static MyFrame_TEMPLATE_EXPORT_PATH: String = format!("{}{}", MyFrame_PATH.with(Clone::clone), String::from("/templates/export")); }
pub static mut MyFrame_statusJLabel: std::cell::LazyCell<Option<Rc<RefCell<JLabel>>>> = std::cell::LazyCell::new(|| None);
impl FileJDialogListenerInterface for MyFrame {
    fn onFiles(&mut self, mut files: Vec<Option<Rc<RefCell<File>>>>) { Self::onFiles(self, files) }
}
impl HashMapJDialogListenerInterface for MyFrame {
    fn onHashMap(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>) { Self::onHashMap(self, hashMap) }
}
// impl KeyListener for MyFrame with the methods of the trait

impl MyFrame {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: JFrame::new_struct(),
            logUtil: None,
            commonStrings: None,
            guiLog: None,
            xslHelper: None,
            JAVA_TEMPLATE: String::new(),
            SAVEEXTENSION: String::new(),
            JAVAEXTENSION: String::new(),
            fileJDialog: None,
            rotateJDialog: None,
            animateJButton: None,
            autoDamageJMenuItem: None,
            autoExplodeJMenuItem: None,
            autoGrowJMenuItem: None,
            autoRotateGenerationJMenuItem: None,
            autoSheildMenuItem: None,
            autoShrinkJMenuItem: None,
            centerJMenuItem: None,
            copyJMenuItem1: None,
            exitJMenuItem: None,
            explodeAllJMenuItem: None,
            exportJMenuItem1: None,
            fileJMenu: None,
            frameJMenu: None,
            generateJMenu: None,
            insertAfterJMenuItem: None,
            insertBeforeJMenuItem: None,
            jButton1: None,
            jButton2: None,
            jButton3: None,
            jButton4: None,
            jButton41: None,
            jButton5: None,
            jMenu1: None,
            jMenuItem1: None,
            jPanel1: None,
            jTabbedPane: None,
            jToolBar1: None,
            mainJPanel: None,
            menuJMenuBar: None,
            newJMenuItem: None,
            nextFrameJMenuItem: None,
            objectJMenu: None,
            openJButton: None,
            openJMenuItem: None,
            playJButton: None,
            previousFrameJMenuItem: None,
            removeJMenuItem: None,
            saveAsJMenuItem: None,
            saveJButton: None,
            saveJMenuItem: None,
            statusJPanel: None,
            stopJjButton: None,
            viewJMenu: None,
            zoomInJMenuItem: None,
            zoomOutJMenuItem: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.guiLog = GuiLog::getInstance();
        this.xslHelper = XslHelper::getInstance();
        this.JAVA_TEMPLATE = String::from("svgExport.xsl");
        this.SAVEEXTENSION = String::from(".gxml");
        this.JAVAEXTENSION = String::from(".javax");
        this.initComponents();
        this.setSize(Dimension::new(550, 400));
        jcall!(this.mainJPanel).setSize(Dimension::new(550, 400));
        jcall!(this.jTabbedPane).setSize(Dimension::new(550, 400));
        this.fileJDialog = FileJDialog::new();
        jcall!(this.fileJDialog).addFinishedListener((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct())));
            this.rotateJDialog = RotateJDialog::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), false);
        this
    }
    

    pub fn initComponents(&mut self) {
        self.jToolBar1 = JToolBar::new();
        self.jButton2 = JButton::new();
        self.saveJButton = JButton::new();
        self.jButton1 = JButton::new();
        self.openJButton = JButton::new();
        self.jButton3 = JButton::new();
        self.jButton4 = JButton::new();
        self.jButton41 = JButton::new();
        self.jButton5 = JButton::new();
        self.animateJButton = JButton::new();
        self.playJButton = JButton::new();
        self.stopJjButton = JButton::new();
        self.jPanel1 = JPanel::new();
        self.mainJPanel = JPanel::new();
        self.jTabbedPane = JTabbedPane::new();
        self.statusJPanel = JPanel::new();
        unsafe { *MyFrame_statusJLabel = JLabel::new() };
        self.menuJMenuBar = JMenuBar::new();
        self.fileJMenu = JMenu::new();
        self.newJMenuItem = JMenuItem::new();
        self.openJMenuItem = JMenuItem::new();
        self.saveJMenuItem = JMenuItem::new();
        self.saveAsJMenuItem = JMenuItem::new();
        self.exportJMenuItem1 = JMenuItem::new();
        self.exitJMenuItem = JMenuItem::new();
        self.viewJMenu = JMenu::new();
        self.zoomInJMenuItem = JMenuItem::new();
        self.zoomOutJMenuItem = JMenuItem::new();
        self.objectJMenu = JMenu::new();
        self.explodeAllJMenuItem = JMenuItem::new();
        self.frameJMenu = JMenu::new();
        self.previousFrameJMenuItem = JMenuItem::new();
        self.nextFrameJMenuItem = JMenuItem::new();
        self.removeJMenuItem = JMenuItem::new();
        self.copyJMenuItem1 = JMenuItem::new();
        self.insertAfterJMenuItem = JMenuItem::new();
        self.insertBeforeJMenuItem = JMenuItem::new();
        self.generateJMenu = JMenu::new();
        self.centerJMenuItem = JMenuItem::new();
        self.autoDamageJMenuItem = JMenuItem::new();
        self.autoExplodeJMenuItem = JMenuItem::new();
        self.autoGrowJMenuItem = JMenuItem::new();
        self.jMenuItem1 = JMenuItem::new();
        self.autoRotateGenerationJMenuItem = JMenuItem::new();
        self.autoSheildMenuItem = JMenuItem::new();
        self.autoShrinkJMenuItem = JMenuItem::new();
        self.jMenu1 = JMenu::new();
        { let java_arguments = (WindowAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn windowClosing(&mut self, mut evt: Option<Rc<RefCell<WindowEvent>>>) {
                self.exitForm(evt.clone());
            }
*/,); self.addWindowListener(java_arguments.0) };
        jcall!(self.jButton2).setText(String::from("New"));
        { let java_receiver = self.jButton2.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.jButton2ActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jToolBar1).add(self.jButton2.clone());
        jcall!(self.saveJButton).setText(String::from("Save"));
        { let java_receiver = self.saveJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.saveJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jToolBar1).add(self.saveJButton.clone());
        jcall!(self.jButton1).setText(String::from("Export"));
        { let java_receiver = self.jButton1.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.jButton1ActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jToolBar1).add(self.jButton1.clone());
        jcall!(self.openJButton).setText(String::from("Open"));
        { let java_receiver = self.openJButton.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.openJButtonActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.jToolBar1).add(self.openJButton.clone());
        jcall!(self.jButton3).setText(String::from("Copy"));
        jcall!(self.jToolBar1).add(self.jButton3.clone());
        jcall!(self.jButton4).setText(String::from("Insert Before"));
        jcall!(self.jToolBar1).add(self.jButton4.clone());
        jcall!(self.jButton41).setText(String::from("Insert After"));
        jcall!(self.jToolBar1).add(self.jButton41.clone());
        jcall!(self.jButton5).setText(String::from("Delete"));
        jcall!(self.jToolBar1).add(self.jButton5.clone());
        jcall!(self.animateJButton).setText(String::from("AnimateEdit"));
        jcall!(self.jToolBar1).add(self.animateJButton.clone());
        jcall!(self.playJButton).setText(String::from("Play"));
        { let java_receiver = self.playJButton.clone(); let java_arguments = (MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.playJButtonMousePressed(evt.clone());
            }
*/,); jcall!(java_receiver).addMouseListener(java_arguments.0); };
        jcall!(self.jToolBar1).add(self.playJButton.clone());
        jcall!(self.stopJjButton).setText(String::from("Stop"));
        { let java_receiver = self.stopJjButton.clone(); let java_arguments = (MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.stopJjButtonMousePressed(evt.clone());
            }
*/,); jcall!(java_receiver).addMouseListener(java_arguments.0); };
        jcall!(self.jToolBar1).add(self.stopJjButton.clone());
        jcall!(self.getContentPane()).add(self.jToolBar1.clone(), jcall!(jcall!(unsafe { java_awt.clone() }).BorderLayout).NORTH.clone());
        jcall!(self.jPanel1).setLayout(GridLayout::new(1, 0));
        jcall!(self.mainJPanel).setLayout(GridLayout::new(1, 0));
        jcall!(self.jTabbedPane).setMinimumSize(Dimension::new(550, 400));
        { let java_receiver = self.jTabbedPane.clone(); let java_arguments = (ChangeListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn stateChanged(&mut self, mut evt: Option<Rc<RefCell<ChangeEvent>>>) {
                self.jTabbedPaneStateChanged(evt.clone());
            }
*/,); jcall!(java_receiver).addChangeListener(java_arguments.0); };
        jcall!(self.mainJPanel).add(self.jTabbedPane.clone());
        jcall!(self.jPanel1).add(self.mainJPanel.clone());
        jcall!(self.getContentPane()).add(self.jPanel1.clone(), jcall!(jcall!(unsafe { java_awt.clone() }).BorderLayout).CENTER.clone());
        jcall!(self.statusJPanel).setLayout(GridLayout::new(1, 0));
        jcall!(unsafe { MyFrame_statusJLabel.clone() }).setText(String::from("Status Bar"));
        jcall!(self.statusJPanel).add(unsafe { MyFrame_statusJLabel.clone() }.clone());
        jcall!(self.getContentPane()).add(self.statusJPanel.clone(), jcall!(jcall!(unsafe { java_awt.clone() }).BorderLayout).SOUTH.clone());
        jcall!(self.fileJMenu).setText(String::from("File"));
        { let java_receiver = self.fileJMenu.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.fileJMenuActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.newJMenuItem).setText(String::from("New"));
        { let java_receiver = self.newJMenuItem.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.newJMenuItemActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.fileJMenu).add(self.newJMenuItem.clone());
        jcall!(self.openJMenuItem).setText(String::from("Open"));
        { let java_receiver = self.openJMenuItem.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.openJMenuItemActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.fileJMenu).add(self.openJMenuItem.clone());
        jcall!(self.saveJMenuItem).setText(String::from("Save"));
        { let java_receiver = self.saveJMenuItem.clone(); let java_arguments = (MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.saveJMenuItemMousePressed(evt.clone());
            }
*/,); jcall!(java_receiver).addMouseListener(java_arguments.0); };
        jcall!(self.fileJMenu).add(self.saveJMenuItem.clone());
        jcall!(self.saveAsJMenuItem).setText(String::from("SaveAs"));
        jcall!(self.fileJMenu).add(self.saveAsJMenuItem.clone());
        jcall!(self.exportJMenuItem1).setText(String::from("Export"));
        { let java_receiver = self.exportJMenuItem1.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.exportJMenuItem1ActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.fileJMenu).add(self.exportJMenuItem1.clone());
        jcall!(self.exitJMenuItem).setText(String::from("Exit"));
        { let java_receiver = self.exitJMenuItem.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.exitJMenuItemActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.fileJMenu).add(self.exitJMenuItem.clone());
        jcall!(self.menuJMenuBar).add(self.fileJMenu.clone());
        jcall!(self.viewJMenu).setText(String::from("View"));
        jcall!(self.zoomInJMenuItem).setText(String::from("Zoom In"));
        { let java_receiver = self.zoomInJMenuItem.clone(); let java_arguments = (MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.zoomInJMenuItemMousePressed(evt.clone());
            }
*/,); jcall!(java_receiver).addMouseListener(java_arguments.0); };
        jcall!(self.viewJMenu).add(self.zoomInJMenuItem.clone());
        jcall!(self.zoomOutJMenuItem).setText(String::from("Zoom Out"));
        { let java_receiver = self.zoomOutJMenuItem.clone(); let java_arguments = (MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.zoomOutJMenuItemMousePressed(evt.clone());
            }
*/,); jcall!(java_receiver).addMouseListener(java_arguments.0); };
        jcall!(self.viewJMenu).add(self.zoomOutJMenuItem.clone());
        jcall!(self.menuJMenuBar).add(self.viewJMenu.clone());
        jcall!(self.objectJMenu).setText(String::from("Objects"));
        jcall!(self.explodeAllJMenuItem).setText(String::from("Explode All"));
        { let java_receiver = self.explodeAllJMenuItem.clone(); let java_arguments = (MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.explodeAllJMenuItemMousePressed(evt.clone());
            }
*/,); jcall!(java_receiver).addMouseListener(java_arguments.0); };
        jcall!(self.objectJMenu).add(self.explodeAllJMenuItem.clone());
        jcall!(self.menuJMenuBar).add(self.objectJMenu.clone());
        jcall!(self.frameJMenu).setText(String::from("Frame"));
        { let java_receiver = self.frameJMenu.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.frameJMenuActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.previousFrameJMenuItem).setText(String::from("Previous"));
        jcall!(self.frameJMenu).add(self.previousFrameJMenuItem.clone());
        { let java_receiver = self.nextFrameJMenuItem.clone(); let java_arguments = (jcall!(CommonLabels::getInstance()).NEXT.clone(),); jcall!(java_receiver).setText(java_arguments.0); };
        jcall!(self.frameJMenu).add(self.nextFrameJMenuItem.clone());
        jcall!(self.removeJMenuItem).setText(String::from("Remove"));
        jcall!(self.frameJMenu).add(self.removeJMenuItem.clone());
        jcall!(self.copyJMenuItem1).setText(String::from("Copy"));
        jcall!(self.frameJMenu).add(self.copyJMenuItem1.clone());
        jcall!(self.insertAfterJMenuItem).setText(String::from("Ins After"));
        jcall!(self.frameJMenu).add(self.insertAfterJMenuItem.clone());
        jcall!(self.insertBeforeJMenuItem).setText(String::from("Ins Before"));
        jcall!(self.frameJMenu).add(self.insertBeforeJMenuItem.clone());
        jcall!(self.menuJMenuBar).add(self.frameJMenu.clone());
        jcall!(self.generateJMenu).setText(String::from("Generate"));
        jcall!(self.centerJMenuItem).setText(String::from("Center"));
        { let java_receiver = self.centerJMenuItem.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.centerJMenuItemActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.generateJMenu).add(self.centerJMenuItem.clone());
        jcall!(self.autoDamageJMenuItem).setText(String::from("Damage"));
        jcall!(self.generateJMenu).add(self.autoDamageJMenuItem.clone());
        jcall!(self.autoExplodeJMenuItem).setText(String::from("Explode"));
        { let java_receiver = self.autoExplodeJMenuItem.clone(); let java_arguments = (MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.autoExplodeJMenuItemMousePressed(evt.clone());
            }
*/,); jcall!(java_receiver).addMouseListener(java_arguments.0); };
        jcall!(self.generateJMenu).add(self.autoExplodeJMenuItem.clone());
        jcall!(self.autoGrowJMenuItem).setText(String::from("Grow"));
        jcall!(self.generateJMenu).add(self.autoGrowJMenuItem.clone());
        jcall!(self.jMenuItem1).setText(String::from("Mirror"));
        { let java_receiver = self.jMenuItem1.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.jMenuItem1ActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.generateJMenu).add(self.jMenuItem1.clone());
        jcall!(self.autoRotateGenerationJMenuItem).setText(String::from("Rotate"));
        { let java_receiver = self.autoRotateGenerationJMenuItem.clone(); let java_arguments = (MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.autoRotateGenerationJMenuItemMousePressed(evt.clone());
            }
*/,); jcall!(java_receiver).addMouseListener(java_arguments.0); };
        jcall!(self.generateJMenu).add(self.autoRotateGenerationJMenuItem.clone());
        jcall!(self.autoSheildMenuItem).setText(String::from("Shield"));
        jcall!(self.generateJMenu).add(self.autoSheildMenuItem.clone());
        jcall!(self.autoShrinkJMenuItem).setText(String::from("Shrink"));
        jcall!(self.generateJMenu).add(self.autoShrinkJMenuItem.clone());
        jcall!(self.menuJMenuBar).add(self.generateJMenu.clone());
        jcall!(self.jMenu1).setText(String::from("Help"));
        jcall!(self.menuJMenuBar).add(self.jMenu1.clone());
        self.setJMenuBar(self.menuJMenuBar.clone());
        self.pack();
    }
    

    pub fn explodeAllJMenuItemMousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            MyCanvasEventService::fire(MyCanvasEvent::new((MyCanvasEventSource::new2(unsafe { crate::org::allbinary::graphics::j2me::workarea::canvas::event::MyCanvasEventService::MyCanvasEventService_EXPLODE.clone() }.clone(), None)).clone() /* as Any */));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("explodeAllJMenuItemMousePressed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn autoExplodeJMenuItemMousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            MyCanvasEventService::fire(MyCanvasEvent::new((MyCanvasEventSource::new2(unsafe { crate::org::allbinary::graphics::j2me::workarea::canvas::event::MyCanvasEventService::MyCanvasEventService_AUTOEXPLODE.clone() }.clone(), None)).clone() /* as Any */));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("autoExplodeJMenuItemMousePressed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn stopJjButtonMousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
        jcall!(self.getSelectedWorkArea()).stop();
    }
    

    pub fn playJButtonMousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
        jcall!(self.getSelectedWorkArea()).play();
    }
    

    pub fn autoRotateGenerationJMenuItemMousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
        jcall!(self.rotateJDialog).setVisible(true);
    }
    

    pub fn zoomOutJMenuItemMousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
        jcall!(self.getSelectedWorkArea()).changeZoom(-1);
    }
    

    pub fn zoomInJMenuItemMousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
        jcall!(self.getSelectedWorkArea()).changeZoom(1);
    }
    

    pub fn openJMenuItemActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        self.openGraphics();
    }
    

    pub fn exportJMenuItem1ActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        self.export();
    }
    

    pub fn saveJMenuItemMousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
        self.save();
    }
    

    pub fn jTabbedPaneStateChanged(&mut self, mut evt: Option<Rc<RefCell<ChangeEvent>>>) {
        self.updateSelectedWorkArea();
    }
    

    pub fn frameJMenuActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
    }
    

    pub fn exitJMenuItemActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        std::process::exit(0);
    }
    

    pub fn newJMenuItemActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        self.newGraphics();
    }
    

    pub fn fileJMenuActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
    }
    

    pub fn exitForm(&mut self, mut evt: Option<Rc<RefCell<WindowEvent>>>) {
        std::process::exit(0);
    }
    

    pub fn openJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        self.openGraphics();
    }
    

    pub fn jButton2ActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        self.newGraphics();
    }
    

    pub fn centerJMenuItemActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            MyCanvasEventService::fire(MyCanvasEvent::new((MyCanvasEventSource::new2(unsafe { crate::org::allbinary::graphics::j2me::workarea::canvas::event::MyCanvasEventService::MyCanvasEventService_CENTER.clone() }.clone(), None)).clone() /* as Any */));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("centerJMenuItemActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn saveJButtonActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        self.save();
    }
    

    pub fn jButton1ActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        self.export();
    }
    

    pub fn jMenuItem1ActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.getSelectedWorkArea()).autoMirror();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("centerJMenuItemActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn onFiles(&mut self, mut files: Vec<Option<Rc<RefCell<File>>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("Reading: "), files), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onFiles"));
            for index in (0..(files.len() as i32)) {
                let mut document: Option<Rc<RefCell<Document>>> = jcall!(DomHelper::getInstance()).createDocument1(files[(index) as usize].clone());
                let mut workAreaJPanel: Option<Rc<RefCell<WorkAreaJPanel>>> = WorkAreaJPanel::new2(WorkAreaDom::new(document.clone()), self.getSize());
                jcall!(workAreaJPanel).setLayout(GridLayout::new(1, 1));
                self.add((workAreaJPanel).clone() /* as WorkAreaJPanelInterface */);
            }
            jcall!(self.fileJDialog).setVisible(false);
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onFiles"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn newGraphics(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut newName: String = format!("{}{}", String::from("graphic"), unsafe { MyFrame_numberOfWorkAreas.clone() });
            let mut dim: Option<Rc<RefCell<Dimension>>> = self.getSize();
            let mut newDialog: Option<Rc<RefCell<NewWorkAreaJDialog>>> = NewWorkAreaJDialog::new(None /* Java this: a &mut self method has no Rc of itself */, false, dim.clone(), newName.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if LOGGING::contains(crate::org::allbinary::log::LOGGING::LOGGING_MENUEVENT.with(Clone::clone).clone()) {
                jcall!(self.guiLog).put4(jcall!(self.commonStrings).EXCEPTION.clone(), None /* Java this: a &mut self method has no Rc of itself */, String::from("newJMenuItemActionPerformed"), e.clone());
            }
            }
        }
    }
    

    pub fn openGraphics(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(String::from("Starting"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("openImageJMenuItemActionPerformed"));
            jcall!(self.fileJDialog).setVisible(true);
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if LOGGING::contains(crate::org::allbinary::log::LOGGING::LOGGING_MENUEVENT.with(Clone::clone).clone()) {
                jcall!(self.guiLog).put4(jcall!(self.commonStrings).EXCEPTION.clone(), None /* Java this: a &mut self method has no Rc of itself */, String::from("open"), e.clone());
            }
            }
        }
    }
    

    pub fn export(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut workAreaJPanel: Option<Rc<RefCell<dyn WorkAreaJPanelInterface>>> = (jcall!(self.jTabbedPane).getSelectedComponent()).clone() /* as WorkAreaJPanelInterface */;
            let mut file: Option<Rc<RefCell<File>>> = File::new(format!("{}{}", format!("{}{}", format!("{}{}", MyFrame_SAVE_PATH.with(Clone::clone), String::from("/")), jcall!(workAreaJPanel).getName()), self.JAVAEXTENSION));
            let mut document: Option<Rc<RefCell<Document>>> = jcall!(workAreaJPanel).toDocument();
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Export: "), jcall!(DomHelper::getInstance()).toString(document.clone())), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("exportJMenuItem1ActionPerformed"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            jcall!(self.xslHelper).export(file.clone(), format!("{}{}", format!("{}{}", MyFrame_TEMPLATE_EXPORT_PATH.with(Clone::clone), String::from("/")), self.JAVA_TEMPLATE), document.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if LOGGING::contains(crate::org::allbinary::log::LOGGING::LOGGING_MENUEVENT.with(Clone::clone).clone()) {
                jcall!(self.guiLog).put4(jcall!(self.commonStrings).EXCEPTION.clone(), None /* Java this: a &mut self method has no Rc of itself */, String::from("export"), e.clone());
            }
            }
        }
    }
    

    pub fn save(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut workAreaJPanel: Option<Rc<RefCell<dyn WorkAreaJPanelInterface>>> = (jcall!(self.jTabbedPane).getSelectedComponent()).clone() /* as WorkAreaJPanelInterface */;
            jcall!(DomHelper::getInstance()).save(File::new(format!("{}{}", format!("{}{}", format!("{}{}", MyFrame_SAVE_PATH.with(Clone::clone), String::from("/")), jcall!(workAreaJPanel).getName()), self.SAVEEXTENSION)), jcall!(workAreaJPanel).toDocument());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if LOGGING::contains(crate::org::allbinary::log::LOGGING::LOGGING_MENUEVENT.with(Clone::clone).clone()) {
                jcall!(self.guiLog).put4(jcall!(self.commonStrings).EXCEPTION.clone(), None /* Java this: a &mut self method has no Rc of itself */, jcall!(self.commonStrings).SAVE.clone(), e.clone());
            }
            }
        }
    }
    

    // throws Exception
    pub fn onHashMap(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>) {
        let mut angleIncrementInteger: Option<i32> = (jcall!(hashMap).get((0).to_string())).clone() /* as Integer */;
        let mut totalRotationInteger: Option<i32> = (jcall!(hashMap).get((1).to_string())).clone() /* as Integer */;
        jcall!(self.getSelectedWorkArea()).autoRotate(((angleIncrementInteger).unwrap() as i32), ((totalRotationInteger).unwrap() as i32));
    }
    

    pub fn getSelectedWorkArea(&mut self) -> Option<Rc<RefCell<dyn WorkAreaJPanelInterface>>> {
        return (jcall!(self.jTabbedPane).getSelectedComponent()).clone() /* as WorkAreaJPanelInterface */;
    }
    

    pub fn updateSelectedWorkArea(&mut self) {
        let mut numberOfTabs: i32 = jcall!(self.jTabbedPane).getTabCount();
        for index in (0..numberOfTabs) {
            let mut workAreaJPanel: Option<Rc<RefCell<dyn WorkAreaJPanelInterface>>> = (jcall!(self.jTabbedPane).getComponentAt(index.clone())).clone() /* as WorkAreaJPanelInterface */;
            jcall!(workAreaJPanel).deselect();
        }
        let mut workAreaJPanel: Option<Rc<RefCell<dyn WorkAreaJPanelInterface>>> = (jcall!(self.jTabbedPane).getSelectedComponent()).clone() /* as WorkAreaJPanelInterface */;
        if workAreaJPanel.is_some() {
            jcall!(workAreaJPanel).select();
        }
    }
    

    // throws Exception
    pub fn add(&mut self, mut newCanvasInterface: Option<Rc<RefCell<dyn WorkAreaJPanelInterface>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            unsafe { *MyFrame_numberOfWorkAreas += 1 };
            if unsafe { MyFrame_numberOfWorkAreas.clone() } == MyFrame_MAXWORKAREAS.with(Clone::clone) {
                panic!("{:?}", format!("{}: {:?}", "Exception", ()));
            }
            { let java_receiver = self.jTabbedPane.clone(); let java_arguments = (jcall!(newCanvasInterface).getName(), (newCanvasInterface).clone() /* as JPanel */,); jcall!(java_receiver).addTab(java_arguments.0, java_arguments.1); };
            jcall!(self.jTabbedPane).setSelectedComponent((newCanvasInterface).clone() /* as JPanel */);
            let mut animationThread: Option<Rc<RefCell<Thread>>> = Thread::new1(panic!("ClassCastException: a Rust dyn WorkAreaJPanelInterface cannot be cast to dyn Runnable"));
            jcall!(animationThread).start();
            self.updateSelectedWorkArea();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if LOGGING::contains(crate::org::allbinary::log::LOGGING::LOGGING_GRAPHICSCREATION.with(Clone::clone).clone()) {
                jcall!(self.guiLog).put4(jcall!(self.commonStrings).EXCEPTION.clone(), None /* Java this: a &mut self method has no Rc of itself */, String::from("addNewCanvas"), e.clone());
            }
            }
        }
    }
    

    pub fn keyPressed(&mut self, mut keyEvent: Option<Rc<RefCell<KeyEvent>>>) {
        jcall!(self.getSelectedWorkArea()).keyPressed(keyEvent.clone());
    }
    

    pub fn keyReleased(&mut self, mut keyEvent: Option<Rc<RefCell<KeyEvent>>>) {
    }
    

    pub fn keyTyped(&mut self, mut keyEvent: Option<Rc<RefCell<KeyEvent>>>) {
    }
    

    pub fn main(mut args: Vec<String>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            AngleFactory::getInstance();
            jcall!(SmallIntegerSingletonFactory::getInstance()).init();
            jcall!(MyFrame::new()).show();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
            jcall!(logUtil).put(jcall!(CommonStrings::getInstance()).EXCEPTION.clone(), None, String::from("main"), crate::java_into_object(e.clone()));
            }
        }
    }
}
impl std::ops::Deref for MyFrame {
    type Target = JFrame;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MyFrame {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



