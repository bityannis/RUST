//Applying Design pattern as seen in OOP

//Initiate pub (for export) struct User:
pub struct User {
    pub name: String,
    pub email: String,
    pub is_active: bool,
    pub sign_in_count: u64,
}

#[derive(Debug)] // Add this to be able to print whole instance (every field)
                 //Struct with method and optional field.
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
    pub area_result: Option<u32>, //Optional field can be initialize with None or Some(default value)
}

//Method (function withing struct definition). Rectangle now has area function that modify area_result field.
impl Rectangle {
    pub fn area(&mut self) {
        self.area_result = Some(self.width * self.height); //Some because Option enum has two variants: Some and None.
    }

    //Other method that return value
    pub fn return_area(&self) -> u32 {
        self.width * self.height
    }

    //Comparing two Rectangles to see which has the bigger area
    pub fn can_hold(&self, rectangle: &Rectangle) -> bool {
        if self.return_area() > rectangle.return_area() {
            true
        } else {
            false
        }

        //or just self.return_area() > rectangle.return_area() without if/else block
    }
}
