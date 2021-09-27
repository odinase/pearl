pub trait Inference {
    type Marginals;

    fn sum_product_algorithm(&self) -> Self::Marginals;
}