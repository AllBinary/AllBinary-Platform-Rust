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
// package org.allbinary.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Runnable;
use super::*;
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::game::displayable::canvas::NullWaitGameRunnable::NullWaitGameRunnable;
use crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas;
use crate::org::allbinary::logic::ABSystemWrapper::ABSystemWrapper;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::thread::NullThread::NullThread;
use crate::org::allbinary::thread::RunnableInterface::RunnableInterface;
use crate::org::allbinary::thread::ThreadObjectUtil::ThreadObjectUtil;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::java::lang::Thread::*;
use super::Processor::Processor;
use super::RunnableCanvasRefreshHelper::RunnableCanvasRefreshHelper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RunnableCanvas {
    pub base: MyCanvas,
    pub nullUtil: Option<Rc<RefCell<NullUtil>>>,
    pub stdUtil: Option<Rc<RefCell<StdUtil>>>,
    pub systemWrapper: Option<Rc<RefCell<ABSystemWrapper>>>,
    pub tsUtil: Option<Rc<RefCell<TsUtil>>>,
    pub thread: Option<Rc<RefCell<Thread>>>,
    pub currentThread: Option<Rc<RefCell<Thread>>>,
    pub running: bool,
    pub loopTimeHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub runnableCanvasRefreshHelper: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>,
    pub commonLabels: Option<Rc<RefCell<CommonLabels>>>,
    pub threadObjectUtil: Option<Rc<RefCell<ThreadObjectUtil>>>,
    pub SET_RUNNING: String,
    pub IS_RUNNING: String,
    pub THREAD: String,
    pub NOT_EQUAL: String,
    pub PAUSE_SLEEP: String,
    pub START_PAUSE: String,
    pub END_PAUSE: String,
    pub PROCESS_LOOP_SLEEP: String,
    pub pauseWait: i64,
    pub notified: bool,
}

impl RunnableInterface for RunnableCanvas {
    fn isRunning(&mut self) -> bool { Self::isRunning(self) }
    fn setRunning(&mut self, mut running: bool) { Self::setRunning(self, running) }
    fn setThread(&mut self, mut thread: Option<Rc<RefCell<Thread>>>) { Self::setThread(self, thread) }
}
impl crate::Runnable for RunnableCanvas {
    fn run(&mut self) { Self::run(self) }
}

impl RunnableCanvas {
    

    pub fn new_struct(mut commandListener: Option<Rc<RefCell<CommandListener>>>, mut childNameList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut hasParam: bool) -> Self {
        let mut this = Self {
            base: MyCanvas::new_struct(jcall!(CommonStrings::getInstance()).UNKNOWN.clone(), childNameList.clone()),
            nullUtil: None,
            stdUtil: None,
            systemWrapper: None,
            tsUtil: None,
            thread: None,
            currentThread: None,
            running: false,
            loopTimeHelper: None,
            runnableCanvasRefreshHelper: None,
            commonLabels: None,
            threadObjectUtil: None,
            SET_RUNNING: String::new(),
            IS_RUNNING: String::new(),
            THREAD: String::new(),
            NOT_EQUAL: String::new(),
            PAUSE_SLEEP: String::new(),
            START_PAUSE: String::new(),
            END_PAUSE: String::new(),
            PROCESS_LOOP_SLEEP: String::new(),
            pauseWait: 0_i64,
            notified: false,
        };
        this.nullUtil = NullUtil::getInstance();
        this.stdUtil = StdUtil::getInstance();
        this.systemWrapper = ABSystemWrapper::getInstance();
        this.tsUtil = TsUtil::getInstance();
        this.thread = crate::org::allbinary::thread::NullThread::NullThread_NULL_THREAD.with(Clone::clone).clone();
        this.currentThread = crate::org::allbinary::thread::NullThread::NullThread_NULL_THREAD.with(Clone::clone).clone();
        this.loopTimeHelper = TimeDelayHelper::new(jcall!(NullWaitGameRunnable::getInstance()).WAIT.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.runnableCanvasRefreshHelper = Processor::getInstance();
        this.commonLabels = CommonLabels::getInstance();
        this.threadObjectUtil = ThreadObjectUtil::getInstance();
        this.SET_RUNNING = String::from("setRunning");
        this.IS_RUNNING = String::from("isRunning");
        this.THREAD = String::from("Thread: ");
        this.NOT_EQUAL = String::from(" != ");
        this.PAUSE_SLEEP = String::from("pause sleep");
        this.START_PAUSE = String::from("start pause - game thread sleep at: ");
        this.END_PAUSE = String::from("end pause - game thread sleep at: ");
        this.PROCESS_LOOP_SLEEP = String::from("processLoopSleep");
        this.pauseWait = 0_i64;
        this.notified = false;
        // super(...) is in the struct literal above
        { let java_receiver = this.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("RunnableCanvas delay: ")).appendint(jcall!(this.loopTimeHelper).as_TimeDelayHelper_mut().delay.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        if commandListener.is_some() {
            this.initCommands(commandListener.clone());
        } else if hasParam {
            jcall!(this.logUtil).putF(String::from("commandListener was null"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("initCommands"));
        }
        this
    }

    pub fn new(mut commandListener: Option<Rc<RefCell<CommandListener>>>, mut childNameList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut hasParam: bool) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(commandListener, childNameList, hasParam))));
            this.runnableCanvasRefreshHelper = RunnableCanvasRefreshHelper::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone())).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>);
        this
    }
    

    pub fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).NOT_IMPLEMENTED.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("initCommands"));
    }
    

    pub fn isMainCanvas(&mut self) -> bool {
        if self.getCustomCommandListener().is_some() {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn setThread(&mut self, mut thread: Option<Rc<RefCell<Thread>>>) {
        self.thread = thread.clone();
    }
    

    pub fn setRunning(&mut self, mut running: bool) {
        self.running = running.clone();
        if !self.running {
            self.thread = crate::org::allbinary::thread::NullThread::NullThread_NULL_THREAD.with(Clone::clone).clone();
            { // synchronized (self)
                jcall!(self.threadObjectUtil).notifyObject(None /* Java this: a &mut self method has no Rc of itself */);
            }
        }
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(self.IS_RUNNING.clone()).appendboolean(self.running.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.SET_RUNNING.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    // synchronized
    pub fn isRunning(&mut self) -> bool {
        if match (&self.thread, &self.currentThread) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return self.running.clone();
        } else {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(self.THREAD.clone());
            if self.thread.is_some() {
                jcall!(stringBuffer).append(jcall!(self.thread).toString());
            }
            jcall!(stringBuffer).append(self.NOT_EQUAL.clone());
            if self.currentThread.is_some() {
                jcall!(stringBuffer).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(self.currentThread.clone())));
            }
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.IS_RUNNING.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            return false;
        }
    }
    

    pub fn getLoopTimeHelperP(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>> {
        return self.loopTimeHelper.clone();
    }
    

    pub fn setWait(&mut self, mut wait: i32) {
        jcall!(self.loopTimeHelper).as_TimeDelayHelper_mut().delay = wait.clone();
        self.pauseWait = ((wait) as i64) * 3_i64;
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("setWait - delay: ")).appendint(jcall!(self.loopTimeHelper).as_TimeDelayHelper_mut().delay.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).CONSTRUCTOR.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn getWait(&mut self) -> i32 {
        return jcall!(self.loopTimeHelper).as_TimeDelayHelper_mut().delay.clone();
    }
    

    pub fn setCurrentThread(&mut self) {
        self.currentThread = Thread::currentThread();
    }
    

    pub fn setCurrentThreadFake(&mut self) {
        self.currentThread = self.thread.clone();
    }
    

    pub fn showNotify(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.stopWaiting();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("showNotify"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn stopWaiting(&mut self) {
        self.notified = true;
        { // synchronized (self)
            jcall!(self.threadObjectUtil).notifyObject(None /* Java this: a &mut self method has no Rc of itself */);
        }
        self.repaint();
    }
    

    // throws Exception
    // synchronized
    pub fn waitOnNotify(&mut self, mut wait: i64) {
        if !self.notified {
            if wait > 0_i64 {
                jcall!(self.threadObjectUtil).waitObjectFor(None /* Java this: a &mut self method has no Rc of itself */, wait);
            } else {
                jcall!(self.threadObjectUtil).waitObject(None /* Java this: a &mut self method has no Rc of itself */);
            }
        }
    }
    

    // throws Exception
    pub fn processSleep(&mut self) {
        std::thread::sleep(std::time::Duration::from_millis((self.pauseWait.clone()) as u64));
    }
    

    pub fn isPausable(&mut self) -> bool {
        return false;
    }
    

    // throws Exception
    pub fn processGameSleep(&mut self, mut sleep: i64) {
        std::thread::sleep(std::time::Duration::from_millis((sleep.clone()) as u64));
    }
    

    // throws Exception
    pub fn processLoopSleep(&mut self) {
        jcall!(self.runnableCanvasRefreshHelper).process();
        if self.isPaused() {
            let mut stringMaker: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            if self.isRunning() && !self.isSingleThread() {
                jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
                let mut currentTimeMillis: i64 = jcall!(self.systemWrapper).currentTimeMillis();
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringMaker).append(self.START_PAUSE.clone()).appendlong(currentTimeMillis.clone()).append(self.PAUSE_SLEEP.clone()).appendlong(self.pauseWait.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.PROCESS_LOOP_SLEEP.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                while self.isPaused() && self.isRunning() && !self.isSingleThread() {
                    self.processSleep();
                    if !self.isPausable() {
                        jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
                        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringMaker).append(self.END_PAUSE.clone()).appendlong(currentTimeMillis.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.PROCESS_LOOP_SLEEP.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                        return;
                    }
                }
            }
        }
        let mut elapsedTime: i64 = jcall!(self.loopTimeHelper).getElapsedTNT();
        let mut wait: i64 = ((jcall!(self.loopTimeHelper).as_TimeDelayHelper_mut().delay) as i64);
        if elapsedTime > wait {
            elapsedTime = wait.clone();
        } else {
            self.processGameSleep(wait - elapsedTime);
        }
    }
    

    pub fn isSingleThread(&mut self) -> bool {
        return false;
    }
    

    pub fn run(&mut self) {
        self.setCurrentThread();
    }
    

    // throws Exception
    pub fn end2(&mut self) {
    }
    

    pub fn getType(&mut self) -> i32 {
        return -1;
    }
}
impl std::ops::Deref for RunnableCanvas {
    type Target = MyCanvas;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RunnableCanvas {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait RunnableCanvasDyn: crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn {
    fn as_RunnableCanvas(&self) -> &RunnableCanvas;
    fn as_RunnableCanvas_mut(&mut self) -> &mut RunnableCanvas;
    fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) { RunnableCanvas::initCommands(self.as_RunnableCanvas_mut(), cmdListener) }
    fn isMainCanvas(&mut self) -> bool { RunnableCanvas::isMainCanvas(self.as_RunnableCanvas_mut()) }
    fn setThread(&mut self, mut thread: Option<Rc<RefCell<Thread>>>) { RunnableCanvas::setThread(self.as_RunnableCanvas_mut(), thread) }
    fn setRunning(&mut self, mut running: bool) { RunnableCanvas::setRunning(self.as_RunnableCanvas_mut(), running) }
    fn isRunning(&mut self) -> bool { RunnableCanvas::isRunning(self.as_RunnableCanvas_mut()) }
    fn getLoopTimeHelperP(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>> { RunnableCanvas::getLoopTimeHelperP(self.as_RunnableCanvas_mut()) }
    fn setWait(&mut self, mut wait: i32) { RunnableCanvas::setWait(self.as_RunnableCanvas_mut(), wait) }
    fn getWait(&mut self) -> i32 { RunnableCanvas::getWait(self.as_RunnableCanvas_mut()) }
    fn setCurrentThread(&mut self) { RunnableCanvas::setCurrentThread(self.as_RunnableCanvas_mut()) }
    fn setCurrentThreadFake(&mut self) { RunnableCanvas::setCurrentThreadFake(self.as_RunnableCanvas_mut()) }
    fn showNotify(&mut self) { RunnableCanvas::showNotify(self.as_RunnableCanvas_mut()) }
    fn stopWaiting(&mut self) { RunnableCanvas::stopWaiting(self.as_RunnableCanvas_mut()) }
    fn waitOnNotify(&mut self, mut wait: i64) { RunnableCanvas::waitOnNotify(self.as_RunnableCanvas_mut(), wait) }
    fn processSleep(&mut self) { RunnableCanvas::processSleep(self.as_RunnableCanvas_mut()) }
    fn isPausable(&mut self) -> bool { RunnableCanvas::isPausable(self.as_RunnableCanvas_mut()) }
    fn processGameSleep(&mut self, mut sleep: i64) { RunnableCanvas::processGameSleep(self.as_RunnableCanvas_mut(), sleep) }
    fn processLoopSleep(&mut self) { RunnableCanvas::processLoopSleep(self.as_RunnableCanvas_mut()) }
    fn isSingleThread(&mut self) -> bool { RunnableCanvas::isSingleThread(self.as_RunnableCanvas_mut()) }
    fn run(&mut self) { RunnableCanvas::run(self.as_RunnableCanvas_mut()) }
    fn end2(&mut self) { RunnableCanvas::end2(self.as_RunnableCanvas_mut()) }
    fn getType(&mut self) -> i32 { RunnableCanvas::getType(self.as_RunnableCanvas_mut()) }
}
impl RunnableCanvasDyn for RunnableCanvas {
    fn as_RunnableCanvas(&self) -> &RunnableCanvas { self }
    fn as_RunnableCanvas_mut(&mut self) -> &mut RunnableCanvas { self }
}

impl crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn for RunnableCanvas {
    fn as_MyCanvas(&self) -> &crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &self.base }
    fn as_MyCanvas_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas { &mut self.base }
}


