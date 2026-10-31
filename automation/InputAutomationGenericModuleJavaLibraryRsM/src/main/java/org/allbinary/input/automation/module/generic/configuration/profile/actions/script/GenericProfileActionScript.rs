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
// package org.allbinary.input.automation.module.generic.configuration.profile.actions.script
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
// use crate::javax::swing::tree::DefaultMutableTreeNode::DefaultMutableTreeNode;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::input::automation::actions::script::ProfileActionScriptNodeInterface::ProfileActionScriptNodeInterface;
use crate::org::allbinary::input::automation::actions::script::condition::ProfileActionScriptConditionFactory::ProfileActionScriptConditionFactory;
use crate::org::allbinary::input::automation::actions::script::condition::ProfileActionScriptConditionInterface::ProfileActionScriptConditionInterface;
use crate::org::allbinary::input::automation::module::generic::configuration::profile::actions::GenericProfileActionData::GenericProfileActionData;
use crate::org::allbinary::input::automation::module::generic::configuration::profile::actions::GenericProfileActionJPanel::GenericProfileActionJPanel;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GenericProfileActionScript {
    pub base: DefaultMutableTreeNode,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub genericProfileActionJPanel: Option<Rc<RefCell<GenericProfileActionJPanel>>>,
    pub profileActionConditionInterfaceVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

thread_local! { pub static GenericProfileActionScript_NAME: String = String::from("Root"); }
// impl ProfileActionScriptNodeInterface for GenericProfileActionScript with the methods of the trait

impl GenericProfileActionScript {
    

    pub fn new_struct2(mut genericProfileActionJPanel: Option<Rc<RefCell<GenericProfileActionJPanel>>>, mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            base: DefaultMutableTreeNode::new_struct1(GenericProfileActionScript_NAME.with(Clone::clone).clone()),
            logUtil: None,
            commonStrings: None,
            genericProfileActionJPanel: None,
            profileActionConditionInterfaceVector: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        // super(...) is in the struct literal above
        this.genericProfileActionJPanel = genericProfileActionJPanel.clone();
        this.setProfileActionConditionInterfaceVector(BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
        let mut actionScriptNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(unsafe { crate::org::allbinary::input::automation::module::generic::configuration::profile::actions::GenericProfileActionData::GenericProfileActionData_SCRIPT.clone() }.clone(), jcall!(node).getChildNodes());
        if actionScriptNode.is_some() {
            let mut nodeList: Option<Rc<RefCell<NodeList>>> = jcall!(actionScriptNode).getChildNodes();
            for index in (0..jcall!(nodeList).getLength()) {
                let mut actionConditionNode: Option<Rc<RefCell<Node>>> = jcall!(nodeList).item(index.clone());
                if match (&jcall!(actionConditionNode).getNodeType(), &unsafe { crate::org::w3c::dom::Node::Node_ELEMENT_NODE.clone() }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    { let java_arguments = (ProfileActionScriptConditionFactory::getInstance(actionConditionNode.clone()),); this.addCondition(java_arguments.0) };
                }
            }
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Profile Action Script Node Null"))));
        }
        this
    }

    pub fn new2(mut genericProfileActionJPanel: Option<Rc<RefCell<GenericProfileActionJPanel>>>, mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(genericProfileActionJPanel, node))))
    }
    

    pub fn new_struct1(mut genericProfileActionJPanel: Option<Rc<RefCell<GenericProfileActionJPanel>>>) -> Self {
        let mut this = Self {
            base: DefaultMutableTreeNode::new_struct(),
            logUtil: None,
            commonStrings: None,
            genericProfileActionJPanel: None,
            profileActionConditionInterfaceVector: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.genericProfileActionJPanel = genericProfileActionJPanel.clone();
        this.setProfileActionConditionInterfaceVector(BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
        this
    }

    pub fn new1(mut genericProfileActionJPanel: Option<Rc<RefCell<GenericProfileActionJPanel>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(genericProfileActionJPanel))))
    }
    

    pub fn addCondition(&mut self, mut profileActionScriptNodeInterface: Option<Rc<RefCell<dyn ProfileActionScriptNodeInterface>>>) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("addCondition"));
        jcall!(self.getProfileActionConditionInterfaceVector()).add(profileActionScriptNodeInterface.clone());
        self.add(profileActionScriptNodeInterface.clone());
    }
    

    pub fn removeCondition(&mut self, mut profileActionScriptNodeInterface: Option<Rc<RefCell<dyn ProfileActionScriptNodeInterface>>>) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("removeCondition"));
        jcall!(self.getProfileActionConditionInterfaceVector()).remove(profileActionScriptNodeInterface.clone());
        self.remove(profileActionScriptNodeInterface.clone());
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("HashMap: "), jcall!(hashMap).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toHashMap()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        return hashMap.clone();
    }
    

    pub fn getProfileActionConditionInterfaceVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.profileActionConditionInterfaceVector.clone();
    }
    

    pub fn setProfileActionConditionInterfaceVector(&mut self, mut profileActionConditionInterfaceVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        self.profileActionConditionInterfaceVector = profileActionConditionInterfaceVector.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(unsafe { crate::org::allbinary::input::automation::module::generic::configuration::profile::actions::GenericProfileActionData::GenericProfileActionData_SCRIPT.clone() }.clone());
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getProfileActionConditionInterfaceVector();
        let mut size: i32 = jcall!(vector).size();
        for index in (0..size) {
            let mut profileActionConditionInterface: Option<Rc<RefCell<dyn ProfileActionScriptConditionInterface>>> = { let java_object = jcall!(vector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn ProfileActionScriptConditionInterface>>>>().expect("ClassCastException").clone()) };
            jcall!(node).appendChild(jcall!(profileActionConditionInterface).toXmlNode(document.clone()));
        }
        return node.clone();
    }
}
impl std::ops::Deref for GenericProfileActionScript {
    type Target = DefaultMutableTreeNode;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GenericProfileActionScript {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



