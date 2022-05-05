import math
def calc(n):
    digits = [int(d) for d in str(n)]
    sum = 0
    for i in digits:
        sum += math.pow(i, 5)
    return sum
i = 2
nums = []
while True:
    if calc(i) == i:
        print(i)
        nums.append(i)
        print(f'sum {sum(nums)}')
    i += 1
