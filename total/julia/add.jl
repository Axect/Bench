#=
add:
- Julia version: 
- Author: rakhan
- Date: 2019-08-24
=#

@inline @fastmath function add() 
    a = zeros(1000, 1000);
    return a + a;
end

add()
