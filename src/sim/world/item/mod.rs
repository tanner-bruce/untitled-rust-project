enum Trait {
    Equipable,
}

enum OrganicSlot {
    Legs,
    Arms,
    Shoulders,
    Head,
    Chest,
    Hands
}

enum MechSlot {
    Head,
    Locomotion,
    Arms,
    Back,
    Shoulders,
    Processor,
    Booster,
    Drone,
}

struct Item {
    name: String,
    quality: i32,
    attractiveness: i32
}