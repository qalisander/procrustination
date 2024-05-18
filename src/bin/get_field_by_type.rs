struct MyStruct {
    field1: i32,
    field2: f64,
    field3: String,
}

trait FieldAccessor<T> {
    fn get_field_ref<'a>(&self, _: &'a MyStruct) -> Option<&'a T>;
}

impl FieldAccessor<i32> for () {
    fn get_field_ref<'a>(&self, my_struct: &'a MyStruct) -> Option<&'a i32> {
        Some(&my_struct.field1)
    }
}

impl FieldAccessor<f64> for () {
    fn get_field_ref<'a>(&self, my_struct: &'a MyStruct) -> Option<&'a f64> {
        Some(&my_struct.field2)
    }
}

impl FieldAccessor<String> for () {
    fn get_field_ref<'a>(&self, my_struct: &'a MyStruct) -> Option<&'a String> {
        Some(&my_struct.field3)
    }
}

fn main() {
    let my_struct = MyStruct {
        field1: 42,
        field2: 3.14,
        field3: "Hello, World!".to_string(),
    };

    // Get references to fields by type
    let accessor = ();
    let field_i32: Option<&i32> = accessor.get_field_ref(&my_struct);
    println!("field_i32 reference: {:?}", field_i32);

    let field_f64: Option<&f64> = accessor.get_field_ref(&my_struct);
    println!("field_f64 reference: {:?}", field_f64);

    let field_String: Option<&String> = accessor.get_field_ref(&my_struct);
    println!("field_f64 reference: {:?}", field_String);
}
