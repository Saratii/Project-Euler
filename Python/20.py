n = 100
factorial = 1
for i in range(1, n+1):
    factorial *= i
print(sum([int(x) for x in str(factorial)]))

