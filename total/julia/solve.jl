using LinearAlgebra
a = rand(100, 100);
b = rand(100);
x = inv(a) * b;