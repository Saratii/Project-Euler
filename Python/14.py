#n → n/2 (n is even)
#n → 3n + 1 (n is odd)
#Using the rule above and starting with 13, we generate the following sequence:
#13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
#It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. 
# Although it has not been proved yet (Collatz Problem), 
# it is thought that all starting numbers finish at 1.
#Which starting number, under one million, produces the longest chain?
def Calc(Number):
    NumberOfTerms = 1
    while Number > 1:
        if Number % 2 == 0:
            Number //= 2
        else: 
            Number = Number*3 + 1
        NumberOfTerms +=1
    return NumberOfTerms

def Collatz(Max):
    Greatest = 0
    greatestNumy = 0
    i = 0
    while i < Max+1:
        if Calc(i) > Greatest: 
            Greatest = Calc(i)
            greatestNumy = i
            print(greatestNumy)
        i+=1
    return greatestNumy
print(Collatz(999999))



