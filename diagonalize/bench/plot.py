import pylab as plt

# Use latex
plt.rc('text', usetex=True)
plt.rc('font', family='serif')

# Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Benchmark for Gram-Schmidt", fontsize=16)
plt.xlabel(r'Frameworks', fontsize=14)
plt.ylabel(r'Time (sec)', fontsize=14)

x = [1,2,3]
y = [2.693, 4.724, 27.329]
plt.bar(x, y, label=r"500 iterations")
plt.xticks(x, ("Julia", "Peroxide", "Numpy"))
plt.legend(fontsize=12)
plt.savefig("bench_gram_schmidt.png", dpi=300)
