mod mauer;

use mauer::Mauer;
use mauer::Position;

fn main() {
    let mut mauer: Mauer = Mauer::new(4);
    mauer.set(Position::new(1, 1), 100);
    mauer.set(Position::new(4, 1), 12);
    mauer.set(Position::new(4, 3), 17);
    mauer.set(Position::new(4, 4), 8);

    println!("{}", mauer);

    mauer.solve();

    println!("{}", mauer);
}
