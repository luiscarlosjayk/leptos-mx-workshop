pub struct Rating {
    rate: f64,
    count: u32,
}

impl Rating {
    fn new(rate: f64, count: u32) -> Self {
        Rating { rate, count }
    }
}
