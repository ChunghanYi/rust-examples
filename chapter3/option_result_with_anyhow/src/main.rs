use std::io::{self};
use std::num::ParseIntError;
use anyhow::{Context, Result, anyhow}; // cargo.toml에 anyhow 추가 필요

// --- 시뮬레이션 함수들 ---
// 개별 함수들은 여전히 구체적인 에러 타입(io::Error 등)을 반환해도 됩니다.
// anyhow가 알아서 변환해줍니다.

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

// 1. 메인 로직: 반환 타입이 anyhow::Result<()> 입니다.
// 커스텀 Enum 정의가 완전히 사라졌습니다.
fn run_application() -> Result<()> {
    println!("1. 설정 파일 읽기 시도...");
    // .context()를 사용하면 에러 발생 시 "어떤 작업 중이었는지" 메시지를 추가할 수 있습니다.
    // 이는 에러 추적(Backtrace)에 매우 유용합니다.
    let content = read_config_file("config.txt")
        .context("설정 파일 읽기 실패")?; 
    println!("   -> 파일 내용: {}", content);

    println!("2. 설정 값 파싱 시도...");
    let number = parse_config_value(&content)
        .context("설정 값 파싱 실패")?;
    println!("   -> 파싱된 숫자: {}", number);

    println!("3. Option 처리 시도...");
    // Option에 대해서도 .context()를 쓰면 바로 Result로 변환됩니다.
    // None일 경우 자동으로 에러가 생성됩니다.
    let mode = get_env_var("USER_KEY")
        .context("USER_KEY 환경변수가 없습니다")?;
        
    println!("   -> 모드: {}", mode);

    Ok(())
}

fn main() {
    // anyhow는 Debug 출력({:?}) 시 에러 체인(Stack trace)을 예쁘게 보여줍니다.
    match run_application() {
        Ok(_) => println!("\n[Main] 프로그램 정상 종료"),
        Err(e) => {
            println!("\n[Main] 프로그램 에러 종료!");
            // {:#}를 사용하면 context와 원본 에러를 모두 출력해줍니다.
            println!("에러 발생: {:#}", e);
        }
    }
}