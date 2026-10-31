// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
// package org.allbinary.game.layer.waypoint
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use super::WaypointRunnableLogHelper::WaypointRunnableLogHelper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WaypointRunnableSelectedLogHelper {
    pub base: WaypointRunnableLogHelper,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

thread_local! { pub static WaypointRunnableSelectedLogHelper_instanceC: Option<Rc<RefCell<WaypointRunnableSelectedLogHelper>>> = WaypointRunnableSelectedLogHelper::new(); }

impl WaypointRunnableSelectedLogHelper {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: WaypointRunnableLogHelper::new_struct(),
            logUtil: None,
            commonStrings: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<WaypointRunnableSelectedLogHelper>>> {
        return WaypointRunnableSelectedLogHelper_instanceC.with(Clone::clone).clone();
    }
    

    pub fn start(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(pathFindingLayerInterface).getName()).append(jcall!(self.commonStrings).START_RUNNABLE.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn end(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(pathFindingLayerInterface).getName()).append(jcall!(self.commonStrings).END_RUNNABLE.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
}
impl std::ops::Deref for WaypointRunnableSelectedLogHelper {
    type Target = WaypointRunnableLogHelper;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for WaypointRunnableSelectedLogHelper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::waypoint::WaypointRunnableLogHelper::WaypointRunnableLogHelperDyn for WaypointRunnableSelectedLogHelper {
    fn as_WaypointRunnableLogHelper(&self) -> &crate::org::allbinary::game::layer::waypoint::WaypointRunnableLogHelper::WaypointRunnableLogHelper { &self.base }
    fn as_WaypointRunnableLogHelper_mut(&mut self) -> &mut crate::org::allbinary::game::layer::waypoint::WaypointRunnableLogHelper::WaypointRunnableLogHelper { &mut self.base }
    fn start(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::start(self, pathFindingLayerInterface) }
    fn end(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::end(self, pathFindingLayerInterface) }
}


