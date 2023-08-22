use super::models::{Music, User};

pub struct InitApp<'a> {
    pub users: Vec<User<'a>>,
    pub music: Vec<Music>,
}

impl InitApp<'static> {
    pub fn init() -> Self {
        let music = InitApp::generate_songs();
        let users = InitApp::generate_users();

        InitApp { users, music }
    }

    pub fn get_song_by_id(self, id: String) -> Option<Music> {
        let song = self.music.into_iter().find(|song| song.id == id);
        song
    }

    pub fn get_user_by_username(self, username: String) -> Option<User<'static>> {
        let user = self.users.into_iter().find(|u| u.user_name == username);
        user
    }

    pub fn generate_users() -> Vec<User<'static>> {
        vec![
            User::new("Jose".to_owned(), "user1.png".to_owned()),
            User::new("Matu".to_owned(), "user2.png".to_owned()),
            User::new("Lucas".to_owned(), "user3.png".to_owned()),
        ]
    }

    pub fn generate_songs() -> Vec<Music> {
        vec![
            Music::new(
                "1".to_owned(),
                "lofi".to_owned(),
                "music/lofi.mp3".to_owned(),
                Some("images/ep1.jpg".to_owned()),
                "Lucas".to_owned(),
            ),
            Music::new(
                "2".to_owned(),
                "bass".to_owned(),
                "music/bass.mp3".to_owned(),
                Some("images/ep2.png".to_owned()),
                "Thiago".to_owned(),
            ),
            Music::new(
                "3".to_owned(),
                "future".to_owned(),
                "music/future.mp3".to_owned(),
                Some("images/ep3.jpeg".to_owned()),
                "Pavo".to_owned(),
            ),
            Music::new(
                "4".to_owned(),
                "universe".to_owned(),
                "music/universe.mp3".to_owned(),
                Some("images/ep4.jpeg".to_owned()),
                "Mati".to_owned(),
            ),
            Music::new(
                "5".to_owned(),
                "Relax".to_owned(),
                "music/relax.mp3".to_owned(),
                None,
                "Mati".to_owned(),
            ),
        ]
    }
}
