// 이 파일 자체가 'utils' 모듈의 본체가 됩니다.

// 하위 모듈 1: 문자열 도구
pub mod string_tools {
    pub fn to_uppercase(s: &str) {
        println!("[StringTools] Result: {}", s.to_uppercase());
    }
}

// 하위 모듈 2: 수학 도구
pub mod math_tools {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}