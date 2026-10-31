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
// package org.allbinary.game.configuration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Choice::Choice;
// use crate::javax::microedition::lcdui::Command::Command;
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;
// use crate::javax::microedition::lcdui::Item::Item;
// use crate::javax::microedition::lcdui::TextField::TextField;
use crate::org::allbinary::game::commands::GameCommandsFactory::GameCommandsFactory;
use crate::org::allbinary::game::configuration::feature::GameFeatureChoiceGroups::GameFeatureChoiceGroups;
use crate::org::allbinary::game::configuration::feature::GameFeatureFormUtil::GameFeatureFormUtil;
use crate::org::allbinary::game::configuration::feature::GameFeatureItemCommandListener::GameFeatureItemCommandListener;
use crate::org::allbinary::game::configuration::feature::GameFeatureItemStateListener::GameFeatureItemStateListener;
use crate::org::allbinary::game::configuration::feature::SensorFeatureFactory::SensorFeatureFactory;
use crate::org::allbinary::game::configuration::persistance::GameConfigurationPersistanceSingleton::GameConfigurationPersistanceSingleton;
use crate::org::allbinary::game::configuration::persistance::KeyValuePersistance::KeyValuePersistance;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::displayable::command::MyCommandsFactory::MyCommandsFactory;
use crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm;
use crate::org::allbinary::input::gyro::OrientationData::OrientationData;
use crate::org::allbinary::logic::MEUtil::MEUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::HashtableUtil::HashtableUtil;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::GameConfigurationTextInput::GameConfigurationTextInput;
use super::GameConfiguration::GameConfiguration;
use super::GameConfigurationGauge::GameConfigurationGauge;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameOptionsForm {
    pub base: CommandForm,
    pub meUtil: Option<Rc<RefCell<MEUtil>>>,
}

impl crate::org::allbinary::game::displayable::canvas::MenuListener::MenuListener for GameOptionsForm {
    fn open(&mut self) { self.base.open() }
    fn close(&mut self) { self.base.close() }
}

impl GameOptionsForm {
    

    pub fn new_struct(mut commandListener: Option<Rc<RefCell<CommandListener>>>, mut title: String, mut backgrounBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: CommandForm::new_struct(commandListener.clone(), title.clone(), backgrounBasicColor.clone(), foregroundBasicColor.clone()),
            meUtil: None,
        };
        this.meUtil = MEUtil::getInstance();
        // super(...) is in the struct literal above
        jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        this.addConfiguration();
        let mut gameFeatureFormUtil: Option<Rc<RefCell<GameFeatureFormUtil>>> = GameFeatureFormUtil::getInstance();
        this.initCommands(commandListener.clone());
        this.addTextFieldsIfSimulated();
        this
    }

    pub fn new(mut commandListener: Option<Rc<RefCell<CommandListener>>>, mut title: String, mut backgrounBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(commandListener, title, backgrounBasicColor, foregroundBasicColor))));
            jcall!(gameFeatureFormUtil).addChoiceGroup((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(GameFeatureChoiceGroups::getExclusiveInstance()).get(), unsafe { crate::javax::microedition::lcdui::Choice::Choice_EXCLUSIVE.clone() }.clone());
            jcall!(gameFeatureFormUtil).addChoiceGroup((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), jcall!(GameFeatureChoiceGroups::getMultipleInstance()).get(), unsafe { crate::javax::microedition::lcdui::Choice::Choice_MULTIPLE.clone() }.clone());
            this.setItemStateListener(GameFeatureItemStateListener::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone())));
        this
    }
    

    // throws Exception
    pub fn closeClientInformation(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) {
        self.base.close();
        self.save(abeClientInformation.clone());
    }
    

    pub fn addTextFieldsIfSimulated(&mut self) {
        let mut key: String = jcall!(OrientationData::getInstance()).ORIENTATION_SENSOR_INPUT.clone();
        let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = jcall!(GameFeatureChoiceGroups::getExclusiveInstance()).get();
        if hashtable.is_some() {
            let mut listCanBeNull: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(hashtable).get(crate::java_into_object(key.clone()));
            if listCanBeNull.is_some() {
                let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = (listCanBeNull).clone() /* as BasicArrayList */;
                if jcall!(list).contains(crate::java_into_object(jcall!(SensorFeatureFactory::getInstance()).SIMULATED_ORIENTATION_SENSORS.clone())) {
                    self.addTextFields();
                }
            }
        }
    }
    

    pub fn addTextFields(&mut self) {
        let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = GameConfigurationTextInput::getHashtable();
        let mut size: i32 = jcall!(hashtable).size();
        let mut objectArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(HashtableUtil::getInstance()).getKeysAsArray(hashtable.clone());
        let mut gameConfigurationTextInput: Option<Rc<RefCell<GameConfigurationTextInput>>> = None;
        let mut textField: Option<Rc<RefCell<TextField>>> = None;
        for index in (0..size) {
            gameConfigurationTextInput = { let java_object = (jcall!(hashtable).get(crate::java_into_object((objectArray[(index) as usize]).clone() /* as Any */))).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<GameConfigurationTextInput>>>>().expect("ClassCastException").clone()) };
            textField = TextField::new(jcall!(gameConfigurationTextInput).getLabel(), jcall!(gameConfigurationTextInput).getText(), 30, unsafe { crate::javax::microedition::lcdui::TextField::TextField_ANY.clone() }.clone());
            jcall!(self.meUtil).appendItem(None /* Java this: a &mut self method has no Rc of itself */, textField.clone());
        }
    }
    

    pub fn addConfiguration(&mut self) {
        let mut METHOD_NAME: String = String::from("addConfiguration");
        let mut NAME: String = String::from("Name: ");
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(GameConfigurationSingleton::GameConfigurationSingleton::getInstance()).getOptionsBasicArrayList();
        let mut GAUGE_CHANGE: Option<Rc<RefCell<Command>>> = jcall!(MyCommandsFactory::getInstance()).GAUGE_CHANGE.clone();
        let mut stringMaker: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut size: i32 = jcall!(list).size();
        let mut gameConfiguration: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>> = None;
        let mut gauge: Option<Rc<RefCell<GameConfigurationGauge>>> = None;
        for index in (0..size) {
            gameConfiguration = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>>>().expect("ClassCastException").clone()) };
            jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringMaker).append(NAME.clone()).append(jcall!(gameConfiguration).toString()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            gauge = GameConfigurationGauge::new(gameConfiguration.clone());
            jcall!(gauge).setDefaultCommand(GAUGE_CHANGE.clone());
            jcall!(gauge).setItemCommandListener(GameFeatureItemCommandListener::new(None /* Java this: a &mut self method has no Rc of itself */));
            jcall!(self.meUtil).appendItem(None /* Java this: a &mut self method has no Rc of itself */, gauge.clone());
        }
    }
    

    pub fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) {
        let mut gameCommandsFactory: Option<Rc<RefCell<GameCommandsFactory>>> = GameCommandsFactory::getInstance();
        self.removeAllCommands();
        self.addCommand(jcall!(gameCommandsFactory).CLOSE_OPTIONS.clone());
        self.addCommand(jcall!(gameCommandsFactory).DEFAULT_OPTIONS.clone());
        self.setCommandListener(cmdListener.clone());
    }
    

    // throws Exception
    pub fn save(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) {
        let mut size: i32 = self.size();
        let mut item: Option<Rc<RefCell<Item>>> = None;
        for index in (0..size) {
            item = self.get(index.clone());
            if (item).is_some() /* instanceof GameConfigurationGauge */ {
                jcall!(GameConfigurationUtil::GameConfigurationUtil::getInstance()).update((item).clone() /* as GameConfigurationGauge */);
            } else if (item).is_some() /* instanceof TextField */ {
                GameConfigurationTextInput::update((item).clone() /* as TextField */);
            }
        }
        jcall!(GameConfigurationUtil::GameConfigurationUtil::getInstance()).updateCompetitionValue();
        let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = jcall!(StdUtil::getInstance()).createHashtable();
        let mut SCALE: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::GameConfiguration::GameConfigurationDyn>>> = jcall!(GameConfigurationCentral::GameConfigurationCentral::getInstance()).SCALE.clone();
        jcall!(hashtable).put(crate::java_into_object(jcall!(SCALE).getName()), crate::java_into_object(jcall!(jcall!(SCALE).getValue()).toString()));
        let mut keyValuePersistance: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::persistance::KeyValuePersistance::KeyValuePersistanceDyn>>> = GameConfigurationPersistanceSingleton::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::persistance::KeyValuePersistance::KeyValuePersistanceDyn>>);
        jcall!(keyValuePersistance).clear();
        jcall!(keyValuePersistance).loadAll(abeClientInformation.clone());
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(keyValuePersistance).getIds();
        jcall!(keyValuePersistance).save(abeClientInformation.clone(), hashtable.clone());
        let mut size2: i32 = jcall!(list).size();
        let mut integer: Option<i32> = None;
        for index in (0..size2) {
            integer = { let java_object = jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]; java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<i32>().copied().or_else(|| object.downcast_ref::<Option<i32>>().copied().flatten()).expect("ClassCastException") }) };
            jcall!(keyValuePersistance).delete(abeClientInformation.clone(), ((integer).unwrap() as i32));
        }
    }
}
impl std::ops::Deref for GameOptionsForm {
    type Target = CommandForm;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GameOptionsForm {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn for GameOptionsForm {
    fn as_CommandForm(&self) -> &crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm { &self.base }
    fn as_CommandForm_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm { &mut self.base }
    fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) { Self::initCommands(self, cmdListener) }
}


