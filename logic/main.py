import logging

from generate import *
from settings import Settings, SWORDLESS

logging.basicConfig(level=logging.DEBUG)
def start_randomizing(debug: bool):
    # seed = random.randint(0, 100000)
    seed = 10000

    if debug:
        print(f"Using the following Seed: {seed}")

    settings_4 = Settings([], swordless=True, sword_mode=SWORDLESS)
    settings_4.progressive_categories["NoSword"] = True
    seed_settings = [
        Settings([])
    ] * 3

    generate_worlds(seed_settings, seed)


start_randomizing(True)