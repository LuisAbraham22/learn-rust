// Super is the parent module `content`
use super::media::Media;

#[derive(Debug)]
pub struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    pub fn new() -> Self {
        Catalog { items: vec![] }
    }

    pub fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    pub fn get_at_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            self.items.get(index)
        } else {
            None
        }
    }
}
