//! BeaglePlay USR Button Interrupt Driver in Rust
//! 
//! 이 드라이버는 Device Tree에 정의된 `beagleplay,usr-button` 노드를 찾아
//! 매칭된 GPIO 인터럽트가 발생할 때마다 메시지를 출력한다.
//!
//! [주의] 아래 코드는 실제 동작이 아직 검증되지 않았다. 참고용으로 작성해 본 것임!

use kernel::prelude::*;
use kernel::{c_str, irq, of, platform};

module! {
    type: UsrButtonModule,
    name: "beagleplay_usr_button",
    author: "Slowboot",
    description: "BeaglePlay USR Button GPIO Interrupt Handler",
    license: "GPL v2",
}

/// 디바이스 상태를 보관하는 구조체
struct UsrButtonModule {
    // 등록된 IRQ 핸들.
    // 구조체가 메모리에서 해제(Drop)될 때 인터럽트 핸들러도 자동으로 커널에서 등록 해제된다.
    _irq_handle: irq::Registration<UsrButtonModule>,
}

/// 인터럽트 핸들러 트레이트 구현
impl irq::Handler for UsrButtonModule {
    type Data = ();

    // 인터럽트(ISR)가 발생하면 호출되는 함수
    fn handle_irq(_data: &Self::Data) -> irq::Return {
        // 인터럽트 발생 시 커널 로그 출력
        pr_info!("*** BeaglePlay USR Button Pressed! (GPIO0_18 Interrupt Triggered) ***\n");
        
        // 인터럽트를 성공적으로 처리했음을 커널에 알림
        irq::Return::Handled
    }
}

/// 플랫폼 드라이버 트레이트 구현
impl platform::Driver for UsrButtonModule {
    type IdInfo = ();
    
    // Device Tree(DTS)의 compatible 속성과 일치해야 모듈이 로드된다.
    const ID_TABLE: &'static [of::DeviceId<Self::IdInfo>] = &[
        of::DeviceId::new("beagleplay,usr-button", ()),
    ];

    // 호환되는 디바이스가 발견되면 실행되는 probe 함수
    fn probe(
        pdev: &mut platform::Device,
        _id_info: Option<&Self::IdInfo>,
    ) -> Result<kernel::init::PinInit<Self, Error>> {
        pr_info!("BeaglePlay USR Button 드라이버 로드 시작...\n");

        // Device Tree 정보를 바탕으로 할당된 첫 번째 인터럽트 번호(IRQ)를 가져온다.
        let irq_num = pdev.irq(0)?;
        pr_info!("할당된 GPIO IRQ 번호: {}\n", irq_num);

        // 인터럽트 핸들러 등록
        // IRQF_TRIGGER_FALLING 플래그를 통해 버튼을 누르는 순간 감지하도록 설정
        let irq_handle = irq::Registration::new(
            irq_num,
            c_str!("beagleplay_usr_button"),
            irq::Flags::TRIGGER_FALLING,
            (), // 핸들러에 전달할 컨텍스트 데이터 (여기서는 빈 튜플)
        )?;

        pr_info!("BeaglePlay USR Button 인터럽트 핸들러 등록 완료.\n");

        // 구조체를 PinInit 패턴으로 초기화하여 안전하게 반환
        Ok(kernel::init::pin_init!(UsrButtonModule {
            _irq_handle: irq_handle,
        }))
    }
}
