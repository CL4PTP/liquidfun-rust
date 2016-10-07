pub enum B2ParticleGroup {}

/// A group of particles. b2ParticleGroup::CreateParticleGroup creates these.
#[derive(Clone)]
pub struct ParticleGroup {
    raw: *mut B2ParticleGroup,
}

impl ParticleGroup {
	pub unsafe fn from_raw(raw: *mut B2ParticleGroup) -> Self {
		ParticleGroup { raw: raw }
	}

	/// Get ParticleGroup's raw pointer.
	pub unsafe fn ptr(&self) -> *mut B2ParticleGroup {
		self.raw
	}
}
