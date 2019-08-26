@inline @fastmath function rand_mul()
    a = rand(1000, 1000);
    return a * a;
end

rand_mul()
