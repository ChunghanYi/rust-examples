use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;
use thiserror::Error; // cargo.toml에 thiserror 추가 필요

// 1. thiserror를 사용하여 매크로로 Display와 From 구현 자동화
#[derive(Debug, Error)]
enum AppError {
    // #[error("...")]는 Display 트레잇을 자동 구현
    // #[from]은 From 트레잇을 자동 구현하여 자동 형변환 지원
    #[error("IO 오류 발생: {0}")]
    Io(#[from] io::Error),

    #[error("파싱 오류 발생: {0}")]
    Parse(#[from] ParseIntError),

    // From 트레잇이 필요 없는 경우(직접 생성하는 에러)는 #[from] 생략
    #[error("데이터 누락: {0}")]
    MissingData(String),
}

// --- 시뮬레이션 함수들 (기존과 동일) ---

fn read_config_file(path: &str) -> Result<String, io::Error> {
    if path == "bad_path" {
        return Err(io::Error::new(io::ErrorKind::NotFound, "파일을 찾을 수 없습니다"));
    }
    Ok(String::from("42"))
}

fn parse_config_value(text: &str) -> Result<i32, ParseIntError> {
    let val: i32 = text.trim().parse()?;
    Ok(val)
}

fn get_env_var(key: &str) -> Option<String> {
    if key == "MODE" {
        Some("PROD".to_string())
    } else {
        None
    }
}

// 2. 메인 로직
fn run_application() -> Result<(), AppError> {
    println!("1. 설정 파일 읽기 시도...");
    // #[from] 덕분에 io::Error -> AppError::Io 자동 변환
    let content = read_config_file("config.txt")?; 
    println!("   -> 파일 내용: {}", content);

    println!("2. 설정 값 파싱 시도...");
    // #[from] 덕분에 ParseIntError -> AppError::Parse 자동 변환
    let number = parse_config_value(&content)?;
    println!("   -> 파싱된 숫자: {}", number);

    println!("3. Option 처리 시도...");
    // Option은 여전히 수동으로 에러 변환 필요
    let mode = get_env_var("USER_KEY")
        .ok_or_else(|| AppError::MissingData("USER_KEY 환경변수가 없습니다".to_string()))?;
        
    println!("   -> 모드: {}", mode);

    Ok(())
}

fn main() {
    match run_application() {
        Ok(_) => println!("\n[Main] 프로그램 정상 종료"),
        Err(e) => {
            println!("\n[Main] 프로그램 에러 종료!");
            println!("에러 상세: {}", e); // thiserror의 #[error] 메시지 출력
            
            // Enum 구조가 살아있으므로 패턴 매칭 가능
            match e {
                AppError::Io(_) => println!("(파일 시스템을 확인하세요)"),
                AppError::MissingData(_) => println!("(설정을 확인하세요)"),
                _ => {}
            }
        }
    }
}