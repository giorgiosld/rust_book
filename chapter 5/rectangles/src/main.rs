struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Self) -> Self {
        let w = self.width.max(other.width);
        let h = self.height.max(other.height);
        Rectangle {
            width: w,
            height: h
        }
    }
}
fn main() {
    let rect = Rectangle {
        width: 0,
        height: 0
    };
    println!("{}", rect.area());

    let other_rect = Rectangle { width: 1, height: 1 };
    let max_rect = rect.max(other_rect);
}