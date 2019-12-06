#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Param {
    Position(i32),
    Immediate(i32)
}

impl Param {
    pub fn build(n: i32, p: i32) -> Param {
        use Param::*;

        match n {
            0 => Position(p),
            1 => Immediate(p),
            _ => panic!("invalid param")
        }
    }

    pub fn resolve(&self, m: &[i32]) -> i32 {
        use Param::*;

        match *self {
            Position(i) => m[i as usize],
            Immediate(n) => n
        }
    }
}

#[test]
fn param_build() {
    use Param::*;

    assert_eq!(Param::build(0, 1), Position(1));
    assert_eq!(Param::build(0, -5), Position(-5));
    assert_eq!(Param::build(1, 1), Immediate(1));
    assert_eq!(Param::build(1, -5), Immediate(-5));
}