import json

with open('db.json', 'r') as file:
    data = json.load(file)

sorted_data = sorted(data["general"], key=lambda x: x["name"])

ordered_general = []
for item in sorted_data:
    ordered_general.append({
        "name": item["name"],
        "description": item["description"],
        "command": item["command"]
    })


with open('db.json', 'w') as outfile:
    json.dump({"general": ordered_general}, outfile, indent=4)
