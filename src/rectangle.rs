
struct Rect {
    width: u32,
    height: u32
}
impl Rect {
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, inner: &Rect) -> bool {
        self.width >= inner.width && self.height >= inner.height
    }
}

pub fn main() {
    let r1: Rect = Rect {
        width: 12,
        height: 6
    };
    let r2: Rect = Rect {
        width: 24,
        height: 1
    };
    println!("{}", r1.can_hold(&r2));
    println!("{}", r2.can_hold(&r1));
}

