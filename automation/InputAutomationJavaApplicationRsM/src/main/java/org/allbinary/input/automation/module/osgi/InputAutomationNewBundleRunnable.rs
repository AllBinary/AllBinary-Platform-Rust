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
// package org.allbinary.input.automation.module.osgi
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Runnable;
use std::collections::HashMap;
use super::*;
// use crate::java::io::File::File;
// use crate::java::io::FileFilter::FileFilter;
// use crate::java::io::FileInputStream::FileInputStream;
// use crate::java::net::URL::URL;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::Set::Set;
// use crate::java::util::jar::JarInputStream::JarInputStream;
// use crate::java::util::jar::Manifest::Manifest;
use crate::org::osgi::framework::Bundle::Bundle;
use crate::org::osgi::framework::BundleContext::BundleContext;
use crate::org::osgi::framework::Constants::Constants;
use crate::bundle::input::automation::InputAutomationBundleActivator::InputAutomationBundleActivator;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::file::FileWrapperUtil::FileWrapperUtil;
use crate::org::allbinary::logic::io::file::directory::SubDirectory::SubDirectory;
use crate::org::allbinary::logic::io::file::filter::BasicFileFilterUtil::BasicFileFilterUtil;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::thread::RunnableInterface::RunnableInterface;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::java::lang::Thread::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputAutomationNewBundleRunnable {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub inputAutomationBundleActivator: Option<Rc<RefCell<InputAutomationBundleActivator>>>,
    pub running: bool,
    pub fileBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

thread_local! { pub static InputAutomationNewBundleRunnable_FILE: String = String::from("file:"); }
thread_local! { pub static InputAutomationNewBundleRunnable_JAR_DIR_PROP: String = String::from("org.knopflerfish.gosg.jars"); }
thread_local! { pub static InputAutomationNewBundleRunnable_INPUT_AUTMATION_MODULE_BUNDLE_JAR_PATH: String = String::from("ia/auto"); }
thread_local! { pub static InputAutomationNewBundleRunnable_MODULES_START_LEVEL: i32 = 8; }
impl RunnableInterface for InputAutomationNewBundleRunnable {
    fn isRunning(&mut self) -> bool { Self::isRunning(self) }
    fn setRunning(&mut self, mut running: bool) { Self::setRunning(self, running) }
    fn setThread(&mut self, mut thread: Option<Rc<RefCell<Thread>>>) { Self::setThread(self, thread) }
}
impl crate::Runnable for InputAutomationNewBundleRunnable {
    fn run(&mut self) { Self::run(self) }
}

impl InputAutomationNewBundleRunnable {
    

    pub fn new_struct(mut inputAutomationBundleActivator: Option<Rc<RefCell<InputAutomationBundleActivator>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            inputAutomationBundleActivator: None,
            running: false,
            fileBasicArrayList: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.inputAutomationBundleActivator = inputAutomationBundleActivator.clone();
        this.fileBasicArrayList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new(mut inputAutomationBundleActivator: Option<Rc<RefCell<InputAutomationBundleActivator>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(inputAutomationBundleActivator))))
    }
    

    // throws Exception
    pub fn setThread(&mut self, mut thread: Option<Rc<RefCell<Thread>>>) {
    }
    

    // synchronized
    pub fn isRunning(&mut self) -> bool {
        return self.running.clone();
    }
    

    // synchronized
    pub fn setRunning(&mut self, mut running: bool) {
        self.running = running.clone();
    }
    

    // throws Exception
    pub fn updateModules(&mut self) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("updateModules"));
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.findNewModules();
        let mut size: i32 = jcall!(list).size();
        let mut bundle: Option<Rc<RefCell<dyn Bundle>>> = None;
        for index in (0..size) {
            bundle = { let java_arguments = ((jcall!(list).get(index.clone())).clone() /* as URL */,); self.install(java_arguments.0) };
            if bundle.is_some() {
                jcall!(bundle).start1(InputAutomationNewBundleRunnable_MODULES_START_LEVEL.with(Clone::clone).clone());
            }
        }
    }
    

    // throws Exception
    pub fn getAllJarSymbolicNameHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getAllJarSymbolicNameHashMap"));
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        let mut jarFileBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getJarModuleFileBasicArrayList();
        jcall!(self.logUtil).putF(format!("{}{}", String::from("Jar Module Files: "), jarFileBasicArrayList), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getAllJarSymbolicNameHashMap"));
        let mut size: i32 = jcall!(jarFileBasicArrayList).size();
        let mut file: Option<Rc<RefCell<File>>> = None;
        for index in (0..size) {
            file = (jcall!(jarFileBasicArrayList).get(index.clone())).clone() /* as File */;
            if !jcall!(file).isDirectory() {
                let mut fileInputStream: Option<Rc<RefCell<FileInputStream>>> = FileInputStream::new(file.clone());
                let mut jarInputStream: Option<Rc<RefCell<JarInputStream>>> = JarInputStream::new(fileInputStream.clone());
                let mut manifest: Option<Rc<RefCell<Manifest>>> = jcall!(jarInputStream).getManifest();
                if manifest.is_none() {
                } else {
                    let mut symbolicName: String = jcall!(jcall!(manifest).getMainAttributes()).getValue(crate::org::osgi::framework::Constants::Constants_BUNDLE_SYMBOLICNAME.with(Clone::clone).clone());
                    if true {
                        jcall!(hashMap).put(symbolicName.clone(), URL::new(format!("{}{}", InputAutomationNewBundleRunnable_FILE.with(Clone::clone), jcall!(file).getAbsolutePath())));
                    }
                }
            }
        }
        return hashMap.clone();
    }
    

    pub fn getJarModuleFileBasicArrayList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getJarModuleFileBasicArrayList"));
        let mut baseJarPath: String = match InputAutomationNewBundleRunnable_JAR_DIR_PROP.with(Clone::clone).clone().as_str() { "os.name" => std::env::consts::OS.to_string(), "os.arch" => std::env::consts::ARCH.to_string(), "user.home" => std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")).unwrap_or_default(), _ => String::new() };
        if baseJarPath.starts_with(&InputAutomationNewBundleRunnable_FILE.with(Clone::clone).clone()) {
            baseJarPath = baseJarPath[((InputAutomationNewBundleRunnable_FILE.with(Clone::clone).chars().count() as i32)) as usize..].to_string();
        }
        let mut jarFileFilter: Option<Rc<RefCell<FileFilter>>> = BasicFileFilterUtil::getInstance(String::from(".jar"));
        let mut path: String = format!("{}{}", baseJarPath, InputAutomationNewBundleRunnable_INPUT_AUTMATION_MODULE_BUNDLE_JAR_PATH.with(Clone::clone));
        jcall!(self.logUtil).putF(format!("{}{}", String::from("Path: "), path), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getJarModuleFileBasicArrayList"));
        let mut file: Option<Rc<RefCell<File>>> = File::new(path.clone());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", format!("{}{}", format!("{}{}", String::from("File: "), jcall!(file).getAbsolutePath()), String::from(" isDirectory: ")), jcall!(file).isDirectory()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getJarModuleFileBasicArrayList"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        return jcall!(SubDirectory::getInstance()).search2(jarFileFilter.clone(), FileWrapperUtil::wrapFile(file.clone()));
    }
    

    // throws Exception
    pub fn getInstalledJarSymbolicNameBasicArrayList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getInstalledJarSymbolicNameBasicArrayList"));
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut bundleContext: Option<Rc<RefCell<dyn BundleContext>>> = InputAutomationBundleActivator::getBundleContext();
        let mut bundleArray: Vec<Option<Rc<RefCell<dyn Bundle>>>> = jcall!(bundleContext).getBundles();
        if bundleArray.is_some() {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("bundleArray: "), (bundleArray.len() as i32)), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getInputAutomationModuleServices"));
            for index in (0..(bundleArray.len() as i32)) {
                let mut bundle: Option<Rc<RefCell<dyn Bundle>>> = bundleArray[(index) as usize].clone();
                jcall!(vector).add(crate::java_into_object(jcall!(bundle).getSymbolicName()));
            }
        }
        return vector.clone();
    }
    

    // throws Exception
    pub fn isInstalled(&mut self, mut symbolicName: String) -> bool {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", jcall!(CommonLabels::getInstance()).START, symbolicName), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isInstalled"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getInstalledJarSymbolicNameBasicArrayList();
        let mut size: i32 = jcall!(list).size();
        let mut nextSymbolicName: String = String::new();
        for index in (0..size) {
            nextSymbolicName = (jcall!(list).get(index.clone())).clone() /* as String */;
            if (nextSymbolicName.cmp(&symbolicName.clone()) as i32) == 0 {
                return true;
            }
        }
        return false;
    }
    

    // throws Exception
    pub fn findNewModules(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("findNewModules"));
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = self.getAllJarSymbolicNameHashMap();
        jcall!(self.logUtil).putF(format!("{}{}", String::from("All: "), hashMap), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("findNewModules"));
        let mut set: Option<Rc<RefCell<dyn Set>>> = jcall!(hashMap).keySet();
        let mut symbolicNameArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(set).toArray();
        let mut size: i32 = (symbolicNameArray.len() as i32).clone();
        for index in (0..size) {
            let mut symbolicName: String = (symbolicNameArray[(index) as usize]).clone() /* as String */;
            if !self.isInstalled(symbolicName.clone()) {
                jcall!(vector).add(crate::java_into_object(jcall!(hashMap).get(symbolicName.clone())));
            }
        }
        return vector.clone();
    }
    

    // throws Exception
    pub fn install(&mut self, mut url: Option<Rc<RefCell<URL>>>) -> Option<Rc<RefCell<dyn Bundle>>> {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", jcall!(CommonLabels::getInstance()).START, url), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("install"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut bundleContext: Option<Rc<RefCell<dyn BundleContext>>> = InputAutomationBundleActivator::getBundleContext();
        return jcall!(bundleContext).installBundle(jcall!(url).toString());
    }
    

    pub fn run(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
            self.setRunning(true);
            let mut timeHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>> = TimeDelayHelper::new(1000).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
            while self.isRunning() {
                jcall!(timeHelper).setStartTimeTNT();
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", jcall!(CommonLabels::getInstance()).ELAPSED, jcall!(timeHelper).getElapsedTNT()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                self.updateModules();
                break;
            }
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).END.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
}

