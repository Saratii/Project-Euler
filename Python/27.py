primes = []
notPrimes = []
def isPrime(n):
    global primes
    global notPrimes
    if n in primes:
        return True 
    elif n in notPrimes:
        return False
    else:
        if n == 1:
            notPrimes.append(n)
            return False
        i = 2
        while i*i <= n:
            if n % i == 0:
                notPrimes.append(n)
                return False
            i += 1
        primes.append(n)
        return True

def solve():
    n = 1
    consecutive = 0
    most = 0
    bestA = 0
    bestB = 0
    for a in range(-999, 1000):
        for b in range(-1000, 1001):
            while True:
                if isPrime(n**n + a * n + b):
                    consecutive += 1
                else: 
                    if consecutive > most:
                        most = consecutive
                        bestA = a
                        bestB = b
                    break
                n += 1
            n = 1
    print(bestA, " ", bestB)
    return bestA * bestB
print(solve())

            
