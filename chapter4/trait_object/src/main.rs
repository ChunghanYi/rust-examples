// 1. 공통 동작을 정의하는 트레이트 정의
trait Draw {
    fn draw(&self);
}

// 2. Button 구조체 정의
struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

// Button에 Draw 트레이트 구현
impl Draw for Button {
    fn draw(&self) {
        println!(
            "버튼 그리기: [ {} ] (크기: {}x{})",
            self.label, self.width, self.height
        );
    }
}

// 3. SelectBox 구조체 정의
struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

// SelectBox에 Draw 트레이트 구현
impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "선택 상자 그리기: 폭 {}px, 높이 {}px, 옵션: {:?}",
            self.width, self.height, self.options
        );
    }
}

// 4. 화면(Screen) 구조체
// 여기서 핵심은 'Box<dyn Draw>'입니다.
// 이는 "Draw 트레이트를 구현한 어떤 타입이든 힙(Heap)에 할당된 포인터로 저장하겠다"는 의미입니다.
struct Screen {
    // 제네릭(Vec<T>)을 쓰면 한 가지 타입만 담을 수 있지만,
    // 트레이트 객체(Box<dyn Draw>)를 쓰면 여러 타입을 섞어서 담을 수 있습니다.
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        println!("--- 화면 렌더링 시작 ---");
        for component in self.components.iter() {
            // 런타임에 각 컴포넌트의 구체적인 타입에 맞는 draw() 메서드가 호출됩니다 (Dynamic Dispatch).
            component.draw();
        }
        println!("--- 화면 렌더링 종료 ---");
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            // Button 인스턴스를 Box로 감싸서 트레이트 객체로 만듦
            Box::new(Button {
                width: 75,
                height: 10,
                label: String::from("확인"),
            }),
            // SelectBox 인스턴스를 Box로 감싸서 트레이트 객체로 만듦
            Box::new(SelectBox {
                width: 200,
                height: 20,
                options: vec![
                    String::from("네"),
                    String::from("아니오"),
                    String::from("취소"),
                ],
            }),
            // 다른 Button 추가 (서로 다른 타입이 한 벡터에 공존 가능)
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("종료"),
            }),
        ],
    };

    screen.run();
}
