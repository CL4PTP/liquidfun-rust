use libc::*;

pub type Float32 = c_float;
pub type Int16 = c_short;
pub type Int32 = c_int;
pub type UInt16 = c_ushort;
pub type UInt32 = c_uint;
pub type UInt8 = c_uchar;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct B2Version {
    pub major: Int32,
	pub minor: Int32,
	pub revision: Int32,
}

extern {
    pub static MaxManifoldPoints: UInt32;
    pub static MaxPolygonVertices: UInt32;
    pub static AabbExtension: Float32;
    pub static AabbMultiplier: Float32;
    pub static LinearSlop: Float32;
    pub static AngularSlop: Float32;
    pub static PolygonRadius: Float32;
    pub static MaxSubSteps: UInt32;
    pub static MaxTOIContacts: UInt32;
    pub static VelocityThreshold: Float32;
    pub static MaxLinearCorrection: Float32;
    pub static MaxAngularCorrection: Float32;
    pub static MaxTranslation: Float32;
    pub static MaxTranslationSquared: Float32;
    pub static MaxRotation: Float32;
    pub static MaxRotationSquared: Float32;
    pub static Baumgarte: Float32;
    pub static ToiBaugarte: Float32;
    pub static ParticleStride: Float32;
    pub static MinParticleWeight: Float32;
    pub static MaxParticlePressure: Float32;
    pub static MaxParticleForce: Float32;
    pub static MaxTriadDistance: UInt32;
    pub static MaxTriadDistanceSquared: UInt32;
    pub static MinParticleSystemBufferCapacity: UInt32;
    pub static BarrierCollisionTime: Float32;
    pub static TimeToSleep: Float32;
    pub static LinearSleepTolerance: Float32;
    pub static AngularSleepTolerance: Float32;

    pub static b2_version: B2Version;
    pub static b2_liquidFunVersion: B2Version;
    pub static b2_liquidFunVersionString: *const c_char;
}
