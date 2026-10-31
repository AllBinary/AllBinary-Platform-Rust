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
// package org.allbinary.graphics.j2me.workarea
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Runnable;
use std::collections::HashMap;
use super::*;
// use crate::java::awt::*;
// use crate::java::util::EventListener::EventListener;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::LinkedList::LinkedList;
// use crate::javax::swing::*;
// use crate::javax::swing::tree::MutableTreeNode::MutableTreeNode;
use crate::org::allbinary::dom::DomHelper::DomHelper;
use crate::org::allbinary::graphics::j2me::StatusFactory::StatusFactory;
use crate::org::allbinary::graphics::j2me::workarea::canvas::CanvasDom::CanvasDom;
use crate::org::allbinary::graphics::j2me::workarea::canvas::CanvasJPanel::CanvasJPanel;
use crate::org::allbinary::graphics::j2me::workarea::canvas::event::MyCanvasEvent::MyCanvasEvent;
use crate::org::allbinary::graphics::j2me::workarea::canvas::event::MyCanvasEventListener::MyCanvasEventListener;
use crate::org::allbinary::graphics::j2me::workarea::canvas::event::MyCanvasEventService::MyCanvasEventService;
use crate::org::allbinary::graphics::j2me::workarea::properties::PropertiesJPanel::PropertiesJPanel;
use crate::org::allbinary::graphics::j2me::workarea::tools::ToolJPanel::ToolJPanel;
use crate::org::allbinary::log::LOGGING::LOGGING;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::java::lang::Thread::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WorkAreaJPanel {
    pub base: JPanel,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub canvasJPanelList: Option<Rc<RefCell<LinkedList>>>,
    pub canvasHolderJPanel: Option<Rc<RefCell<JPanel>>>,
    pub name: String,
    pub selectedFrame: i32,
    pub propertiesJPanel: Option<Rc<RefCell<PropertiesJPanel>>>,
    pub workAreaJTreeJPanel: Option<Rc<RefCell<WorkAreaJTreeJPanel>>>,
    pub isPlaying: bool,
    pub innerJPanel: Option<Rc<RefCell<JPanel>>>,
}

pub static mut WorkAreaJPanel_document: std::cell::LazyCell<Option<Rc<RefCell<Document>>>> = std::cell::LazyCell::new(|| None);
// impl WorkAreaJPanelInterface for WorkAreaJPanel with the methods of the trait
// impl MyCanvasEventListener for WorkAreaJPanel with the methods of the trait
// impl EventListener for WorkAreaJPanel with the methods of the trait
impl Runnable for WorkAreaJPanel {
    fn run(&mut self) { Self::run(self) }
}

impl WorkAreaJPanel {
    

    pub fn new_struct4(mut name: String, mut dimension: Option<Rc<RefCell<Dimension>>>, mut x: i32, mut y: i32) -> Self {
        let mut this = Self {
            base: JPanel::new_struct(),
            logUtil: None,
            canvasJPanelList: None,
            canvasHolderJPanel: None,
            name: String::new(),
            selectedFrame: 0,
            propertiesJPanel: None,
            workAreaJTreeJPanel: None,
            isPlaying: false,
            innerJPanel: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.isPlaying = false;
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            this.initComponents();
            this.name = name.clone();
            this.initMyComponents(dimension.clone());
            this.workAreaJTreeJPanel = WorkAreaJTreeJPanel::new(this.name.clone());
            let mut canvasJPanel: Option<Rc<RefCell<CanvasJPanel>>> = CanvasJPanel::new4(this.workAreaJTreeJPanel.clone(), this.getSize(), x.clone(), y.clone());
            jcall!(this.canvasJPanelList).add(canvasJPanel.clone());
            this.selectedFrame = 0;
            jcall!(this.propertiesJPanel).set(this.workAreaJTreeJPanel.clone());
            jcall!(this.canvasHolderJPanel).add((canvasJPanel).clone() /* as Component */);
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
        this
    }

    pub fn new4(mut name: String, mut dimension: Option<Rc<RefCell<Dimension>>>, mut x: i32, mut y: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct4(name, dimension, x, y))))
    }
    

    pub fn new_struct2(mut workAreaDom: Option<Rc<RefCell<WorkAreaDom>>>, mut dimension: Option<Rc<RefCell<Dimension>>>) -> Self {
        let mut this = Self {
            base: JPanel::new_struct(),
            logUtil: None,
            canvasJPanelList: None,
            canvasHolderJPanel: None,
            name: String::new(),
            selectedFrame: 0,
            propertiesJPanel: None,
            workAreaJTreeJPanel: None,
            isPlaying: false,
            innerJPanel: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.isPlaying = false;
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            this.initComponents();
            this.name = jcall!(workAreaDom).getName();
            this.initMyComponents(dimension.clone());
            let mut canvasNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(workAreaDom).getCanvasNodes();
            let mut numberOfFrames: i32 = jcall!(canvasNodeList).size();
            for index in (0..numberOfFrames) {
                let mut node: Option<Rc<RefCell<Node>>> = (jcall!(canvasNodeList).get(index.clone())).clone() /* as Node */;
                let mut canvasJPanel: Option<Rc<RefCell<CanvasJPanel>>> = CanvasJPanel::new3(this.workAreaJTreeJPanel.clone(), this.getSize(), CanvasDom::new1(node.clone()));
                jcall!(this.canvasJPanelList).add(canvasJPanel.clone());
            }
            this.selectedFrame = 0;
            { let java_receiver = this.canvasHolderJPanel.clone(); let java_arguments = ((this.getCurrentFrame()).clone() /* as Component */,); jcall!(java_receiver).add(java_arguments.0); };
            jcall!(this.propertiesJPanel).set(this.workAreaJTreeJPanel.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if LOGGING::contains(crate::org::allbinary::log::LOGGING::LOGGING_LOADINGERROR.with(Clone::clone).clone()) {
                jcall!(this.logUtil).put(String::from("Constructor Error"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("WorkAreaJPanel"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
        this
    }

    pub fn new2(mut workAreaDom: Option<Rc<RefCell<WorkAreaDom>>>, mut dimension: Option<Rc<RefCell<Dimension>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(workAreaDom, dimension))))
    }
    

    pub fn initMyComponents(&mut self, mut dimension: Option<Rc<RefCell<Dimension>>>) {
        self.workAreaJTreeJPanel = WorkAreaJTreeJPanel::new(self.name.clone());
        self.canvasHolderJPanel = JPanel::new();
        jcall!(self.canvasHolderJPanel).setSize(dimension.clone());
        jcall!(self.canvasHolderJPanel).setLayout(GridLayout::new(1, 1));
        self.setSize(dimension.clone());
        self.canvasJPanelList = LinkedList::new();
        let mut gridBagLayout: Option<Rc<RefCell<GridBagLayout>>> = GridBagLayout::new();
        let mut gridBagConstraints: Option<Rc<RefCell<GridBagConstraints>>> = GridBagConstraints::new();
        jcall!(self.innerJPanel).setLayout(gridBagLayout.clone());
        jcall!(gridBagConstraints).gridx = 0;
        jcall!(gridBagConstraints).gridy = 0;
        jcall!(gridBagConstraints).gridwidth = 1;
        jcall!(gridBagConstraints).fill = jcall!(jcall!(unsafe { java_awt.clone() }).GridBagConstraints).BOTH.clone();
        jcall!(gridBagConstraints).weightx = .05;
        jcall!(gridBagConstraints).weighty = .05;
        let mut toolJPanel: Option<Rc<RefCell<ToolJPanel>>> = ToolJPanel::new();
        jcall!(gridBagLayout).setConstraints(toolJPanel.clone(), gridBagConstraints.clone());
        jcall!(self.innerJPanel).add(toolJPanel.clone());
        gridBagConstraints = GridBagConstraints::new();
        jcall!(gridBagConstraints).gridx = 1;
        jcall!(gridBagConstraints).gridy = 0;
        jcall!(gridBagConstraints).gridwidth = 6;
        jcall!(gridBagConstraints).fill = jcall!(jcall!(unsafe { java_awt.clone() }).GridBagConstraints).BOTH.clone();
        jcall!(gridBagConstraints).weightx = 1;
        jcall!(gridBagConstraints).weighty = 1;
        jcall!(gridBagLayout).setConstraints(self.canvasHolderJPanel.clone(), gridBagConstraints.clone());
        jcall!(self.innerJPanel).add(self.canvasHolderJPanel.clone());
        gridBagConstraints = GridBagConstraints::new();
        jcall!(gridBagConstraints).gridx = 8;
        jcall!(gridBagConstraints).gridy = 0;
        jcall!(gridBagConstraints).gridwidth = jcall!(jcall!(unsafe { java_awt.clone() }).GridBagConstraints).REMAINDER.clone();
        jcall!(gridBagConstraints).fill = jcall!(jcall!(unsafe { java_awt.clone() }).GridBagConstraints).BOTH.clone();
        jcall!(gridBagConstraints).weightx = .1;
        jcall!(gridBagConstraints).weighty = .1;
        self.propertiesJPanel = PropertiesJPanel::new();
        jcall!(gridBagLayout).setConstraints(self.propertiesJPanel.clone(), gridBagConstraints.clone());
        jcall!(self.innerJPanel).add(self.propertiesJPanel.clone());
        MyCanvasEventService::addListener((self).clone() /* as MyCanvasEventListener */);
    }
    

    pub fn getCanvasHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut canvasJPanelHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        let mut size: i32 = jcall!(self.canvasJPanelList).size();
        for index in (0..size) {
            let mut canvasJPanel: Option<Rc<RefCell<CanvasJPanel>>> = (jcall!(self.canvasJPanelList).get(index.clone())).clone() /* as CanvasJPanel */;
            jcall!(canvasJPanelHashMap).put(jcall!(canvasJPanel).getTreeNode(), Some((index.clone()) as i32));
        }
        return canvasJPanelHashMap.clone();
    }
    

    // synchronized
    pub fn play(&mut self) {
        self.isPlaying = true;
    }
    

    // synchronized
    pub fn stop(&mut self) {
        self.isPlaying = false;
    }
    

    // synchronized
    pub fn isPlaying(&mut self) -> bool {
        return self.isPlaying.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn setName(&mut self, mut value: String) {
        self.name = value.clone();
    }
    

    pub fn disableCanvas(&mut self) {
        jcall!(self.canvasHolderJPanel).removeAll();
    }
    

    pub fn enableCanvas(&mut self) {
        { let java_receiver = self.canvasHolderJPanel.clone(); let java_arguments = ((self.getCurrentFrame()).clone() /* as Component */,); jcall!(java_receiver).add(java_arguments.0); };
    }
    

    pub fn deselect(&mut self) {
        self.base.setVisible(false);
    }
    

    pub fn select(&mut self) {
        self.base.setVisible(true);
        if jcall!(self.canvasHolderJPanel).getComponentCount() == 1 {
            let mut component: Option<Rc<RefCell<Component>>> = jcall!(self.canvasHolderJPanel).getComponent(0);
            if component.is_some() {
                jcall!(component).repaint();
            }
        }
    }
    

    pub fn changeZoom(&mut self, mut factor: i32) {
        let mut size: i32 = jcall!(self.canvasJPanelList).size();
        for index in (0..size) {
            let mut canvasJPanel: Option<Rc<RefCell<CanvasJPanel>>> = (jcall!(self.canvasJPanelList).get(index.clone())).clone() /* as CanvasJPanel */;
            jcall!(jcall!(canvasJPanel).getGrid()).setZoom(jcall!(jcall!(canvasJPanel).getGrid()).getZoom() + factor);
        }
    }
    

    pub fn getCurrentFrame(&mut self) -> Option<Rc<RefCell<CanvasJPanel>>> {
        return (jcall!(self.canvasJPanelList).get(self.selectedFrame.clone())).clone() /* as CanvasJPanel */;
    }
    

    pub fn initDuplicate(&mut self, mut newCanvasJPanel: Option<Rc<RefCell<CanvasJPanel>>>) {
        if newCanvasJPanel.is_some() {
            jcall!(self.canvasJPanelList).add(newCanvasJPanel.clone());
            { let java_arguments = (jcall!(self.canvasJPanelList).size() - 1,); self.select1(java_arguments.0) };
        }
    }
    

    // throws Exception
    pub fn duplicate(&mut self, mut treeNode: Option<Rc<RefCell<MutableTreeNode>>>) {
        let mut canvasJPanelHashMap: Option<Rc<RefCell<HashMap>>> = self.getCanvasHashMap();
        if jcall!(canvasJPanelHashMap).containsKey(treeNode.clone()) {
            let mut frameInteger: Option<i32> = (jcall!(canvasJPanelHashMap).get(treeNode.clone())).clone() /* as Integer */;
            let mut canvasJPanel: Option<Rc<RefCell<CanvasJPanel>>> = ({ let java_receiver = self.canvasJPanelList.clone(); let java_arguments = (((frameInteger).unwrap() as i32),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as CanvasJPanel */;
            { let java_arguments = (jcall!(canvasJPanel).duplicate(),); self.initDuplicate(java_arguments.0) };
        }
    }
    

    // throws Exception
    pub fn autoRotate(&mut self, mut increments: i32, mut totalAngle: i32) {
        let mut incrementAngle: i32 = (totalAngle / increments);
        for index in (1..increments) {
            let mut canvasJPanel: Option<Rc<RefCell<CanvasJPanel>>> = (self.getCurrentFrame()).clone() /* as CanvasJPanel */;
            let mut newCanvasJPanel: Option<Rc<RefCell<CanvasJPanel>>> = jcall!(canvasJPanel).duplicate();
            jcall!(newCanvasJPanel).setAngle(incrementAngle * index);
            self.initDuplicate(newCanvasJPanel.clone());
        }
    }
    

    pub fn explodeAll(&mut self) {
        let mut canvasJPanel: Option<Rc<RefCell<CanvasJPanel>>> = (self.getCurrentFrame()).clone() /* as CanvasJPanel */;
        jcall!(canvasJPanel).explodeAll();
    }
    

    // throws Exception
    pub fn autoExplode2(&mut self, mut numberOfFrames: i32, mut explodeType: i32) {
        for index in (1..numberOfFrames) {
            let mut canvasJPanel: Option<Rc<RefCell<CanvasJPanel>>> = (self.getCurrentFrame()).clone() /* as CanvasJPanel */;
            let mut newCanvasJPanel: Option<Rc<RefCell<CanvasJPanel>>> = jcall!(canvasJPanel).duplicate();
            jcall!(newCanvasJPanel).explode(index + 1, explodeType.clone());
            self.initDuplicate(newCanvasJPanel.clone());
        }
    }
    

    // throws Exception
    pub fn autoMirror(&mut self) {
        let mut canvasJPanel: Option<Rc<RefCell<CanvasJPanel>>> = (self.getCurrentFrame()).clone() /* as CanvasJPanel */;
        let mut newCanvasJPanel: Option<Rc<RefCell<CanvasJPanel>>> = jcall!(canvasJPanel).duplicate();
        jcall!(newCanvasJPanel).mirror();
        self.initDuplicate(newCanvasJPanel.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    // synchronized
    pub fn select1(&mut self, mut index: i32) {
        if index < jcall!(self.canvasJPanelList).size() {
            let mut canvasJPanel: Option<Rc<RefCell<CanvasJPanel>>> = (jcall!(self.canvasJPanelList).get(index.clone())).clone() /* as CanvasJPanel */;
            if canvasJPanel.is_some() {
                jcall!(self.canvasHolderJPanel).removeAll();
                jcall!(self.canvasHolderJPanel).add(canvasJPanel.clone());
                self.selectedFrame = index.clone();
                jcall!(canvasJPanel).changed();
                jcall!(self.canvasHolderJPanel).repaint();
                jcall!(canvasJPanel).repaint();
            }
        }
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn select1(&mut self, mut treeNode: Option<Rc<RefCell<MutableTreeNode>>>) {
        let mut canvasJPanelHashMap: Option<Rc<RefCell<HashMap>>> = self.getCanvasHashMap();
        let mut frameInteger: Option<i32> = (jcall!(canvasJPanelHashMap).get(treeNode.clone())).clone() /* as Integer */;
        if frameInteger.is_some() {
            { let java_arguments = (((frameInteger).unwrap() as i32),); self.select1(java_arguments.0) };
        }
    }
    

    pub fn getDocument() -> Option<Rc<RefCell<Document>>> {
        return unsafe { WorkAreaJPanel_document.clone() }.clone();
    }
    

    // throws Exception
    pub fn toDocument(&mut self) -> Option<Rc<RefCell<Document>>> {
        unsafe { *WorkAreaJPanel_document = jcall!(DomHelper::getInstance()).createDocument() };
        let mut workAreaNode: Option<Rc<RefCell<Node>>> = jcall!(unsafe { WorkAreaJPanel_document.clone() }).createElement(WorkAreaDom_WORKAREA.with(Clone::clone).clone());
        let mut nameNode: Option<Rc<RefCell<Node>>> = jcall!(unsafe { WorkAreaJPanel_document.clone() }).createElement(WorkAreaDom_NAME.with(Clone::clone).clone());
        let mut nameTextNode: Option<Rc<RefCell<Node>>> = jcall!(unsafe { WorkAreaJPanel_document.clone() }).createTextNode(self.name.clone());
        jcall!(nameNode).appendChild(nameTextNode.clone());
        let mut framesNode: Option<Rc<RefCell<Node>>> = jcall!(unsafe { WorkAreaJPanel_document.clone() }).createElement(WorkAreaDom_FRAMES.with(Clone::clone).clone());
        let mut size: i32 = jcall!(self.canvasJPanelList).size();
        for index in (0..size) {
            let mut canvasJPanel: Option<Rc<RefCell<CanvasJPanel>>> = (jcall!(self.canvasJPanelList).get(index.clone())).clone() /* as CanvasJPanel */;
            jcall!(framesNode).appendChild(jcall!(canvasJPanel).toDom());
        }
        jcall!(workAreaNode).appendChild(nameNode.clone());
        jcall!(workAreaNode).appendChild(framesNode.clone());
        jcall!(unsafe { WorkAreaJPanel_document.clone() }).appendChild(workAreaNode.clone());
        return unsafe { WorkAreaJPanel_document.clone() }.clone();
    }
    

    pub fn initComponents(&mut self) {
        self.innerJPanel = JPanel::new();
        self.setLayout(GridLayout::new(1, 1));
        { let java_receiver = self.innerJPanel.clone(); let java_arguments = (KeyAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn keyTyped(&mut self, mut evt: Option<Rc<RefCell<KeyEvent>>>) {
                self.innerJPanelKeyTyped(evt.clone());
            }
            

            pub fn keyPressed(&mut self, mut evt: Option<Rc<RefCell<KeyEvent>>>) {
                self.innerJPanelKeyPressed(evt.clone());
            }
            

            pub fn keyReleased(&mut self, mut evt: Option<Rc<RefCell<KeyEvent>>>) {
                self.innerJPanelKeyReleased(evt.clone());
            }
*/,); jcall!(java_receiver).addKeyListener(java_arguments.0); };
        self.add(self.innerJPanel.clone());
    }
    

    pub fn innerJPanelKeyReleased(&mut self, mut evt: Option<Rc<RefCell<KeyEvent>>>) {
        jcall!(StatusFactory::getInstance()).setStatus(String::from("Key Pressed1"));
    }
    

    pub fn innerJPanelKeyPressed(&mut self, mut evt: Option<Rc<RefCell<KeyEvent>>>) {
        jcall!(StatusFactory::getInstance()).setStatus(String::from("Key Pressed2"));
    }
    

    pub fn innerJPanelKeyTyped(&mut self, mut evt: Option<Rc<RefCell<KeyEvent>>>) {
        jcall!(StatusFactory::getInstance()).setStatus(String::from("Key Pressed3"));
    }
    

    pub fn keyPressed(&mut self, mut keyEvent: Option<Rc<RefCell<KeyEvent>>>) {
        jcall!(self.getCurrentFrame()).keyPressed(keyEvent.clone());
    }
    

    pub fn keyReleased(&mut self, mut keyEvent: Option<Rc<RefCell<KeyEvent>>>) {
    }
    

    pub fn keyTyped(&mut self, mut keyEvent: Option<Rc<RefCell<KeyEvent>>>) {
    }
    

    pub fn delete(&mut self, mut evt: Option<Rc<RefCell<MyCanvasEvent>>>) {
        let mut canvasJPanelHashMap: Option<Rc<RefCell<HashMap>>> = self.getCanvasHashMap();
        if jcall!(canvasJPanelHashMap).containsKey(jcall!(evt).getTreeNode()) {
            jcall!(self.canvasHolderJPanel).removeAll();
            let mut frameInt: Option<i32> = (jcall!(canvasJPanelHashMap).get(jcall!(evt).getTreeNode())).clone() /* as Integer */;
            { let java_receiver = self.canvasJPanelList.clone(); let java_arguments = (((frameInt).unwrap() as i32),); jcall!(java_receiver).remove(java_arguments.0); };
            { let java_arguments = (((frameInt).unwrap() as i32),); self.select1(java_arguments.0) };
        }
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn duplicate(&mut self, mut evt: Option<Rc<RefCell<MyCanvasEvent>>>) {
        { let java_arguments = (jcall!(evt).getTreeNode(),); self.duplicate(java_arguments.0) };
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn select1(&mut self, mut evt: Option<Rc<RefCell<MyCanvasEvent>>>) {
        { let java_arguments = (jcall!(evt).getTreeNode(),); self.select1(java_arguments.0) };
        jcall!(StatusFactory::getInstance()).setStatus(String::from("Frame Selected"));
    }
    

    pub fn rotate(&mut self, mut evt: Option<Rc<RefCell<MyCanvasEvent>>>) {
        let mut canvasJPanelHashMap: Option<Rc<RefCell<HashMap>>> = self.getCanvasHashMap();
        let mut treeNode: Option<Rc<RefCell<MutableTreeNode>>> = jcall!(evt).getTreeNode();
        let mut frameInt: Option<i32> = (jcall!(canvasJPanelHashMap).get(jcall!(evt).getTreeNode())).clone() /* as Integer */;
        let mut canvasJPanel: Option<Rc<RefCell<CanvasJPanel>>> = ({ let java_receiver = self.canvasJPanelList.clone(); let java_arguments = (((frameInt).unwrap() as i32),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as CanvasJPanel */;
        if canvasJPanel.is_some() {
            jcall!(canvasJPanel).addAngle(jcall!(evt).getAngle());
            { let java_arguments = (((frameInt).unwrap() as i32),); self.select1(java_arguments.0) };
        }
        jcall!(StatusFactory::getInstance()).setStatus(String::from("Canvas Rotated"));
    }
    

    pub fn explode(&mut self, mut evt: Option<Rc<RefCell<MyCanvasEvent>>>) {
        self.explodeAll();
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn autoExplode(&mut self, mut evt: Option<Rc<RefCell<MyCanvasEvent>>>) {
        self.autoExplode2(10, 0);
    }
    

    // throws Exception
    pub fn center(&mut self) {
        let mut canvasJPanel: Option<Rc<RefCell<CanvasJPanel>>> = (self.getCurrentFrame()).clone() /* as CanvasJPanel */;
        jcall!(canvasJPanel).center();
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn center1(&mut self, mut evt: Option<Rc<RefCell<MyCanvasEvent>>>) {
        self.center();
    }
    

    pub fn run(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            while true {
                if self.isPlaying() {
                    let mut size: i32 = jcall!(self.canvasJPanelList).size();
                    for index in (0..size) {
                        if !self.isPlaying() {
                            break;
                        }
                        self.select1(index.clone());
                        std::thread::sleep(std::time::Duration::from_millis((100) as u64));
                    }
                } else {
                    std::thread::sleep(std::time::Duration::from_millis((500) as u64));
                }
                std::thread::sleep(std::time::Duration::from_millis((250) as u64));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            }
        }
    }
}
impl std::ops::Deref for WorkAreaJPanel {
    type Target = JPanel;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for WorkAreaJPanel {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



