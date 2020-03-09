import numpy as np
import sys

n = int(sys.argv[1])

a = np.random.random((n, n))
print(np.linalg.det(a))
