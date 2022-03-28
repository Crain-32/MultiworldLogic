import sys
import json
from collections import OrderedDict

import extras
# Example command `python world_items_count.py input_file.json`

def world_to_item_count(obj) -> dict[str, int]:
    item_pool = obj["item_pool"]
    item_pool.extend(obj["starting_items"])
    return calculation_wrapper(item_pool)

def calculation_wrapper(worlds_item_list: list[list[dict[str, int]]]) -> dict[str, int]:
    item_amount = {key: 0 for key in list(range(256))}
    for item_list in worlds_item_list:
        for item in item_list:
            old_amount = item_amount[item["game_item_id"]]
            old_amount += 1
            item_amount[item["game_item_id"]] = old_amount
    return {extras.item_id_to_name_dict[key]: val for key, val in item_amount.items() if key in extras.item_id_to_name_dict}

def world_wrapper():
    with open("./" + sys.argv[1]) as input_file:
        json_input = json.load(input_file)

    if isinstance(json_input, list):
        output_count = []
        for world in json_input:
            output_count.append(world_to_item_count(world))
    else:
        output_count = world_to_item_count(json_input)

    with open("./dump/items/" + sys.argv[1].split('/')[-1], 'w') as output_file:
        json.dump(output_count, output_file, indent='\t')

    with open("./dump/items/reduced_" + sys.argv[1].split('/')[-1], 'w') as output_file:
        output_count = {key: val for key, val in output_count.items() if val != 0}
        ordered_keys = sorted(output_count)
        filtered_output = OrderedDict()
        for key in ordered_keys:
            filtered_output[key] = output_count[key]

        json.dump(filtered_output, output_file, indent='\t')

def item_list_wrapper():

    with open("./" + sys.argv[1]) as input_file:
        json_input = json.load(input_file)
    output_info = calculation_wrapper(json_input)

    with open("./dump/items/list_" + sys.argv[1].split('/')[-1], 'w') as output_file:
        output_count = {key: val for key, val in output_info.items() if val != 0}
        ordered_keys = sorted(output_count)
        filtered_output = OrderedDict()
        for key in ordered_keys:
            filtered_output[key] = output_count[key]
        json.dump(filtered_output, output_file, indent='\t')

if __name__ == "__main__":
    if len(sys.argv) == 2:
        world_wrapper()
    elif  len(sys.argv) == 3:
        item_list_wrapper()
    else:
        print(f"Invalid Arguments {sys.argv}")