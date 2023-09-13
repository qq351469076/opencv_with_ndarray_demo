use ndarray::prelude::*;
use opencv::core::CV_8UC4;
use opencv::highgui::{imshow, wait_key};
use opencv::prelude::*;

use opencv::imgcodecs::{imread, IMREAD_COLOR};
use opencv::{core, highgui, imgcodecs, imgproc, videoio, Result};

fn color_cvt() {
    highgui::named_window("photo", 1).unwrap();

    let mat = imread("C:\\Users\\0002\\Desktop\\aa.png", IMREAD_COLOR).unwrap();

    let mut gray = Mat::default();

    // BGR -> RGB
    imgproc::cvt_color(&mat, &mut gray, imgproc::COLOR_BGR2RGB, 0).unwrap();

    imshow("photo", &gray).unwrap();

    wait_key(10000).unwrap();
}

fn get_custom_array() {
    use std::ffi::c_void;

    let mut a = Array::from_elem((3, 360, 640), 255);

    // 第一个参数为 选择某个维度, 比如有3维有3个数组, 选择第一个维度就是0, 第二维度是1, 第三维度是2
    // 第二个参数为  选择维度下的第多少行
    // 第三个参数为 某行的多少列, 也可以用范围, 比如0-2实际上就是选择第一列和第二列

    // 修改多维数组
    a.slice_mut(s![.., .., 200]).fill(28. as i32);

    // 无损将ndarray转化为Mat
    let mat = unsafe {
        Mat::new_rows_cols_with_data(360, 640, CV_8UC4, a.as_mut_ptr() as *mut c_void, 0)
    }
    .unwrap();
    imshow("img", &mat).unwrap();

    wait_key(10000).unwrap();

    // // 从矩阵中提取两个互不相交的可变切片
    // let (s0, s1) = a.multi_slice_mut((s![.., ..;2], s![.., 1..;2]));

    // // 通过数组转换成多维数组
    // let mut a = Array::range(0., 12., 1.).into_shape([3 ,4]).unwrap();

    println!("{:?}", a.slice(s![.., .., ..]));
}

fn main() {
    // get_custom_array();
    color_cvt()
}
