#If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
#The sum of these multiples is 23.
#Find the sum of all the multiples of 3 or 5 below 1000.
def sum(rangy, m1, m2):
    count = 0
    for i in range(rangy): 
        if i % m1 == 0 or i % m2 == 0:
            count+=i
    return count
print(sum(1000, 3, 5))