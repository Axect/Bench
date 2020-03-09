import numpy as np
import sys


n = int(sys.argv[1])
a = np.random.random((n, n))
b = np.random.random((n, n))

c = np.matmul(a, b)

print(c[n-1, n-1])
