// Crate root for the generated Rust sources.
// The generated code uses paths such as crate::org::allbinary::string::CommonSeps so build.rs generates the module tree from the Java package directories.
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

include!(concat!(env!("OUT_DIR"), "/modules.rs"));

// Minimal Java standard-library compatibility types used by generated sources.
pub trait Runnable {
    fn run(&mut self);
}
pub type JavaObject = Option<std::rc::Rc<std::cell::RefCell<Box<dyn std::any::Any>>>>;
pub fn java_into_object<T: 'static>(value: T) -> JavaObject {
    let boxed: Box<dyn std::any::Any> = Box::new(value);
    match boxed.downcast::<JavaObject>() {
        Ok(reference) => *reference,
        Err(value) => Some(std::rc::Rc::new(std::cell::RefCell::new(value))),
    }
}
pub fn java_object_cast<T: 'static>(value: &JavaObject) -> Option<std::rc::Rc<std::cell::RefCell<T>>> {
    value.as_ref().and_then(|object| {
        object.borrow().downcast_ref::<Option<std::rc::Rc<std::cell::RefCell<T>>>>()
            .expect("ClassCastException").clone()
    })
}
pub fn java_reference_into_object<T: 'static>(value: Option<std::rc::Rc<std::cell::RefCell<T>>>) -> JavaObject {
    value.map(|reference| std::rc::Rc::new(std::cell::RefCell::new(
        Box::new(Some(reference)) as Box<dyn std::any::Any>
    )))
}
// Erased values retain Java reference identity; boxed primitive and String keys
// compare by value. Generated objects without an equals override use identity.
pub fn java_object_equals(left: &JavaObject, right: &JavaObject) -> bool {
    match (left, right) {
        (None, None) => true,
        (Some(left), Some(right)) => {
            if std::rc::Rc::ptr_eq(left, right) { return true; }
            let left = left.borrow();
            let right = right.borrow();
            macro_rules! value_eq {
                ($($kind:ty),*) => { $(
                    if let (Some(a), Some(b)) = (left.downcast_ref::<$kind>(), right.downcast_ref::<$kind>()) { return a == b; }
                )* };
            }
            value_eq!(String, bool, char, i8, i16, i32, i64);
            if let (Some(a), Some(b)) = (left.downcast_ref::<f32>(), right.downcast_ref::<f32>()) {
                return (a.is_nan() && b.is_nan()) || a.to_bits() == b.to_bits();
            }
            if let (Some(a), Some(b)) = (left.downcast_ref::<f64>(), right.downcast_ref::<f64>()) {
                return (a.is_nan() && b.is_nan()) || a.to_bits() == b.to_bits();
            }
            false
        }
        _ => false,
    }
}
pub trait Enumeration {
    fn hasMoreElements(&mut self) -> bool;
    fn nextElement(&mut self) -> JavaObject;
}
pub struct ObjectEnumeration { values: std::vec::IntoIter<JavaObject> }
impl ObjectEnumeration {
    pub fn new(values: Vec<JavaObject>) -> Self { Self { values: values.into_iter() } }
}
impl Enumeration for ObjectEnumeration {
    fn hasMoreElements(&mut self) -> bool { self.values.len() != 0 }
    fn nextElement(&mut self) -> JavaObject { self.values.next().expect("NoSuchElementException") }
}

#[derive(Clone, Default)]
pub struct Vector<E> { pub values: Vec<E> }
impl<E> Vector<E> {
    pub fn new_struct() -> Self { Self { values: Vec::new() } }
    pub fn size(&mut self) -> i32 { self.values.len() as i32 }
}
impl<E: Clone> Vector<E> {
    pub fn elementAt(&mut self, index: i32) -> E { self.values[index as usize].clone() }
}
pub type Stack<E> = Vector<E>;

#[derive(Clone, Default)]
pub struct Hashtable<K, V> { pub entries: Vec<(K, V)> }
impl<K, V> Hashtable<K, V> {
    pub fn new_struct() -> Self { Self { entries: Vec::new() } }
    pub fn new_struct1(initial_capacity: i32) -> Self { Self { entries: Vec::with_capacity(initial_capacity.max(0) as usize) } }
    pub fn size(&self) -> i32 { self.entries.len() as i32 }
}
impl Hashtable<JavaObject, JavaObject> {
    pub fn keys(&self) -> Option<std::rc::Rc<std::cell::RefCell<dyn Enumeration>>> {
        Some(std::rc::Rc::new(std::cell::RefCell::new(ObjectEnumeration::new(
            self.entries.iter().map(|(key, _)| key.clone()).collect()
        ))))
    }
    pub fn get(&self, key: JavaObject) -> JavaObject {
        assert!(key.is_some(), "NullPointerException");
        self.entries.iter().find(|(stored, _)| java_object_equals(stored, &key))
            .and_then(|(_, value)| value.clone())
    }
    pub fn put(&mut self, key: JavaObject, value: JavaObject) -> JavaObject {
        assert!(key.is_some() && value.is_some(), "NullPointerException");
        if let Some((_, stored)) = self.entries.iter_mut().find(|(stored, _)| java_object_equals(stored, &key)) {
            return std::mem::replace(stored, value);
        }
        self.entries.push((key, value));
        None
    }
}

#[derive(Clone)]
pub struct Date(pub std::time::SystemTime);
impl Date {
    pub fn toString(&mut self) -> String { format!("{:?}", self.0) }
}

#[derive(Clone, Default)]
pub struct Calendar;
impl Calendar {
    pub fn getInstance() -> Option<std::rc::Rc<std::cell::RefCell<Self>>> { Some(std::rc::Rc::new(std::cell::RefCell::new(Self))) }
    pub fn getTime(&mut self) -> Option<std::rc::Rc<std::cell::RefCell<Date>>> { Some(std::rc::Rc::new(std::cell::RefCell::new(Date(std::time::SystemTime::now())))) }
}