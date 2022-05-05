def divide(num, den):
    finitePart = []
    infinitePart = []
    remainders = []
    while True: 
        remainder = num % den
        times = num // den
        finitePart.append(times)
        for i in range(len(remainders)):
            if remainders[i] == remainder:
                infinitePart = finitePart[i+1:]
                finitePart = finitePart[:i+1]
                return finitePart, infinitePart
        remainders.append(remainder)
        num = remainder * 10
        
most = 0
for d in range(1, 1000):
    finite, infinite = divide(1, d)
    if len(infinite) > most:
        most = d
print(most)
