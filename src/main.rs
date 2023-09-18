use ndarray::prelude::*;
use opencv::core::{
    bitwise_and, bitwise_not, bitwise_or, bitwise_xor, no_array, Mat_AUTO_STEP, Point, Rect,
    Scalar, CV_8UC1, CV_8UC3, CV_8UC4,
};
use opencv::highgui::{imshow, wait_key};
use opencv::imgcodecs::{imread, IMREAD_COLOR};
use opencv::imgproc::{COLOR_BGR2RGB, LINE_8};
use opencv::prelude::*;
use opencv::{highgui, imgproc};
use std::ffi::c_void;

/// 异或操作, 交集为0, 不交集为1
///
/// 涉及创建单通道及索引赋值
fn xor_operate() {
    // A数组
    let mut a = Array::<u8, _>::zeros((200, 200).f());

    // 按行进行遍历修改
    a.slice_mut(s![20..120, 20..120]).fill(255);

    // 将多维数组免拷贝转换至Mat
    let mat_a = unsafe {
        Mat::new_rows_cols_with_data(200, 200, CV_8UC1, a.as_mut_ptr() as *mut c_void, 0).unwrap()
    };

    imshow("mat_a", &mat_a).unwrap();

    // B数组
    let mut b = Array::<u8, _>::zeros((200, 200).f());

    // 按行进行遍历修改
    b.slice_mut(s![80..180, 80..180]).fill(255);

    // 将多维数组免拷贝转换至Mat
    let mat_b = unsafe {
        Mat::new_rows_cols_with_data(200, 200, CV_8UC1, b.as_mut_ptr() as *mut c_void, 0).unwrap()
    };

    imshow("mat_b", &mat_b).unwrap();

    let mut new_mat = Mat::default();

    bitwise_xor(&mat_a, &mat_b, &mut new_mat, &opencv::core::no_array()).unwrap();

    imshow("and", &new_mat).unwrap();

    wait_key(10000).unwrap();
}

/// 或操作, 取两个图片共有的值, 无论是0还是非0
///
/// 涉及创建单通道及索引赋值
fn or_operate() {
    // A数组
    let mut a = Array::<u8, _>::zeros((200, 200).f());

    // 按行进行遍历修改
    a.slice_mut(s![20..120, 20..120]).fill(255);

    // 将多维数组免拷贝转换至Mat
    let mat_a = unsafe {
        Mat::new_rows_cols_with_data(200, 200, CV_8UC1, a.as_mut_ptr() as *mut c_void, 0).unwrap()
    };

    imshow("mat_a", &mat_a).unwrap();

    // B数组
    let mut b = Array::<u8, _>::zeros((200, 200).f());

    // 按行进行遍历修改
    b.slice_mut(s![80..180, 80..180]).fill(255);

    // 将多维数组免拷贝转换至Mat
    let mat_b = unsafe {
        Mat::new_rows_cols_with_data(200, 200, CV_8UC1, b.as_mut_ptr() as *mut c_void, 0).unwrap()
    };

    imshow("mat_b", &mat_b).unwrap();

    let mut new_mat = Mat::default();

    bitwise_or(&mat_a, &mat_b, &mut new_mat, &opencv::core::no_array()).unwrap();

    imshow("and", &new_mat).unwrap();

    wait_key(10000).unwrap();
}

/// 与操作, 取两个图片的交集, 或者同时为真, 则为真, 否则为假
///
/// 涉及创建单通道及索引赋值
fn and_operate() {
    // A数组
    let mut a = Array::<u8, _>::zeros((200, 200).f());

    // 按行进行遍历修改
    a.slice_mut(s![20..120, 20..120]).fill(255);

    // 将多维数组免拷贝转换至Mat
    let mat_a = unsafe {
        Mat::new_rows_cols_with_data(200, 200, CV_8UC1, a.as_mut_ptr() as *mut c_void, 0).unwrap()
    };

    imshow("mat_a", &mat_a).unwrap();

    // B数组
    let mut b = Array::<u8, _>::zeros((200, 200).f());

    // 按行进行遍历修改
    b.slice_mut(s![80..180, 80..180]).fill(255);

    // 将多维数组免拷贝转换至Mat
    let mat_b = unsafe {
        Mat::new_rows_cols_with_data(200, 200, CV_8UC1, b.as_mut_ptr() as *mut c_void, 0).unwrap()
    };

    imshow("mat_b", &mat_b).unwrap();

    let mut new_mat = Mat::default();

    bitwise_and(&mat_a, &mat_b, &mut new_mat, &opencv::core::no_array()).unwrap();

    imshow("and", &new_mat).unwrap();

    wait_key(10000).unwrap();
}

/// 非操作, 翻转数组元素, 0变1, 1变0
///
/// 涉及创建单通道及索引赋值
fn not_operate() {
    let mut n = Array::<u8, _>::zeros((200, 200).f());

    // 按行进行遍历修改
    n.slice_mut(s![50..150, 50..150]).fill(255);

    // 将多维数组免拷贝转换至Mat
    let mat = unsafe {
        Mat::new_rows_cols_with_data(200, 200, CV_8UC1, n.as_mut_ptr() as *mut c_void, 0).unwrap()
    };

    imshow("3333", &mat).unwrap();

    let mut new_mat = Mat::default();

    bitwise_not(&mat, &mut new_mat, &opencv::core::no_array()).unwrap();

    imshow("asdasd", &new_mat).unwrap();

    wait_key(10000).unwrap();
}

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
    // not_operate();
    // and_operate();
    // or_operate();
    // xor_operate();
}
