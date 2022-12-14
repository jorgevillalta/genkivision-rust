pub struct Song {
    pub name: String,
    pub artist: String,
    pub user: String,
    pub up_votes: u8,
    pub down_votes: u8,
}

impl Song {
    pub fn new(name: String, artist: String, user: String) -> Self {
        Self {
            name,
            artist,
            user,
            up_votes: 0,
            down_votes: 0,
        }
    }
}

pub trait Voting {
    fn up_vote(&mut self);
    fn down_vote(&mut self);
    fn clear_votes(&mut self);
    fn votes(&self) -> (u8, u8) {
        (0, 0)
    }
}

impl Voting for Song {
    fn up_vote(&mut self) {
        self.up_votes += 1;
    }

    fn down_vote(&mut self) {
        self.down_votes += 1;
    }

    fn clear_votes(&mut self) {
        self.up_votes = 0;
        self.down_votes = 0;
    }

    fn votes(&self) -> (u8, u8) {
        (self.up_votes, self.down_votes)
    }
}

/* fn up_votes<T: Voting>(voting: &mut T) {
    voting.up_vote();
} */
