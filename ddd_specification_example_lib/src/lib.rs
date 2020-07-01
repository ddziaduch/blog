mod discount;
mod product;

#[cfg(test)]
mod tests {
    use crate::product::{Product, ProductSpecification};
    use crate::discount::DiscountedProductSpecification;

    #[test]
    fn discounted_product_specification_should_be_specified_by_product_with_certain_ean_13() {
        let product = Product::new(501235678900);
        let specification = DiscountedProductSpecification::new();
        assert_eq!(specification.is_satisfied_by(product), true);
    }

    #[test]
    fn discounted_product_specification_should_not_be_specified_by_other_product() {
        let product = Product::new(501235678933);
        let specification = DiscountedProductSpecification::new();
        assert_eq!(specification.is_satisfied_by(product), false);
    }
}
