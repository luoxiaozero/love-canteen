pub enum OrderStatus {
    WAIT,
}

impl OrderStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderStatus::WAIT => "wait",
        }
    }
}
