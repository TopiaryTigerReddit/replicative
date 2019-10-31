use replicative::{Leaf, set::GrowOnly, Replicative};
use std::collections::HashSet;

use futures::{executor::block_on, StreamExt};

fn main() {
    let mut set = GrowOnly::<HashSet<_>>::new();
    set.insert(Leaf::new("one".to_string()));
    let mut set_two = GrowOnly::<HashSet<_>>::new();
    set_two.insert(Leaf::new("two".to_string()));

    println!("{:?}", set);
    println!("{:?}", set_two);

    block_on(async {
        while let Some(op) = set.next().await {
            set_two.apply(op);
        }
    });

    println!("{:?}", set);
    println!("{:?}", set_two);

    set.merge(set_two.fetch()).unwrap();

    println!("{:?}", set);
    println!("{:?}", set_two);

    block_on(async {
        while let Some(op) = set_two.next().await {
            set.apply(op);
        }
    });

    println!("{:?}", set);
    println!("{:?}", set_two);
}
