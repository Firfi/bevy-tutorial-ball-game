
// note it's related to score rather than to stars
#[derive(Debug, Clone)]
pub struct StarCount(pub usize);

impl StarCount {
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}