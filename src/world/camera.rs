/// A camera that represents a rectangular viewport.
///
/// The camera is defined by its center position and viewport dimensions.
/// It can be used to determine which points or objects are visible within
/// the camera's current view.
///
/// # Public fields
///
/// * `center_x` - The x-coordinate of the camera's center position in world space
/// * `center_y` - The y-coordinate of the camera's center position in world space
/// * `width` - The width of the camera's viewport in pixels
/// * `height` - The height of the camera's viewport in pixels
pub struct Camera {
    pub center_x: f32,
    pub center_y: f32,
    pub width: u16,
    pub height: u16,
}

impl Camera {
    /// Creates a new Camera instance with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `center_x` - The x-coordinate of the camera's center position
    /// * `center_y` - The y-coordinate of the camera's center position  
    /// * `width` - The width of the camera's viewport in pixels
    /// * `height` - The height of the camera's viewport in pixels
    ///
    /// # Returns
    ///
    /// A new `Camera` instance with all values initialized to specified arguments.
    pub fn new(center_x: f32, center_y: f32, width: u16, height: u16) -> Self {
        Self { center_x, center_y, width, height }
    }

    /// Checks if a point is visible within the camera's viewport.
    ///
    /// A point is considered visible if it falls within the camera's rectangular viewport.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the point
    /// * `y` - The y-coordinate of the point
    ///
    /// # Returns
    ///
    /// `true` if the point is visible within the camera's viewport, `false` otherwise.
    pub fn is_visible(&self, x: f32, y: f32) -> bool {
        ((self.center_x - x).abs() < (self.width as f32) / 2.0)
            && ((self.center_y - y).abs() < (self.height as f32) / 2.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that Camera initializes with values initialized to specified arguments
    #[test]
    fn test_camera_creation() {
        let camera = Camera::new(100.0, 200.0, 800, 600);
        assert_eq!(camera.center_x, 100.0);
        assert_eq!(camera.center_y, 200.0);
        assert_eq!(camera.width, 800);
        assert_eq!(camera.height, 600);
    }

    /// Test that the center point of the camera is visible
    #[test]
    fn test_is_visible_center_point() {
        let camera = Camera::new(100.0, 200.0, 800, 600);
        assert!(camera.is_visible(100.0, 200.0));
    }

    /// Test points that should be within the camera's viewport
    #[test]
    fn test_is_visible_within_bounds() {
        let camera = Camera::new(100.0, 200.0, 800, 600);
        // Points within the viewport
        assert!(camera.is_visible(150.0, 250.0));
        assert!(camera.is_visible(50.0, 150.0));
        assert!(camera.is_visible(100.0, 350.0));
        assert!(camera.is_visible(100.0, 50.0));
        assert!(camera.is_visible(450.0, 200.0));
        assert!(camera.is_visible(-250.0, 200.0));
    }

    /// Test points that should be outside the camera's viewport
    #[test]
    fn test_is_visible_out_of_bounds() {
        let camera = Camera::new(100.0, 200.0, 800, 600);
        // Points outside the viewport
        assert!(!camera.is_visible(500.0, 200.0));
        assert!(!camera.is_visible(-300.0, 200.0));
        assert!(!camera.is_visible(100.0, 600.0));
        assert!(!camera.is_visible(100.0, -200.0));
        assert!(!camera.is_visible(500.0, 500.0));
    }

    /// Test edge cases with points very close to the viewport boundaries
    #[test]
    fn test_is_visible_edge_cases() {
        let camera = Camera::new(100.0, 200.0, 800, 600);

        // Points near the boundary (should be visible since it's < width/2)
        assert!(camera.is_visible(100.0 + 399.9, 200.0)); // Just inside near right edge
        assert!(camera.is_visible(100.0 - 399.9, 200.0)); // Just inside near left edge
        assert!(camera.is_visible(100.0, 200.0 + 299.9)); // Just inside near top edge
        assert!(camera.is_visible(100.0, 200.0 - 299.9)); // Just inside near bottom edge

        // Points exactly at the boundary (should not be visible since it's == width/2)
        assert!(!camera.is_visible(100.0 + 400.0, 200.0)); // Exactly at right edge
        assert!(!camera.is_visible(100.0 - 400.0, 200.0)); // Exactly at left edge
        assert!(!camera.is_visible(100.0, 200.0 + 300.0)); // Exactly at top edge
        assert!(!camera.is_visible(100.0, 200.0 - 300.0)); // Exactly at bottom edge
    }

    /// Test cameras with different sizes
    #[test]
    fn test_is_visible_different_camera_sizes() {
        let small_camera = Camera::new(0.0, 0.0, 100, 100);
        assert!(small_camera.is_visible(25.0, 25.0));
        assert!(!small_camera.is_visible(60.0, 60.0));

        let large_camera = Camera::new(0.0, 0.0, 2000, 2000);
        assert!(large_camera.is_visible(500.0, 500.0));
        assert!(!large_camera.is_visible(1100.0, 1100.0));
    }

    /// Test camera with different center positions
    #[test]
    fn test_is_visible_different_centers() {
        let negative_center_camera = Camera::new(-100.0, -200.0, 800, 600);
        assert!(negative_center_camera.is_visible(-100.0, -200.0));
        assert!(negative_center_camera.is_visible(-300.0, -200.0));
        assert!(negative_center_camera.is_visible(100.0, -200.0));
        assert!(!negative_center_camera.is_visible(-600.0, -200.0));
        assert!(!negative_center_camera.is_visible(400.0, -200.0));
    }
}
