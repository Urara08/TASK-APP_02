pub struct inputvalidator {}


// 入力のバリデーション
impl inputvalidator {
    pub fn validate_service_type(Service_type: u32) {
        match Service_type {
            0 | 1 => {}
            _ => panic!("入力値が不正です"),
        }
    }
}