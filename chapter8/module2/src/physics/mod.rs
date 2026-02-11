// 이 파일 자체가 'physics' 모듈의 본체가 됩니다.

// 하위 모듈 1: 중력
pub mod gravity {
    pub fn calculate_drop() {
        println!("[Gravity] Calculating drop speed...");
    }
}

// 하위 모듈 2: 열역학
pub mod thermodynamics {
    pub struct Temperature {
        pub degrees: f64,
        unit: String,
    }

    impl Temperature {
        pub fn new(degrees: f64) -> Temperature {
            Temperature {
                degrees,
                unit: String::from("Celsius"),
            }
        }
        
        pub fn show(&self) {
            println!("[Thermodynamics] Temp: {} {}", self.degrees, self.unit);
        }
    }
}