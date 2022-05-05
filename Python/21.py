import math
def factors(n):
    factorList = []
    for i in range(1, int(math.ceil(n/2))+1):
        if n%i == 0:
            factorList.append(i)
    return factorList
amicables = []
for i in range(1, 10000):
    if i == sum(factors(sum(factors(i)))) and sum(factors(sum(factors(i)))) != sum(factors(i)):
        amicables.append(i)
print(sum(amicables))

