#ifndef C_B2_BODY
#define C_B2_BODY

#ifdef __cplusplus
extern "C" {
#endif

	b2Fixture* b2Body_CreateFixture(b2Body* self, const b2FixtureDef* def);
	b2Fixture* b2Body_CreateFixture_FromShape(b2Body* self, const b2Shape* shape, float32 density);
	const b2Vec2* b2Body_GetPosition(const b2Body* self);
	float32 b2Body_GetAngle(const b2Body* self);
	const b2Vec2* b2Body_GetWorldCenter(b2Body* self);
	const b2Vec2* b2Body_GetLocalCenter(b2Body* self);
	b2Fixture* b2Body_GetFixtureList(b2Body* self);
	b2Body* b2Body_GetNext(b2Body* self);
	void* b2Body_GetUserData(const b2Body* self);
	b2World* b2Body_GetWorld(b2Body* self);
	c_b2Vec2 b2Body_GetLocalPoint(const b2Body* self, const b2Vec2& worldPoint);
	void b2Body_SetLinearVelocity(b2Body* self, const b2Vec2& v);
	const b2Vec2* b2Body_GetLinearVelocity(b2Body* self);
	void b2Body_SetAngularVelocity(b2Body* self, float32 omega);
	float32 b2Body_GetAngularVelocity(b2Body* self);
	void b2Body_ApplyForce(b2Body* self, const b2Vec2& force, const b2Vec2& point, bool wake);
	void b2Body_ApplyForceToCenter(b2Body* self, const b2Vec2& force, bool wake);
	void b2Body_ApplyTorque(b2Body* self, float32 torque, bool wake);
	void b2Body_ApplyLinearImpulse(b2Body* self, const b2Vec2& impulse, const b2Vec2& point, bool wake);
	void b2Body_ApplyAngularImpulse(b2Body* self, float32 impulse, bool wake);

#ifdef __cplusplus
} // extern C
#endif
#endif
