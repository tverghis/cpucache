use std::time::Instant;

fn main() {
    let strides = [1, 16];
    let mut array = vec![0i32; 64 * 1024 * 1024];

    for stride in strides {
        let start = Instant::now();
        workload(&mut array, stride);
        let dur = start.elapsed().as_millis();

        println!("(stride={stride}) duration: {}ms", dur);
    }
}

fn workload(arr: &mut [i32], stride: usize) {
    for i in (0..arr.len()).step_by(stride) {
        arr[i] *= 3;
    }
}
