use std::borrow::Borrow;
use std::convert::TryFrom;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

// ==========================================
// 1. Default
// 기본값을 정의하는 트레이트입니다.
// ==========================================
#[derive(Debug)]
struct GameConfig {
    volume: i32,
    resolution: String,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            volume: 50,
            resolution: String::from("1920x1080"),
        }
    }
}

// ==========================================
// 2. Clone & Copy
// Clone: 명시적인 복사 (.clone()) / 힙 메모리 데이터 복사 가능
// Copy: 암묵적인 비트 단위 복사 (stack-only), 소유권 이동이 일어나지 않음
// ==========================================
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)] // String을 포함하므로 Copy 불가능
struct Person {
    name: String,
}

// ==========================================
// 3. Drop
// 스코프를 벗어날 때 실행되는 소멸자 로직을 정의합니다.
// ==========================================
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!(">>> [Drop] 메모리 해제됨: {}", self.data);
    }
}

// ==========================================
// 4. Deref & DerefMut
// 스마트 포인터가 내부 데이터의 메서드에 접근할 수 있게 해줍니다 (* 연산자 오버로딩).
// ==========================================
struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// ==========================================
// 5. From & Into
// 타입 변환을 위한 트레이트입니다. From을 구현하면 Into는 자동으로 구현됩니다.
// ==========================================
#[derive(Debug)]
struct NumberWrapper(i32);

impl From<i32> for NumberWrapper {
    fn from(item: i32) -> Self {
        NumberWrapper(item)
    }
}

// ==========================================
// 6. TryFrom & TryInto
// 실패할 수 있는 타입 변환입니다. Result를 반환합니다.
// ==========================================
#[derive(Debug)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err("짝수가 아닙니다!")
        }
    }
}

// ==========================================
// 7. AsRef & AsMut
// 비용이 적게 드는 참조 변환(Reference-to-Reference conversion)을 수행합니다.
// ==========================================
fn print_length<T: AsRef<str>>(s: T) {
    // T가 String이든 &str이든 상관없이 &str로 취급
    println!("길이: {}", s.as_ref().len());
}

// ==========================================
// 8. Borrow & BorrowMut
// 데이터 구조(HashMap 등)에서 키를 조회할 때 유용합니다.
// 소유한 데이터(String)를 참조 형태(&str)로 빌릴 수 있게 해줍니다.
// Eq, Hash, Ord가 원본과 빌린 형태에서 동일하게 동작해야 한다는 계약이 있습니다.
// ==========================================
fn check_key<K, Q>(key: &K, query: &Q)
where
    K: Borrow<Q>, // K 타입을 Q 타입으로 빌릴 수 있어야 함
    Q: PartialEq + ?Sized,
{
    if key.borrow() == query {
        println!("키가 일치합니다!");
    } else {
        println!("키가 다릅니다.");
    }
}

// ==========================================
// 9. ToOwned
// Borrow된 데이터(예: &str)에서 소유권이 있는 데이터(예: String)를 생성합니다.
// Clone의 일반화된 형태입니다.
// ==========================================
fn make_owned(s: &str) -> String {
    s.to_owned() // &str -> String 변환 (내부적으로 복사 발생)
}

// ==========================================
// 10. Sized
// 컴파일 타임에 크기가 알려진 타입을 나타내는 마커 트레이트입니다.
// ?Sized는 크기가 알려지지 않을 수도 있음(DST)을 나타냅니다.
// ==========================================
fn generic_sized<T: Sized>(t: T) {
    println!("이 타입은 컴파일 타임에 크기가 정해져 있습니다.");
    // std::mem::size_of_val(&t); // 가능
}

fn generic_maybe_unsized<T: ?Sized>(t: &T) {
    println!("이 타입은 크기가 동적일 수 있으므로 참조로만 다룹니다.");
}


fn main() {
    println!("=== Rust Utility Traits Demo ===\n");

    // 1. Default
    let conf = GameConfig::default();
    println!("[Default] 기본 설정: {:?}", conf);

    // 2. Clone & Copy
    let p1 = Point { x: 10, y: 20 };
    let p2 = p1; // Copy 발생 (p1 여전히 사용 가능)
    println!("[Copy] p1: {:?}, p2: {:?}", p1, p2);

    let person1 = Person { name: "Alice".into() };
    let person2 = person1.clone(); // Clone (깊은 복사)
    // let person3 = person1; // 이 줄을 활성화하면 person1은 소유권 이동으로 사용 불가
    println!("[Clone] {:?} 복제됨", person2);

    // 3. Drop
    {
        let _ptr = CustomSmartPointer { data: String::from("중요한 데이터") };
        println!("[Drop] 스코프 내부");
    } // 여기서 Drop 호출됨
    println!("[Drop] 스코프 외부");

    // 4. Deref & DerefMut
    let mut my_box = MyBox(String::from("Hello"));
    // MyBox에는 len()이 없지만, Deref를 통해 String의 len() 호출 가능
    println!("[Deref] 길이: {}", my_box.len()); 
    // DerefMut을 통해 내부 String 수정 가능
    my_box.push_str(" World"); 
    println!("[DerefMut] 내용: {}", *my_box);

    // 5. From & Into
    let num = NumberWrapper::from(100);
    let num2: NumberWrapper = 200.into();
    println!("[From/Into] {:?}, {:?}", num, num2);

    // 6. TryFrom & TryInto
    let even = EvenNumber::try_from(4);
    let odd: Result<EvenNumber, _> = 5.try_into();
    println!("[TryFrom] 짝수 성공: {:?}", even);
    println!("[TryInto] 홀수 실패: {:?}", odd);

    // 7. AsRef
    println!("[AsRef] String 전달:");
    print_length(String::from("Rust"));
    println!("[AsRef] &str 전달:");
    print_length("Programming");

    // 8. Borrow
    let owner = String::from("key_value");
    // String을 소유하고 있지만 &str로 비교 가능 (Borrow 트레이트 덕분)
    print!("[Borrow] ");
    check_key(&owner, "key_value");

    // 9. ToOwned
    let borrowed_str: &str = "im borrowed";
    let owned_string: String = make_owned(borrowed_str);
    println!("[ToOwned] 소유권 생성: {:?}", owned_string);

    // 10. Sized vs ?Sized
    let x = 10;
    generic_sized(x);
    
    let slice: &str = "Dynamic Size";
    generic_maybe_unsized(slice); // str은 Sized가 아니지만 ?Sized로 허용
}