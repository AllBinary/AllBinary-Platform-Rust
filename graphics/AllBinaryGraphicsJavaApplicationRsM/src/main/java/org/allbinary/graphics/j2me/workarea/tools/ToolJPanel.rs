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
// package org.allbinary.graphics.j2me.workarea.tools
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::JPanel::JPanel;
use super::swing::swing;
use super::javax::javax;
use super::JButton::JButton;
use super::JList::JList;
use super::GridLayout::GridLayout;
use super::awt::awt;
use super::java::java;
use super::Font::Font;
use super::MouseAdapter::MouseAdapter;
use super::event::event;
use super::MouseEvent::MouseEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ToolJPanel {
    pub base: JPanel,
    pub arcJButton: Option<Rc<RefCell<JButton>>>,
    pub charJButton: Option<Rc<RefCell<JButton>>>,
    pub colorJList: Option<Rc<RefCell<JList>>>,
    pub colorJPanel: Option<Rc<RefCell<JPanel>>>,
    pub dottedArcJButton: Option<Rc<RefCell<JButton>>>,
    pub dottedFillArcJButton: Option<Rc<RefCell<JButton>>>,
    pub dottedFillRectJButton: Option<Rc<RefCell<JButton>>>,
    pub dottedFillRountRectJButton: Option<Rc<RefCell<JButton>>>,
    pub dottedLineJButton: Option<Rc<RefCell<JButton>>>,
    pub dottedRectButton: Option<Rc<RefCell<JButton>>>,
    pub fillArcJButton: Option<Rc<RefCell<JButton>>>,
    pub fillRectJButton: Option<Rc<RefCell<JButton>>>,
    pub fillRoundRectJButton: Option<Rc<RefCell<JButton>>>,
    pub imageJButton: Option<Rc<RefCell<JButton>>>,
    pub lineJButton: Option<Rc<RefCell<JButton>>>,
    pub rectJButton: Option<Rc<RefCell<JButton>>>,
    pub roundRectJButton: Option<Rc<RefCell<JButton>>>,
    pub stringJButton: Option<Rc<RefCell<JButton>>>,
}


impl ToolJPanel {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: JPanel::new_struct(),
            arcJButton: None,
            charJButton: None,
            colorJList: None,
            colorJPanel: None,
            dottedArcJButton: None,
            dottedFillArcJButton: None,
            dottedFillRectJButton: None,
            dottedFillRountRectJButton: None,
            dottedLineJButton: None,
            dottedRectButton: None,
            fillArcJButton: None,
            fillRectJButton: None,
            fillRoundRectJButton: None,
            imageJButton: None,
            lineJButton: None,
            rectJButton: None,
            roundRectJButton: None,
            stringJButton: None,
        };
        this.initComponents();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn initComponents(&mut self) {
        self.lineJButton = JButton::new();
        self.rectJButton = JButton::new();
        self.arcJButton = JButton::new();
        self.stringJButton = JButton::new();
        self.charJButton = JButton::new();
        self.roundRectJButton = JButton::new();
        self.imageJButton = JButton::new();
        self.fillArcJButton = JButton::new();
        self.fillRectJButton = JButton::new();
        self.fillRoundRectJButton = JButton::new();
        self.dottedLineJButton = JButton::new();
        self.dottedRectButton = JButton::new();
        self.dottedArcJButton = JButton::new();
        self.dottedFillRectJButton = JButton::new();
        self.dottedFillArcJButton = JButton::new();
        self.dottedFillRountRectJButton = JButton::new();
        self.colorJPanel = JPanel::new();
        self.colorJList = JList::new();
        self.setLayout(GridLayout::new(17, 1));
        jcall!(self.lineJButton).setFont(Font::new(String::from("Dialog"), 0, 10));
        jcall!(self.lineJButton).setText(LinesGraphicItem::LinesGraphicItem::getStaticName());
        jcall!(self.lineJButton).addMouseListener(MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mouseClicked(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.lineJButtonMouseClicked(evt.clone());
            }
*/);
        self.add(self.lineJButton.clone());
        jcall!(self.rectJButton).setFont(Font::new(String::from("Dialog"), 0, 10));
        jcall!(self.rectJButton).setText(String::from("Rect"));
        { let java_receiver = self.rectJButton.clone(); let java_arguments = (MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mouseClicked(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.rectJButtonMouseClicked(evt.clone());
            }
*/,); jcall!(java_receiver).addMouseListener(java_arguments.0); };
        self.add(self.rectJButton.clone());
        jcall!(self.arcJButton).setFont(Font::new(String::from("Dialog"), 0, 10));
        jcall!(self.arcJButton).setText(String::from("Arc"));
        { let java_receiver = self.arcJButton.clone(); let java_arguments = (MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.arcJButtonMousePressed(evt.clone());
            }
*/,); jcall!(java_receiver).addMouseListener(java_arguments.0); };
        self.add(self.arcJButton.clone());
        jcall!(self.stringJButton).setFont(Font::new(String::from("Dialog"), 0, 10));
        jcall!(self.stringJButton).setText(String::from("String"));
        { let java_receiver = self.stringJButton.clone(); let java_arguments = (MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.stringJButtonMousePressed(evt.clone());
            }
*/,); jcall!(java_receiver).addMouseListener(java_arguments.0); };
        self.add(self.stringJButton.clone());
        jcall!(self.charJButton).setFont(Font::new(String::from("Dialog"), 0, 10));
        jcall!(self.charJButton).setText(String::from("Char"));
        { let java_receiver = self.charJButton.clone(); let java_arguments = (MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.charJButtonMousePressed(evt.clone());
            }
*/,); jcall!(java_receiver).addMouseListener(java_arguments.0); };
        self.add(self.charJButton.clone());
        jcall!(self.roundRectJButton).setFont(Font::new(String::from("Dialog"), 0, 10));
        jcall!(self.roundRectJButton).setText(String::from("Round Rect"));
        { let java_receiver = self.roundRectJButton.clone(); let java_arguments = (MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.roundRectJButtonMousePressed(evt.clone());
            }
*/,); jcall!(java_receiver).addMouseListener(java_arguments.0); };
        self.add(self.roundRectJButton.clone());
        jcall!(self.imageJButton).setFont(Font::new(String::from("Dialog"), 0, 10));
        jcall!(self.imageJButton).setText(String::from("Image"));
        { let java_receiver = self.imageJButton.clone(); let java_arguments = (MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.imageJButtonMousePressed(evt.clone());
            }
*/,); jcall!(java_receiver).addMouseListener(java_arguments.0); };
        self.add(self.imageJButton.clone());
        jcall!(self.fillArcJButton).setFont(Font::new(String::from("Dialog"), 0, 10));
        jcall!(self.fillArcJButton).setText(String::from("Fill Arc"));
        { let java_receiver = self.fillArcJButton.clone(); let java_arguments = (MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.fillArcJButtonMousePressed(evt.clone());
            }
*/,); jcall!(java_receiver).addMouseListener(java_arguments.0); };
        self.add(self.fillArcJButton.clone());
        jcall!(self.fillRectJButton).setFont(Font::new(String::from("Dialog"), 0, 10));
        jcall!(self.fillRectJButton).setText(String::from("Fill Rect"));
        { let java_receiver = self.fillRectJButton.clone(); let java_arguments = (MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.fillRectJButtonMousePressed(evt.clone());
            }
*/,); jcall!(java_receiver).addMouseListener(java_arguments.0); };
        self.add(self.fillRectJButton.clone());
        jcall!(self.fillRoundRectJButton).setFont(Font::new(String::from("Dialog"), 0, 10));
        jcall!(self.fillRoundRectJButton).setText(String::from("Fill Rnd Rect"));
        { let java_receiver = self.fillRoundRectJButton.clone(); let java_arguments = (MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.fillRoundRectJButtonMousePressed(evt.clone());
            }
*/,); jcall!(java_receiver).addMouseListener(java_arguments.0); };
        self.add(self.fillRoundRectJButton.clone());
        jcall!(self.dottedLineJButton).setFont(Font::new(String::from("Dialog"), 0, 10));
        jcall!(self.dottedLineJButton).setText(String::from("Dot Line"));
        { let java_receiver = self.dottedLineJButton.clone(); let java_arguments = (MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.dottedLineJButtonMousePressed(evt.clone());
            }
*/,); jcall!(java_receiver).addMouseListener(java_arguments.0); };
        self.add(self.dottedLineJButton.clone());
        jcall!(self.dottedRectButton).setFont(Font::new(String::from("Dialog"), 0, 10));
        jcall!(self.dottedRectButton).setText(String::from("Dot Rect"));
        { let java_receiver = self.dottedRectButton.clone(); let java_arguments = (MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.dottedRectButtonMousePressed(evt.clone());
            }
*/,); jcall!(java_receiver).addMouseListener(java_arguments.0); };
        self.add(self.dottedRectButton.clone());
        jcall!(self.dottedArcJButton).setFont(Font::new(String::from("Dialog"), 0, 10));
        jcall!(self.dottedArcJButton).setText(String::from("Dot Arc"));
        { let java_receiver = self.dottedArcJButton.clone(); let java_arguments = (MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.dottedArcJButtonMousePressed(evt.clone());
            }
*/,); jcall!(java_receiver).addMouseListener(java_arguments.0); };
        self.add(self.dottedArcJButton.clone());
        jcall!(self.dottedFillRectJButton).setFont(Font::new(String::from("Dialog"), 0, 10));
        jcall!(self.dottedFillRectJButton).setText(String::from("Dot Fill Rect"));
        { let java_receiver = self.dottedFillRectJButton.clone(); let java_arguments = (MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.dottedFillRectJButtonMousePressed(evt.clone());
            }
*/,); jcall!(java_receiver).addMouseListener(java_arguments.0); };
        self.add(self.dottedFillRectJButton.clone());
        jcall!(self.dottedFillArcJButton).setFont(Font::new(String::from("Dialog"), 0, 10));
        jcall!(self.dottedFillArcJButton).setText(String::from("Dot Fill Arc"));
        { let java_receiver = self.dottedFillArcJButton.clone(); let java_arguments = (MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.dottedFillArcJButtonMousePressed(evt.clone());
            }
*/,); jcall!(java_receiver).addMouseListener(java_arguments.0); };
        self.add(self.dottedFillArcJButton.clone());
        jcall!(self.dottedFillRountRectJButton).setFont(Font::new(String::from("Dialog"), 0, 10));
        jcall!(self.dottedFillRountRectJButton).setText(String::from("Dot Fill Rnd Rect"));
        { let java_receiver = self.dottedFillRountRectJButton.clone(); let java_arguments = (MouseAdapter::new() /* Java anonymous class body: implement it as a named type
            

            pub fn mousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
                self.dottedFillRountRectJButtonMousePressed(evt.clone());
            }
*/,); jcall!(java_receiver).addMouseListener(java_arguments.0); };
        self.add(self.dottedFillRountRectJButton.clone());
        jcall!(self.colorJPanel).setLayout(GridLayout::new(1, 1));
        jcall!(self.colorJPanel).add(self.colorJList.clone());
        self.add(self.colorJPanel.clone());
    }
    

    pub fn dottedFillRountRectJButtonMousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
        jcall!(ToolFactory::ToolFactory::getInstance()).setSelectedTool(jcall!(self.dottedFillRountRectJButton).getText());
    }
    

    pub fn dottedFillArcJButtonMousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
        jcall!(ToolFactory::ToolFactory::getInstance()).setSelectedTool(jcall!(self.dottedFillArcJButton).getText());
    }
    

    pub fn dottedFillRectJButtonMousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
        jcall!(ToolFactory::ToolFactory::getInstance()).setSelectedTool(jcall!(self.dottedFillRectJButton).getText());
    }
    

    pub fn dottedArcJButtonMousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
        jcall!(ToolFactory::ToolFactory::getInstance()).setSelectedTool(jcall!(self.dottedArcJButton).getText());
    }
    

    pub fn dottedRectButtonMousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
        jcall!(ToolFactory::ToolFactory::getInstance()).setSelectedTool(jcall!(self.dottedRectButton).getText());
    }
    

    pub fn dottedLineJButtonMousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
        jcall!(ToolFactory::ToolFactory::getInstance()).setSelectedTool(jcall!(self.dottedLineJButton).getText());
    }
    

    pub fn fillRoundRectJButtonMousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
        jcall!(ToolFactory::ToolFactory::getInstance()).setSelectedTool(jcall!(self.fillRoundRectJButton).getText());
    }
    

    pub fn fillRectJButtonMousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
        jcall!(ToolFactory::ToolFactory::getInstance()).setSelectedTool(jcall!(self.fillRectJButton).getText());
    }
    

    pub fn fillArcJButtonMousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
        jcall!(ToolFactory::ToolFactory::getInstance()).setSelectedTool(jcall!(self.fillArcJButton).getText());
    }
    

    pub fn imageJButtonMousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
        jcall!(ToolFactory::ToolFactory::getInstance()).setSelectedTool(jcall!(self.imageJButton).getText());
    }
    

    pub fn roundRectJButtonMousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
        jcall!(ToolFactory::ToolFactory::getInstance()).setSelectedTool(jcall!(self.roundRectJButton).getText());
    }
    

    pub fn charJButtonMousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
        jcall!(ToolFactory::ToolFactory::getInstance()).setSelectedTool(jcall!(self.charJButton).getText());
    }
    

    pub fn stringJButtonMousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
        jcall!(ToolFactory::ToolFactory::getInstance()).setSelectedTool(jcall!(self.stringJButton).getText());
    }
    

    pub fn arcJButtonMousePressed(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
        jcall!(ToolFactory::ToolFactory::getInstance()).setSelectedTool(jcall!(self.arcJButton).getText());
    }
    

    pub fn rectJButtonMouseClicked(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
        jcall!(ToolFactory::ToolFactory::getInstance()).setSelectedTool(jcall!(self.rectJButton).getText());
    }
    

    pub fn lineJButtonMouseClicked(&mut self, mut evt: Option<Rc<RefCell<MouseEvent>>>) {
        jcall!(ToolFactory::ToolFactory::getInstance()).setSelectedTool(jcall!(self.lineJButton).getText());
    }
}
impl std::ops::Deref for ToolJPanel {
    type Target = JPanel;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ToolJPanel {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



