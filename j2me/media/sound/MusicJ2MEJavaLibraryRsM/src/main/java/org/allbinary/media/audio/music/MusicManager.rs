// Generated Code Do Not Modify
// package org.allbinary.media.audio.music
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::thread::ARunnable::ARunnable;
use crate::javax::microedition::media::Player::Player;
use crate::javax::microedition::media::PlayerListener::PlayerListener;
use crate::org::allbinary::AvianUtil::AvianUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::media::audio::NoSound::NoSound;
use crate::org::allbinary::media::audio::PlayerStateUtil::PlayerStateUtil;
use crate::org::allbinary::media::audio::Sound::Sound;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::time::GameTickTimeDelayHelper::GameTickTimeDelayHelper;
use crate::org::allbinary::time::GameTickTimeDelayHelperFactory::GameTickTimeDelayHelperFactory;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;
use crate::java::lang::Thread::*;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MusicManager {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub commonSeps: Option<Rc<RefCell<CommonSeps>>>,
    pub basicArrayListUtil: Option<Rc<RefCell<BasicArrayListUtil>>>,
    pub gameTickTimeDelayHelper: Option<Rc<RefCell<GameTickTimeDelayHelper>>>,
    pub playerStateUtil: Option<Rc<RefCell<PlayerStateUtil>>>,
    pub timeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub playerListener: Option<Rc<RefCell<dyn PlayerListener>>>,
    pub PLAY: String,
    pub SONG: String,
    pub NEXT_SONG: String,
    pub STOPPING: String,
    pub ENDING: String,
    pub WAITING_FOR_MEDIA_TO_END: String,
    pub ALREADY_PLAYING: String,
    pub ALREADY_ENDED: String,
    pub songList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub currentSongSound: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>,
    pub nextSongSound: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>,
    pub reset: bool,
    pub noDuration: bool,
    pub stopped: bool,
}


impl MusicManager {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct(mut musicServiceClass: &'static str, mut songList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Self {
        let mut this = Self {
            commonStrings: None,
            commonSeps: None,
            basicArrayListUtil: None,
            gameTickTimeDelayHelper: None,
            playerStateUtil: None,
            timeDelayHelper: None,
            playerListener: None,
            PLAY: String::new(),
            SONG: String::new(),
            NEXT_SONG: String::new(),
            STOPPING: String::new(),
            ENDING: String::new(),
            WAITING_FOR_MEDIA_TO_END: String::new(),
            ALREADY_PLAYING: String::new(),
            ALREADY_ENDED: String::new(),
            songList: None,
            currentSongSound: None,
            nextSongSound: None,
            reset: false,
            noDuration: false,
            stopped: false,
        };
        this.commonStrings = CommonStrings::getInstance();
        this.commonSeps = CommonSeps::getInstance();
        this.basicArrayListUtil = BasicArrayListUtil::getInstance();
        this.gameTickTimeDelayHelper = GameTickTimeDelayHelperFactory::getInstance();
        this.playerStateUtil = PlayerStateUtil::getInstance();
        this.timeDelayHelper = TimeDelayHelper::new(0).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.playerListener = MusicManagerPlayerListener::new().map(|reference| reference as Rc<RefCell<dyn PlayerListener>>);
        this.PLAY = String::from("Play ");
        this.SONG = String::from(" for: ");
        this.NEXT_SONG = format!("{}{}", { let java_field = jcall!(CommonLabels::getInstance()).NEXT.clone(); java_field }, String::from(" Song: "));
        this.STOPPING = String::from("Stopping Current Song: ");
        this.ENDING = String::from("Ending Current Song: ");
        this.WAITING_FOR_MEDIA_TO_END = String::from("Waiting for media to end");
        this.ALREADY_PLAYING = String::from("Already Playing: ");
        this.ALREADY_ENDED = String::from("Last Song already ended: ");
        this.currentSongSound = NoSound::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>);
        this.nextSongSound = NoSound::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>);
        this.stopped = true;
        this.songList = songList.clone();
        this
    }

    pub fn new(mut musicServiceClass: &'static str, mut songList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(musicServiceClass, songList))))
    }
    

    pub fn startNewSong(&mut self) {
        if match (&self.nextSongSound, &NoSound::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut randomSongSound: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>> = { let java_object = (jcall!(self.basicArrayListUtil).getRandom(self.songList.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>>().expect("ClassCastException").clone()) };
            self.nextSong(randomSongSound.clone(), 0, 0);
        }
        self.process();
    }
    

    pub fn nextSong(&mut self, mut nextSongSound: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>, mut leftVolume: i32, mut rightVolume: i32) {
        PreLogUtil::put(jcall!(StringMaker::new()).append(self.NEXT_SONG.clone()).append(jcall!(nextSongSound).as_Sound_mut().getResource()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).PROCESS.clone(); java_field }.clone());
        self.nextSongSound = nextSongSound.clone();
        self.reset();
        self.stopped = false;
    }
    

    pub fn reset(&mut self) {
        self.reset = true;
    }
    

    pub fn process(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if self.stopped {
                return;
            }
            if jcall!(self.songList).size() == 0 {
                return;
            }
            if (jcall!(self.timeDelayHelper).isTime({ let java_field = jcall!(self.gameTickTimeDelayHelper).startTime.clone(); java_field }.clone()) && !self.noDuration) || self.reset {
                self.reset = false;
                self.noDuration = false;
                let mut endingCurrentSongSound: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>> = self.currentSongSound.clone();
                let mut nextSongSound: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>> = self.nextSongSound.clone();
                self.nextSongSound = { let java_value = (NoSound::getInstance()).clone(); java_value }.map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>);
                if match (&nextSongSound, &NoSound::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    self.currentSongSound = { let java_value = ({ let java_object = (jcall!(BasicArrayListUtil::getInstance()).getRandom(self.songList.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>>().expect("ClassCastException").clone()) }).clone(); java_value };
                } else {
                    self.currentSongSound = nextSongSound.clone();
                }
                let mut startingCurrentSongSound: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>> = self.currentSongSound.clone();
                let mut duration: i64 = ((jcall!(self.currentSongSound).getDuration()) as i64);
                jcall!(self.timeDelayHelper).as_TimeDelayHelper_mut().delay = ((duration) as i32);
                if duration <= 0_i64 {
                    let mut NO_DURATION_FOR: String = String::from("No Duration for: ");
                    PreLogUtil::put(jcall!(StringMaker::new()).append(NO_DURATION_FOR.clone()).append(jcall!(self.currentSongSound).as_Sound_mut().getResource()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).PROCESS.clone(); java_field }.clone());
                    jcall!(jcall!(self.currentSongSound).as_Sound_mut().getPlayerP()).addPlayerListener(self.playerListener.clone());
                    self.noDuration = true;
                }
                if !match (&endingCurrentSongSound, &NoSound::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    if match (&endingCurrentSongSound, &startingCurrentSongSound) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } && jcall!(jcall!(endingCurrentSongSound).as_Sound_mut().getPlayerP()).getState() == crate::javax::microedition::media::Player::Player_STARTED.with(Clone::clone) {
                        PreLogUtil::put(jcall!(StringMaker::new()).append(self.ALREADY_PLAYING.clone()).append(jcall!(endingCurrentSongSound).as_Sound_mut().getResource()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).PROCESS.clone(); java_field }.clone());
                        PreLogUtil::put(jcall!(StringMaker::new()).append(self.STOPPING.clone()).append(jcall!(endingCurrentSongSound).as_Sound_mut().getResource()).append(self.SONG.clone()).appendlong(duration.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).PROCESS.clone(); java_field }.clone());
                        jcall!(jcall!(endingCurrentSongSound).as_Sound_mut().getPlayerP()).stop();
                        self.waitForStateChange(endingCurrentSongSound.clone(), startingCurrentSongSound.clone());
                    } else {
                        if jcall!(jcall!(endingCurrentSongSound).as_Sound_mut().getPlayerP()).getState() == crate::javax::microedition::media::Player::Player_STARTED.with(Clone::clone) {
                            PreLogUtil::put(jcall!(StringMaker::new()).append(self.STOPPING.clone()).append(jcall!(endingCurrentSongSound).as_Sound_mut().getResource()).append(self.SONG.clone()).appendlong(duration.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).PROCESS.clone(); java_field }.clone());
                            jcall!(jcall!(endingCurrentSongSound).as_Sound_mut().getPlayerP()).stop();
                            self.waitForStateChange(endingCurrentSongSound.clone(), startingCurrentSongSound.clone());
                        } else {
                            PreLogUtil::put(jcall!(StringMaker::new()).append(self.ALREADY_ENDED.clone()).append(self.PLAY.clone()).append(jcall!(startingCurrentSongSound).as_Sound_mut().getResource()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).PROCESS.clone(); java_field }.clone());
                            if AvianUtil::isAvian() {
                                jcall!(jcall!(endingCurrentSongSound).as_Sound_mut().getPlayerP()).stop();
                                jcall!(jcall!(startingCurrentSongSound).as_Sound_mut().getPlayerP()).stop();
                            }
                            jcall!(jcall!(startingCurrentSongSound).as_Sound_mut().getPlayerP()).start();
                        }
                    }
                } else {
                    PreLogUtil::put(jcall!(StringMaker::new()).append(self.PLAY.clone()).append(jcall!(self.currentSongSound).as_Sound_mut().getResource()).append(self.SONG.clone()).appendlong(duration.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).PROCESS.clone(); java_field }.clone());
                    jcall!(jcall!(self.currentSongSound).as_Sound_mut().getPlayerP()).start();
                }
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut resource: String = { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone();
            if !match (&self.currentSongSound, &NoSound::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                resource = { let java_value = (jcall!(self.currentSongSound).as_Sound_mut().getResource()).clone(); java_value };
            }
            PreLogUtil::putOE(format!("{}{}", { let java_field = jcall!(self.commonStrings).EXCEPTION_LABEL.clone(); java_field }, resource), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).PROCESS.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn waitForStateChange(&mut self, mut endingCurrentSongSound: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>, mut startingCurrentSongSound: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>) {
        while jcall!(jcall!(endingCurrentSongSound).as_Sound_mut().getPlayerP()).getState() == crate::javax::microedition::media::Player::Player_STARTED.with(Clone::clone) {
            PreLogUtil::put(self.WAITING_FOR_MEDIA_TO_END.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).PROCESS.clone(); java_field }.clone());
            std::thread::sleep(std::time::Duration::from_millis((100) as u64));
        }
        PreLogUtil::put(jcall!(StringMaker::new()).append({ let java_receiver = self.playerStateUtil.clone(); let java_arguments = (jcall!(jcall!(endingCurrentSongSound).as_Sound_mut().getPlayerP()).getState(),); let java_result = jcall!(java_receiver).convert(java_arguments.0); java_result }).append({ let java_field = jcall!(self.commonSeps).SPACE.clone(); java_field }.clone()).append(self.PLAY.clone()).append(jcall!(startingCurrentSongSound).as_Sound_mut().getResource()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).PROCESS.clone(); java_field }.clone());
        jcall!(jcall!(startingCurrentSongSound).as_Sound_mut().getPlayerP()).start();
    }
    

    // throws Exception
    pub fn stop(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut currentSongSound: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>> = self.currentSongSound.clone();
            if !match (&currentSongSound, &NoSound::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                self.stopped = true;
                PreLogUtil::put(jcall!(StringMaker::new()).append(self.ENDING.clone()).append(jcall!(currentSongSound).as_Sound_mut().getResource()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).PROCESS.clone(); java_field }.clone());
                jcall!(jcall!(currentSongSound).as_Sound_mut().getPlayerP()).stop();
            }
            jcall!(self.timeDelayHelper).setStartTime(0);
            PreLogUtil::put(jcall!(StringMaker::new()).append({ let java_field = jcall!(self.commonStrings).END.clone(); java_field }.clone()).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(currentSongSound.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).END.clone(); java_field }.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut resource: String = { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone();
            if !match (&currentSongSound, &NoSound::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                resource = { let java_value = (jcall!(currentSongSound).as_Sound_mut().getResource()).clone(); java_value };
            }
            PreLogUtil::putOE(format!("{}{}", { let java_field = jcall!(self.commonStrings).EXCEPTION_LABEL.clone(); java_field }, resource), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).END.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
}

//Handle Media ending for Avian
pub struct MusicManagerPlayerListener {
}

impl PlayerListener for MusicManagerPlayerListener {
    fn playerUpdate(&mut self, mut player: Option<Rc<RefCell<dyn Player>>>, mut event: String, mut eventData: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { Self::playerUpdate(self, player, event, eventData) }
}

impl MusicManagerPlayerListener {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
        

        pub fn playerUpdate(&mut self, mut player: Option<Rc<RefCell<dyn Player>>>, mut event: String, mut eventData: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
            PreLogUtil::put(event.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).PROCESS.clone(); java_field }.clone());
            if event == crate::javax::microedition::media::PlayerListener::PlayerListener_END_OF_MEDIA.with(Clone::clone) || event == crate::javax::microedition::media::PlayerListener::PlayerListener_STOPPED.with(Clone::clone) || event == crate::javax::microedition::media::PlayerListener::PlayerListener_CLOSED.with(Clone::clone) {
                self.reset();
            }
        }
}

