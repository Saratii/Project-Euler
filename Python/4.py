Palindromes = []
biggest = 0
def isPalindrome(x):
    if str(x)[::-1] == str(x):
        return True
for i in range(100000, 1000000):
    if isPalindrome(i):
        Palindromes.append(i)
for l in range(100, 1000):
    for k in range(100, 1000):
        c = l*k
        if c > biggest and c in Palindromes:
            biggest = c 
print(biggest)

