use std::collections::VecDeque;

pub struct MovingAverage3D{

    window_size: usize,

    x: VecDeque<f32>,
    y: VecDeque<f32>,
    z: VecDeque<f32>,

    sum_x: f32,
    sum_y: f32,
    sum_z: f32,
    
    result_x: f32,
    result_y: f32,
    result_z: f32,
}

impl MovingAverage3D{

    pub fn new(size: usize) -> Self{

        assert!(size > 0);

        Self{

            window_size: size,
            x: VecDeque::with_capacity(size),
            y: VecDeque::with_capacity(size),
            z: VecDeque::with_capacity(size),

            sum_x: 0.0,
            sum_y: 0.0,
            sum_z: 0.0,

            result_x: 0.0,
            result_y: 0.0,
            result_z: 0.0,
        }
    }

    pub fn update(&mut self, data: [f32; 3]){

        if self.x.len() == self.window_size {
            self.sum_x -= self.x.pop_front().unwrap();
            self.sum_y -= self.y.pop_front().unwrap();
            self.sum_z -= self.z.pop_front().unwrap();
        }

        self.x.push_back(data[0]);
        self.y.push_back(data[1]);
        self.z.push_back(data[2]);

        self.sum_x += data[0];
        self.sum_y += data[1];
        self.sum_z += data[2];

        let data_count = self.x.len() as f32;

        self.result_x = self.sum_x / data_count;
        self.result_y = self.sum_y / data_count;
        self.result_z = self.sum_z / data_count;

    }   

    pub fn getdata(&self) -> [f32; 3]{

        let data: [f32; 3]= [self.result_x, self.result_y, self.result_z];
        data
    }
}

#[cfg(test)]
mod tests {
    use super::MovingAverage3D;

    fn assert_array_near(actual: [f32; 3], expected: [f32; 3]) {
        for index in 0..3 {
            assert!(
                (actual[index] - expected[index]).abs() < f32::EPSILON,
                "index {index}: actual {}, expected {}",
                actual[index],
                expected[index]
            );
        }
    }

    #[test]
    fn first_n_updates_are_divided_by_available_data_count() {
        let mut moving_average = MovingAverage3D::new(3);

        moving_average.update([3.0, 6.0, 9.0]);
        assert_array_near(moving_average.getdata(), [3.0, 6.0, 9.0]);

        moving_average.update([6.0, 9.0, 12.0]);
        assert_array_near(moving_average.getdata(), [4.5, 7.5, 10.5]);

        moving_average.update([9.0, 12.0, 15.0]);
        assert_array_near(moving_average.getdata(), [6.0, 9.0, 12.0]);
    }

    #[test]
    fn after_n_updates_oldest_value_is_removed() {
        let mut moving_average = MovingAverage3D::new(3);

        moving_average.update([3.0, 6.0, 9.0]);
        moving_average.update([6.0, 9.0, 12.0]);
        moving_average.update([9.0, 12.0, 15.0]);
        moving_average.update([12.0, 15.0, 18.0]);

        assert_array_near(moving_average.getdata(), [9.0, 12.0, 15.0]);
    }
}
