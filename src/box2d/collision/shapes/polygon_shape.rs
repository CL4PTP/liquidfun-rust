use std::slice;
use super::shape::*;
use super::super::super::common::settings::*;
use super::super::super::common::math::*;

pub enum B2PolygonShape {}

extern {
    fn b2PolygonShape_Delete(ptr: *mut B2PolygonShape);
    fn b2PolygonShape_GetVertex(ptr: *mut B2PolygonShape, index: Int32) -> &Vec2;
    fn b2PolygonShape_GetVertexCount(ptr: *const B2PolygonShape) -> Int32;
    fn b2PolygonShape_New() -> *mut B2PolygonShape;
    fn b2PolygonShape_Set(ptr: *mut B2PolygonShape, points: *const Vec2, count: Int32);
    fn b2PolygonShape_SetAsBox(ptr: *mut B2PolygonShape, hx: Float32, hy: Float32);
    fn b2PolygonShape_SetAsBox_Oriented(ptr: *mut B2PolygonShape, hx: Float32, hy: Float32, center: &Vec2, angle: Float32);
    fn b2PolygonShape_Upcast(ptr: *mut B2PolygonShape) -> *mut B2Shape;
    fn b2PolygonShape_m_vertices(ptr: *mut B2PolygonShape) -> *const Vec2;
}

/// A convex polygon. It is assumed that the interior of the polygon is to
/// the left of each edge.
/// Polygons have a maximum number of vertices equal to b2_maxPolygonVertices.
/// In most cases you should not need many vertices for a convex polygon.
pub struct PolygonShape {
    raw: *mut B2PolygonShape,
    owned: bool,
}

/// Cast a PolygonShape from a B2Shape.
pub fn from_shape(ptr: *mut B2Shape) -> PolygonShape {
    unsafe { PolygonShape::from_raw(ptr as *mut _, false) }
}

impl Shape for PolygonShape {
    // Is the up-cast even necessary? Can't we just use  directly?
    fn handle(&self) -> *mut B2Shape {
        unsafe {
            b2PolygonShape_Upcast(self.ptr())
        }
    }
}

impl PolygonShape {
	pub unsafe fn from_raw(raw: *mut B2PolygonShape, owned: bool) -> Self {
		PolygonShape { raw: raw, owned: owned }
	}

	pub unsafe fn ptr(&self) -> *mut B2PolygonShape {
		self.raw
	}

    /// Create a new PolygonShape.
    pub fn new() -> PolygonShape {
        unsafe { PolygonShape::from_raw(b2PolygonShape_New(), true) }
    }

    /// Get a vertex by index.
    pub fn get_vertex(&self, index: i32) -> &Vec2 {
        unsafe {
            b2PolygonShape_GetVertex(self.ptr(), index)
        }
    }

    /// Get the vertex count.
    pub fn get_vertex_count(&self) -> i32 {
        unsafe {
            b2PolygonShape_GetVertexCount(self.ptr())
        }
    }

    pub fn set(&mut self, points: &[Vec2])
    {
        unsafe {
            b2PolygonShape_Set(self.ptr(), points.as_ptr(), points.len() as _);
        }
    }

    /// Build vertices to represent an axis-aligned box centered on the local origin.
    /// @param hx the half-width.
    /// @param hy the half-height.
    pub fn set_as_box(&mut self, hx:f32, hy:f32) {
        unsafe {
            b2PolygonShape_SetAsBox(self.ptr(), hx, hy);
        }
    }

    /// Build vertices to represent an oriented box.
    /// @param hx the half-width.
    /// @param hy the half-height.
    /// @param center the center of the box in local coordinates.
    /// @param angle the rotation of the box in local coordinates.
    pub fn set_as_box_oriented(&mut self, hx: f32, hy: f32, center: &Vec2, angle: f32) {
        unsafe {
            b2PolygonShape_SetAsBox_Oriented(self.ptr(), hx, hy, center, angle);
        }
    }

	/// The vertices. Owned by this class.
 	pub fn get_vertices(&self) -> &[Vec2] {
        unsafe {
			slice::from_raw_parts(b2PolygonShape_m_vertices(self.ptr()), self.get_vertex_count() as usize)
        }
    }
}

impl Drop for PolygonShape {
    fn drop(&mut self) {
        if self.owned {
            unsafe { b2PolygonShape_Delete(self.ptr()); }
        }
    }
}
