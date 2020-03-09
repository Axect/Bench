import numpy as np
import numpy.linalg as nl

a = np.random.rand(100, 100)
b = np.random.rand(100)
x = nl.solve(a, b)
print(x)