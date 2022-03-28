import random
from dataclasses import dataclass
from logic.extras import item_id_to_name_dict

ITEM_IS_TRIFORCE_SHARD = 0
ITEM_IS_PROGRESSIVE = 1
ITEM_IS_MAIL = 3
ITEM_IS_MISC = 4
ITEM_IS_TREASURE_CHART = 5
ITEM_IS_TRIFORCE_CHART = 6
ITEM_IS_DUNGEON_ITEM = 7
ITEM_IS_INVENTORY = 8
ITEM_IS_UNCATEGORIZED = 9

item_type_to_name = {
    ITEM_IS_TRIFORCE_SHARD: "Triforce Shards",
    ITEM_IS_PROGRESSIVE: "Progressive Items",
    ITEM_IS_MAIL: "Mail Items",
    ITEM_IS_MISC: "Misc Items",
    ITEM_IS_TREASURE_CHART: "Treasure Charts",
    ITEM_IS_TRIFORCE_CHART: "Triforce Charts",
    ITEM_IS_DUNGEON_ITEM: "Dungeon Items",
    ITEM_IS_INVENTORY: "Inventory Items",
    ITEM_IS_UNCATEGORIZED: "Uncategorized Items"
}

@dataclass
class GameItem:

    game_item_id: int
    delayed_item_id: int = 0xFF
    major_item: bool = False
    chart_for_sunken_treasure: bool = False
    junk_item: bool = False
    world_id: int = -1
    item_type: int = 9

    @staticmethod
    def random_junk(world_id: int):
        item_val =  random.choice(junk_ids)
        return GameItem(game_item_id=item_val, world_id=world_id, junk_item=True)

    def __eq__(self, game_item) -> bool:
        return self.game_item_id == game_item.game_item_id and self.world_id == game_item.world_id

    def soft_equality(self, game_item) -> bool:
        return self.game_item_id == game_item.game_item_id and self.world_id == game_item.world_id

    def simple_rep(self):
        return f"W{self.world_id}: {item_id_to_name_dict[self.game_item_id]}"

    def set_item_type(self):
        if is_triforce_shard(self):
            self.item_type = ITEM_IS_TRIFORCE_SHARD
        elif is_progressive(self):
            self.item_type = ITEM_IS_PROGRESSIVE
        elif is_mail_item(self):
            self.item_type = ITEM_IS_MAIL
        elif is_misc_item(self):
            self.item_type = ITEM_IS_MISC
        elif is_treasure_chart(self):
            self.item_type = ITEM_IS_TREASURE_CHART
        elif is_triforce_chart(self):
            self.item_type = ITEM_IS_TRIFORCE_CHART
        elif is_dungeon_item(self):
            self.item_type = ITEM_IS_DUNGEON_ITEM
        elif is_inventory_item(self):
            self.item_type = ITEM_IS_INVENTORY
        else:
            self.item_type = ITEM_IS_UNCATEGORIZED
        return self

def junk_item_check(item: GameItem) -> GameItem:
    item.junk_item = item.game_item_id in junk_ids
    return item

def is_triforce_shard(item: GameItem) -> bool:
    return 0x60 < item.game_item_id < 0x69

def is_sword(item: GameItem) -> bool:
    return item.game_item_id in [0x38, 0x39, 0x3A, 0x3E]

def is_bow(item: GameItem) -> bool:
    return item.game_item_id in [0x27, 0x35, 0x36]

def is_treasure_chart(item: GameItem) -> bool:
    return item.game_item_id in treasure_chart_ids

def is_triforce_chart(item: GameItem) -> bool:
    return item.game_item_id in triforce_chart_ids

def is_mail_item(item: GameItem) -> bool:
    return item.game_item_id in [0x30, 0x99, 0x9A, 0x9B, 0x9C]

def is_misc_item(item: GameItem) -> bool:
    return item.game_item_id in misc_item_ids

def is_dungeon_item(item: GameItem) -> bool:
    return item.game_item_id in dungeon_item_ids

def is_progressive(item: GameItem) -> bool:
    return item.game_item_id in progressive_item_ids

def is_inventory_item(item: GameItem) -> bool:
    return item.game_item_id in inventory_item_ids

junk_ids = list(range(0x13))
junk_ids.extend(list(range(0x15,0x20)))
junk_ids.extend(list(range(0x3F, 0x4B)))
junk_ids.extend([0x82, 0x83])

misc_item_ids = list(range(0x69, 0x73)) # Pearls + Songs
misc_item_ids.extend(list(range(0xA3, 0xAB))) # Tingle Statues + Hurricane Spin
misc_item_ids.append(0xB2) # Magic Meter Upgrade
misc_item_ids.append(0x78) # Boat's Sail
misc_item_ids.append(0xDB) # Ghost Ship Chart
misc_item_ids.append(0x43) # Hero's Charm
misc_item_ids.append(0x24) # Spoils Bag
misc_item_ids.append(0x2C) # Bait Bag

treasure_chart_ids = list(range(0xCC,0xDB))
treasure_chart_ids.extend(list(range(0xDD,0xF7)))

triforce_chart_ids = list(range(0xF7,0xFF))

dungeon_item_ids = [0x13, 0x14, 0x40, 0x1D, 0x5C, 0x5B, 0x74, 0x73, 0x81, 0x77]

inventory_item_ids = [
    0x20,
    0x21,
    0x22,
    0x24,
    0x25,
    0x28,
    0x29,
    0x2C,
    0x2D,
    0x2F,
    0x31,
    0x33,
    0x34,
    0x50
]

progressive_item_ids = [
    0x23,
    0x26,
    0x38,
    0x39,
    0x3A,
    0x3B,
    0x3C,
    0x3E,
    0x27,
    0x35,
    0x36,
    0xAB,
    0xAC,
    0xAD,
    0xAE,
    0xAF,
    0xB0
]