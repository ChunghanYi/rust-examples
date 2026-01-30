// src/bin/server.rs

use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use aes_gcm::{aead::{Aead, KeyInit}, Aes256Gcm, Nonce};
use aes_gcm::AeadCore;
use base64::{engine::general_purpose, Engine as _};
use rand::{rngs::OsRng, RngCore};

// ecdh.rs 파일을 모듈로 불러옵니다. (파일 경로가 ../ecdh.rs 라고 가정)
#[path = "../ecdh/ecdhkey.rs"]
mod ecdhkey;
//mod ecdh;
//use ecdh::ecdhkey;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("🚀 채팅 서버(ECDH Key Exchange)가 시작되었습니다.");

    // 1. 서버 실행 시, 채팅방 전용 랜덤 키(Room Key) 생성 (이 키로 대화함)
    let mut room_key_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut room_key_bytes);
    let room_key_vec = room_key_bytes.to_vec(); // 클론하여 태스크로 넘기기 위해 Vec 사용
    
    // 서버 로그용 복호화 객체
    let server_room_cipher = Aes256Gcm::new(&room_key_bytes.into());

    let (tx, _rx) = broadcast::channel(100);

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("✨ 클라이언트 접속 시도: {}", addr);

        let tx = tx.clone();
        let mut rx = tx.subscribe();
        let room_key = room_key_vec.clone();
        let server_room_cipher = server_room_cipher.clone();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);

            // ==========================================
            // [ECDH 핸드셰이크 단계]
            // ==========================================
            
            // 1. 서버의 임시 키 쌍 생성
            let server_ecdh = ecdhkey::EcdhKey::create();
            let server_pub_b64 = general_purpose::STANDARD.encode(server_ecdh.public_key_bytes());
            
            // 2. 클라이언트에게 서버 공개키 전송
            if let Err(_) = writer.write_all(format!("{}\n", server_pub_b64).as_bytes()).await {
                return;
            }

            // 3. 클라이언트로부터 공개키 수신 대기
            let mut client_pub_line = String::new();
            if reader.read_line(&mut client_pub_line).await.unwrap_or(0) == 0 {
                return; // 연결 끊김
            }
            let client_pub_bytes = match general_purpose::STANDARD.decode(client_pub_line.trim()) {
                Ok(b) => b,
                Err(_) => return,
            };

            // 4. 핸드셰이크 키(Session Key) 유도
            let session_key = match server_ecdh.derive_aes_key(&client_pub_bytes) {
                Ok(k) => k,
                Err(e) => {
                    eprintln!("키 교환 실패: {}", e);
                    return;
                }
            };

            // 5. 유도된 세션 키로 'Room Key'를 암호화하여 클라이언트에게 전송
            //    (이 과정이 끝나면 이제 둘 다 Room Key를 알게 됨)
            let session_cipher = Aes256Gcm::new(&session_key.into());
            let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits unique
            let encrypted_room_key = session_cipher.encrypt(&nonce, room_key.as_slice()).unwrap();
            
            // 전송 포맷: Base64( Nonce + EncryptedRoomKey )
            let mut payload = nonce.to_vec();
            payload.extend_from_slice(&encrypted_room_key);
            let payload_b64 = general_purpose::STANDARD.encode(payload);
            
            if let Err(_) = writer.write_all(format!("{}\n", payload_b64).as_bytes()).await {
                return;
            }
            
            println!("🔒 [{}] 핸드셰이크 완료 및 Room Key 전달됨", addr);


            // ==========================================
            // [메인 채팅 루프 (Room Key 사용)]
            // ==========================================
            let mut line = String::new();
            loop {
                tokio::select! {
                    // 메시지 수신 (암호화된 상태)
                    result = reader.read_line(&mut line) => {
                        if result.unwrap_or(0) == 0 { break; }

                        // 로깅: 서버도 Room Key가 있으므로 복호화해서 내용을 볼 수 있음
                        let trimmed = line.trim();
                        if let Ok(data) = general_purpose::STANDARD.decode(trimmed) {
                            if data.len() > 12 {
                                let (nonce, cipher) = data.split_at(12);
                                if let Ok(pt) = server_room_cipher.decrypt(Nonce::from_slice(nonce), cipher) {
                                     println!("수신 [{}]: {}", addr, String::from_utf8_lossy(&pt));
                                }
                            }
                        }

                        // 브로드캐스트 (암호문 그대로 전달)
                        let msg = format!("[{}]: {}", addr, line);
                        let _ = tx.send((msg, addr));
                        line.clear();
                    }

                    // 다른 사람의 메시지 전송
                    result = rx.recv() => {
                        if let Ok((msg, other_addr)) = result {
                            if addr != other_addr {
                                let _ = writer.write_all(msg.as_bytes()).await;
                            }
                        }
                    }
                }
            }
            println!("👋 클라이언트 접속 종료: {}", addr);
        });
    }
}
