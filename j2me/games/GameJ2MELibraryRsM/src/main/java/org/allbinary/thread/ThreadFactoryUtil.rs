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
// package org.allbinary.thread
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Runnable;
use super::*;
use crate::org::allbinary::J2MEUtil::J2MEUtil;
use crate::org::allbinary::canvas::RunnableCanvas::RunnableCanvas;
use crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas;
use crate::org::allbinary::game::displayable::canvas::DemoCanvas::DemoCanvas;
use crate::org::allbinary::game::displayable::canvas::GameCanvasRunnableInterface::GameCanvasRunnableInterface;
use crate::org::allbinary::game::displayable::canvas::RunnableCanvasSingleThreadStartRunnable::RunnableCanvasSingleThreadStartRunnable;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Thread::*;
use super::ABRunnable::ABRunnable;
use super::ThreadPool::ThreadPool;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ThreadFactoryUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static ThreadFactoryUtil_instance: Option<Rc<RefCell<ThreadFactoryUtil>>> = ThreadFactoryUtil::new(); }

impl ThreadFactoryUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ThreadFactoryUtil>>> {
        return ThreadFactoryUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn getInstanceGameCanvasRunnable(&mut self, mut runnable: Option<Rc<RefCell<dyn GameCanvasRunnableInterface>>>) -> Option<Rc<RefCell<Thread>>> {
        return { let java_arguments = (runnable.clone(), jcall!(runnable).getType(),); self.getInstanceType(java_arguments.0, java_arguments.1) };
    }
    

    pub fn getInstanceForRunnable(&mut self, mut runnable: Option<Rc<RefCell<dyn crate::org::allbinary::thread::ABRunnable::ABRunnableDyn>>>) -> Option<Rc<RefCell<Thread>>> {
        return { let java_arguments = (runnable.clone(), jcall!(runnable).getType(),); self.getInstanceType(java_arguments.0, java_arguments.1) };
    }
    

    pub fn getInstanceType(&mut self, mut runnable: Option<Rc<RefCell<dyn Runnable>>>, mut r#type: i32) -> Option<Rc<RefCell<Thread>>> {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        if J2MEUtil::isHTML() {
            if r#type == crate::org::allbinary::game::displayable::canvas::DemoCanvas::DemoCanvas_TYPE.with(Clone::clone) || r#type == crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas_TYPE.with(Clone::clone) {
                let mut demoGameSingleThreadStartRunnable: Option<Rc<RefCell<RunnableCanvasSingleThreadStartRunnable>>> = RunnableCanvasSingleThreadStartRunnable::new((runnable).clone() /* as RunnableCanvas */);
                PreLogUtil::put(jcall!(StringMaker::new()).append(String::from("Using Pseudo Thread for DemoCanvas/AllBinaryGameCanvas under PlayN/HTML5: ")).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(runnable.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).CONSTRUCTOR.clone());
                let mut primaryThreadPool: Option<Rc<RefCell<dyn crate::org::allbinary::thread::ThreadPool::ThreadPoolDyn>>> = PrimaryThreadPool::PrimaryThreadPool::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::thread::ThreadPool::ThreadPoolDyn>>);
                jcall!(primaryThreadPool).runTask(demoGameSingleThreadStartRunnable.clone().map(|reference| reference as Rc<RefCell<dyn Runnable>>));
            } else {
                PreLogUtil::put(jcall!(StringMaker::new()).append(String::from("Using Pseudo Thread for Runnable under PlayN/HTML5: ")).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(runnable.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).CONSTRUCTOR.clone());
                let mut primaryThreadPool: Option<Rc<RefCell<dyn crate::org::allbinary::thread::ThreadPool::ThreadPoolDyn>>> = PrimaryThreadPool::PrimaryThreadPool::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::thread::ThreadPool::ThreadPoolDyn>>);
                jcall!(primaryThreadPool).runTask(runnable.clone());
            }
        }
        let mut thread: Option<Rc<RefCell<Thread>>> = Thread::new2(runnable.clone(), jcall!(runnable).toString());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(thread).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).CONSTRUCTOR.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        return thread.clone();
    }
    

    pub fn start(&mut self, mut thread: Option<Rc<RefCell<Thread>>>) {
        jcall!(thread).start();
    }
}

