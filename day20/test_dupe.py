s = set()

with open("input.txt") as f:
    for line in f:
        if int(line) in s:
            print(f"Duplicate found: {line}")
        s.add(int(line))

