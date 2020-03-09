using LinearAlgebra

function rand_det()
    a = rand(100, 100);
    return det(a)
end

rand_det()
