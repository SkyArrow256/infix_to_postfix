mod calc;
use calc::Calc;

fn main() {
    let mut calc = Calc::new();
    loop {
        calc.run();
    }
}
