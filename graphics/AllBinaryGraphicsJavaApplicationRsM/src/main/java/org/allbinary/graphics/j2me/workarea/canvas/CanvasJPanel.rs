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
// package org.allbinary.graphics.j2me.workarea.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::awt::*;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::swing::tree::DefaultMutableTreeNode::DefaultMutableTreeNode;
// use crate::javax::swing::tree::MutableTreeNode::MutableTreeNode;
use crate::org::allbinary::animation::VectorExplosionGenerator::VectorExplosionGenerator;
use crate::org::allbinary::animation::VectorMirrorGenerator::VectorMirrorGenerator;
use crate::org::allbinary::animation::vector::VectorCenterGenerator::VectorCenterGenerator;
use crate::org::allbinary::game::input::GameInputStrings::GameInputStrings;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::j2me::GraphicsException::GraphicsException;
use crate::org::allbinary::graphics::j2me::StatusFactory::StatusFactory;
use crate::org::allbinary::graphics::j2me::workarea::WorkAreaJTreeJPanel::WorkAreaJTreeJPanel;
use crate::org::allbinary::graphics::j2me::workarea::tools::GraphicItemFactory::GraphicItemFactory;
use crate::org::allbinary::graphics::j2me::workarea::tools::GraphicItemInterface::GraphicItemInterface;
use crate::org::allbinary::graphics::j2me::workarea::tools::LinesGraphicItem::LinesGraphicItem;
use crate::org::allbinary::graphics::j2me::workarea::tools::MouseStrings::MouseStrings;
use crate::org::allbinary::graphics::j2me::workarea::tools::Points::Points;
use crate::org::allbinary::graphics::j2me::workarea::tools::ToolFactory::ToolFactory;
use crate::org::allbinary::graphics::j2me::workarea::tools::event::MyGraphicItemEvent::MyGraphicItemEvent;
use crate::org::allbinary::graphics::j2me::workarea::tools::event::MyGraphicItemEventListener::MyGraphicItemEventListener;
use crate::org::allbinary::graphics::j2me::workarea::tools::event::MyGraphicItemEventService::MyGraphicItemEventService;
use crate::org::allbinary::graphics::pipeline::RandomRotationFactory::RandomRotationFactory;
use crate::org::allbinary::log::LOGGING::LOGGING;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::GuiLog::GuiLog;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::math::PositionStrings::PositionStrings;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::w3c::dom::Node::Node;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CanvasJPanel {
    pub base: JPanel,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub guiLog: Option<Rc<RefCell<GuiLog>>>,
    pub gameInputStrings: Option<Rc<RefCell<GameInputStrings>>>,
    pub selectedTool: Option<Rc<RefCell<dyn GraphicItemInterface>>>,
    pub graphicItemHashMap: Option<Rc<RefCell<HashMap>>>,
    pub canvasDimension: Option<Rc<RefCell<IntegerDimension>>>,
    pub angle: f64,
    pub canvasTreeNode: Option<Rc<RefCell<DefaultMutableTreeNode>>>,
    pub frameLabel: Option<Rc<RefCell<CanvasTreeLabel>>>,
    pub workAreaJTreeJPanel: Option<Rc<RefCell<WorkAreaJTreeJPanel>>>,
    pub grid: Option<Rc<RefCell<Grid>>>,
}

pub static mut CanvasJPanel_frame: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| 0);
thread_local! { pub static CanvasJPanel_gridColor: Option<Rc<RefCell<Color>>> = Color::new(jcall!(jcall!(BasicColorFactory::getInstance()).WHITE).intValue()); }
thread_local! { pub static CanvasJPanel_backgroundColor: Option<Rc<RefCell<Color>>> = Color::new(jcall!(jcall!(BasicColorFactory::getInstance()).CLEAR_COLOR).intValue()); }
// impl MouseListener for CanvasJPanel with the methods of the trait
// impl MouseMotionListener for CanvasJPanel with the methods of the trait
// impl KeyListener for CanvasJPanel with the methods of the trait
impl MyGraphicItemEventListener for CanvasJPanel {
    fn delete(&mut self, mut evt: Option<Rc<RefCell<MyGraphicItemEvent>>>) { Self::delete(self, evt) }
    fn duplicate(&mut self, mut event: Option<Rc<RefCell<MyGraphicItemEvent>>>) { Self::duplicate1(self, event) }
    fn rotate(&mut self, mut event: Option<Rc<RefCell<MyGraphicItemEvent>>>) { Self::rotate(self, event) }
    fn highlight(&mut self, mut evt: Option<Rc<RefCell<MyGraphicItemEvent>>>) { Self::highlight(self, evt) }
    fn deselect(&mut self, mut evt: Option<Rc<RefCell<MyGraphicItemEvent>>>) { Self::deselect(self, evt) }
}

impl CanvasJPanel {
    

    pub fn new_struct4(mut workAreaJTreeJPanel: Option<Rc<RefCell<WorkAreaJTreeJPanel>>>, mut dimension: Option<Rc<RefCell<Dimension>>>, mut x: i32, mut y: i32) -> Self {
        let mut this = Self {
            base: JPanel::new_struct(),
            logUtil: None,
            guiLog: None,
            gameInputStrings: None,
            selectedTool: None,
            graphicItemHashMap: None,
            canvasDimension: None,
            angle: 0.0,
            canvasTreeNode: None,
            frameLabel: None,
            workAreaJTreeJPanel: None,
            grid: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.guiLog = GuiLog::getInstance();
        this.gameInputStrings = GameInputStrings::getInstance();
        this.selectedTool = None;
        this.angle = 0.0;
        this.grid = Grid::new();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            this.workAreaJTreeJPanel = workAreaJTreeJPanel.clone();
            this.initComponents();
            this.initMyComponents();
            this.setSize(dimension.clone());
            this.setCanvasSize(x.clone(), y.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if LOGGING::contains(crate::org::allbinary::log::LOGGING::LOGGING_GRAPHICSCREATION.with(Clone::clone).clone()) {
                jcall!(this.guiLog).put4(String::from("Constructor Error"), (|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), String::from("contructor"), e.clone());
            }
            }
        }
        this
    }

    pub fn new4(mut workAreaJTreeJPanel: Option<Rc<RefCell<WorkAreaJTreeJPanel>>>, mut dimension: Option<Rc<RefCell<Dimension>>>, mut x: i32, mut y: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct4(workAreaJTreeJPanel, dimension, x, y))))
    }
    

    pub fn new_struct3(mut workAreaJTreeJPanel: Option<Rc<RefCell<WorkAreaJTreeJPanel>>>, mut dimension: Option<Rc<RefCell<Dimension>>>, mut canvasDom: Option<Rc<RefCell<CanvasDom>>>) -> Self {
        let mut this = Self {
            base: JPanel::new_struct(),
            logUtil: None,
            guiLog: None,
            gameInputStrings: None,
            selectedTool: None,
            graphicItemHashMap: None,
            canvasDimension: None,
            angle: 0.0,
            canvasTreeNode: None,
            frameLabel: None,
            workAreaJTreeJPanel: None,
            grid: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.guiLog = GuiLog::getInstance();
        this.gameInputStrings = GameInputStrings::getInstance();
        this.selectedTool = None;
        this.angle = 0.0;
        this.grid = Grid::new1(jcall!(canvasDom).getGrid());
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            this.workAreaJTreeJPanel = workAreaJTreeJPanel.clone();
            this.initComponents();
            this.initMyComponents();
            this.setSize(dimension.clone());
            { let java_arguments = (jcall!(jcall!(canvasDom).getDimension()).getWidth(), jcall!(jcall!(canvasDom).getDimension()).getHeight(),); this.setCanvasSize(java_arguments.0, java_arguments.1) };
            this.graphicItemHashMap = jcall!(canvasDom).getGraphicItemHashMap();
            let mut graphicItemArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(jcall!(this.graphicItemHashMap).keySet()).toArray();
            let mut size: i32 = (graphicItemArray.len() as i32).clone();
            for index in (0..size) {
                let mut graphicItemTreeNode: Option<Rc<RefCell<MutableTreeNode>>> = (graphicItemArray[(index) as usize]).clone() /* as MutableTreeNode */;
                jcall!(this.canvasTreeNode).add(graphicItemTreeNode.clone());
            }
            this.repaint();
            { let java_arguments = (jcall!(canvasDom).getAngle(),); this.setAngle(java_arguments.0) };
            jcall!(this.workAreaJTreeJPanel).updateTree();
            jcall!(this.workAreaJTreeJPanel).expand();
            jcall!(this.workAreaJTreeJPanel).repaint();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if LOGGING::contains(crate::org::allbinary::log::LOGGING::LOGGING_GRAPHICSCREATION.with(Clone::clone).clone()) {
                jcall!(this.guiLog).put4(String::from("Constructor Error"), (|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()), String::from("contructor"), e.clone());
            }
            }
        }
        this
    }

    pub fn new3(mut workAreaJTreeJPanel: Option<Rc<RefCell<WorkAreaJTreeJPanel>>>, mut dimension: Option<Rc<RefCell<Dimension>>>, mut canvasDom: Option<Rc<RefCell<CanvasDom>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct3(workAreaJTreeJPanel, dimension, canvasDom))))
    }
    

    // throws Exception
    pub fn initMyComponents(&mut self) {
        self.frameLabel = CanvasTreeLabel::new(format!("{}{}", String::from("FrameLabel"), jcall!(Some((unsafe { CanvasJPanel_frame.clone() }.clone()) as i32)).toString()));
        self.canvasTreeNode = DefaultMutableTreeNode::new(self.frameLabel.clone());
        unsafe { *CanvasJPanel_frame += 1 };
        { let java_receiver = self.workAreaJTreeJPanel.clone(); let java_arguments = (self.getTreeNode(),); jcall!(java_receiver).add(java_arguments.0); };
        self.graphicItemHashMap = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(self.grid).grid = jcall!(PointFactory::getInstance()).createXY(0, 0);
        self.setCanvasDimension(IntegerDimension::new(0, 0));
        self.selectedTool = None;
        self.addKeyListener(None /* Java this: a &mut self method has no Rc of itself */);
        self.addMouseListener(None /* Java this: a &mut self method has no Rc of itself */);
        self.addMouseMotionListener(None /* Java this: a &mut self method has no Rc of itself */);
        MyGraphicItemEventService::addListener(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    pub fn getTreeNode(&mut self) -> Option<Rc<RefCell<MutableTreeNode>>> {
        return self.canvasTreeNode.clone();
    }
    

    pub fn initComponents(&mut self) {
        self.setLayout(GridLayout::new(1, 1));
    }
    

    // throws Exception
    pub fn setCanvasSize(&mut self, mut x: i32, mut y: i32) {
        jcall!(self.grid).isGridPossible = true;
        if x <= 0 {
            jcall!(self.grid).isGridPossible = false;
            panic!("{:?}", format!("{}: {:?}", "GraphicsException", (format!("{}{}", String::from("X Size Error: "), x), None /* Java this: a &mut self method has no Rc of itself */, String::from("setWorkAreaSize"))));
        }
        if y <= 0 {
            jcall!(self.grid).isGridPossible = false;
            panic!("{:?}", format!("{}: {:?}", "GraphicsException", (format!("{}{}", String::from("Y Size Error: "), y), None /* Java this: a &mut self method has no Rc of itself */, String::from("setWorkAreaSize"))));
        }
        if x > self.getWidth() {
            jcall!(self.grid).isGridPossible = false;
        }
        if y > self.getHeight() {
            jcall!(self.grid).isGridPossible = false;
        }
        self.setCanvasDimension(IntegerDimension::new(x.clone(), y.clone()));
        jcall!(self.grid).isChanged = true;
    }
    

    pub fn setGrid(&mut self, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) {
        { let java_arguments = (jcall!(point).getX(), jcall!(point).getY(),); self.setGrid2(java_arguments.0, java_arguments.1) };
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn setGrid2(&mut self, mut xSize: i32, mut ySize: i32) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if xSize >= jcall!(self.getCanvasDimension()).getWidth() / 2 || xSize < 1 {
                panic!("{:?}", format!("{}: {:?}", "GraphicsException", (format!("{}{}", format!("{}{}", format!("{}{}", String::from("X Size Error: "), xSize), String::from(" WorkArea X: ")), jcall!(self.getCanvasDimension()).getWidth()), None /* Java this: a &mut self method has no Rc of itself */, String::from("setGrid"))));
            }
            if ySize >= jcall!(self.getCanvasDimension()).getHeight() / 2 || ySize < 1 {
                panic!("{:?}", format!("{}: {:?}", "GraphicsException", (format!("{}{}", format!("{}{}", format!("{}{}", String::from("Y Size Error"), ySize), String::from(" WorkArea Y: ")), jcall!(self.getCanvasDimension()).getHeight()), None /* Java this: a &mut self method has no Rc of itself */, String::from("setGrid"))));
            }
            jcall!(self.grid).grid = jcall!(PointFactory::getInstance()).createXY(xSize.clone(), ySize.clone());
            jcall!(self.grid).isChanged = true;
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            }
        }
    }
    

    // throws Exception
    pub fn setDefaultGrid(&mut self) {
        let mut xPixelsPerWorkAreaPixel: i32 = self.getWidth() / jcall!(self.getCanvasDimension()).getWidth();
        let mut yPixelsPerWorkAreaPixel: i32 = self.getHeight() / jcall!(self.getCanvasDimension()).getHeight();
        self.setGrid2(1, 1);
        jcall!(self.grid).isChanged = true;
    }
    

    pub fn getAngle(&mut self) -> f64 {
        return self.angle.clone();
    }
    

    pub fn setAngle(&mut self, mut angle: f64) {
        self.angle = angle.clone();
    }
    

    pub fn addAngle(&mut self, mut angle: f64) {
        self.setAngle(self.angle + angle);
    }
    

    pub fn explodeAll(&mut self) {
        let mut newPoints: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut graphicItemArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(jcall!(self.getGraphicItemHashMap()).keySet()).toArray();
        let mut size: i32 = (graphicItemArray.len() as i32).clone();
        for index in (0..size) {
            let mut graphicItemNode: Option<Rc<RefCell<MutableTreeNode>>> = (graphicItemArray[(index) as usize]).clone() /* as MutableTreeNode */;
            let mut graphicItem: Option<Rc<RefCell<dyn GraphicItemInterface>>> = (jcall!(self.getGraphicItemHashMap()).get(graphicItemNode.clone())).clone() /* as GraphicItemInterface */;
            if jcall!(graphicItem).getName() == LinesGraphicItem::getStaticName() {
                while jcall!(jcall!(graphicItem).getPointsInterface()).getSize() > 2 {
                    let mut newGraphicItem: Option<Rc<RefCell<dyn GraphicItemInterface>>> = jcall!(jcall!(GraphicItemFactory::getInstance()).getInstance1(jcall!(graphicItem).getName())).getInstance(None /* Java this: a &mut self method has no Rc of itself */);
                    let mut pointOne: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = GPoint::getInstance(jcall!(graphicItem).removePoint());
                    let mut pointTwo: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = GPoint::getInstance(jcall!(graphicItem).removePoint());
                    if pointOne.is_some() && pointTwo.is_some() {
                        jcall!(newGraphicItem).addPoint(pointOne.clone());
                        jcall!(newGraphicItem).addPoint(pointTwo.clone());
                    }
                    jcall!(newPoints).add(crate::java_into_object(newGraphicItem.clone()));
                }
            }
        }
        let mut size2: i32 = jcall!(newPoints).size();
        for index in (0..size2) {
            let mut newGraphicItem: Option<Rc<RefCell<dyn GraphicItemInterface>>> = { let java_object = jcall!(newPoints).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn GraphicItemInterface>>>>().expect("ClassCastException").clone()) };
            { let java_receiver = self.canvasTreeNode.clone(); let java_arguments = (jcall!(newGraphicItem).getTreeNode(),); jcall!(java_receiver).add(java_arguments.0); };
            jcall!(self.getGraphicItemHashMap()).put(jcall!(newGraphicItem).getTreeNode(), newGraphicItem.clone());
        }
        jcall!(self.workAreaJTreeJPanel).updateTree();
        jcall!(self.workAreaJTreeJPanel).expand();
        jcall!(self.workAreaJTreeJPanel).repaint();
    }
    

    // throws Exception
    pub fn explode(&mut self, mut howMuch: i32, mut explosionType: i32) {
        let mut graphicItemArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(jcall!(self.getGraphicItemHashMap()).keySet()).toArray();
        let mut size: i32 = (graphicItemArray.len() as i32).clone();
        for index in (0..size) {
            let mut item: Option<Rc<RefCell<dyn GraphicItemInterface>>> = (jcall!(self.getGraphicItemHashMap()).get(graphicItemArray[(index) as usize].clone())).clone() /* as GraphicItemInterface */;
            let mut basicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(VectorExplosionGenerator::getInstance()).createList(jcall!(jcall!(item).getPointsInterface()).getPoints(), howMuch.clone(), jcall!(VectorExplosionGenerator::getInstance()).RANDOM.clone());
            let mut newPoints: Option<Rc<RefCell<Points>>> = Points::new();
            jcall!(newPoints).as_BasicArrayList_mut().addPoints(basicArrayList.clone());
            jcall!(item).setPointsInterface(newPoints.clone());
            let mut angleDelta: i32 = jcall!(RandomRotationFactory::getInstance()).getNextRandomAngle(howMuch.clone());
            jcall!(item).setAngle(angleDelta.clone());
        }
        self.repaint();
    }
    

    // throws Exception
    pub fn mirror(&mut self) {
        let mut width: i32 = jcall!(self.getCanvasDimension()).getWidth();
        let mut graphicItemArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(jcall!(self.getGraphicItemHashMap()).keySet()).toArray();
        let mut size: i32 = (graphicItemArray.len() as i32).clone();
        for index in (0..size) {
            let mut item: Option<Rc<RefCell<dyn GraphicItemInterface>>> = (jcall!(self.getGraphicItemHashMap()).get(graphicItemArray[(index) as usize].clone())).clone() /* as GraphicItemInterface */;
            let mut basicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(VectorMirrorGenerator::getInstance()).getInstance2(jcall!(jcall!(item).getPointsInterface()).getPoints(), width.clone());
            let mut newPoints: Option<Rc<RefCell<Points>>> = Points::new();
            jcall!(newPoints).as_BasicArrayList_mut().addPoints(basicArrayList.clone());
            jcall!(item).setPointsInterface(newPoints.clone());
        }
        self.repaint();
    }
    

    // throws Exception
    pub fn center(&mut self) {
        jcall!(VectorCenterGenerator::new()).transform(self.getGraphicItemHashMap());
        self.repaint();
    }
    

    pub fn setNewTool(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(StatusFactory::getInstance()).setStatus(String::from("Tool Selected"));
            self.selectedTool = jcall!(jcall!(ToolFactory::getInstance()).getSelectedToolFactory()).getInstance(None /* Java this: a &mut self method has no Rc of itself */);
            let mut newNode: Option<Rc<RefCell<MutableTreeNode>>> = jcall!(self.selectedTool).getTreeNode();
            jcall!(self.getGraphicItemHashMap()).put(newNode.clone(), self.selectedTool.clone());
            jcall!(self.canvasTreeNode).add(newNode.clone());
            if self.getSelectedTool().is_none() {
                panic!("{:?}", format!("{}: {:?}", "GraphicsException", (String::from("No Tool Selected"), None /* Java this: a &mut self method has no Rc of itself */, String::from("setSelectedTool"))));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            }
        }
    }
    

    // throws Exception
    pub fn duplicateGraphicItem(&mut self, mut graphicItem: Option<Rc<RefCell<dyn GraphicItemInterface>>>) {
        let mut graphicItemClone: Option<Rc<RefCell<dyn GraphicItemInterface>>> = jcall!(graphicItem).duplicate();
        jcall!(self.getGraphicItemHashMap()).put(jcall!(graphicItemClone).getTreeNode(), graphicItemClone.clone());
        { let java_receiver = self.canvasTreeNode.clone(); let java_arguments = (jcall!(graphicItemClone).getTreeNode(),); jcall!(java_receiver).add(java_arguments.0); };
    }
    

    // throws Exception
    pub fn duplicateGraphicItemHashMap(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>) {
        let mut mutableTreeNodeArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(jcall!(hashMap).keySet()).toArray();
        let mut size: i32 = (mutableTreeNodeArray.len() as i32).clone();
        for index in (0..size) {
            let mut treeNode: Option<Rc<RefCell<MutableTreeNode>>> = (mutableTreeNodeArray[(index) as usize]).clone() /* as MutableTreeNode */;
            let mut graphicItem: Option<Rc<RefCell<dyn GraphicItemInterface>>> = (jcall!(hashMap).get(treeNode.clone())).clone() /* as GraphicItemInterface */;
            self.duplicateGraphicItem(graphicItem.clone());
        }
    }
    

    pub fn getGraphicItemHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        return self.graphicItemHashMap.clone();
    }
    

    pub fn getXPixelsPerWorkAreaPixel(&mut self) -> i32 {
        let mut x: i32 = jcall!(jcall!(self.getGrid()).grid).getX() * (self.getWidth() - 10) / jcall!(self.getCanvasDimension()).getWidth();
        if jcall!(self.getGrid()).getZoom() == 1 {
            return 1;
        } else {
            return x.clone();
        }
    }
    

    pub fn getYPixelsPerWorkAreaPixel(&mut self) -> i32 {
        let mut y: i32 = jcall!(jcall!(self.getGrid()).grid).getY() * (self.getHeight() - 10) / jcall!(self.getCanvasDimension()).getHeight();
        if jcall!(self.getGrid()).getZoom() == 1 {
            return 1;
        } else {
            return y.clone();
        }
    }
    

    pub fn getSelectedTool(&mut self) -> Option<Rc<RefCell<dyn GraphicItemInterface>>> {
        return self.selectedTool.clone();
    }
    

    pub fn drawItems(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut angleDouble: Option<f64> = Some((self.angle.clone()) as f64);
        let mut graphicItemArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(jcall!(self.getGraphicItemHashMap()).keySet()).toArray();
        let mut size: i32 = (graphicItemArray.len() as i32).clone();
        for index in (0..size) {
            let mut item: Option<Rc<RefCell<dyn GraphicItemInterface>>> = (jcall!(self.getGraphicItemHashMap()).get(graphicItemArray[(index) as usize].clone())).clone() /* as GraphicItemInterface */;
            jcall!(item).paint(graphics.clone(), angleDouble.clone(), self.getCanvasDimension(), self.getXPixelsPerWorkAreaPixel(), self.getYPixelsPerWorkAreaPixel());
        }
    }
    

    pub fn drawGrid(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut xAdjust: i32 = (self.getWidth() % self.getXPixelsPerWorkAreaPixel()) / 2;
        let mut yAdjust: i32 = (self.getHeight() % self.getYPixelsPerWorkAreaPixel()) / 2;
        let mut xLeft: i32 = xAdjust.clone();
        let mut yUpper: i32 = yAdjust.clone();
        let mut xRight: i32 = self.getWidth() - xAdjust;
        let mut yLower: i32 = self.getHeight() - yAdjust;
        jcall!(graphics).setColor(CanvasJPanel_gridColor.with(Clone::clone).clone());
        for value in (yAdjust..=self.getHeight() - yAdjust).step_by((self.getYPixelsPerWorkAreaPixel()) as usize) {
            jcall!(graphics).drawLine(xLeft.clone(), value.clone(), xRight.clone(), value.clone());
        }
        for value in (xAdjust..=self.getWidth() - xAdjust).step_by((self.getYPixelsPerWorkAreaPixel()) as usize) {
            jcall!(graphics).drawLine(value.clone(), yUpper.clone(), value.clone(), yLower.clone());
        }
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(self.getGrid()).isChanged {
                jcall!(graphics).setColor(CanvasJPanel_backgroundColor.with(Clone::clone).clone());
                jcall!(graphics).fillRect(0, 0, self.getWidth(), self.getHeight());
                if jcall!(self.getGrid()).getZoom() > 2 && jcall!(self.getGrid()).isGridOn && jcall!(self.getGrid()).isGridPossible {
                    self.setDefaultGrid();
                    self.drawGrid(graphics.clone());
                }
                self.drawItems(graphics.clone());
                jcall!(graphics).drawString(jcall!(Some((self.getAngle()) as f64)).toString(), self.getWidth() - 75, self.getHeight() - 15);
                jcall!(self.grid).isChanged = false;
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            }
        }
    }
    

    pub fn repaint(&mut self) {
        if self.grid.is_some() {
            jcall!(self.grid).isChanged = true;
        }
        self.base.repaint();
    }
    

    // throws Exception
    pub fn toDom(&mut self) -> Option<Rc<RefCell<Node>>> {
        return jcall!(CanvasDom::new1(None /* Java this: a &mut self method has no Rc of itself */)).toDom();
    }
    

    pub fn changed(&mut self) {
        jcall!(self.grid).isChanged = true;
    }
    

    pub fn mouseClicked(&mut self, mut mouseEvent: Option<Rc<RefCell<MouseEvent>>>) {
        jcall!(StatusFactory::getInstance()).setStatusNoLog(format!("{}{}", jcall!(MouseStrings::getInstance()).MOUSE_CLICKED_LABEL, self.logMouseEvent(mouseEvent.clone())));
        if self.getSelectedTool().is_some() && jcall!(self.getSelectedTool()).isActive() {
            let mut graphicItem: Option<Rc<RefCell<dyn GraphicItemInterface>>> = self.getSelectedTool();
            jcall!(graphicItem).mouseClicked(mouseEvent.clone(), self.getXPixelsPerWorkAreaPixel(), self.getYPixelsPerWorkAreaPixel());
        } else {
            self.setNewTool();
            let mut graphicItem: Option<Rc<RefCell<dyn GraphicItemInterface>>> = self.getSelectedTool();
            if graphicItem.is_some() {
                jcall!(graphicItem).mouseClicked(mouseEvent.clone(), self.getXPixelsPerWorkAreaPixel(), self.getYPixelsPerWorkAreaPixel());
            }
        }
        jcall!(self.grid).isChanged = true;
        self.repaint();
        jcall!(self.workAreaJTreeJPanel).updateTree();
        jcall!(self.workAreaJTreeJPanel).expand();
        jcall!(self.workAreaJTreeJPanel).repaint();
    }
    

    pub fn mouseEntered(&mut self, mut mouseEvent: Option<Rc<RefCell<MouseEvent>>>) {
        jcall!(StatusFactory::getInstance()).setStatusNoLog(self.logMouseEvent(mouseEvent.clone()));
    }
    

    pub fn mouseExited(&mut self, mut mouseEvent: Option<Rc<RefCell<MouseEvent>>>) {
        jcall!(StatusFactory::getInstance()).setStatusNoLog(self.logMouseEvent(mouseEvent.clone()));
    }
    

    pub fn mousePressed(&mut self, mut mouseEvent: Option<Rc<RefCell<MouseEvent>>>) {
        jcall!(StatusFactory::getInstance()).setStatusNoLog(format!("{}{}", jcall!(MouseStrings::getInstance()).MOUSE_PRESSED_LABEL, self.logMouseEvent(mouseEvent.clone())));
        if self.getSelectedTool().is_some() {
            let mut graphicItem: Option<Rc<RefCell<dyn GraphicItemInterface>>> = self.getSelectedTool();
            jcall!(graphicItem).mousePressed(mouseEvent.clone(), self.getXPixelsPerWorkAreaPixel(), self.getYPixelsPerWorkAreaPixel());
        }
        self.requestFocus();
        jcall!(self.grid).isChanged = true;
        self.repaint();
    }
    

    pub fn mouseReleased(&mut self, mut mouseEvent: Option<Rc<RefCell<MouseEvent>>>) {
        jcall!(StatusFactory::getInstance()).setStatusNoLog(format!("{}{}", jcall!(MouseStrings::getInstance()).MOUSE_RELEASED_LABEL, self.logMouseEvent(mouseEvent.clone())));
        let mut graphicItem: Option<Rc<RefCell<dyn GraphicItemInterface>>> = self.getSelectedTool();
        if graphicItem.is_some() {
            jcall!(graphicItem).mouseReleased(mouseEvent.clone(), self.getXPixelsPerWorkAreaPixel(), self.getYPixelsPerWorkAreaPixel());
        }
        self.requestFocus();
        jcall!(self.grid).isChanged = true;
        self.repaint();
    }
    

    pub fn mouseDragged(&mut self, mut mouseEvent: Option<Rc<RefCell<MouseEvent>>>) {
        jcall!(StatusFactory::getInstance()).setStatusNoLog(format!("{}{}", jcall!(MouseStrings::getInstance()).MOUSE_DRAGGED_LABEL, self.logMouseEvent(mouseEvent.clone())));
        let mut graphicItem: Option<Rc<RefCell<dyn GraphicItemInterface>>> = self.getSelectedTool();
        if graphicItem.is_some() {
            jcall!(graphicItem).mouseDragged(mouseEvent.clone(), self.getXPixelsPerWorkAreaPixel(), self.getYPixelsPerWorkAreaPixel());
        }
        self.requestFocus();
        jcall!(self.grid).isChanged = true;
        self.repaint();
    }
    

    pub fn mouseMoved(&mut self, mut mouseEvent: Option<Rc<RefCell<MouseEvent>>>) {
        jcall!(StatusFactory::getInstance()).setStatusNoLog(format!("{}{}", jcall!(MouseStrings::getInstance()).MOUSE_MOVED_LABEL, self.logMouseEvent(mouseEvent.clone())));
        if self.getSelectedTool().is_some() && jcall!(self.getSelectedTool()).isActive() {
            let mut graphicItem: Option<Rc<RefCell<dyn GraphicItemInterface>>> = self.getSelectedTool();
            jcall!(graphicItem).mouseMoved(mouseEvent.clone(), self.getXPixelsPerWorkAreaPixel(), self.getYPixelsPerWorkAreaPixel());
        }
        self.requestFocus();
        jcall!(self.grid).isChanged = true;
        self.repaint();
    }
    

    pub fn logMouseEvent(&mut self, mut mouseEvent: Option<Rc<RefCell<MouseEvent>>>) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(PositionStrings::getInstance()).X_LABEL.clone());
        jcall!(stringBuffer).appendint(((jcall!(mouseEvent).getX() / self.getXPixelsPerWorkAreaPixel()) as i32));
        jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(PositionStrings::getInstance()).Y_LABEL.clone());
        jcall!(stringBuffer).appendint(((jcall!(mouseEvent).getY() / self.getYPixelsPerWorkAreaPixel()) as i32));
        return jcall!(stringBuffer).toString();
    }
    

    pub fn keyPressed(&mut self, mut keyEvent: Option<Rc<RefCell<KeyEvent>>>) {
        jcall!(StatusFactory::getInstance()).setStatus(String::from("Key Pressed"));
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut graphicItemArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(jcall!(self.graphicItemHashMap).keySet()).toArray();
            let mut size: i32 = (graphicItemArray.len() as i32).clone();
            for index in (0..size) {
                let mut graphicItemInterface: Option<Rc<RefCell<dyn GraphicItemInterface>>> = (jcall!(self.graphicItemHashMap).get(graphicItemArray[(index) as usize].clone())).clone() /* as GraphicItemInterface */;
                let mut keyCode: i32 = jcall!(keyEvent).getKeyCode();
                if jcall!(graphicItemInterface).isActive() {
                    if keyCode == jcall!(keyEvent).VK_UP {
                        jcall!(graphicItemInterface).translate(0, -1);
                    } else if keyCode == jcall!(keyEvent).VK_DOWN {
                        jcall!(graphicItemInterface).translate(0, 1);
                    } else if keyCode == jcall!(keyEvent).VK_LEFT {
                        jcall!(graphicItemInterface).translate(-1, 0);
                    } else if keyCode == jcall!(keyEvent).VK_RIGHT {
                        jcall!(graphicItemInterface).translate(1, 0);
                    }
                }
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.gameInputStrings).KEY_PRESSED.clone(), crate::java_into_object(e.clone()));
            }
        }
        if self.getSelectedTool().is_some() && jcall!(self.getSelectedTool()).isActive() {
            jcall!(StatusFactory::getInstance()).setStatus(String::from("Key Pressed for Tool"));
            let mut graphicItem: Option<Rc<RefCell<dyn GraphicItemInterface>>> = self.getSelectedTool();
            jcall!(graphicItem).keyPressed(keyEvent.clone());
        } else {
            self.setNewTool();
        }
        jcall!(self.grid).isChanged = true;
        self.repaint();
    }
    

    pub fn keyReleased(&mut self, mut keyEvent: Option<Rc<RefCell<KeyEvent>>>) {
    }
    

    pub fn keyTyped(&mut self, mut keyEvent: Option<Rc<RefCell<KeyEvent>>>) {
    }
    

    pub fn delete(&mut self, mut evt: Option<Rc<RefCell<MyGraphicItemEvent>>>) {
        jcall!(self.getGraphicItemHashMap()).remove(jcall!(evt).getTreeNode());
        jcall!(self.workAreaJTreeJPanel).updateTree();
        jcall!(self.workAreaJTreeJPanel).expand();
        jcall!(self.workAreaJTreeJPanel).repaint();
    }
    

    pub fn deselect(&mut self, mut evt: Option<Rc<RefCell<MyGraphicItemEvent>>>) {
        let mut graphicItemInterface: Option<Rc<RefCell<dyn GraphicItemInterface>>> = (jcall!(self.getGraphicItemHashMap()).get(jcall!(evt).getTreeNode())).clone() /* as GraphicItemInterface */;
        if graphicItemInterface.is_some() {
            jcall!(graphicItemInterface).setColor(unsafe { Color_WHITE.clone() }.clone());
            jcall!(graphicItemInterface).deactivate();
            jcall!(self.workAreaJTreeJPanel).repaint();
        }
    }
    

    pub fn highlight(&mut self, mut evt: Option<Rc<RefCell<MyGraphicItemEvent>>>) {
        let mut graphicItemInterface: Option<Rc<RefCell<dyn GraphicItemInterface>>> = (jcall!(self.getGraphicItemHashMap()).get(jcall!(evt).getTreeNode())).clone() /* as GraphicItemInterface */;
        if graphicItemInterface.is_some() {
            jcall!(graphicItemInterface).setColor(unsafe { Color_YELLOW.clone() }.clone());
            jcall!(graphicItemInterface).activate();
            jcall!(self.workAreaJTreeJPanel).repaint();
        }
    }
    

    // throws Exception
    pub fn duplicate(&mut self) -> Option<Rc<RefCell<CanvasJPanel>>> {
        let mut newCanvasJPanel: Option<Rc<RefCell<CanvasJPanel>>> = CanvasJPanel::new4(self.workAreaJTreeJPanel.clone(), self.getSize(), jcall!(self.getCanvasDimension()).getWidth(), jcall!(self.getCanvasDimension()).getHeight());
        jcall!(newCanvasJPanel).setGrid(jcall!(self.getGrid()).getGrid());
        jcall!(newCanvasJPanel).setAngle(self.getAngle());
        jcall!(newCanvasJPanel).duplicateGraphicItemHashMap(self.getGraphicItemHashMap());
        jcall!(self.workAreaJTreeJPanel).updateTree();
        jcall!(self.workAreaJTreeJPanel).expand();
        jcall!(self.workAreaJTreeJPanel).repaint();
        return newCanvasJPanel.clone();
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn duplicate1(&mut self, mut event: Option<Rc<RefCell<MyGraphicItemEvent>>>) {
        let mut treeNode: Option<Rc<RefCell<MutableTreeNode>>> = jcall!(event).getTreeNode();
        let mut graphicItem: Option<Rc<RefCell<dyn GraphicItemInterface>>> = (jcall!(self.getGraphicItemHashMap()).get(treeNode.clone())).clone() /* as GraphicItemInterface */;
        if graphicItem.is_some() {
            self.duplicateGraphicItem(graphicItem.clone());
            jcall!(self.workAreaJTreeJPanel).updateTree();
            jcall!(self.workAreaJTreeJPanel).expand();
            jcall!(self.workAreaJTreeJPanel).repaint();
        }
    }
    

    pub fn rotate(&mut self, mut event: Option<Rc<RefCell<MyGraphicItemEvent>>>) {
        let mut treeNode: Option<Rc<RefCell<MutableTreeNode>>> = jcall!(event).getTreeNode();
        let mut graphicItem: Option<Rc<RefCell<dyn GraphicItemInterface>>> = (jcall!(self.getGraphicItemHashMap()).get(treeNode.clone())).clone() /* as GraphicItemInterface */;
        if graphicItem.is_some() {
            jcall!(graphicItem).addAngle(jcall!(event).getAngle());
        }
    }
    

    pub fn getCanvasDimension(&mut self) -> Option<Rc<RefCell<IntegerDimension>>> {
        return self.canvasDimension.clone();
    }
    

    pub fn setCanvasDimension(&mut self, mut canvasDimension: Option<Rc<RefCell<IntegerDimension>>>) {
        self.canvasDimension = canvasDimension.clone();
    }
    

    pub fn getGrid(&mut self) -> Option<Rc<RefCell<Grid>>> {
        return self.grid.clone();
    }
}
impl std::ops::Deref for CanvasJPanel {
    type Target = JPanel;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CanvasJPanel {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



