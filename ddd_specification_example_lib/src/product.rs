pub struct Product {
    ean_13: usize
}

impl Product {
    pub fn new(ean_13: usize) -> Product {
        Product {
            ean_13
        }
    }

    pub fn get_ean_13(&self) -> usize {
        self.ean_13
    }
}

pub trait ProductSpecification {
    fn is_satisfied_by(&self, product: Product) -> bool;
}
