use qrcode::QrCode;

pub fn create_qr_code(text: String) -> QrCode {
    let code = QrCode::new(text.into_bytes()).unwrap();
    return code;
}
