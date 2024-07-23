use crate::typings::{AirdropGameConstants, GameConstants, PlayerGameConstants};

pub enum TeamSize {
	Solo = 1,
	Duo = 2,
	Trio = 3,
	Squad = 4,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)] // we need all of those for the object pool
pub enum ObjectCategory {
	Player,
	Obstacle,
	DeathMarker,
	Loot,
	Building,
	Decal,
	Parachute,
	ThrowableProjectile,
	SyncedParticle
}

pub enum AnimationType {
	None,
	Melee,
	Downed,
	ThrowableCook,
	ThrowableThrow,
	GunFire,
	GunFireAlt,
	GunClick,
	LastShot,
	Revive,
}

pub enum KillfeedMessageType {
	DeathOrDown,
	KillLeaderAssigned,
	KillLeaderDeadOrDisconnected,
	KillLeaderUpdated
}

pub enum GasState {
	Inactive,
	Waiting,
	Advancing
}

pub enum FireMode {
	Single,
	Burst,
	Auto
}

pub enum InputActions {
	EquipItem,
	EquipLastItem,
	DropWeapon,
	DropItem,
	SwapGunSlots,
	LockSlot,
	UnlockSlot,
	ToggleSlotLock,
	Interact,
	Reload,
	Cancel,
	UseItem,
	Emote,
	MapPing,
	Loot
}

pub enum SpectateActions {
	BeginSpectating,
	SpectatePrevious,
	SpectateNext,
	SpectateSpecific,
	SpectateKillLeader,
	Report
}

pub enum PlayerActions {
	None,
	Reload,
	UseItem,
	Revive
}

pub enum KillfeedEventType {
	Suicide,
	NormalTwoParty,
	FinishedOff,
	FinallyKilled,
	Gas,
	BleedOut,
	Airdrop
}

pub enum KillfeedEventSeverity {
	Kill,
	Down
}

// TODO: get together the default inventory (needs item definitions); TS code below
// export const DEFAULT_INVENTORY: Record<string, number> = {};

// for (const item of [...HealingItems, ...Ammos, ...Scopes, ...Throwables]) {
//     let amount = 0;

//     switch (true) {
//         case item.itemType === ItemType.Ammo && item.ephemeral: amount = Infinity; break;
//         case item.itemType === ItemType.Scope && item.giveByDefault: amount = 1; break;
//     }

//     DEFAULT_INVENTORY[item.idString] = amount;
// }

pub const GAME_CONSTANTS: GameConstants = GameConstants {
	// !!!!! NOTE: Increase this every time a bit stream change is made between latest release and master
	// or a new item is added to a definition list
	protocol_version: 24,
	grid_size: 32,
	bleed_out_dpms: 0.002,
	max_position: 1632,
	player: PlayerGameConstants {
		radius: 2.25,
		name_max_length: 16,
		default_name: "Player",
		default_skin: "hazel_jumpsuit",
		default_health: 100,
		max_adrenaline: 100,
		kill_leader_min_kills: 3,
		max_mouse_dist: 128,
		revive_time: 8000, // 8000 milliseconds = 8 seconds
		max_revive_dist: 5.0,
	},
	loot_spawn_distance: 0.7,
	airdrop: AirdropGameConstants {
		fall_time: 8000, // 8000 milliseconds = 8 seconds
		fly_time: 30000, // 30000 milliseconds = 30 seconds
		damage: 300,
	},
};

pub enum ZIndexes {
	Ground,
	UnderWaterDeathMarkers,
	UnderWaterDeadObstacles,
	UnderWaterObstacles,
	UnderWaterLoot,
	UnderwaterGroundedThrowables,
	UnderwaterDownedPlayers,
	UnderwaterPlayers,
	BuildingsFloor,
	Decals,
	DeadObstacles,
	DeathMarkers,
	/**
	 * This is the default layer for obstacles
	 */
	ObstaclesLayer1,
	Loot,
	GroundedThrowables,
	ObstaclesLayer2,
	Bullets,
	DownedPlayers,
	Players,
	/**
	 * bushes, tables etc
	 */
	ObstaclesLayer3,
	AirborneThrowables,
	/**
	 * trees
	 */
	ObstaclesLayer4,
	BuildingsCeiling,
	/**
	 * obstacles that should show on top of ceilings
	 */
	ObstaclesLayer5,
	Emotes,
	Gas
}
