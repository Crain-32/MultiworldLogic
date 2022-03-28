from dataclasses import dataclass

from logic.gameitem import GameItem
from logic.extras import item_id_dict, item_id_to_name_dict
from logic.requirements import Requirement


@dataclass
class LocationModificationMethod:
    filePath: str
    modification_type: str
    offsets: list[str]


@dataclass
class Location:
    name: str
    category_set: list[str]
    area_name: str
    requirement: Requirement
    location_modification_method: LocationModificationMethod
    world_id: int = -1
    logical: bool = False
    current_item: GameItem = GameItem(game_item_id=item_id_dict["Nothing"])

    @staticmethod
    def from_dict(dict_obj: dict[str, object], area_name: str):
        return Location(name=dict_obj["Name"],
                        category_set=list(dict_obj["Category"]),
                        requirement=Requirement.from_dict(dict_obj["Needs"]),
                        location_modification_method=LocationModificationMethod(
                            filePath=dict_obj["Path"],
                            modification_type=dict_obj["Type"],
                            offsets=dict_obj["Offsets"]
                            ),
                        area_name=area_name
                        )

    def is_logical_location(self):
        return self.logical and self.name != "DefeatGanondorf"

    def make_logical(self):
        self.logical = True
        return self

    def set_world_id(self, world_id: int):
        self.world_id = world_id
        return self

    def spoiler_representation(self):
        return f"W{self.world_id}: {self.name} - W{self.current_item.world_id} {item_id_to_name_dict[self.current_item.game_item_id]}"

    def short_hand_loc_rep(self):
        return f"W{self.world_id}- {self.name}"

    def short_hand_item_rep(self):
        return f"W{self.current_item.world_id} {item_id_to_name_dict[self.current_item.game_item_id]}"

    @staticmethod
    def partial_list_comparison(source_list, other_location) -> bool:
        return any(map(other_location.similar, source_list))

    def similar(self, other):
        return self.name == other.name and self.world_id == other.world_id

    def set_item_type(self):
        self.current_item = self.current_item.set_item_type()
        return self

    def json_output(self):
        if self.current_item.world_id == -1:
            self.current_item.world_id = self.world_id
        return {
            "Name": self.name,
            "WorldId": self.world_id,
            "Item": item_id_to_name_dict[self.current_item.game_item_id],
            "ItemWorldId": self.current_item.world_id
        }

    def __eq__(self, other):
        return self.name == other.name and self.world_id == other.world_id
