use qrcode::QrCode;

pub fn create_qr_code(text: String) -> QrCode {
    let code = QrCode::new(text.into_bytes()).unwrap();
    println!("This code is {} wide", code.width());
    println!("{}", code.to_debug_str('#', '_'));
    return code;
}
