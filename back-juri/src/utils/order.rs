pub enum OrderStatus {
    WAIT,
    ACCEPT,
    REFUSE,
}

impl OrderStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderStatus::WAIT => "wait",
            OrderStatus::ACCEPT => "accept",
            OrderStatus::REFUSE => "refuse",
        }
    }
}
