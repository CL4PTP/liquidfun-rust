use super::super::common::settings::*;
use super::super::collision::*;
use super::fixture::*;

pub enum B2Contact {}

extern {
    fn b2Contact_GetManifold(this: *const B2Contact) -> *const Manifold;
	fn b2Contact_GetWorldManifold(this: *const B2Contact, worldManifold: *const WorldManifold);
	fn b2Contact_IsTouching(this: *const B2Contact) -> bool;
	fn b2Contact_SetEnabled(this: *const B2Contact, flag: bool);
	fn b2Contact_IsEnabled(this: *const B2Contact) -> bool;
	fn b2Contact_GetNext(this: *const B2Contact) -> *mut B2Contact;
	fn b2Contact_GetFixtureA(this: *const B2Contact) -> *mut B2Fixture;
	fn b2Contact_GetChildIndexA(this: *const B2Contact) -> Int32;
	fn b2Contact_GetFixtureB(this: *const B2Contact) -> *mut B2Fixture;
	fn b2Contact_GetChildIndexB(this: *const B2Contact) -> Int32;
	fn b2Contact_SetFriction(this: *const B2Contact, friction: Float32);
	fn b2Contact_GetFriction(this: *const B2Contact) -> Float32;
	fn b2Contact_ResetFriction(this: *const B2Contact);
	fn b2Contact_SetRestitution(this: *const B2Contact, restitution: Float32);
	fn b2Contact_GetRestitution(this: *const B2Contact) -> Float32;
	fn b2Contact_ResetRestitution(this: *const B2Contact);
	fn b2Contact_SetTangentSpeed(this: *const B2Contact, speed: Float32);
	fn b2Contact_GetTangentSpeed(this: *const B2Contact) -> Float32;
}

pub struct Contact {
	pub ptr: *mut B2Contact,
}

impl Contact {

    pub fn get_manifold(&self) -> &Manifold {
        unsafe { &*b2Contact_GetManifold(self.ptr) }
    }

    pub fn get_world_manifold(&self, world_manifold: &WorldManifold) {
        unsafe { b2Contact_GetWorldManifold(self.ptr, world_manifold) }
    }

    pub fn is_touching(&self) -> bool {
        unsafe { b2Contact_IsTouching(self.ptr) }
    }

    pub fn set_enabled(&self, flag: bool) {
        unsafe { b2Contact_SetEnabled(self.ptr, flag) }
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { b2Contact_IsEnabled(self.ptr) }
    }

    pub fn get_next(&self) -> Contact {
        unsafe { Contact { ptr: b2Contact_GetNext(self.ptr) } }
    }

    pub fn get_fixture_a(&self) -> Fixture {
        unsafe { Fixture { ptr: b2Contact_GetFixtureA(self.ptr) } }
    }

    pub fn get_child_index_a(&self) -> Int32 {
        unsafe { b2Contact_GetChildIndexA(self.ptr) }
    }

    pub fn get_fixture_b(&self) -> Fixture {
        unsafe { Fixture { ptr: b2Contact_GetFixtureB(self.ptr) } }
    }

    pub fn get_child_index_b(&self) -> Int32 {
        unsafe { b2Contact_GetChildIndexB(self.ptr) }
    }

    pub fn set_friction(&self, friction: Float32) {
        unsafe { b2Contact_SetFriction(self.ptr, friction) }
    }

    pub fn get_friction(&self) -> Float32 {
        unsafe { b2Contact_GetFriction(self.ptr) }
    }

    pub fn reset_friction(&self) {
        unsafe { b2Contact_ResetFriction(self.ptr) }
    }

    pub fn set_restitution(&self, restitution: Float32) {
        unsafe { b2Contact_SetRestitution(self.ptr, restitution) }
    }

    pub fn get_restitution(&self) -> Float32 {
        unsafe { b2Contact_GetRestitution(self.ptr) }
    }

    pub fn reset_restitution(&self) {
        unsafe { b2Contact_ResetRestitution(self.ptr) }
    }

    pub fn set_tangent_speed(&self, speed: Float32) {
        unsafe { b2Contact_SetTangentSpeed(self.ptr, speed) }
    }

    pub fn get_tangent_speed(&self) -> Float32 {
        unsafe { b2Contact_GetTangentSpeed(self.ptr) }
    }
}
