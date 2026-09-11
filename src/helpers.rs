pub struct Tab {
    pub file_name: String,
    pub input_box: String,
    pub cursor_x: i32,
    pub cursor_y: i32,
    pub gcursor: i32
}

impl Tab {
    pub fn new() -> Self
    {
        Self { 
            file_name: String::from(""),
            input_box : String::new(),
            cursor_x : 0,
            cursor_y : 0,
            gcursor : 0
        }
    }
}
