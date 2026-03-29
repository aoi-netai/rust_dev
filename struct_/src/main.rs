use std::iter::Cloned;

struct SensorData{

    imu_active: bool,
    accel: [f32; 3], 
    gyro: [f32; 3],
}

// タプル構造体
struct ColorRGB(u8, u8, u8);

// SensorDataのインスタンスの可変の参照を受け取る
fn imu_activate(sensor: &mut SensorData){

    sensor.imu_active = true;
}

fn imu_create(imu_active: bool) -> SensorData{

    // 仮引数と構造体のメンバーが同じ場合は省略初期化ができる
    let mut sensor = SensorData{

        imu_active,
        accel: [0.0, 0.0, 0.0],
        gyro: [0.0, 0.0, 0.0],
    };

    sensor
}

fn main() {

    println!("Program Start");

    // これはインスタンス全体が可変ではない
    let imu_01 = SensorData{

        imu_active: true,
        accel: [0.0, 0.0, 0.0],
        gyro: [0.0, 0.0, 0.0],
    };

    println!("imu_01_status: {}, accel {:?}, gyro {:?}", imu_01.imu_active, imu_01.accel, imu_01.gyro);

    // これはインスタンス全体が可変
    let mut imu_02 = SensorData{

        imu_active: false,
        accel: [0.0, 0.0, 0.0],
        gyro: [0.0, 0.0, 0.0],
    };

    println!("imu_02_status: {}, accel {:?}, gyro {:?}", imu_02.imu_active, imu_02.accel, imu_02.gyro);

    // 可変の参照を渡す
    imu_activate(&mut imu_02);   

    imu_02.accel = [0.1, -0.2, 9.8];

    println!("imu_02_status: {}, accel {:?}, gyro {:?}", imu_02.imu_active, imu_02.accel, imu_02.gyro);

    let imu_03 = imu_create(false);
    println!("imu_03_status: {}, accel {:?}, gyro {:?}", imu_03.imu_active, imu_03.accel, imu_03.gyro);

    // タプル構造体
    let black = ColorRGB(0,0,0);
    let White = ColorRGB(255, 255, 255);

}
