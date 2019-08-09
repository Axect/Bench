import numpy as np

def mat_to_vec(m):
    return np.squeeze(np.asarray(m))

def vec_to_mat(v):
    return np.matrix(v).T

def proj(u, v):
    uv = np.dot(u,v)
    uu = np.dot(u,u)
    return (uv / uu) * u

def gram_schmidt(m):
    (r, c) = m.shape
    result = np.matrix(np.zeros(m.shape))
    col_norm = np.zeros(c)
    result[:,0] = m[:,0]
    col_norm[0] = np.linalg.norm(result[:,0])
    for k in range(1, c):
        vk = mat_to_vec(m[:,k])
        puv = np.zeros(len(vk))
        for j in range(0, k):
            puv += proj(mat_to_vec(result[:,j]), vk)
        result[:,k] = vec_to_mat(vk - puv)
        col_norm[k] = np.linalg.norm(result[:,k])
    for i in range(0, c):
        result[:,i] = result[:,i] / col_norm[i]
    return result

m = np.matrix(np.random.rand(100,100))
for i in range(0,100):
    for j in range(0, i):
        m[i,j] = m[j,i]



# dm = gram_schmidt(m)
# for i in range(0, 100):
#     m = dm.T * m * dm
#     dm = gram_schmidt(m)

np.set_printoptions(precision=3, suppress=True)

print(m*m)

# print(m)
