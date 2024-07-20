use super::rating::Rating;
pub struct Product {
    id: u32,
    name: String,
    price: usize,
    description: String,
    category: String,
    image: String,
    rating: Rating,
}

impl Product {
    fn new(
        id: u32,
        name: String,
        price: usize,
        description: String,
        category: String,
        image: String,
        rating: Rating,
    ) -> Self {
        Product {
            id,
            name,
            price,
            description,
            category,
            image,
            rating,
        }
    }
}
