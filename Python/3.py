#The prime factors of 13195 are 5, 7, 13 and 29.
#What is the largest prime factor of the number 600851475143 ?
num = 600851475143
def GreatestPrime(num):
    greatest = 0
    i = 1
    while True:
        if num%i == 0:
            if i > greatest: greatest = i
            num /= i
        i+=1
        if i > num:
            return greatest
            
print(GreatestPrime(num))