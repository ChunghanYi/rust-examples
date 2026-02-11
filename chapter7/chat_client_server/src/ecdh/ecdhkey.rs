// src/ecdhkey.rs
// 이 모듈은 Elliptic Curve Diffie-Hellman (P-256) 키 교환 로직을 담당합니다.

use p256::{
    ecdh::EphemeralSecret,
    PublicKey,
};
//use rand_core::OsRng;
use crate::OsRng;
use hkdf::Hkdf;
use sha2::Sha256;

// 공개키를 주고받기 쉽도록 바이트 배열(SEC1 인코딩)로 정의
pub type PubKeyBytes = Vec<u8>;

pub struct EcdhKey {
    secret: EphemeralSecret,
    public_key: PublicKey,
}

impl EcdhKey {
    // 1. 내 일회용 키 쌍(비공개키, 공개키) 생성
    pub fn create() -> Self {
        let secret = EphemeralSecret::random(&mut OsRng);
        let public_key = PublicKey::from(&secret);
        Self { secret, public_key }
    }

    // 내 공개키를 바이트로 변환 (상대방에게 전송용)
    pub fn public_key_bytes(&self) -> PubKeyBytes {
        // 압축된 형식(33bytes)으로 변환
        self.public_key.to_sec1_bytes().to_vec()
    }

    // 2. 상대방의 공개키와 내 비밀키를 조합하여 공유 비밀(Shared Secret) 생성
    // 생성된 비밀값으로 32바이트 AES 키를 유도하여 반환
    pub fn derive_aes_key(self, other_pubkey_bytes: &[u8]) -> Result<[u8; 32], String> {
        // 상대방 공개키 디코딩
        let other_pk = PublicKey::from_sec1_bytes(other_pubkey_bytes)
            .map_err(|_| "상대방 공개키 형식이 잘못되었습니다.".to_string())?;

        // Diffie-Hellman 연산 수행
        let shared_secret = self.secret.diffie_hellman(&other_pk);

        // HKDF를 사용하여 공유 비밀에서 안전한 AES-256 키 추출
        let hkdf = Hkdf::<Sha256>::new(None, shared_secret.raw_secret_bytes());
        let mut okm = [0u8; 32];
        hkdf.expand(b"chat-handshake-v1", &mut okm)
            .map_err(|_| "키 유도 실패".to_string())?;

        Ok(okm)
    }
}