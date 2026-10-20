l1 = []
l2 = []
with open("input.txt") as f:
    for line in f.readlines():
        num1, num2 = map(int, map(lambda x: x.strip(), line.split("   ")))
        l1.append(num1)
        l2.append(num2)

l1.sort()
l2.sort()

def part1():
    total = 0
    for i in range(len(l1)):
        total += abs(l1[i] - l2[i])

    return total

def part2():
    n = len(l1)
    i, j = 0, 0
    count = 0
    score = 0
    while i < n and j < n:
        if l1[i] == l2[j]:
            count += 1
            j += 1
        elif l1[i] < l2[j]:
            score += l1[i] * count
            count = 0
            i += 1
        else:
            j += 1
    if i < n:
        score += l1[i] * count
    
    return score

print("part 1:", part1())
print("part 2:", part2())

