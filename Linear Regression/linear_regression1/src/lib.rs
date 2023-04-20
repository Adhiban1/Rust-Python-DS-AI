#[no_mangle]
pub extern "C" fn f(arr: *const f64, length:usize) -> * const f64{

    // get data from python
    let slice = unsafe{std::slice::from_raw_parts(arr, length)};

    // converting that data to x and y
    let x = slice[..length/2].to_vec();
    let y = slice[length/2..].to_vec();

    // initializing the variables
    let mut m = 0.0;
    let mut c = 0.0;
    const LR:f64 = 0.0001;
    // println!("m:{}, c:{}, Loss: {}", m, c, loss(&y, &x, m, c));
    let mut low_loss = loss(&y, &x, m, c);
    let mut corret_m = 0.0;
    let mut corret_c = 0.0;

    // iteration
    for _ in 0..100000 {
        let mut temp_m = 0.0;
        let mut temp_c = 0.0;
        for i in 0..length/2 {
            temp_m = x[i]*(y[i]-m*x[i]-c);
            temp_c = y[i]-m*x[i]-c;
        }
        m -= LR * -2.0 * temp_m / (length as f64); // update m
        c -= LR * -2.0 * temp_c / (length as f64); // update c
        if low_loss > loss(&y, &x, m, c) {
            low_loss = loss(&y, &x, m, c);
            corret_m = m; // take the best m value
            corret_c = c; // take the best c value
        }
        // println!("m:{}, c:{}, Loss: {}", m, c, loss(&y, &x, m, c));
    }

    // strore the values in vector
    let v = vec![corret_m,corret_c];
    let output = v.as_ptr();
    std::mem::forget(v);

    // return output as pointer f64
    output
}

fn loss(y:&Vec<f64>, x:&Vec<f64>, m:f64, c:f64) -> f64 {
    let mut l = 0.0;
    for i in 0..y.len() {
        l += (y[i] - m*x[i] - c).powi(2);
    }
    l / (y.len() as f64)
}