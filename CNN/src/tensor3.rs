use rand::Rng;
use crate::tensor2;

pub type Tensor3 = Vec<Vec<Vec<i32>>>;

pub fn rand_tensor(cnl:usize, r:usize, c:usize) -> Tensor3 {
    let mut v = vec![vec![vec![0;c];r];cnl];
    let mut rng = rand::thread_rng();

    for i in 0..cnl {
        for j in 0..r {
            for k in 0..c {
                v[i][j][k] = rng.gen_range(-100..100);
            }
        }
    }
    v
}

pub fn conv2d(image:&Tensor3, kernel:&Tensor3) -> Tensor3 {
    let (ir, ic) = (image[0].len(), image[0][0].len());
    let (kr, kc) = (kernel[0].len(), kernel[0][0].len());
    let r = ir - kr + 1;
    let c = ic - kc + 1;
    let mut v = Vec::new();
    for kernel_cnl in kernel {
        let mut temp = vec![vec![0;c];r];
        for image_cnl in image {
            temp = tensor2::tensor_add(&temp, &tensor2::conv2d(&image_cnl, &kernel_cnl));
        }
        v.push(temp);
    }
    v
}

pub fn print_tensor(a:&Tensor3) {
    for i in a {
        tensor2::print_tensor(&i);
    }
}