# Read data from file
f = open("../input.txt", "r")
raw_data = []
for line in f.readlines():
    temp = line.split(")")
    temp[-1] = temp[-1].rstrip()
    raw_data.append(temp)

class Vertex:
    def __init__(self):
        self.adjacent = []
        self.distance = 0
        self.visited = False

# Process data into dict such that node: neighbours
data = {}

for entry in raw_data:
    if entry[0] not in data.keys():
        data[entry[0]] = Vertex()
    if entry[1] not in data.keys():
        data[entry[1]] = Vertex()
    data[entry[0]].adjacent.append(entry[1])
    data[entry[1]].adjacent.append(entry[0])

# Simple breath-first search function
def search(start, end):
    queue = [start]
    while len(queue):
        curr_node = queue.pop(0)
        if(curr_node == end):
            return data[curr_node].distance
        data[curr_node].visited = True
        for entry in data[curr_node].adjacent:
            if(not data[entry].visited):
                data[entry].distance = data[curr_node].distance + 1
                queue.append(entry)

print(search("YOU", "SAN") - 2) # - 2 to exclude YOU and SAN