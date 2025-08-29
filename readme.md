
Works on Linux.

Rust:

```rs
#![allow(non_upper_case_globals, non_snake_case, unused)]

pub mod C_BaseEntity {
	pub const m_CBodyComponent: usize = 0x38; // CBodyComponent*
	pub const m_NetworkTransmitComponent: usize = 0x40; // CNetworkTransmitComponent
	pub const m_nLastThinkTick: usize = 0x4a0; // GameTick_t
	pub const m_pGameSceneNode: usize = 0x4a8; // CGameSceneNode*
	pub const m_pRenderComponent: usize = 0x4b0; // CRenderComponent*
	pub const m_pCollision: usize = 0x4b8; // CCollisionProperty*
	pub const m_iMaxHealth: usize = 0x4c0; // int32
	pub const m_iHealth: usize = 0x4c4; // int32
    ...
```
C#:

```cs
namespace Offsets {
	public static class C_BaseEntity
	{
			public static const IntPtr m_CBodyComponent = 0x38; // CBodyComponent*
			public static const IntPtr m_NetworkTransmitComponent = 0x40; // CNetworkTransmitComponent
			public static const IntPtr m_nLastThinkTick = 0x4a0; // GameTick_t
			public static const IntPtr m_pGameSceneNode = 0x4a8; // CGameSceneNode*
			public static const IntPtr m_pRenderComponent = 0x4b0; // CRenderComponent*
			public static const IntPtr m_pCollision = 0x4b8; // CCollisionProperty*
			public static const IntPtr m_iMaxHealth = 0x4c0; // int32
			public static const IntPtr m_iHealth = 0x4c4; // int32
    ...
```