use my_macro::make_answer;

make_answer!();

fn main() {
    assert!(answer() == 42);
}
