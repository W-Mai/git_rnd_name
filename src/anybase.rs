pub type OrdResult = Option<usize>;

pub struct AnyBase {
    base: usize,
    digits: String,
}

impl AnyBase {
    pub(crate) fn new(digits: &str) -> Self {
        let base = digits.chars().count();
        let digits = digits.to_string();
        Self { base, digits }
    }

    pub fn map_ord(&self, name: &str) -> OrdResult {
        let ord_base = self.digits.chars().count();
        name.chars().try_fold(0, |res, c| {
            self.digits.chars()
                .position(|e| e == c)
                .map(|pos| res * ord_base + pos + 1)
        })
    }

    pub fn map_emoji(&self, ord: usize) -> String {
        let mut ord_res = ord;
        let mut result = String::new();
        while ord_res > 0 {
            ord_res -= 1;
            let c = self.digits.chars().nth(ord_res % self.base).unwrap();
            result.push(c);
            ord_res /= self.base;
        }
        result.chars().rev().collect()
    }
}
