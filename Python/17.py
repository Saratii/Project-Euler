#If the numbers 1 to 5 are written out in words:
#one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
#If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words,
#how many letters would be used?
def t1to19(j):
    j = str(j)
    if j == "0" or j == "00":
        return 0
    elif j == "1" or j == "01":
        return len("one")
    elif j == "2" or j == "02":
        return len("two")
    elif j == "3" or j == "03":
        return len("three")
    elif j == "4" or j == "04":
        return len("four")
    elif j == "5" or j == "05":
        return len("five")
    elif j == "6" or j == "06":
        return len("six")
    elif j == "7" or j == "07":
        return len("seven")
    elif j == "8" or j == "08":
        return len("eight")
    elif j == "9" or j == "09":
        return len("nine")
    elif j == "10":
        return len("ten")
    elif j == "11":
        return len("eleven")
    elif j == "12":
        return len("twelve")
    elif j == "13":
        return len("thirteen")
    elif j == "14":
        return len("fourteen")
    elif j == "15":
        return len("fifteen")
    elif j == "16":
        return len("sixteen")
    elif j == "17":
        return len("seventeen")
    elif j == "18":
        return len("eighteen")
    elif j == "19":
        return len("nineteen")
def t20to99(x):
    tot = 0
    x = str(x)
    if x[0] == "2":
        tot += len("twenty")
    elif x[0] == "3":
        tot += len("thirty")
    elif x[0] == "4":
        tot += len("forty")
    elif x[0] == "5":
        tot += len("fifty")
    elif x[0] == "6":
        tot += len("sixty")
    elif x[0] == "7":
        tot += len("seventy")
    elif x[0] == "8":
        tot += len("eighty")
    elif x[0] == "9":
        tot += len("ninety")
    if x[1] == "0":
        tot += 0
    elif x[1] == "1":
        tot += len("one")
    elif x[1] == "2":
        tot += len("two")
    elif x[1] == "3":
        tot += len("three")
    elif x[1] == "4":
        tot += len("four")
    elif x[1] == "5":
        tot += len("five")
    elif x[1] == "6":
        tot += len("six")
    elif x[1] == "7":
        tot += len("seven")
    elif x[1] == "8":
        tot += len("eight")
    elif x[1] == "9":
        tot += len("nine")
    return(tot)

def threeDigit(i):
    tot = 0
    i = str(i)
    tot += t1to19(i[0])
    if i[1] != "0" and i[2] != "0":
        tot += 3
    elif i[1] != "0": tot += 3
    elif i[2] != "0": tot += 3
    tot += len("hundred")

    if int(i[1]+i[2]) > 19: tot += t20to99(i[1]+i[2])
    elif int(i[1]+i[2]) < 20:
        tot += t1to19(i[1]+i[2])
    return tot

def calc():
    total = 0
    for i in range(1, 20):
        total += t1to19(i)
    for i in range(20, 100):
        total += t20to99(i)
    for i in range(100, 1000):
        total += threeDigit(i)
    total += len("onethousand")
    return total
print(calc())



