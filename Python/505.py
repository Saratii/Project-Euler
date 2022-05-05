from functools import lru_cache
from math import floor

@lru_cache(maxsize=None)
def x(n):
    if n == 0:
        return 0
    elif n == 1:
        return 1
    elif n % 2 == 0:
        return 3*x(n/2)+2*x(floor(n/4)) % pow(2, 60)
    elif n % 2 != 0:
        k = (n-1)/2
        return 2*x(k) + 3 * x(floor(k/2)) % pow(2, 60)

@lru_cache(maxsize=None)
def y(n, k):
    if k >= n:
        return x(k)
    else:
        return pow(2, 60) - 1 - max(y(n, 2*k), y(n, 2*k+1))
def A(n):
    return y(n, 1)
print(A(pow(10, 12))) 

