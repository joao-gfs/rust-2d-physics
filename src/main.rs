use rust_2d_physics::{utils, Body};

fn main(){
    let mut b1 = Body::new(0.0, 0.0, 0.0, 0.0, 1.0, 1.0);
    println!("Passo 1: {b1:?}\n");

    b1.apply_force(9.8965, 3.654);
    println!("Passo 2: {b1:?}\n");

    b1.update(1.0);
    println!("Passo 3: {b1:?}\n");

    b1.update(0.543354);
    println!("Passo 4: {b1:?}\n");

    b1.apply_force(-9.8965, -3.654);
    b1.update(1.0);
    println!("Passo 5: {b1:?}\n");

    b1.apply_force(-4.0, -16.0);
    b1.update(1.0);
    println!("Passo 6: {b1:?}\n");

    let mut b2 = Body::new(-50.0, -50.0, 5.0, 5.0, 1.0, 15.0);
    let mut b3 = Body::new(50.0, 50.0, -5.0, -5.0, 1.0, 23.0);

    let mut has_collided = false;
    let mut step = 0;
    while !has_collided{
        b2.update(1.0);
        b3.update(1.0);
        has_collided = utils::check_collision(&b2, &b3);
        step += 1;
    }

    println!("Colidiu no step {step}");

}