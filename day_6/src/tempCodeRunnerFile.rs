for row in grid.iter() {
        row.iter().for_each(|n| {print!("{}",match n {0 => '.', 1 => '#', _ => 'X'})});
        println!();
    }