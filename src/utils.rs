use qrcode::QrCode;

pub fn create_qr_code(text: String) -> QrCode {
    
    QrCode::new(text.into_bytes()).unwrap()
}
