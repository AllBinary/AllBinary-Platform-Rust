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
// package org.allbinary.business.category.store.theme
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::category::Category::Category;
use crate::org::allbinary::business::category::CategoryInterface::CategoryInterface;
use crate::org::allbinary::business::category::properties::CategoryPropertiesFactoryInterface::CategoryPropertiesFactoryInterface;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::visual::theme::ThemeData::ThemeData;
use crate::org::allbinary::logic::visual::theme::ThemeValidation::ThemeValidation;
use crate::org::allbinary::logic::visual::theme::ThemesData::ThemesData;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;
use super::StoreThemeCategoryInterface::StoreThemeCategoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StoreThemeCategory {
    pub base: Category,
    pub transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>,
    pub themeVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

// impl StoreThemeCategoryInterface for StoreThemeCategory with the methods of the trait

impl StoreThemeCategory {
    

    pub fn new_struct3(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>>, mut level: i32) -> Self {
        let mut this = Self {
            base: Category::new_struct2(categoryPropertiesFactoryInterface.clone(), level.clone()),
            transformInfoInterface: None,
            themeVector: None,
        };
        // super(...) is in the struct literal above
        this.transformInfoInterface = transformInfoInterface.clone();
        this.themeVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new3(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>>, mut level: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct3(transformInfoInterface, categoryPropertiesFactoryInterface, level))))
    }
    

    pub fn new_struct4(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut rootCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut parentCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>>) -> Self {
        let mut this = Self {
            base: Category::new_struct3(rootCategoryInterface.clone(), parentCategoryInterface.clone(), categoryPropertiesFactoryInterface.clone()),
            transformInfoInterface: None,
            themeVector: None,
        };
        // super(...) is in the struct literal above
        this.transformInfoInterface = transformInfoInterface.clone();
        this.themeVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new4(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut rootCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut parentCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct4(transformInfoInterface, rootCategoryInterface, parentCategoryInterface, categoryPropertiesFactoryInterface))))
    }
    

    pub fn new_struct4(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut node: Option<Rc<RefCell<Node>>>, mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>>, mut level: i32) -> Self {
        let mut this = Self {
            base: Category::new_struct2(categoryPropertiesFactoryInterface.clone(), level.clone()),
            transformInfoInterface: None,
            themeVector: None,
        };
        // super(...) is in the struct literal above
        this.transformInfoInterface = transformInfoInterface.clone();
        this.themeVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.addThemes(node.clone());
        this
    }

    pub fn new4(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut node: Option<Rc<RefCell<Node>>>, mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>>, mut level: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct4(transformInfoInterface, node, categoryPropertiesFactoryInterface, level))))
    }
    

    pub fn new_struct5(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut node: Option<Rc<RefCell<Node>>>, mut rootCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut parentCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>>) -> Self {
        let mut this = Self {
            base: Category::new_struct3(rootCategoryInterface.clone(), parentCategoryInterface.clone(), categoryPropertiesFactoryInterface.clone()),
            transformInfoInterface: None,
            themeVector: None,
        };
        // super(...) is in the struct literal above
        this.transformInfoInterface = transformInfoInterface.clone();
        this.themeVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.addThemes(node.clone());
        this
    }

    pub fn new5(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut node: Option<Rc<RefCell<Node>>>, mut rootCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut parentCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut categoryPropertiesFactoryInterface: Option<Rc<RefCell<dyn CategoryPropertiesFactoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct5(transformInfoInterface, node, rootCategoryInterface, parentCategoryInterface, categoryPropertiesFactoryInterface))))
    }
    

    pub fn getTransformInfoInterface(&mut self) -> Option<Rc<RefCell<dyn TransformInfoInterface>>> {
        return self.transformInfoInterface.clone();
    }
    

    // throws Exception
    pub fn addThemes(&mut self, mut node: Option<Rc<RefCell<Node>>>) {
        let mut themesNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(jcall!(ThemesData::getInstance()).NAME.clone(), jcall!(node).getChildNodes());
        let mut nodeList: Option<Rc<RefCell<NodeList>>> = jcall!(themesNode).getChildNodes();
        for index in (0..jcall!(nodeList).getLength()) {
            let mut themeNode: Option<Rc<RefCell<Node>>> = jcall!(nodeList).item(index.clone());
            if jcall!(jcall!(themeNode).getNodeName()).compareTo(jcall!(ThemeData::getInstance()).NAME.clone()) == 0 {
                jcall!(self.themeVector).add(crate::java_into_object(ThemeValidation::new2(None /* Java this: a &mut self method has no Rc of itself */, themeNode.clone())));
            }
        }
    }
    

    pub fn addTheme(&mut self, mut themeValidation: Option<Rc<RefCell<ThemeValidation>>>) {
        jcall!(self.themeVector).add(crate::java_into_object(themeValidation.clone()));
    }
    

    pub fn getThemes(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.themeVector.clone();
    }
    

    // throws Exception
    pub fn isValid(&mut self) -> Option<bool> {
        if !self.base.isValid().unwrap() {
            return false;
        }
        return true;
    }
    

    // throws Exception
    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut categoryHashMap: Option<Rc<RefCell<HashMap>>> = self.base.toHashMap();
        jcall!(categoryHashMap).put(jcall!(ThemesData::getInstance()).NAME.clone(), self.themeVector.clone());
        return categoryHashMap.clone();
    }
    

    // throws Exception
    pub fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut categoryVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.base.toVector();
        return categoryVector.clone();
    }
}
impl std::ops::Deref for StoreThemeCategory {
    type Target = Category;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for StoreThemeCategory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::business::category::Category::CategoryDyn for StoreThemeCategory {
    fn as_Category(&self) -> &crate::org::allbinary::business::category::Category::Category { &self.base }
    fn as_Category_mut(&mut self) -> &mut crate::org::allbinary::business::category::Category::Category { &mut self.base }
    fn isValid(&mut self) -> Option<bool> { Self::isValid(self) }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { Self::toHashMap(self) }
    fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::toVector(self) }
}


