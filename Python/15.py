#Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, 
#there are exactly 6 routes to the bottom right corner.
#How many such routes are there through a 20×20 grid?
def factorial(i):
    number = 1
    while i > 0:
        number *= i
        i -= 1
    return number
print(factorial(20))
def choose(n, r):
    num = factorial(n)/(factorial((n-r))*factorial(r))
    return num
print(choose(40, 20))

#n!/(n-r)! is the number of r things from n things where you do care about the order they're chosen in
#permutations
#The number of ways to choose r cookies from a jar with n cookies,
# where the order they're chosen in does matter

#n!((n-r)!r!) is the number of r things from n total things where you do not care about the order
#combinatorics
#the number of ways to choose r cookies from a jar with n cookies,
#  where the order they're chosen in doesnt matter
#5 choose 3 = 5!/((5-3)!3!)
#where n = 5 and 3 = r
# n choose r
