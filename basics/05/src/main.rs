fn main() {
    let mut i = 0;
    let mut j = 0;
    println!("i (before loop) = {}", i);
    println!("j (before loop) = {}", j);

    'parent_loop: loop {
        i += 1;
        println!("i = {}", i);


        'child_loop: loop {
            j += 1;
            println!("j = {}", j);
            
            if j > 10 {
                break 'child_loop; 
                // break 'parent_loop; 
            }
        }


        if i > 10 {
           break 'parent_loop; 
        }
    }


    println!("i = {}", i);
}


