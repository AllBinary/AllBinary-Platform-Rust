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
// package org.allbinary.input.automation
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::thread::ARunnable::ARunnable;
// use crate::java::net::URI::URI;
// use crate::java::net::URL::URL;
// use crate::java::awt::Desktop::Desktop;
// use crate::java::util::List::List;
// use crate::javax::swing::ImageIcon::ImageIcon;
// use crate::javax::swing::JDialog::JDialog;
// use crate::javax::help::HelpSet::HelpSet;
// use crate::javax::help::event::HelpSetEvent::HelpSetEvent;
// use crate::javax::help::event::HelpSetListener::HelpSetListener;
use crate::bundle::input::automation::InputAutomationBundleActivatorListenerInterface::InputAutomationBundleActivatorListenerInterface;
use crate::bundle::input::automation::module::configuration::InputAutomationConfigurationModuleChangeListener::InputAutomationConfigurationModuleChangeListener;
use crate::bundle::input::automation::robot::InputAutomationRobotChangeListener::InputAutomationRobotChangeListener;
use crate::org::allbinary::globals::AppUrlGlobals::AppUrlGlobals;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::gui::dialog::BasicTextJDialog::BasicTextJDialog;
use crate::org::allbinary::gui::swing::workers::JListSwingWorker::JListSwingWorker;
use crate::org::allbinary::input::automation::configuration::InputAutomationConfiguration::InputAutomationConfiguration;
use crate::org::allbinary::input::automation::configuration::InputAutomationConfigurationFactory::InputAutomationConfigurationFactory;
use crate::org::allbinary::input::automation::configuration::InputAutomationConfigurationModuleChangeEvent::InputAutomationConfigurationModuleChangeEvent;
use crate::org::allbinary::input::automation::module::InputAutomationModuleFactoryFactory::InputAutomationModuleFactoryFactory;
use crate::org::allbinary::input::automation::module::InputAutomationModuleFactoryInterface::InputAutomationModuleFactoryInterface;
use crate::org::allbinary::input::automation::module::configuration::InputAutomationModuleConfiguration::InputAutomationModuleConfiguration;
use crate::org::allbinary::input::automation::module::configuration::InputAutomationModuleConfigurations::InputAutomationModuleConfigurations;
use crate::org::allbinary::input::automation::osgi::DesktopBundle::DesktopBundle;
use crate::org::allbinary::input::automation::robot::InputRobotFactory::InputRobotFactory;
use crate::org::allbinary::input::automation::robot::osgi::InputAutomationRobotChangeEvent::InputAutomationRobotChangeEvent;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypeFactory::LogConfigTypeFactory;
use crate::org::allbinary::logic::java::help::JavaHelpUtil::JavaHelpUtil;
use crate::org::allbinary::logic::system::loader::WebappClassLoaderInfo::WebappClassLoaderInfo;
use crate::org::allbinary::logic::system::security::licensing::InputAutomationClientInformationFactory::InputAutomationClientInformationFactory;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::thread::RunnableInterface::RunnableInterface;
use crate::java::lang::Thread::*;
use super::JFrame::JFrame;
use super::swing::swing;
use super::javax::javax;
use super::JLabel::JLabel;
use super::JTabbedPane::JTabbedPane;
use super::JPanel::JPanel;
use super::JScrollPane::JScrollPane;
use super::JList::JList;
use super::JMenuBar::JMenuBar;
use super::JMenu::JMenu;
use super::JMenuItem::JMenuItem;
use super::JCheckBoxMenuItem::JCheckBoxMenuItem;
use super::Dimension::Dimension;
use super::awt::awt;
use super::java::java;
use super::GroupLayout::GroupLayout;
use super::WindowFocusListener::WindowFocusListener;
use super::event::event;
use super::WindowEvent::WindowEvent;
use super::FocusAdapter::FocusAdapter;
use super::FocusEvent::FocusEvent;
use super::ListSelectionListener::ListSelectionListener;
use super::ListSelectionEvent::ListSelectionEvent;
use super::ActionListener::ActionListener;
use super::ActionEvent::ActionEvent;
use super::util::util;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputAutomationJFrame {
    pub base: JFrame,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub inputAutomationModuleConfigurations: Option<Rc<RefCell<InputAutomationModuleConfigurations>>>,
    pub inputAutomationModuleFactory: Option<Rc<RefCell<InputAutomationModuleFactoryFactory>>>,
    pub inputAutomationModuleInterface: Option<Rc<RefCell<dyn InputAutomationModuleFactoryInterface>>>,
    pub runnableInterface: Option<Rc<RefCell<dyn RunnableInterface>>>,
    pub thread: Option<Rc<RefCell<Thread>>>,
    pub helpSet: Option<Rc<RefCell<HelpSet>>>,
    pub uri: Option<Rc<RefCell<URI>>>,
    pub aboutJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub automationModuleConfigurationJPanel: Option<Rc<RefCell<JPanel>>>,
    pub gameRobotJTabbedPane: Option<Rc<RefCell<JTabbedPane>>>,
    pub helpJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub inputAutomationModuleJList: Option<Rc<RefCell<JList>>>,
    pub inputAutomationModuleJPanel: Option<Rc<RefCell<JPanel>>>,
    pub jScrollPane1: Option<Rc<RefCell<JScrollPane>>>,
    pub mainJMenuBar: Option<Rc<RefCell<JMenuBar>>>,
    pub modulesJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub noModuleSelectedJDialog: Option<Rc<RefCell<JDialog>>>,
    pub noModuleSelectedJLabel: Option<Rc<RefCell<JLabel>>>,
    pub optionsJMenu: Option<Rc<RefCell<JMenu>>>,
    pub processingJMenu: Option<Rc<RefCell<JMenu>>>,
    pub startJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub stopJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub stopOnFocusJCheckBoxMenuItem: Option<Rc<RefCell<JCheckBoxMenuItem>>>,
    pub subscriptionJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
    pub toolsJMenu: Option<Rc<RefCell<JMenu>>>,
    pub updatesJMenuItem: Option<Rc<RefCell<JMenuItem>>>,
}

pub static mut InputAutomationJFrame_INPUTAUTOMATION_JFRAME: std::cell::LazyCell<Option<Rc<RefCell<InputAutomationJFrame>>>> = std::cell::LazyCell::new(|| None);
// impl InputAutomationConfigurationModuleChangeListener for InputAutomationJFrame with the methods of the trait
// impl InputAutomationRobotChangeListener for InputAutomationJFrame with the methods of the trait
// impl HelpSetListener for InputAutomationJFrame with the methods of the trait

impl InputAutomationJFrame {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: JFrame::new_struct(),
            logUtil: None,
            commonStrings: None,
            inputAutomationModuleConfigurations: None,
            inputAutomationModuleFactory: None,
            inputAutomationModuleInterface: None,
            runnableInterface: None,
            thread: None,
            helpSet: None,
            uri: None,
            aboutJMenuItem: None,
            automationModuleConfigurationJPanel: None,
            gameRobotJTabbedPane: None,
            helpJMenuItem: None,
            inputAutomationModuleJList: None,
            inputAutomationModuleJPanel: None,
            jScrollPane1: None,
            mainJMenuBar: None,
            modulesJMenuItem: None,
            noModuleSelectedJDialog: None,
            noModuleSelectedJLabel: None,
            optionsJMenu: None,
            processingJMenu: None,
            startJMenuItem: None,
            stopJMenuItem: None,
            stopOnFocusJCheckBoxMenuItem: None,
            subscriptionJMenuItem: None,
            toolsJMenu: None,
            updatesJMenuItem: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.inputAutomationModuleConfigurations = InputAutomationModuleConfigurations::new();
        this.uri = URI::new(String::from("http://geocities.com/allbinary/"));
        this.initComponents();
        let mut url: Option<Rc<RefCell<URL>>> = jcall!(std::any::type_name_of_val(&this)).getResource(String::from("/help/Help.hs"));
        jcall!(this.logUtil).putF(format!("{}{}", String::from("URL: "), url), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        this.helpSet = jcall!(JavaHelpUtil::getInstance()).getHelpSet(url.clone());
        url = jcall!(std::any::type_name_of_val(&this)).getResource(String::from("/resources/allbinaryicon8bit.jpg"));
        let mut imageIcon: Option<Rc<RefCell<ImageIcon>>> = ImageIcon::new(url.clone());
        { let java_arguments = (jcall!(imageIcon).getImage(),); this.setIconImage(java_arguments.0) };
        this.init();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn helpSetAdded(&mut self, mut helpSetEvent: Option<Rc<RefCell<HelpSetEvent>>>) {
        { let java_receiver = self.helpSet.clone(); let java_arguments = (jcall!(helpSetEvent).getHelpSet(),); jcall!(java_receiver).add(java_arguments.0); };
    }
    

    pub fn helpSetRemoved(&mut self, mut helpSetEvent: Option<Rc<RefCell<HelpSetEvent>>>) {
        { let java_receiver = self.helpSet.clone(); let java_arguments = (jcall!(helpSetEvent).getHelpSet(),); jcall!(java_receiver).remove(java_arguments.0); };
    }
    

    // throws Exception
    pub fn init(&mut self) {
        let mut inputAutomationConfigurationFactory: Option<Rc<RefCell<InputAutomationConfigurationFactory>>> = InputAutomationConfigurationFactory::getInstance();
        jcall!(inputAutomationConfigurationFactory).init(InputAutomationClientInformationFactory::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface>>));
        let mut inputAutomationModuleConfigurationList: Option<Rc<RefCell<dyn List<Option<Rc<RefCell<InputAutomationModuleConfiguration>>>>>>> = jcall!(jcall!(inputAutomationConfigurationFactory).inputAutomationConfiguration).getInputAutomationModuleConfigurationList();
        if inputAutomationModuleConfigurationList.is_none() {
        } else {
            self.inputAutomationModuleConfigurations = InputAutomationModuleConfigurations::new1(inputAutomationModuleConfigurationList.clone());
        }
        self.inputAutomationModuleFactory = InputAutomationModuleFactoryFactory::new(self.inputAutomationModuleConfigurations.clone(), None /* Java this: a &mut self method has no Rc of itself */);
        jcall!((JListSwingWorker::new(self.inputAutomationModuleJList.clone(), jcall!(self.inputAutomationModuleFactory).getListModel()))).execute();
        jcall!(self.gameRobotJTabbedPane).setEnabledAt(1, false);
        jcall!(self.gameRobotJTabbedPane).setSelectedIndex(0);
    }
    

    pub fn initComponents(&mut self) {
        self.noModuleSelectedJDialog = JDialog::new();
        self.noModuleSelectedJLabel = JLabel::new();
        self.gameRobotJTabbedPane = JTabbedPane::new();
        self.inputAutomationModuleJPanel = JPanel::new();
        self.jScrollPane1 = JScrollPane::new();
        self.inputAutomationModuleJList = JList::new();
        self.automationModuleConfigurationJPanel = JPanel::new();
        self.mainJMenuBar = JMenuBar::new();
        self.processingJMenu = JMenu::new();
        self.startJMenuItem = JMenuItem::new();
        self.stopJMenuItem = JMenuItem::new();
        self.optionsJMenu = JMenu::new();
        self.stopOnFocusJCheckBoxMenuItem = JCheckBoxMenuItem::new();
        self.toolsJMenu = JMenu::new();
        self.helpJMenuItem = JMenuItem::new();
        self.updatesJMenuItem = JMenuItem::new();
        self.subscriptionJMenuItem = JMenuItem::new();
        self.modulesJMenuItem = JMenuItem::new();
        self.aboutJMenuItem = JMenuItem::new();
        jcall!(self.noModuleSelectedJDialog).setMinimumSize(Dimension::new(200, 100));
        jcall!(self.noModuleSelectedJLabel).setText(String::from("Please Select A Module"));
        let mut noModuleSelectedJDialogLayout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(jcall!(self.noModuleSelectedJDialog).getContentPane());
        jcall!(jcall!(self.noModuleSelectedJDialog).getContentPane()).setLayout(noModuleSelectedJDialogLayout.clone());
        jcall!(noModuleSelectedJDialogLayout).setHorizontalGroup(jcall!(jcall!(noModuleSelectedJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(jcall!(noModuleSelectedJDialogLayout).createSequentialGroup()).addContainerGap()).addComponent(self.noModuleSelectedJLabel.clone())).addContainerGap(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone())));
        jcall!(noModuleSelectedJDialogLayout).setVerticalGroup(jcall!(jcall!(noModuleSelectedJDialogLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addComponent(self.noModuleSelectedJLabel.clone(), jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 48, i16::MAX.clone()));
        self.setDefaultCloseOperation(jcall!(jcall!(unsafe { javax_swing.clone() }).WindowConstants).EXIT_ON_CLOSE.clone());
        self.setMinimumSize(Dimension::new(325, 215));
        self.setResizable(false);
        { let java_arguments = (WindowFocusListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn windowGainedFocus(&mut self, mut evt: Option<Rc<RefCell<WindowEvent>>>) {
                self.formWindowGainedFocus(evt.clone());
            }
            

            pub fn windowLostFocus(&mut self, mut evt: Option<Rc<RefCell<WindowEvent>>>) {
                self.formWindowLostFocus(evt.clone());
            }
*/,); self.addWindowFocusListener(java_arguments.0) };
        { let java_arguments = (FocusAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn focusGained(&mut self, mut evt: Option<Rc<RefCell<FocusEvent>>>) {
                self.formFocusGained(evt.clone());
            }
            

            pub fn focusLost(&mut self, mut evt: Option<Rc<RefCell<FocusEvent>>>) {
                self.formFocusLost(evt.clone());
            }
*/,); self.addFocusListener(java_arguments.0) };
        jcall!(self.gameRobotJTabbedPane).setMinimumSize(Dimension::new(320, 200));
        jcall!(self.inputAutomationModuleJList).setMinimumSize(Dimension::new(200, 0));
        { let java_receiver = self.inputAutomationModuleJList.clone(); let java_arguments = (ListSelectionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn valueChanged(&mut self, mut evt: Option<Rc<RefCell<ListSelectionEvent>>>) {
                self.inputAutomationModuleJListValueChanged(evt.clone());
            }
*/,); jcall!(java_receiver).addListSelectionListener(java_arguments.0); };
        jcall!(self.jScrollPane1).setViewportView(self.inputAutomationModuleJList.clone());
        let mut inputAutomationModuleJPanelLayout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(self.inputAutomationModuleJPanel.clone());
        jcall!(self.inputAutomationModuleJPanel).setLayout(inputAutomationModuleJPanelLayout.clone());
        jcall!(inputAutomationModuleJPanelLayout).setHorizontalGroup(jcall!(jcall!(inputAutomationModuleJPanelLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(inputAutomationModuleJPanelLayout).createSequentialGroup()).addComponent(self.jScrollPane1.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 199, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addContainerGap(367, i16::MAX.clone())));
        jcall!(inputAutomationModuleJPanelLayout).setVerticalGroup(jcall!(jcall!(inputAutomationModuleJPanelLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGroup(jcall!(jcall!(jcall!(inputAutomationModuleJPanelLayout).createSequentialGroup()).addComponent(self.jScrollPane1.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), 330, jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone())).addContainerGap(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), i16::MAX.clone())));
        jcall!(self.gameRobotJTabbedPane).addTab(String::from("Modules"), self.inputAutomationModuleJPanel.clone());
        let mut automationModuleConfigurationJPanelLayout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(self.automationModuleConfigurationJPanel.clone());
        jcall!(self.automationModuleConfigurationJPanel).setLayout(automationModuleConfigurationJPanelLayout.clone());
        jcall!(automationModuleConfigurationJPanelLayout).setHorizontalGroup(jcall!(jcall!(automationModuleConfigurationJPanelLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGap(0, 566, i16::MAX.clone()));
        jcall!(automationModuleConfigurationJPanelLayout).setVerticalGroup(jcall!(jcall!(automationModuleConfigurationJPanelLayout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGap(0, 334, i16::MAX.clone()));
        jcall!(self.gameRobotJTabbedPane).addTab(String::from("Configuration"), self.automationModuleConfigurationJPanel.clone());
        jcall!(self.processingJMenu).setText(String::from("Processing"));
        jcall!(self.processingJMenu).setToolTipText(String::from("Set module processing state"));
        jcall!(self.startJMenuItem).setText(jcall!(self.commonStrings).START.clone());
        { let java_receiver = self.startJMenuItem.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.startJMenuItemActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.processingJMenu).add(self.startJMenuItem.clone());
        jcall!(self.stopJMenuItem).setText(String::from("Stop"));
        { let java_receiver = self.stopJMenuItem.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.stopJMenuItemActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.processingJMenu).add(self.stopJMenuItem.clone());
        jcall!(self.mainJMenuBar).add(self.processingJMenu.clone());
        jcall!(self.optionsJMenu).setText(String::from("Options"));
        { let java_receiver = self.optionsJMenu.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.optionsJMenuActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.stopOnFocusJCheckBoxMenuItem).setSelected(true);
        jcall!(self.stopOnFocusJCheckBoxMenuItem).setEnabled(false);
        jcall!(self.stopOnFocusJCheckBoxMenuItem).setLabel(String::from("Stop Module On Focus"));
        { let java_receiver = self.stopOnFocusJCheckBoxMenuItem.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.stopOnFocusJCheckBoxMenuItemActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.optionsJMenu).add(self.stopOnFocusJCheckBoxMenuItem.clone());
        jcall!(self.mainJMenuBar).add(self.optionsJMenu.clone());
        jcall!(self.toolsJMenu).setText(String::from("Help"));
        jcall!(self.helpJMenuItem).setText(String::from("Help"));
        { let java_receiver = self.helpJMenuItem.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.helpJMenuItemActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.toolsJMenu).add(self.helpJMenuItem.clone());
        jcall!(self.updatesJMenuItem).setText(String::from("Updates"));
        { let java_receiver = self.updatesJMenuItem.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.updatesJMenuItemActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.toolsJMenu).add(self.updatesJMenuItem.clone());
        jcall!(self.subscriptionJMenuItem).setText(String::from("Subscription"));
        { let java_receiver = self.subscriptionJMenuItem.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.subscriptionJMenuItemActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.toolsJMenu).add(self.subscriptionJMenuItem.clone());
        jcall!(self.modulesJMenuItem).setText(String::from("Module Manager"));
        { let java_receiver = self.modulesJMenuItem.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.modulesJMenuItemActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.toolsJMenu).add(self.modulesJMenuItem.clone());
        jcall!(self.aboutJMenuItem).setText(String::from("About"));
        { let java_receiver = self.aboutJMenuItem.clone(); let java_arguments = (ActionListener::new() /* Java anonymous class body: implement it as a named type
            

            pub fn actionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
                self.aboutJMenuItemActionPerformed(evt.clone());
            }
*/,); jcall!(java_receiver).addActionListener(java_arguments.0); };
        jcall!(self.toolsJMenu).add(self.aboutJMenuItem.clone());
        jcall!(self.mainJMenuBar).add(self.toolsJMenu.clone());
        self.setJMenuBar(self.mainJMenuBar.clone());
        let mut layout: Option<Rc<RefCell<GroupLayout>>> = GroupLayout::new(self.getContentPane());
        jcall!(self.getContentPane()).setLayout(layout.clone());
        jcall!(layout).setHorizontalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addComponent(self.gameRobotJTabbedPane.clone(), jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 571, i16::MAX.clone()));
        jcall!(layout).setVerticalGroup(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addComponent(self.gameRobotJTabbedPane.clone(), jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).TRAILING.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), 359, i16::MAX.clone()));
        self.pack();
    }
    

    pub fn subscriptionJMenuItemActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(Desktop::getDesktop()).browse(self.uri.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("subscriptionJMenuItemActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn updatesJMenuItemActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(Desktop::getDesktop()).browse(self.uri.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("updatesJMenuItemActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn aboutJMenuItemActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        let mut textJDialog: Option<Rc<RefCell<JDialog>>> = BasicTextJDialog::new1(String::from("Copyright (c) 2002-2007 AllBinary")).map(|reference| Rc::new(RefCell::new(Rc::try_unwrap(reference).ok().expect("new object is not shared").into_inner().base)));
        jcall!(textJDialog).setVisible(true);
        jcall!(textJDialog).setLocationRelativeTo(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    pub fn modulesJMenuItemActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(DesktopBundle::new()).start();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("modulesJMenuItemActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn helpJMenuItemActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        jcall!(JavaHelpUtil::getInstance()).show(self.helpSet.clone());
    }
    

    pub fn stopOnFocusJCheckBoxMenuItemActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
    }
    

    pub fn optionsJMenuActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
    }
    

    pub fn formWindowLostFocus(&mut self, mut evt: Option<Rc<RefCell<WindowEvent>>>) {
        jcall!(self.logUtil).putF(String::from("Nothing"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("focusLost"));
    }
    

    pub fn formWindowGainedFocus(&mut self, mut evt: Option<Rc<RefCell<WindowEvent>>>) {
        if jcall!(self.stopOnFocusJCheckBoxMenuItem).isSelected() {
            jcall!(self.logUtil).putF(String::from("Stopping"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("focusGained"));
            if self.runnableInterface.is_some() {
                jcall!(self.runnableInterface).setRunning(false);
            }
        }
    }
    

    pub fn formFocusLost(&mut self, mut evt: Option<Rc<RefCell<FocusEvent>>>) {
    }
    

    pub fn formFocusGained(&mut self, mut evt: Option<Rc<RefCell<FocusEvent>>>) {
    }
    

    pub fn inputAutomationModuleJListValueChanged(&mut self, mut evt: Option<Rc<RefCell<ListSelectionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut gameAutomationRobotModuleNameString: String = (jcall!(self.inputAutomationModuleJList).getSelectedValue()).clone() /* as String */;
            if true {
                self.inputAutomationModuleInterface = jcall!(self.inputAutomationModuleFactory).getInstance(gameAutomationRobotModuleNameString.clone());
                jcall!(self.getAutomationModuleConfigurationJPanel()).removeAll();
                let mut layout: Option<Rc<RefCell<GroupLayout>>> = (jcall!(self.getAutomationModuleConfigurationJPanel()).getLayout()).clone() /* as GroupLayout */;
                jcall!(layout).setHorizontalGroup(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGap(0, 395, i16::MAX.clone())).addComponent(jcall!(self.inputAutomationModuleInterface).getConfigurationJPanel(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()));
                jcall!(layout).setVerticalGroup(jcall!(jcall!(jcall!(layout).createParallelGroup(jcall!(jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).Alignment).LEADING.clone())).addGap(0, 256, i16::MAX.clone())).addComponent(jcall!(self.inputAutomationModuleInterface).getConfigurationJPanel(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).DEFAULT_SIZE.clone(), jcall!(jcall!(unsafe { javax_swing.clone() }).GroupLayout).PREFERRED_SIZE.clone()));
                jcall!(self.gameRobotJTabbedPane).setEnabledAt(1, true);
                jcall!(self.logUtil).putF(format!("{}{}", format!("{}{}", String::from("Setting Module: "), gameAutomationRobotModuleNameString), String::from(" and Configuration JPanel")), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("gameAutomationRobotModuleJListValueChanged"));
            } else {
                jcall!(self.logUtil).putF(String::from("Module name was null"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("gameAutomationRobotModuleJListValueChanged"));
                jcall!(self.gameRobotJTabbedPane).setEnabledAt(1, false);
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("gameAutomationRobotModuleJListValueChanged"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn stopJMenuItemActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        if self.runnableInterface.is_some() {
            jcall!(self.runnableInterface).setRunning(false);
        } else {
            jcall!(self.noModuleSelectedJDialog).setVisible(true);
            jcall!(self.noModuleSelectedJDialog).setLocationRelativeTo(None /* Java this: a &mut self method has no Rc of itself */);
        }
    }
    

    pub fn startJMenuItemActionPerformed(&mut self, mut evt: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(String::from("Starting"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("startJMenuItemActionPerformed"));
            if self.inputAutomationModuleInterface.is_some() {
                self.runnableInterface = jcall!(self.inputAutomationModuleInterface).getInstance();
                if self.runnableInterface.is_some() {
                    self.thread = Thread::new1(self.runnableInterface.clone());
                    jcall!(self.thread).setPriority(unsafe { Thread_MIN_PRIORITY.clone() } + 1);
                    jcall!(self.thread).start();
                } else {
                    jcall!(self.logUtil).putF(String::from("Runnnable was null"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("startJMenuItemActionPerformed"));
                    jcall!(self.noModuleSelectedJDialog).setVisible(true);
                    jcall!(self.noModuleSelectedJDialog).setLocationRelativeTo(None /* Java this: a &mut self method has no Rc of itself */);
                }
            } else {
                jcall!(self.logUtil).putF(String::from("InputAutomationModuleInterface was null"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("startJMenuItemActionPerformed"));
                jcall!(self.noModuleSelectedJDialog).setVisible(true);
                jcall!(self.noModuleSelectedJDialog).setLocationRelativeTo(None /* Java this: a &mut self method has no Rc of itself */);
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("startJMenuItemActionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<InputAutomationJFrame>>> {
        return unsafe { InputAutomationJFrame_INPUTAUTOMATION_JFRAME.clone() }.clone();
    }
    

    pub fn destroy() {
        jcall!(unsafe { InputAutomationJFrame_INPUTAUTOMATION_JFRAME.clone() }).setVisible(false);
    }
    

    pub fn create(mut inputAutomationBundleActivatorListenerInterface: Option<Rc<RefCell<dyn InputAutomationBundleActivatorListenerInterface>>>) {
        LogConfigTypeFactory::getInstance();
        let mut appUrlGlobals: Option<Rc<RefCell<AppUrlGlobals>>> = AppUrlGlobals::new();
        jcall!(appUrlGlobals).setWebappPath(String::from("G:/mnt/bc/mydev/work_automation/automation/run/"));
        URLGLOBALS::init(appUrlGlobals.clone());
        WebappClassLoaderInfo::setLoader(jcall!(std::any::type_name_of_val(&appUrlGlobals)).getClassLoader());
        jcall!(jcall!(unsafe { java_awt.clone() }).EventQueue).invokeLater(ARunnable::new() /* Java anonymous class body: implement it as a named type
            

            pub fn run(&mut self) {
                let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    jcall!(logUtil).putF(String::from("Running"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).RUN.clone());
                    unsafe { *InputAutomationJFrame_INPUTAUTOMATION_JFRAME = InputAutomationJFrame::new() };
                    jcall!(InputRobotFactory::getInstance()).addListener(InputAutomationJFrame::getInstance());
                    if inputAutomationBundleActivatorListenerInterface.is_some() {
                        jcall!(inputAutomationBundleActivatorListenerInterface).registerAsService();
                        jcall!(inputAutomationBundleActivatorListenerInterface).useServices();
                    }
                    jcall!(unsafe { InputAutomationJFrame_INPUTAUTOMATION_JFRAME.clone() }).setVisible(true);
                })) {
                    Ok(_) => {}
                    Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                    jcall!(logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).RUN.clone(), crate::java_into_object(e.clone()));
                    }
                }
            }
*/);
    }
    

    // throws Exception
    pub fn main(mut args: Vec<String>) {
        InputAutomationJFrame::create(None);
    }
    

    pub fn getAutomationModuleConfigurationJPanel(&mut self) -> Option<Rc<RefCell<JPanel>>> {
        return self.automationModuleConfigurationJPanel.clone();
    }
    

    pub fn setAutomationModuleConfigurationJPanel(&mut self, mut automationModuleConfigurationJPanel: Option<Rc<RefCell<JPanel>>>) {
        self.automationModuleConfigurationJPanel = automationModuleConfigurationJPanel.clone();
    }
    

    pub fn onAdd(&mut self, mut inputAutomationRobotChangeEvent: Option<Rc<RefCell<InputAutomationRobotChangeEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onAdd"));
            jcall!(InputRobotFactory::getInstance()).add(jcall!(jcall!(inputAutomationRobotChangeEvent).getInputAutomationRobotInterfaceWrapper()).getInputRobotInterface());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onAdd"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn onRemove(&mut self, mut inputAutomationRobotChangeEvent: Option<Rc<RefCell<InputAutomationRobotChangeEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onRemove"));
            jcall!(InputRobotFactory::getInstance()).add(jcall!(jcall!(inputAutomationRobotChangeEvent).getInputAutomationRobotInterfaceWrapper()).getInputRobotInterface());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onRemove"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn onAdd(&mut self, mut inputAutomationConfigurationChangeEvent: Option<Rc<RefCell<InputAutomationConfigurationModuleChangeEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onAdd"));
            { let java_receiver = self.inputAutomationModuleConfigurations.clone(); let java_arguments = (jcall!(inputAutomationConfigurationChangeEvent).getInputAutomationModuleConfiguration(),); jcall!(java_receiver).add(java_arguments.0); };
            self.init();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onAdd"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn onRemove(&mut self, mut inputAutomationConfigurationChangeEvent: Option<Rc<RefCell<InputAutomationConfigurationModuleChangeEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onRemove"));
            let mut inputAutomationConfiguration: Option<Rc<RefCell<InputAutomationConfiguration>>> = jcall!(InputAutomationConfigurationFactory::getInstance()).inputAutomationConfiguration.clone();
            let mut inputAutomationModuleConfigurationList: Option<Rc<RefCell<dyn List<Option<Rc<RefCell<InputAutomationModuleConfiguration>>>>>>> = jcall!(inputAutomationConfiguration).getInputAutomationModuleConfigurationList();
            if inputAutomationModuleConfigurationList.is_none() {
                let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(logUtil).putF(format!("{}{}", String::from("inputAutomationModuleConfigurationList: "), inputAutomationModuleConfigurationList), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).INIT.clone());
                panic!("{:?}", format!("{}: {:?}", "Exception", ()));
            }
            let mut inputAutomationModuleConfigurations: Option<Rc<RefCell<InputAutomationModuleConfigurations>>> = InputAutomationModuleConfigurations::new1(jcall!(inputAutomationConfiguration).getInputAutomationModuleConfigurationList());
            jcall!(inputAutomationModuleConfigurations).remove(jcall!(inputAutomationConfigurationChangeEvent).getInputAutomationModuleConfiguration());
            self.init();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onRemove"), crate::java_into_object(e.clone()));
            }
        }
    }
}
impl std::ops::Deref for InputAutomationJFrame {
    type Target = JFrame;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for InputAutomationJFrame {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



