from enum import Enum
from functools import cmp_to_key

class Result(Enum):
    LESSTHAN = -1
    EQUAL = 0
    MORETHAN = 1

def compare(left, right):
    return check_pair(left, right).value

def check_pair(left, right):
    if isinstance(left, list) and isinstance(right, list):
        i = 0
        while i < len(left) and i < len(right):
            check = check_pair(left[i], right[i])
            if check == Result.MORETHAN:
                return Result.MORETHAN
                break
            elif check == Result.LESSTHAN:
                return Result.LESSTHAN
            i += 1
        if i == len(right) and i < len(left): # right side ran out
            return Result.MORETHAN
        elif i == len(left) and i < len(right): # left side ran out
            return Result.LESSTHAN
        else:
            return Result.EQUAL

    elif isinstance(left, list):
        return check_pair(left, [right]);
    elif isinstance(right, list):
        return check_pair([left], right);
    
    if left == right:
        return Result.EQUAL
    elif left < right:
        return Result.LESSTHAN
    return Result.MORETHAN
    

if __name__ == "__main__":
    f = open("input.txt")
    answer = 0
    for i, line in enumerate(f):
        if i % 3 == 0:
            left = eval(line)
        elif i % 3 == 1:
            right = eval(line)
            if check_pair(left, right) == Result.LESSTHAN:
                answer += i // 3 + 1
    print("Part 1:", answer)
    # ----- part 2 -----
    # Sort the signals with the custom compare function from part 1,
    # then print out all the signals in order
    f.seek(0)
    all_signals = []
    for line in f:
        if line == "\n":
            continue
        else:
            all_signals.append(eval(line))
    all_signals.sort(key=cmp_to_key(compare))
    # Manually analyze the output and determine the answer
    for (i, signal) in enumerate(all_signals):
        print(i + 1, ":", signal)





