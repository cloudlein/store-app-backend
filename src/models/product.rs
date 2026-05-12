// Product models will go here
#[derive(Debug, Deserialize, Validate)]
pub struct CreateProduct {
    #[validate(min = 1 , max = 255, message = "title cannot be empty!")]
    pub title: String,

    #[validate(range(min = 0.0, message = "price must be greater than 0 or equal to 0."))]
    pub price: f64,

    #[validate(min = 1, message = "description cannot be empty!")]
    pub description: String,

    #[validate(url(message = "image url must be a valid url!"))]
    pub image_url: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProduct {
    pub title: String,
    pub price: f64,
    pub description: String,
    pub image_url: String,
}

