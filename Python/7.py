import math
list = []
def isPrime(x):
    count = 0
    for i in range(1, math.ceil(x**0.5+1)):
        if x%i ==0:
            count +=1
    if count < 2:
        return True
l = 1
while len(list) < 10001:
    if isPrime(l):
        list.append(l)
    l +=1
print(list[-1])
