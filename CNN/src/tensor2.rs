use rand::Rng;

pub type Tensor2 = Vec<Vec<i32>>;

#[allow(unused)]
pub fn rand_tensor(r:usize, c:usize) -> Tensor2 {
    let mut v = vec![vec![0;c];r];
    let mut rng = rand::thread_rng();

    for i in 0..r {
        for j in 0..c {
            v[i][j] = rng.gen_range(-100..100);
        }
    }
    v
}

pub fn tensor_add(t1:&Tensor2, t2:&Tensor2) -> Tensor2 {
    let (r, c) = (t1.len(), t1[0].len());
    let mut t3 = vec![vec![0;c];r];
    for i in 0..r {
        for j in 0..c {
            t3[i][j] = t1[i][j] + t2[i][j];
        }
    }
    t3
}

pub fn conv2d(image:&Tensor2, kernel:&Tensor2) -> Tensor2 {
    let (ir, ic) = (image.len(), image[0].len());
    let (kr, kc) = (kernel.len(), kernel[0].len());
    let r = ir - kr + 1;
    let c = ic - kc + 1;
    let mut v = vec![vec![0;c];r];
    for i in 0..r {
        for j in 0..c {
            let mut val = 0;
            for l in 0..kr {
                for m in 0..kr {
                    val += image[i+l][j+m] * kernel[l][m];
                }
            }
            v[i][j] = val;
        }
    }
    v
}

#[allow(unused)]
pub fn print_tensor(a:&Tensor2) {
    for i in a {
        println!("{:?}", i);
    }
    println!();
}