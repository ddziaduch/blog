use crate::product::{ProductSpecification, Product};

pub struct DiscountedProductSpecification {
    discounted_ean_13: usize
}

impl DiscountedProductSpecification {
    pub fn new() -> DiscountedProductSpecification {
        DiscountedProductSpecification {
            discounted_ean_13: 501235678900
        }
    }
}

impl ProductSpecification for DiscountedProductSpecification {
    fn is_satisfied_by(&self, product: Product) -> bool {
        product.get_ean_13() == self.discounted_ean_13
    }
}
