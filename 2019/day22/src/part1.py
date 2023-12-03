pos = 2019
deck_size = 10007

# Read data from file and process each line, simple maffs
f = open("../input.txt", "r")
for line in f.readlines():
    print(line.rstrip())
    line = line.rstrip().split(" ")
    if line[1] == "into":
        pos = deck_size - pos - 1
    elif line[1] == "with":
        pos = (pos * int(line[3])) % deck_size
    else: #cut
        pos = (pos - int(line[1])) % deck_size

print(pos)