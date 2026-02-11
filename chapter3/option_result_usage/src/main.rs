use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

// 1. 모든 에러를 통합할 사용자 정의 Enum 정의
// 여기서는 IO 에러, 숫자 파싱 에러, 그리고 데이터가 없는 경우(Option 처리)를 포함합니다.
#[derive(Debug)]
enum AppError {
    Io(io::Error),              // 파일 읽기 실패 등
    Parse(ParseIntError),       // 문자열 -> 숫자 변환 실패
    MissingData(String),        // Option::None 처리를 위한 커스텀 에러
}

// 에러 출력을 위해 Display 트레잇 구현 (사용자 친화적 메시지)
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "IO 오류 발생: {}", err),
            AppError::Parse(err) => write!(f, "파싱 오류 발생: {}", err),
            AppError::MissingData(msg) => write!(f, "데이터 누락: {}", msg),
        }
    }
}

// std::error::Error 트레잇 구현 (선택 사항이지만 모범 사례임)
impl std::error::Error for AppError {}

// 2. 'From' 트레잇 구현: 하위 에러들을 AppError로 자동 변환하기 위함
// 이를 구현하면 '?' 연산자 사용 시 자동으로 변환됩니다.

// io::Error -> AppError 변환
impl From<io::Error> for AppError {
    fn from(err: io::Error) -> AppError {
        AppError::Io(err)
    }
}

// ParseIntError -> AppError 변환
impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> AppError {
        AppError::Parse(err)
    }
}

// --- 시뮬레이션 함수들 ---

// (A) Result를 반환하는 함수: 파일 읽기 시뮬레이션 (IO Error 발생 가능)
fn read_config_file(path: &str) -> Result<String, io::Error> {
    // 실제 파일이 없으므로 에러를 낼 수도 있고 성공을 흉내낼 수도 있습니다.
    // 여기서는 데모를 위해 성공한 척 하거나, 경로가 "bad"면 에러를 냅니다.
    if path == "bad_path" {
        return Err(io::Error::new(io::ErrorKind::NotFound, "파일을 찾을 수 없습니다"));
    }
    
    // 정상 케이스: 파일 내용에 숫자가 들어있다고 가정
    Ok(String::from("42"))
}

// (B) Result를 반환하는 함수: 파싱 시뮬레이션 (ParseIntError 발생 가능)
fn parse_config_value(text: &str) -> Result<i32, ParseIntError> {
    let val: i32 = text.trim().parse()?; // 여기서 에러나면 ParseIntError 리턴
    Ok(val)
}

// (C) Option을 반환하는 함수: 환경변수 가져오기 시뮬레이션
fn get_env_var(key: &str) -> Option<String> {
    if key == "MODE" {
        Some("PROD".to_string())
    } else {
        None
    }
}

// 3. 메인 로직: 여러 에러를 하나의 AppError로 처리
fn run_application() -> Result<(), AppError> {
    println!("1. 설정 파일 읽기 시도...");
    // [중요] ? 연산자 사용
    // read_config_file은 io::Error를 뱉지만, impl From 덕분에 자동으로 AppError::Io로 변환되어 반환됨
    let content = read_config_file("config.txt")?; 
    println!("   -> 파일 내용: {}", content);

    println!("2. 설정 값 파싱 시도...");
    // parse_config_value는 ParseIntError를 뱉지만, 자동으로 AppError::Parse로 변환됨
    let number = parse_config_value(&content)?;
    println!("   -> 파싱된 숫자: {}", number);

    println!("3. Option 처리 시도...");
    // get_env_var는 Option을 반환합니다.
    // Option을 Result로 변환하려면 .ok_or()를 사용합니다.
    // None일 경우 AppError::MissingData로 변환하여 에러 전파
    let mode = get_env_var("USER_KEY")
        .ok_or(AppError::MissingData("USER_KEY 환경변수가 없습니다".to_string()))?;
        
    println!("   -> 모드: {}", mode);

    println!("모든 작업 성공!");
    Ok(())
}

fn main() {
    // 최종적으로 main에서 단일한 AppError 타입으로 결과를 받습니다.
    match run_application() {
        Ok(_) => println!("\n[Main] 프로그램 정상 종료"),
        Err(e) => {
            // e는 AppError 타입입니다.
            println!("\n[Main] 프로그램 에러 종료!");
            println!("에러 상세: {}", e); // Display 트레잇 덕분에 깔끔하게 출력
            
            // 필요하다면 에러 종류에 따라 다른 처리를 할 수도 있습니다.
            match e {
                AppError::Io(_) => println!("(파일 시스템을 확인하세요)"),
                AppError::MissingData(_) => println!("(설정을 확인하세요)"),
                _ => {}
            }
        }
    }
}