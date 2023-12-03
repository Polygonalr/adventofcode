# Read data from file
f = open("../input.txt", "r")
raw_data = []
for line in f.readlines():
    temp = line.split(")")
    temp[-1] = temp[-1].rstrip()
    raw_data.append(temp)

# Process data into dict such that parent: [children]
data = {}

for entry in raw_data:
    if entry[0] not in data.keys():
        data[entry[0]] = []
    data[entry[0]].append(entry[1])

# Recursive function to compile height of each node and return the sum of all heights
def compile(current_node, current_height):
    if current_node not in data.keys():
        return current_height
    sum = 0
    for child in data[current_node]:
        sum += compile(child, current_height + 1)
    return sum + current_height

print(compile("COM", 0))
