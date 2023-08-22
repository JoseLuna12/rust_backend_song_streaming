use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
    pub code: String,
}

#[derive(Serialize)]
pub struct Music {
    pub id: String,
    pub name: String,
    pub location: String,
    pub image: String,
    pub author: String,
}

impl Music {
    pub fn new(
        id: String,
        name: String,
        location: String,
        image: Option<String>,
        author: String,
    ) -> Self {
        let current_image = match image {
            Some(img) => img,
            None => "images/default_image.jpeg".to_owned(),
        };

        Music {
            id,
            name,
            location,
            image: current_image,
            author,
        }
    }
}

#[derive(Serialize)]
pub struct User<'a> {
    pub user_name: String,
    pub user_image: String,
    pub music: Vec<&'a Music>,
    pub listening_to: Option<&'a Music>,
}

impl<'user> User<'user> {
    pub fn new(name: String, image: String) -> Self {
        User {
            user_name: name,
            user_image: image,
            music: Vec::new(),
            listening_to: None,
        }
    }
}
