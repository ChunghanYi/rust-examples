// 이 파일 자체가 'communication' 모듈의 본체가 됩니다.

// 하위 모듈 1: 라디오
pub mod radio {
    pub fn broadcast(message: &str) {
        println!("[Radio] Broadcasting: {}", message);
    }
}

// 하위 모듈 2: 네트워크
pub mod network {
    pub fn connect() {
        println!("[Network] Connected to server.");
    }
    
    // 상대 경로 테스트
    pub fn check_status() {
        print!("[Network Status] ");
        // 여기서 super는 이 파일(mod.rs), 즉 communication 모듈을 가리킵니다.
        // 따라서 같은 파일 내에 있는 radio 모듈에 접근 가능합니다.
        super::radio::broadcast("Internal Check OK"); 
    }
}