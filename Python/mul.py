import numpy as np

a = np.empty((100, 100))
for i in range(0, 100):
    for j in range(0, 100):
        a[i, j] = i * 100 + j + 1

print(np.matmul(a,a))
