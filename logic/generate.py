
from world import World
from random import Random
from fill import fill
from search import generate_playthrough
from settings import Settings
from extras import dump_object, dump_simple_world_locations
import timer

@timer.timer
def generate_worlds(seed_settings: list[Settings], seed: int):

    initial_worlds = [World(settings, world_id) for world_id, settings in enumerate(seed_settings)]
    random_state = Random(seed)
    worlds = []
    for world in initial_worlds:
        world.load_world("./world.json", "./Macros.json")
        world.determine_chart_mappings(random_state)
        world.determine_progression_locations()
        world.determine_race_mode_dungeons(random_state)
        world.set_item_pools()
        # world.randomize_entrances() Waiting on the proper implementation
        worlds.append(world)

    result = fill(worlds, random_state)
    # dump_simple_world_locations(result, "three-world-seed")
    # print("Generating Playthrough")
    # generate_playthrough(worlds)
