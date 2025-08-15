use gpo::Population;

fn main() {
    let mut population = Population::init()
        .dimensions(&[("x", -10.10, 10.0), ("y", -10.10, 10.0)])
        .objective(|p| {
            let x = p[0];
            let y = p[1];
            (x * x) + (y * y)
        })
        // .search(vec![
        //     |x: f64| { x + rand::random::<f64>() - 0.5 },
        //     |y: f64| { y + rand::random::<f64>() - 0.5 }
        // ])
        .cognitive(1.0)
        .inertia(0.5)
        .social(1.0)
        .build();

    let result = population.run(10_000);
    println!("{:?}", result);
}