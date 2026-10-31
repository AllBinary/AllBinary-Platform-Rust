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
// use crate::java::awt::event::ActionEvent::ActionEvent;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
// use crate::javax::swing::*;
// use crate::javax::swing::tree::TreePath::TreePath;
use crate::org::allbinary::input::automation::actions::script::JTreeInterfaceFactory::JTreeInterfaceFactory;
use crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItem;
use crate::org::allbinary::input::automation::actions::script::ProfileActionScriptNodeInterface::ProfileActionScriptNodeInterface;
use crate::org::allbinary::input::automation::actions::script::condition::processors::ProfileActionScriptProcessorInterface::ProfileActionScriptProcessorInterface;
use crate::org::allbinary::input::automation::actions::script::condition::processors::input::GenericProfileActionScriptInputData::GenericProfileActionScriptInputData;
use crate::org::allbinary::input::automation::actions::script::condition::processors::input::KeyboardActionScriptInput::KeyboardActionScriptInput;
use crate::org::allbinary::input::automation::actions::script::condition::processors::input::MouseActionScriptInput::MouseActionScriptInput;
use crate::org::allbinary::input::automation::actions::script::condition::processors::input::ProfileActionScriptInputFactory::ProfileActionScriptInputFactory;
use crate::org::allbinary::input::automation::module::actions::script::condition::processors::output::GenericProfileActionScriptOutputData::GenericProfileActionScriptOutputData;
use crate::org::allbinary::input::automation::module::actions::script::condition::processors::output::ImageActionScriptOutput::ImageActionScriptOutput;
use crate::org::allbinary::input::automation::module::actions::script::condition::processors::output::ProfileActionScriptOutputFactory::ProfileActionScriptOutputFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicProfileActionScriptCondition {
    pub base: ProfileActionScriptItem,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub profileActionProcessorInterfaceVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub profileActionConditionInterfaceVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

// impl ProfileActionScriptConditionInterface for BasicProfileActionScriptCondition with the methods of the trait

impl BasicProfileActionScriptCondition {
    

    pub fn new_struct2(mut label: String, mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            base: ProfileActionScriptItem::new_struct2(label.clone(), node.clone()),
            logUtil: None,
            profileActionProcessorInterfaceVector: None,
            profileActionConditionInterfaceVector: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this.init();
        let mut nodeList: Option<Rc<RefCell<NodeList>>> = jcall!(node).getChildNodes();
        for index in (0..jcall!(nodeList).getLength()) {
            let mut actionItemNode: Option<Rc<RefCell<Node>>> = jcall!(nodeList).item(index.clone());
            if match (&jcall!(actionItemNode).getNodeType(), &unsafe { crate::org::w3c::dom::Node::Node_ELEMENT_NODE.clone() }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                if jcall!(jcall!(actionItemNode).getNodeName()).compareTo(unsafe { GenericProfileActionScriptConditionData_NAME.clone() }.clone()) == 0 {
                    { let java_arguments = (ProfileActionScriptConditionFactory::ProfileActionScriptConditionFactory::getInstance(actionItemNode.clone()),); this.addCondition(java_arguments.0) };
                } else if jcall!(jcall!(actionItemNode).getNodeName()).compareTo(unsafe { crate::org::allbinary::input::automation::actions::script::condition::processors::input::GenericProfileActionScriptInputData::GenericProfileActionScriptInputData_NAME.clone() }.clone()) == 0 {
                    { let java_arguments = (ProfileActionScriptInputFactory::getInstance(actionItemNode.clone()),); this.addProcessor(java_arguments.0) };
                } else if jcall!(jcall!(actionItemNode).getNodeName()).compareTo(unsafe { crate::org::allbinary::input::automation::module::actions::script::condition::processors::output::GenericProfileActionScriptOutputData::GenericProfileActionScriptOutputData_NAME.clone() }.clone()) == 0 {
                    { let java_arguments = (ProfileActionScriptOutputFactory::getInstance(actionItemNode.clone()),); this.addProcessor(java_arguments.0) };
                }
            }
        }
        this
    }

    pub fn new2(mut label: String, mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(label, node))))
    }
    

    // throws Exception
    pub fn addProcessorNodes(&mut self, mut node: Option<Rc<RefCell<Node>>>) {
        let mut nodeList: Option<Rc<RefCell<NodeList>>> = jcall!(node).getChildNodes();
        for index in (0..jcall!(nodeList).getLength()) {
            let mut processorActionItemNode: Option<Rc<RefCell<Node>>> = jcall!(nodeList).item(index.clone());
            if jcall!(jcall!(processorActionItemNode).getNodeName()).compareTo(unsafe { crate::org::allbinary::input::automation::actions::script::condition::processors::input::GenericProfileActionScriptInputData::GenericProfileActionScriptInputData_NAME.clone() }.clone()) == 0 {
                { let java_arguments = (ProfileActionScriptInputFactory::getInstance(processorActionItemNode.clone()),); self.addProcessor(java_arguments.0) };
            } else if jcall!(jcall!(processorActionItemNode).getNodeName()).compareTo(unsafe { crate::org::allbinary::input::automation::module::actions::script::condition::processors::output::GenericProfileActionScriptOutputData::GenericProfileActionScriptOutputData_NAME.clone() }.clone()) == 0 {
                { let java_arguments = (ProfileActionScriptOutputFactory::getInstance(processorActionItemNode.clone()),); self.addProcessor(java_arguments.0) };
            }
        }
    }
    

    pub fn new_struct1(mut label: String) -> Self {
        let mut this = Self {
            base: ProfileActionScriptItem::new_struct1(label.clone()),
            logUtil: None,
            profileActionProcessorInterfaceVector: None,
            profileActionConditionInterfaceVector: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this.init();
        this
    }

    pub fn new1(mut label: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(label))))
    }
    

    pub fn init(&mut self) {
        self.setProfileActionProcessorInterfaceVector(BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
        self.setProfileActionConditionInterfaceVector(BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
        jcall!(self.getJPopupMenu()).add(self.getConditionJPopupMenu());
        jcall!(self.getJPopupMenu()).add(self.getInputJPopupMenu());
        jcall!(self.getJPopupMenu()).add(self.getOutputJPopupMenu());
    }
    

    pub fn getConditionJPopupMenu(&mut self) -> Option<Rc<RefCell<JMenu>>> {
        let mut jMenu: Option<Rc<RefCell<JMenu>>> = JMenu::new(NEW_CONDITION.clone());
        let mut jMenuItemColorAt: Option<Rc<RefCell<JMenuItem>>> = JMenuItem::new(NEW_COLOR_AT.clone());
        let mut jMenuItemAlways: Option<Rc<RefCell<JMenuItem>>> = JMenuItem::new(NEW_ALWAYS.clone());
        let mut jMenuItemTimeInterval: Option<Rc<RefCell<JMenuItem>>> = JMenuItem::new(NEW_TIME_INTERVAL.clone());
        jcall!(jMenuItemColorAt).addActionListener(None /* Java this: a &mut self method has no Rc of itself */);
        jcall!(jMenuItemAlways).addActionListener(None /* Java this: a &mut self method has no Rc of itself */);
        jcall!(jMenuItemTimeInterval).addActionListener(None /* Java this: a &mut self method has no Rc of itself */);
        jcall!(jMenu).add(jMenuItemColorAt.clone());
        jcall!(jMenu).add(jMenuItemAlways.clone());
        jcall!(jMenu).add(jMenuItemTimeInterval.clone());
        return jMenu.clone();
    }
    

    pub fn getInputJPopupMenu(&mut self) -> Option<Rc<RefCell<JMenu>>> {
        let mut jMenu: Option<Rc<RefCell<JMenu>>> = JMenu::new(NEW_INPUT.clone());
        let mut jMenuItemKey: Option<Rc<RefCell<JMenuItem>>> = JMenuItem::new(NEW_KEY_INPUT.clone());
        let mut jMenuItemMouse: Option<Rc<RefCell<JMenuItem>>> = JMenuItem::new(NEW_MOUSE_INPUT.clone());
        let mut jMenuItemJoystick: Option<Rc<RefCell<JMenuItem>>> = JMenuItem::new(NEW_JOYSTICK_INPUT.clone());
        jcall!(jMenuItemKey).addActionListener(None /* Java this: a &mut self method has no Rc of itself */);
        jcall!(jMenuItemMouse).addActionListener(None /* Java this: a &mut self method has no Rc of itself */);
        jcall!(jMenuItemJoystick).addActionListener(None /* Java this: a &mut self method has no Rc of itself */);
        jcall!(jMenu).add(jMenuItemKey.clone());
        jcall!(jMenu).add(jMenuItemMouse.clone());
        jcall!(jMenu).add(jMenuItemJoystick.clone());
        return jMenu.clone();
    }
    

    pub fn getOutputJPopupMenu(&mut self) -> Option<Rc<RefCell<JMenu>>> {
        let mut jMenu: Option<Rc<RefCell<JMenu>>> = JMenu::new(NEW_OUTPUT.clone());
        let mut jMenuItemImage: Option<Rc<RefCell<JMenuItem>>> = JMenuItem::new(NEW_IMAGE_OUTPUT.clone());
        jcall!(jMenuItemImage).addActionListener(None /* Java this: a &mut self method has no Rc of itself */);
        jcall!(jMenu).add(jMenuItemImage.clone());
        return jMenu.clone();
    }
    

    pub fn addProcessor(&mut self, mut profileActionProcessorInterface: Option<Rc<RefCell<dyn ProfileActionScriptProcessorInterface>>>) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("addInput"));
        jcall!(self.getProfileActionProcessorInterfaceVector()).add(profileActionProcessorInterface.clone());
        self.add(profileActionProcessorInterface.clone());
    }
    

    pub fn removeProcessor(&mut self, mut profileActionProcessorInterface: Option<Rc<RefCell<dyn ProfileActionScriptProcessorInterface>>>) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("removeInput"));
        jcall!(self.getProfileActionProcessorInterfaceVector()).remove(profileActionProcessorInterface.clone());
        self.remove(profileActionProcessorInterface.clone());
    }
    

    pub fn removeCondition(&mut self, mut profileActionScriptNodeInterface: Option<Rc<RefCell<dyn ProfileActionScriptNodeInterface>>>) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("removeCondition"));
        jcall!(self.getProfileActionConditionInterfaceVector()).remove(profileActionScriptNodeInterface.clone());
        self.remove(profileActionScriptNodeInterface.clone());
    }
    

    pub fn addCondition(&mut self, mut profileActionScriptNodeInterface: Option<Rc<RefCell<dyn ProfileActionScriptNodeInterface>>>) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("addCondition"));
        jcall!(self.getProfileActionConditionInterfaceVector()).add(profileActionScriptNodeInterface.clone());
        self.add(profileActionScriptNodeInterface.clone());
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(unsafe { GenericProfileActionScriptConditionData_NAME.clone() }.clone());
        let mut profileActionConditionInterfaceVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getProfileActionConditionInterfaceVector();
        let mut size: i32 = jcall!(profileActionConditionInterfaceVector).size();
        for index in (0..size) {
            let mut profileActionConditionInterface: Option<Rc<RefCell<dyn ProfileActionScriptConditionInterface>>> = { let java_object = jcall!(profileActionConditionInterfaceVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn ProfileActionScriptConditionInterface>>>>().expect("ClassCastException").clone()) };
            jcall!(node).appendChild(jcall!(profileActionConditionInterface).toXmlNode(document.clone()));
        }
        let mut profileActionProcessorInterfaceVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getProfileActionProcessorInterfaceVector();
        let mut size2: i32 = jcall!(profileActionProcessorInterfaceVector).size();
        for index in (0..size2) {
            let mut profileActionProcessorInterface: Option<Rc<RefCell<dyn ProfileActionScriptProcessorInterface>>> = { let java_object = jcall!(profileActionProcessorInterfaceVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn ProfileActionScriptProcessorInterface>>>>().expect("ClassCastException").clone()) };
            jcall!(node).appendChild(jcall!(profileActionProcessorInterface).toXmlNode(document.clone()));
        }
        return node.clone();
    }
    

    pub fn getProfileActionConditionInterfaceVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.profileActionConditionInterfaceVector.clone();
    }
    

    pub fn setProfileActionConditionInterfaceVector(&mut self, mut profileActionConditionInterfaceVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        self.profileActionConditionInterfaceVector = profileActionConditionInterfaceVector.clone();
    }
    

    pub fn actionPerformed(&mut self, mut actionEvent: Option<Rc<RefCell<ActionEvent>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.base.actionPerformed(actionEvent.clone());
            if jcall!(jcall!(actionEvent).getActionCommand()).compareTo(DELETE.clone()) == 0 {
                if (self.getParent()).is_some() /* instanceof ProfileActionScriptNodeInterface */ {
                    let mut profileActionScriptNodeInterface: Option<Rc<RefCell<dyn ProfileActionScriptNodeInterface>>> = (self.getParent()).clone() /* as ProfileActionScriptNodeInterface */;
                    jcall!(profileActionScriptNodeInterface).removeCondition(None /* Java this: a &mut self method has no Rc of itself */);
                    jcall!(jcall!(JTreeInterfaceFactory::getInstance()).getJTreeInterface()).updateJTree();
                }
            } else if jcall!(jcall!(actionEvent).getActionCommand()).compareTo(NEW_COLOR_AT.clone()) == 0 {
                self.addCondition(ColorAtActionScriptCondition::new());
                self.updateTree();
            } else if jcall!(jcall!(actionEvent).getActionCommand()).compareTo(NEW_ALWAYS.clone()) == 0 {
                self.addCondition(AlwaysActionScriptCondition::new());
                self.updateTree();
            } else if jcall!(jcall!(actionEvent).getActionCommand()).compareTo(NEW_TIME_INTERVAL.clone()) == 0 {
                self.addCondition(TimeIntervalActionScriptCondition::new());
                self.updateTree();
            } else if jcall!(jcall!(actionEvent).getActionCommand()).compareTo(NEW_KEY_INPUT.clone()) == 0 {
                self.addProcessor(KeyboardActionScriptInput::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::input::automation::actions::script::condition::processors::input::KeyboardActionScriptInput::KeyboardActionScriptInputDyn>>));
                self.updateTree();
            } else if jcall!(jcall!(actionEvent).getActionCommand()).compareTo(NEW_MOUSE_INPUT.clone()) == 0 {
                self.addProcessor(MouseActionScriptInput::new());
                self.updateTree();
            } else if jcall!(jcall!(actionEvent).getActionCommand()).compareTo(NEW_IMAGE_OUTPUT.clone()) == 0 {
                self.addProcessor(ImageActionScriptOutput::new());
                self.updateTree();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(String::from("Error"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("actionPerformed"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn getProfileActionProcessorInterfaceVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.profileActionProcessorInterfaceVector.clone();
    }
    

    pub fn setProfileActionProcessorInterfaceVector(&mut self, mut profileActionProcessorInterfaceVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        self.profileActionProcessorInterfaceVector = profileActionProcessorInterfaceVector.clone();
    }
    

    pub fn updateTree(&mut self) {
        jcall!(jcall!(JTreeInterfaceFactory::getInstance()).getJTreeInterface()).updateJTree();
        let mut treePath: Option<Rc<RefCell<TreePath>>> = TreePath::new(None /* Java this: a &mut self method has no Rc of itself */);
        jcall!(jcall!(jcall!(JTreeInterfaceFactory::getInstance()).getJTreeInterface()).getActionScriptJTree()).expandPath(treePath.clone());
    }
    

    // throws Exception
    pub fn shouldProcess(&mut self, mut frame: Option<i64>) -> bool {
        let mut profileActionConditionInterfaceVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getProfileActionConditionInterfaceVector();
        let mut size: i32 = jcall!(profileActionConditionInterfaceVector).size();
        for index in (0..size) {
            let mut profileActionScriptConditionInterface: Option<Rc<RefCell<dyn ProfileActionScriptConditionInterface>>> = { let java_object = jcall!(profileActionConditionInterfaceVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn ProfileActionScriptConditionInterface>>>>().expect("ClassCastException").clone()) };
            if !jcall!(profileActionScriptConditionInterface).shouldProcess(frame.clone()) {
                return false;
            }
        }
        return true;
    }
    

    // throws Exception
    pub fn process(&mut self, mut frame: Option<i64>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Start - Processing ")).appendint(jcall!(self.getProfileActionProcessorInterfaceVector()).size()).append(String::from(" inputs")).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).PROCESS.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut profileActionProcessorInterfaceVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getProfileActionProcessorInterfaceVector();
        let mut size: i32 = jcall!(profileActionProcessorInterfaceVector).size();
        for index in (0..size) {
            let mut profileActionScriptProcessorInterface: Option<Rc<RefCell<dyn ProfileActionScriptProcessorInterface>>> = { let java_object = jcall!(profileActionProcessorInterfaceVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn ProfileActionScriptProcessorInterface>>>>().expect("ClassCastException").clone()) };
            jcall!(profileActionScriptProcessorInterface).process(frame.clone());
        }
    }
}
impl std::ops::Deref for BasicProfileActionScriptCondition {
    type Target = ProfileActionScriptItem;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicProfileActionScriptCondition {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait BasicProfileActionScriptConditionDyn: crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItemDyn {
    fn as_BasicProfileActionScriptCondition(&self) -> &BasicProfileActionScriptCondition;
    fn as_BasicProfileActionScriptCondition_mut(&mut self) -> &mut BasicProfileActionScriptCondition;
    fn addProcessorNodes(&mut self, mut node: Option<Rc<RefCell<Node>>>) { BasicProfileActionScriptCondition::addProcessorNodes(self.as_BasicProfileActionScriptCondition_mut(), node) }
    fn getConditionJPopupMenu(&mut self) -> Option<Rc<RefCell<JMenu>>> { BasicProfileActionScriptCondition::getConditionJPopupMenu(self.as_BasicProfileActionScriptCondition_mut()) }
    fn getInputJPopupMenu(&mut self) -> Option<Rc<RefCell<JMenu>>> { BasicProfileActionScriptCondition::getInputJPopupMenu(self.as_BasicProfileActionScriptCondition_mut()) }
    fn getOutputJPopupMenu(&mut self) -> Option<Rc<RefCell<JMenu>>> { BasicProfileActionScriptCondition::getOutputJPopupMenu(self.as_BasicProfileActionScriptCondition_mut()) }
    fn addProcessor(&mut self, mut profileActionProcessorInterface: Option<Rc<RefCell<dyn ProfileActionScriptProcessorInterface>>>) { BasicProfileActionScriptCondition::addProcessor(self.as_BasicProfileActionScriptCondition_mut(), profileActionProcessorInterface) }
    fn removeProcessor(&mut self, mut profileActionProcessorInterface: Option<Rc<RefCell<dyn ProfileActionScriptProcessorInterface>>>) { BasicProfileActionScriptCondition::removeProcessor(self.as_BasicProfileActionScriptCondition_mut(), profileActionProcessorInterface) }
    fn removeCondition(&mut self, mut profileActionScriptNodeInterface: Option<Rc<RefCell<dyn ProfileActionScriptNodeInterface>>>) { BasicProfileActionScriptCondition::removeCondition(self.as_BasicProfileActionScriptCondition_mut(), profileActionScriptNodeInterface) }
    fn addCondition(&mut self, mut profileActionScriptNodeInterface: Option<Rc<RefCell<dyn ProfileActionScriptNodeInterface>>>) { BasicProfileActionScriptCondition::addCondition(self.as_BasicProfileActionScriptCondition_mut(), profileActionScriptNodeInterface) }
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { BasicProfileActionScriptCondition::toXmlNode(self.as_BasicProfileActionScriptCondition_mut(), document) }
    fn getProfileActionConditionInterfaceVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { BasicProfileActionScriptCondition::getProfileActionConditionInterfaceVector(self.as_BasicProfileActionScriptCondition_mut()) }
    fn setProfileActionConditionInterfaceVector(&mut self, mut profileActionConditionInterfaceVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { BasicProfileActionScriptCondition::setProfileActionConditionInterfaceVector(self.as_BasicProfileActionScriptCondition_mut(), profileActionConditionInterfaceVector) }
    fn getProfileActionProcessorInterfaceVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { BasicProfileActionScriptCondition::getProfileActionProcessorInterfaceVector(self.as_BasicProfileActionScriptCondition_mut()) }
    fn setProfileActionProcessorInterfaceVector(&mut self, mut profileActionProcessorInterfaceVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { BasicProfileActionScriptCondition::setProfileActionProcessorInterfaceVector(self.as_BasicProfileActionScriptCondition_mut(), profileActionProcessorInterfaceVector) }
    fn updateTree(&mut self) { BasicProfileActionScriptCondition::updateTree(self.as_BasicProfileActionScriptCondition_mut()) }
    fn shouldProcess(&mut self, mut frame: Option<i64>) -> bool { BasicProfileActionScriptCondition::shouldProcess(self.as_BasicProfileActionScriptCondition_mut(), frame) }
}
impl BasicProfileActionScriptConditionDyn for BasicProfileActionScriptCondition {
    fn as_BasicProfileActionScriptCondition(&self) -> &BasicProfileActionScriptCondition { self }
    fn as_BasicProfileActionScriptCondition_mut(&mut self) -> &mut BasicProfileActionScriptCondition { self }
}

impl crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItemDyn for BasicProfileActionScriptCondition {
    fn as_ProfileActionScriptItem(&self) -> &crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItem { &self.base }
    fn as_ProfileActionScriptItem_mut(&mut self) -> &mut crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItem { &mut self.base }
    fn init(&mut self) { Self::init(self) }
    fn process(&mut self, mut frame: Option<i64>) { Self::process(self, frame) }
    fn actionPerformed(&mut self, mut actionEvent: Option<Rc<RefCell<ActionEvent>>>) { Self::actionPerformed(self, actionEvent) }
}


