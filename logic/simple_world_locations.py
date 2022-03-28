import sys
import json
import extras
# Example command `python simple_world_locations.py input_file.json`

def wrapper():
    with open(sys.argv[1]) as input_worlds:
        input_json = json.load(input_worlds)

    if isinstance(input_json, list):
        output_info = list(map(lambda world: simplify_world_locations(world["location_entries"]), input_json))
    else:
        output_info = simplify_world_locations(input_json["location_entries"])

    with open("./dump/locs/" + sys.argv[1].split("/")[-1], 'w') as output_file:
        json.dump(output_info, output_file, indent='\t')

def simplify_world_locations(locations):
    location_output = []
    for loc in locations:
        output = dict()
        output["Name"] = f'{loc["name"]}: World {loc["world_id"]}'
        output["Item"] = f'{extras.item_id_to_name_dict[loc["current_item"]["game_item_id"]]}'
        output["InLogic"] = f'{loc["logical"]}'
        location_output.append(output)
    return location_output

def loc_only_wrapper():
    with open(sys.argv[1]) as input_worlds:
        input_json = json.load(input_worlds)

    output_info = simplify_world_locations(input_json)
    with open("./dump/locs/" + sys.argv[1].split("/")[-1], 'w') as output_file:
        json.dump(output_info, output_file, indent='\t')

if __name__ == "__main__":
    if len(sys.argv) == 2:
        wrapper()
    elif len(sys.argv) == 3:
        loc_only_wrapper()
    else:
        print(f"Invalid Arguments passed {sys.argv}")