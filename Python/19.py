import datetime
x = datetime.datetime(1900, 1, 1)
print(x.strftime("%A"))
sundayCount = 0
for year in range(1901, 2001):
    for month in range(1, 13):
        if datetime.datetime(year, month, 1).strftime("%A") == "Sunday":
            sundayCount += 1
print(sundayCount)