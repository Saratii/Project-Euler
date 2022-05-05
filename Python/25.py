fibs = [0, 1]
i = 1
while True:
    old = fibs[1]
    fibs[1] = fibs[0] + fibs[1]
    fibs[0] = old
    i += 1
    if len(str(fibs[1])) == 1000:
        break
print(i)
