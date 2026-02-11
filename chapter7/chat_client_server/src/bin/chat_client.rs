// src/bin/client.rs

use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use aes_gcm::{aead::{Aead, KeyInit}, Aes256Gcm, Nonce};
use base64::{engine::general_purpose, Engine as _};
use rand::{rngs::OsRng, RngCore};

// ecdh.rs 파일을 모듈로 불러옵니다.
#[path = "../ecdh/ecdhkey.rs"]
mod ecdhkey;
//mod ecdh;
//use super::ecdh::ecdhkey;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut socket = TcpStream::connect("127.0.0.1:8080").await?;
    println!("connecting...");

    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader);

    // ==========================================
    // [ECDH 핸드셰이크 단계]
    // ==========================================
    
    // 1. 서버 공개키 수신
    let mut server_pub_line = String::new();
    reader.read_line(&mut server_pub_line).await?;
    let server_pub_bytes = general_purpose::STANDARD.decode(server_pub_line.trim())?;

    // 2. 내 임시 키 쌍 생성 및 공개키 전송
    let client_ecdh = ecdhkey::EcdhKey::create();
    let client_pub_b64 = general_purpose::STANDARD.encode(client_ecdh.public_key_bytes());
    writer.write_all(format!("{}\n", client_pub_b64).as_bytes()).await?;

    // 3. 세션 키 유도 (핸드셰이크 암호화용)
    let session_key = client_ecdh.derive_aes_key(&server_pub_bytes)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let session_cipher = Aes256Gcm::new(&session_key.into());

    // 4. 암호화된 Room Key 수신 및 복호화
    let mut room_key_line = String::new();
    reader.read_line(&mut room_key_line).await?;
    let room_key_packet = general_purpose::STANDARD.decode(room_key_line.trim())?;
    
    let (nonce_bytes, ciphertext) = room_key_packet.split_at(12);
    let room_key_bytes = session_cipher.decrypt(Nonce::from_slice(nonce_bytes), ciphertext)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Room Key 복호화 실패"))?;
    
    // 5. 채팅용 암호화 객체 생성
    // (이제부터 이 키로 모든 채팅 메시지를 암호화/복호화합니다)
    let room_cipher = Aes256Gcm::new_from_slice(&room_key_bytes)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Invalid Key Size"))?;

    println!("✅ 보안 핸드셰이크 성공! 안전한 채팅을 시작합니다.");

    
    // ==========================================
    // [메인 채팅 루프]
    // ==========================================
    let mut stdin = BufReader::new(tokio::io::stdin());
    let mut socket_line = String::new();
    let mut input_line = String::new();

    loop {
        tokio::select! {
            // 메시지 수신 (Room Key로 복호화)
            result = reader.read_line(&mut socket_line) => {
                if result? == 0 { break; }

                if let Some((sender, content)) = parse_message(&socket_line) {
                    if let Ok(data) = general_purpose::STANDARD.decode(content.trim()) {
                        if data.len() > 12 {
                            let (nonce, cipher) = data.split_at(12);
                            match room_cipher.decrypt(Nonce::from_slice(nonce), cipher) {
                                Ok(pt) => println!("{}: {}", sender, String::from_utf8_lossy(&pt)),
                                Err(_) => println!("{} (복호화 실패)", sender),
                            }
                        }
                    }
                } else {
                    print!("{}", socket_line);
                }
                socket_line.clear();
            }

            // 메시지 전송 (Room Key로 암호화)
            result = stdin.read_line(&mut input_line) => {
                if result? == 0 { break; }
                
                let plaintext = input_line.trim_end();
                if !plaintext.is_empty() {
                    let mut nonce_bytes = [0u8; 12];
                    OsRng.fill_bytes(&mut nonce_bytes);
                    let nonce = Nonce::from_slice(&nonce_bytes);

                    let ciphertext = room_cipher.encrypt(nonce, plaintext.as_bytes()).expect("Enc Fail");

                    let mut payload = nonce_bytes.to_vec();
                    payload.extend_from_slice(&ciphertext);
                    let b64 = general_purpose::STANDARD.encode(payload);
                    
                    writer.write_all(format!("{}\n", b64).as_bytes()).await?;
                }
                input_line.clear();
            }
        }
    }
    Ok(())
}

fn parse_message(line: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = line.splitn(2, "]: ").collect();
    if parts.len() == 2 {
        Some((parts[0].trim_start_matches('['), parts[1]))
    } else {
        None
    }
}