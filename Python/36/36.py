from math import floor
def isPalindrome(n):
    digits = [int(d) for d in str(n)]
    for i in range(floor(len(digits)/2)):
        if digits[i] != digits[len(digits)-i-1]:
            return False
    return True

def getBin(n):
    n = bin(n)
    list = [c for c in str(n)]
    list.remove(list[0])
    list.remove(list[0])
    number = ""
    for i in list:
        number += i
    return number

count = 0
for i in range(1, 1000000):
    if isPalindrome(i):
        if isPalindrome(getBin(i)):
            count += i

print(count)

