list = []
for i in range(2, 101):
    for j in range(2, 101):
        num = pow(i, j)
        if num not in list:
            list.append(num)
print(len(sorted(list)))