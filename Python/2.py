list = [0]
list2 = []
x = 1
y = 0
while x < 4000000:
    x = x+y
    list.append(x)
    y = list[-2]
for i in list:
    if i%2 == 0:
        list2.append(i)
print(sum(list2))