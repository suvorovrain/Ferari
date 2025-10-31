import json, random

with open("input.json", "r", encoding="utf-8") as f:
    base = json.load(f)

def gen_mobs(n):
    mobs = {
        "player": base["mobs"]["player"]
    }
    for i in range(1, n + 1):
        mob_type = random.choice(["imp_20_0", "ghost_30_0"])
        speed = 0.5 if "imp" in mob_type else 0.42
        direction = random.choice(["left", "right"])
        mobs[f"mob_{i}"] = {
            "x_start": random.randint(100, 600),
            "y_start": random.randint(100, 600),
            "asset": mob_type,
            "is_player": False,
            "behaviour": {
                "type": "walker",
                "direction": direction,
                "speed": speed
            }
        }
    return mobs

for n in [500, 5000, 10000, 100000, 1000000]:
    new_map = base.copy()
    new_map["mobs"] = gen_mobs(n)
    with open(f"demo_map_{n}.json", "w", encoding="utf-8") as f:
        json.dump(new_map, f, indent=2)
    print(f"demo_map_{n}.json created")
