fn main() {
    // functions
    let y = |x:f32| x.powi(2);
    let loss = |x:f32, yt:f32| (yt - y(x)).powi(2);
    let grad = |x:f32, yt:f32| (loss(x+0.000001, yt) - loss(x-0.000001, yt))/(0.000002);
    let yt:f32 = 0.0;

    // Variables
    let mut x:f32 = 10.0;
    let lr = 0.0001;

    // loop
    for i in 1..1001 {
        x -= lr*((i as f32).powi(3))*grad(x, yt);
        if i >= 995 {
            println!("Loss: {}", loss(x, yt));
        }
    }

    // result
    println!("x: {}", x);
}
