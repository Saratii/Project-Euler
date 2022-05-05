import time
import math
start = time.time()

def divisors(n):
	divs = [1]
	for i in range(2, int(math.sqrt(n))+1):
		if n%i == 0:
			divs.append(i)
			divs.append(n/i)
	return list(set(divs))

ab = []

for i in range(12,28123):
	if sum(divisors(i))>i:
		ab.append(i)

non_ab_sum = [x for x in range(28123)]

for i in range(len(ab)):
	for j in range(i,28123):
		if ab[i]+ab[j] < 28123:
			non_ab_sum[ab[i]+ab[j]] = 0
		else:
			break

print(sum(non_ab_sum))
print(time.time() -start)