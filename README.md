## Logic Optimization for Multiworld ##

Note, Race Mode is Plando with Extra Steps

### Input
```yaml
Locations:
- 123: # Outset Island - Savage Labyrinth - Floor 50
  Need:
    - 0x20000002341213
    - 0x00000000234221
  Count:
    - 0x000010000: # Item Id or Index
      needed: 2
  AccessPoints:
    - 3453
  Category:
    - 1 # Savage
    - 2
  Region:
    Specific:
      125 # ET Miniboss
    Connected: # Can be randomized to a different location
      False

Region:
  ET:
    - 123 # Basement
    - 124 # Left Side
    - 125 # Miniboss
    - 126 # Boss
AccessMacros:
  - 3453:
    Need:
      - 0x12341231
  - 3454:
    Need:
      - 0x10123111
Items:
  - 0x000010000: # DRC Small Key
      Types: 0x0100001 # Dungeons, Tingle Chest
      Regions: 
        - 120 # If needed in Region 100%, if region not in total logic, it is not a required item.
      Count: 4

Mappings:
  Locations:
    - Outset Island - Savage Labyrinth - Floor 50: 123
    - Can Access Item Location "Outset Island - Savage Labyrinth - Floor 30": 3453
  Items:
    - DRC Small Key: 0x000010000
  Flags:
    - Ganon's Castle Trials: 0x001000

PresetLocations:
  - Location_Str: Item_ID
  - Gohma Heart Container: 0x01000

Settings:
  - Value_One
  - Value_Two
  BannedRegions:
    General:
      - ET
    Specific:
      - 123
  Flags:
    - 0x001000: # Bit Index
       Types: 0x0011000 # Settings required to care about this
       type: number
       min: 0
       max: 6
    - 0x010000:
        Types: 0x0011000 # Settings required to care about this
        type: boolean
    - 0x100000:
        Types: 0x0011000 # Settings required to care about this
        # Trade Sequence
        type: range
        options:
          - FASEF
          - AFAEFA
          - AEFAEFA
          - Cuckoo
# Review Flag collisions and propagating
# Can we propagate by Region?
       
```