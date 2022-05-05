#Find the sum of all the primes below two million.
import math
def isPrime(x):
    for i in range(2, math.floor(x**0.5)+1):
        if x % i == 0: # if i is a factor of the number:
            return False # then the number is not a prime; return False
    # if it can get through the for-loop without returning False, that means it's prime
    return True
        

def SumOfPrimes(MAX):
    count = 0
    for x in range(2, MAX+1):
        if isPrime(x):
            count+=x
    return count
print(SumOfPrimes(2000000))