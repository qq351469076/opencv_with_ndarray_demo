use ndarray::prelude::*;
use opencv::core::{Point, Rect, Scalar, CV_8UC3, CV_8UC4};
use opencv::highgui::{imshow, wait_key};
use opencv::imgcodecs::{imread, IMREAD_COLOR};
use opencv::imgproc::LINE_8;
use opencv::prelude::*;
use opencv::{highgui, imgproc};
use std::ffi::c_void;

/// 绘制文字
fn draw_text() {
    // 定义3通道, 640*480的图像
    let mut n = Array::<u8, _>::zeros((3, 480, 640).f());

    // 将多维数组免拷贝转换至Mat
    let mut mat = unsafe {
        Mat::new_rows_cols_with_data(480, 640, CV_8UC3, n.as_mut_ptr() as *mut c_void, 0).unwrap()
    };

    println!("{:?}", mat.channels());

    // 起始坐标, 终止点
    imgproc::put_text(
        &mut mat,
        "hello world",
        // 文本字符串在图像中的左下角坐标。
        Point { x: 10, y: 400 },
        // 字体类型
        highgui::QT_FONT_NORMAL,
        // 字体缩放因子，乘以字体特定的基础大小。
        0.8,
        // 字体颜色
        Scalar::new(255., 0., 0., 0.),
        // 线宽
        2,
        // 线条连接方式, 默认为8连通线  FILLED:线条内填充   LINE_4   LINE_8   LINE_AA:抗锯齿
        LINE_8,
        // 当为true时，图像数据原点位于左下角；否则，位于左上角。
        false,
    )
    .unwrap();

    imshow("ddd", &mat).unwrap();

    wait_key(10000).unwrap();
}

/// 画矩形
fn draw_rectangle() {
    // 定义3通道, 640*480的图像
    let mut n = Array::<u8, _>::zeros((3, 480, 640).f());

    // 将多维数组免拷贝转换至Mat
    let mut mat = unsafe {
        Mat::new_rows_cols_with_data(480, 640, CV_8UC3, n.as_mut_ptr() as *mut c_void, 0).unwrap()
    };

    // 起始坐标, 终止点
    imgproc::rectangle(
        &mut mat,
        Rect {
            x: 10,
            y: 20,
            // 矩形需要多宽
            width: 10 * 10,
            // 矩形需要多高
            height: 20 * 4,
        },
        // 线条颜色, 前三个是通道数BGR, 最后一个是图像透明度
        Scalar::new(0f64, 0f64, 255f64, 0f64),
        // 线宽
        -1,
        // 线条连接方式, 默认为8连通线  FILLED:线条内填充   LINE_4   LINE_8   LINE_AA:抗锯齿
        LINE_8,
        // 坐标缩放比例, 简单来说就是控制线条坐标的精度
        0,
    )
    .unwrap();

    imshow("ddd", &mat).unwrap();

    wait_key(10000).unwrap();
}

/// 画直线
fn draw_line() {
    // 定义3通道, 640*480的图像
    let mut n = Array::<u8, _>::zeros((3, 480, 640).f());

    // 将多维数组免拷贝转换至Mat
    let mut mat = unsafe {
        Mat::new_rows_cols_with_data(480, 640, CV_8UC3, n.as_mut_ptr() as *mut c_void, 0).unwrap()
    };

    // 起始坐标, 终止点
    imgproc::line(
        &mut mat,
        // 起始点
        Point { x: 10, y: 20 },
        // 终止点
        Point { x: 300, y: 400 },
        // 线条颜色, 前三个是通道数BGR, 最后一个是图像透明度
        Scalar::new(0f64, 0f64, 255f64, 0f64),
        // 线宽
        1,
        // 线条连接方式, 默认为8连通线  FILLED:线条内填充   LINE_4   LINE_8   LINE_AA:抗锯齿
        LINE_8,
        // 坐标缩放比例, 简单来说就是控制线条坐标的精度
        0,
    )
    .unwrap();

    imshow("ddd", &mat).unwrap();

    wait_key(10000).unwrap();
}

/// 颜色转换
fn color_cvt() {
    highgui::named_window("photo", 1).unwrap();

    let mat = imread("C:\\Users\\0002\\Desktop\\aa.png", IMREAD_COLOR).unwrap();

    let mut gray = Mat::default();

    // BGR -> RGB
    imgproc::cvt_color(&mat, &mut gray, imgproc::COLOR_BGR2RGB, 0).unwrap();

    imshow("photo", &gray).unwrap();

    wait_key(10000).unwrap();
}

/// 自定义数组
fn get_custom_array() {
    use std::ffi::c_void;

    let mut a = Array::from_elem((3, 480, 640), 255);

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
    // color_cvt()
    // draw_line();
    // draw_rectangle();
    // draw_text();
}
