struct  DataManager {

    number: i32,
}

impl DataManager {
    
    fn set_data(&mut self, number: i32){

        self.number = number;
    }

    fn get_data(&self) -> i32{

        self.number
    }

    fn mul2(&mut self){

        self.number *= 2;
    }
}

fn main() {

    let mut data_manager: DataManager = DataManager{
        
        number: 0,
    };

    data_manager.set_data(120);

    println!("number: {}", data_manager.get_data());

    data_manager.mul2();

    println!("number: {}", data_manager.get_data());
}
