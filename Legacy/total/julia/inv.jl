using LinearAlgebra

function rand_inv()
    a = rand(100, 100)
    return inv(a)
end

b = rand_inv()
