#[derive(Clone, Debug)]
pub struct Uptime;

impl Uptime {
    pub fn new() -> Self {
        Self {}
    }

    pub fn size() -> usize {
        core::mem::size_of::<f64>()
    }
}
