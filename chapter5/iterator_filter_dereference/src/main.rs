fn main() {
    // ==========================================================
    // 1. 기본 개념: 참조(Reference)와 이중 참조(Double Reference)
    // ==========================================================
    println!("--- 1. 기본 개념 ---");

    let val: i32 = 10;
    let ref1: &i32 = &val;        // 싱글 참조
    let ref2: &&i32 = &ref1;      // 이중 참조 (참조에 대한 참조)

    // 값을 꺼내기 위해서는 참조된 횟수만큼 역참조(*)해야 합니다.
    println!("val: {}", val);
    println!("*ref1: {}", *ref1);
    println!("**ref2: {}", **ref2); // 두 번 벗겨야 원본 i32 도달

    // ==========================================================
    // 2. Closure와 이중 참조 (가장 헷갈리는 부분)
    // ==========================================================
    println!("\n--- 2. Closure와 이중 참조 ---");

    let numbers = vec![1, 2, 3, 4, 5];

    // 시나리오: vector의 iter()는 요소를 빌려줍니다(&i32).
    // filter()는 그 빌려온 요소에 대한 참조를 또 만듭니다(&(&i32)).
    // 즉, 클로저 인자 x의 타입은 &&i32가 됩니다.

    // [Case A] 정석적인 완전 명시적 방법
    // x는 &&i32이므로, 값 비교를 위해 **를 사용해 i32로 만듦
    let count_explicit = numbers.iter().filter(|x| {
        // x의 타입: &&i32
        **x > 3 
    }).count();
    println!("Case A (Explicit **): {}", count_explicit);

    // [Case B] 구조 분해 (Destructuring) 사용 - 추천 방식
    // 인자 패턴 매칭에서 &&를 벗겨버림. 이러면 내부에서 x는 i32가 됨.
    let count_destructure = numbers.iter().filter(|&&x| {
        // x의 타입: i32 (이미 껍질을 벗기고 받음)
        x > 3
    }).count();
    println!("Case B (Destructuring &&): {}", count_destructure);

    // [Case C] 암묵적 처리 (Auto-deref & Method Call)
    // 질문하신 "내부적으로 처리되는 부분"입니다.
    let count_implicit = numbers.iter().filter(|x| {
        // x의 타입: &&i32
        
        // 1. Dot Operator (.)의 마법
        // Rust에서 메서드를 호출할 때(.), 컴파일러는 필요한 만큼 자동으로 역참조를 수행합니다.
        // x가 &&i32라도, i32의 메서드인 abs() 등을 찾기 위해 자동으로 Deref를 수행합니다.
        //let _test_method = x.abs(); // (*(*x)).abs() 와 동일하게 처리됨

        // 2. println! 등의 매크로
        // 매크로는 참조를 자동으로 따라가서 값을 출력해줍니다.
        // println!("{}", x); // 가능함

        // 3. 비교 연산자 (PartialEq)
        // Rust는 &T와 &T, 혹은 T와 T의 비교를 엄격히 따집니다.
        // 하지만 **x > 3 처럼 하지 않고, 참조 레벨을 맞춰주면 비교가 가능합니다.
        // x는 &&i32, &3은 &i32. 서로 타입이 안 맞아서 아래는 원래 에러가 날 수 있지만,
        // 보통은 **x > 3으로 값을 비교하거나, 구조 분해를 씁니다.
        
        // 여기서는 명시적 역참조가 가장 안전하고 확실한 방법입니다.
        **x > 3 
    }).count();
    println!("Case C (Implicit/Method): {}", count_implicit);


    // ==========================================================
    // 3. 헷갈리는 상황 정리 (x vs &x vs &&x)
    // ==========================================================
    println!("\n--- 3. 인자 패턴에 따른 x의 타입 변화 ---");
    
    // 상황: iter() -> &i32 yield
    //      filter() -> 인자로 &(&i32) 전달

    numbers.iter().filter(|val| {
        // val: &&i32
        // 값을 쓰려면 **val 필요
        **val % 2 == 0
    });

    numbers.iter().filter(|&val| {
        // &val 패턴이 &&i32와 매칭됨 -> 껍질 하나 벗겨짐
        // val: &i32
        // 값을 쓰려면 *val 필요
        *val % 2 == 0
    });

    numbers.iter().filter(|&&val| {
        // &&val 패턴이 &&i32와 매칭됨 -> 껍질 두 개 벗겨짐
        // val: i32
        // 그냥 val 사용 가능
        val % 2 == 0
    });
    
    println!("예제 실행 완료");
}