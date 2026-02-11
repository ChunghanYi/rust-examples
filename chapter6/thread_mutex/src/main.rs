use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // 1. 데이터(0)를 Mutex로 감싸서 상호 배제(Mutual Exclusion)를 보장합니다.
    // 2. 이를 다시 Arc(Atomic Reference Count)로 감싸서 여러 스레드가 소유권을 공유할 수 있게 합니다.
    //    (일반 Rc는 스레드 안전하지 않기 때문에 Arc를 사용해야 합니다.)
    let counter = Arc::new(Mutex::new(0));
    
    // 생성된 스레드 핸들을 저장할 벡터
    let mut handles = vec![];

    println!("--- 스레드 10개를 생성하여 카운트 시작 ---");

    for i in 0..10 {
        // Arc::clone을 사용하여 참조 카운트를 증가시키고, 각 스레드에 소유권을 복제해줍니다.
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            // 스레드 내부 로직
            
            // [중요] 락 획득 시도
            // .lock() 메서드는 Result<MutexGuard<T>, ...>를 반환합니다.
            // unwrap()을 통해 얻은 'num' 변수가 바로 'MutexGuard<i32>' 타입입니다.
            let mut num = counter_clone.lock().unwrap();

            // MutexGuard는 스마트 포인터이므로, 역참조(*)를 통해 내부 데이터에 접근합니다.
            *num += 1;
            
            println!("스레드 #{} 작업 중... (현재 값: {})", i, *num);
            
            // 잠시 대기 (경합 상황 시뮬레이션용)
            thread::sleep(Duration::from_millis(10));

        }); 
        // [중요] 스코프가 끝나는 시점에 'num'(MutexGuard)변수가 Drop 됩니다.
        // 이때 자동으로 락이 해제(Unlock)되어 다른 스레드가 접근할 수 있게 됩니다.

        handles.push(handle);
    }

    // 모든 스레드가 작업을 마칠 때까지 메인 스레드 대기
    for handle in handles {
        handle.join().unwrap();
    }

    // 최종 결과 확인
    // 여기서도 데이터 값을 읽기 위해 lock()을 걸어야 합니다.
    println!("--- 작업 완료 ---");
    println!("최종 결과: {}", *counter.lock().unwrap());
}
