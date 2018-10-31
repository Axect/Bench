a = zeros(100, 100)
for i = 1:100
  for j = 1:100
    a[i, j] = (i - 1) * 100 + j
  end
end
println(a * a)
