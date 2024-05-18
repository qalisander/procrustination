struct MyStruct {
    field1: i32,
    field2: f64,
    field3: String,
}

impl MyStruct {
    // Method to get a reference to an inner field by its type
    fn get_field_ref<T>(&self) -> Option<&T> {
        if std::any::type_name::<T>() == std::any::type_name::<i32>() {
            // Casting to the desired type if it matches
            Some(&self.field1)
        } else if std::any::type_name::<T>() == std::any::type_name::<f64>() {
            Some(&self.field2)
        } else if std::any::type_name::<T>() == std::any::type_name::<String>() {
            Some(&self.field3)
        } else {
            None
        }
    }
}

fn main() {
    let my_struct = MyStruct {
        field1: 42,
        field2: 3.14,
        field3: "Hello, World!".to_string(),
    };

    // Get references to fields by type
    if let Some(field1_ref) = my_struct.get_field_ref::<i32>() {
        println!("Field1 reference: {}", field1_ref);
    }

    if let Some(field2_ref) = my_struct.get_field_ref::<f64>() {
        println!("Field2 reference: {}", field2_ref);
    }

    if let Some(field3_ref) = my_struct.get_field_ref::<String>() {
        println!("Field3 reference: {}", field3_ref);
    }
}
