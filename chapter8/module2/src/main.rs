// =================================================================
// [1] 모듈 선언
// 컴파일러에게 "communication", "physics", "utils"라는 모듈이
// 별도의 파일(폴더/mod.rs)에 존재한다고 알립니다.
// =================================================================
mod communication;
mod physics;
mod utils;

// =================================================================
// [2] Main 함수
// 사용 코드는 이전과 거의 동일합니다.
// =================================================================
fn main() {
    println!("--- Rust Multi-File Module Demo ---\n");

    // 1. 절대 경로 사용
    crate::communication::radio::broadcast("Hello via Absolute Path!");
    crate::physics::gravity::calculate_drop();

    // 2. 상대 경로 사용 (main과 communication은 같은 레벨)
    communication::network::connect();
    communication::network::check_status();

    // 3. use 키워드 사용
    use crate::utils::string_tools;
    string_tools::to_uppercase("hello use keyword");

    // 4. 별칭(Alias) 사용
    use crate::utils::math_tools::add as sum;
    println!("[Math alias] 10 + 20 = {}", sum(10, 20));

    // 5. 구조체 사용
    use crate::physics::thermodynamics::Temperature;
    let mut temp = Temperature::new(36.5);
    temp.degrees = 37.2;
    temp.show();

    // 6. 중첩 경로 import
    use crate::communication::radio::{self, broadcast};
    radio::broadcast("Nested path works!");
    broadcast("Direct function works!");
}