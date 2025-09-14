use bevy::math::{URect, UVec2};

/// Container for a 2D grid of cells.
pub(crate) struct Gridmap<T>(usize, Vec<T>);

impl<T> Gridmap<T> {
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default,
    {
        Self(width, (0..width * height).map(|_| T::default()).collect())
    }

    pub fn new_filled(width: usize, height: usize, fill: T) -> Self
    where
        T: Clone,
    {
        Self(width, vec![fill; width * height])
    }

    pub fn bounds(&self) -> URect {
        URect {
            min: UVec2::ZERO,
            max: UVec2::new(self.0 as u32, (self.1.len() / self.0) as u32),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gridmap() {
        let gm = Gridmap::<i32>::new(4, 3);
        assert_eq!(
            gm.bounds(),
            URect {
                min: UVec2::ZERO,
                max: UVec2::new(4, 3),
            }
        );
    }
}
