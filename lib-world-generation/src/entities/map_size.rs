#[derive(Debug, PartialEq)]
pub enum MapSizeError {
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapSize {
    width: u32,
    height: u32,
}

impl MapSize {
    pub fn new(width: u32, height: u32) -> Result<Self, MapSizeError> {
        if width == 0 || height == 0 {
            return Err(MapSizeError::Empty);
        }

        Ok(Self { width, height })
    }

    #[must_use]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[must_use]
    pub fn height(&self) -> u32 {
        self.height
    }
}
