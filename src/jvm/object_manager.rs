use std::collections::HashMap;

use crate::util::heap::{Heap, ReferencePtr};

use super::{
    bootstrap_class_loader::BootstrapClassLoader,
    interpreter::Interpreter,
    types::{
        array::{Array, PrimitiveArray},
        byte::Byte,
        field::Field,
        object::Object,
        reference::Reference,
        Types, Value,
    },
};

static mut INSTANCE: Option<ObjectManager> = None;

pub struct ObjectManager {
    bootstrap_class_loader: BootstrapClassLoader,
    objects: HashMap<String, Object>,
    associated_class_objects: HashMap<String, Reference>,
    initialized: HashMap<String, bool>,
    being_initialized: HashMap<String, bool>,
    jdk_base_path: String,
}

impl ObjectManager {
    pub fn initialize(jdk_base_path: String) {
        unsafe {
            INSTANCE = Some(ObjectManager::new(jdk_base_path));
        }
    }

    pub fn load_object(name: &str) {
        let instance = ObjectManager::it();
        instance.load_impl(name);
    }

    pub fn initialize_object(name: &str) {
        let instance = ObjectManager::it();
        instance.initialize_impl(name);
    }

    pub fn get(name: &str) -> &mut Object {
        let instance = ObjectManager::it();
        instance.get_impl(name)
    }

    pub fn instantiate(name: &str) -> Reference {
        let instance = ObjectManager::it();
        instance.instantiate_impl(name)
    }

    pub fn get_associated_class_object(name: &str) -> Reference {
        match ObjectManager::it().associated_class_objects.get(name) {
            Some(reference) => reference.clone(),
            None => panic!("Associated class object not found: {}", name),
        }
    }

    pub fn is_class(name: &str) -> bool {
        if name.starts_with('[') {
            return false;
        }
        !matches!(ObjectManager::get(name).fields().capacity(), 0)
    }

    pub fn is_array_class(name: &str) -> bool {
        if name.starts_with('[') {
            return true;
        }
        false
    }

    pub fn is_interface(name: &str) -> bool {
        if name.starts_with('[') {
            return false;
        }
        matches!(ObjectManager::get(name).fields().capacity(), 0)
    }

    pub fn get_string_instance(text: String) -> Reference {
        let object_ref = ObjectManager::instantiate("java/lang/String");
        let mut byte_array = PrimitiveArray::new(Types::Byte(Byte::new()), text.len());
        for (index, byte) in text.bytes().enumerate() {
            byte_array.set(index, Types::Byte(Byte::from_value(byte as i8)));
        }
        let arry_ref = Heap::allocate_primitive_array(byte_array);
        let reference = object_ref.get();
        let mut object = match reference {
            ReferencePtr::Class(ref object) => object.borrow_mut(),
            _ => panic!("Expected class reference"),
        };
        object.put_field(Field::new(
            "value".to_string(),
            "[B".to_string(),
            0x0002 | 0x0010,
            Some(Types::Reference(arry_ref)),
        ));
        object_ref
    }

    fn it() -> &'static mut ObjectManager {
        unsafe {
            if INSTANCE.is_some() {
                INSTANCE.as_mut().unwrap()
            } else {
                panic!("ObjectManager has not been initialized")
            }
        }
    }

    fn new(jdk_base_path: String) -> ObjectManager {
        let mut bootstrap_class_loader = BootstrapClassLoader::new();
        bootstrap_class_loader.initialize(jdk_base_path.clone());
        let objects: HashMap<String, Object> = HashMap::new();
        let associated_class_objects: HashMap<String, Reference> = HashMap::new();
        let initialized: HashMap<String, bool> = HashMap::new();
        let being_initialized: HashMap<String, bool> = HashMap::new();

        ObjectManager {
            bootstrap_class_loader,
            objects,
            associated_class_objects,
            initialized,
            being_initialized,
            jdk_base_path,
        }
    }

    fn get_impl(&mut self, name: &str) -> &mut Object {
        match self.objects.contains_key(name) {
            true => match self.initialized.contains_key(name) {
                true => self.objects.get_mut(name).unwrap(),
                false => match self.being_initialized.contains_key(name) {
                    true => self.objects.get_mut(name).unwrap(),
                    false => panic!("Object not initialized or being initialized: {}", name),
                },
            },
            false => {
                self.load_impl(name);
                self.get_impl(name)
            }
        }
    }

    fn load_impl(&mut self, name: &str) {
        let mut objects = self.bootstrap_class_loader.load_object(name);
        for object in objects.drain(..) {
            self.initialized.insert(object.name().clone(), false);
            let name = object.name().clone();
            self.objects.insert(object.name().clone(), object);
            self.construct_associated_class_object(&name);
        }
    }

    fn initialize_impl(&mut self, name: &str) {
        match self.objects.contains_key(name) {
            true => {}
            false => self.load_impl(name),
        }
        match self.initialized.get(name) {
            Some(initialized) => {
                if !initialized {
                    if let Some(true) = self.being_initialized.get(name) {
                        return;
                    }
                    self.being_initialized.insert(name.to_string(), true);
                    let object = self.objects.get(name).unwrap();
                    if object.get_method("<clinit>", "()V").is_some() {
                        let mut interpreter = Interpreter::new();
                        interpreter.run(name, "<clinit>", "()V");
                    }
                    self.initialized.insert(name.to_string(), true);
                    self.being_initialized.remove_entry(name);
                }
            }
            _ => panic!("Error initializing object: {}", name),
        };
    }

    fn instantiate_impl(&mut self, name: &str) -> Reference {
        let object = self.get_impl(name).clone();
        match object.fields().capacity() {
            0 => Heap::allocate_interface(object),
            _ => Heap::allocate_class(object),
        }
    }

    fn construct_associated_class_object(&mut self, name: &str) {
        let base = self.get_impl(name).clone();
        let mut object = self.get_impl("java/lang/Class").clone();
        // FIXME: Set module
        object.put_field(Field::new(
            "module".to_string(),
            "Ljava/lang/Module;".to_string(),
            0x0002 | 0x0080,
            Some(Types::Reference(Reference::new())),
        ));
        // FIXME: Set classLoader
        object.put_field(Field::new(
            "classLoader".to_string(),
            "Ljava/lang/ClassLoader;".to_string(),
            0x0002 | 0x0010,
            Some(Types::Reference(Reference::new())),
        ));
        if object.is_class() {
            let reference = Heap::allocate_class(base);
            object.put_field(Field::new(
                "classData".to_string(),
                "Ljava/lang/Object;".to_string(),
                0x0002 | 0x0080,
                Some(Types::Reference(reference)),
            ));
        } else if object.is_interface() {
            let reference = Heap::allocate_interface(base);
            object.put_field(Field::new(
                "classData".to_string(),
                "Ljava/lang/Object;".to_string(),
                0x0002 | 0x0080,
                Some(Types::Reference(reference)),
            ));
        } else {
            panic!("Could not construct associated class object for {} because it is neither a class nor an interface", name);
        }
        let reference = Heap::allocate_class(object);
        self.associated_class_objects
            .insert(name.to_string(), reference);
    }
}
