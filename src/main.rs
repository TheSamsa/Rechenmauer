mod mauer;

use mauer::Mauer;

fn main() {
    let mut mauer = Mauer::new(4);
    mauer.set((1, 1), 89);
    mauer.set((4, 1), 12);
    mauer.set((4, 3), 17);
    mauer.set((4, 4), 8);

    println!("{}", mauer);

    let new_mauer = mauer.solve();

    if let Some(new_mauer) = new_mauer {
        println!("{}", new_mauer);
    } else {
        println!("no possible result found");
    }
}
