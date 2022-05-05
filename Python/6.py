count1, count2 = 0, 0
for i in range(101):
    count1 += i**2
for j in range(101):
    count2 +=j
count2 = count2**2
print(count2-count1)