import pylab as plt
import pandas as pd

add = pd.read_csv("add.csv")
mul = pd.read_csv("mul.csv")
det = pd.read_csv("det.csv")
inv = pd.read_csv("inv.csv")
sol = pd.read_csv("solve.csv")

rust = [add["mean"][0], mul["mean"][0], det["mean"][0], inv["mean"][0], sol["mean"][0]]
julia = [add["mean"][1], mul["mean"][1], det["mean"][1], inv["mean"][1], sol["mean"][1]]
python = [add["mean"][2], mul["mean"][2], det["mean"][2], inv["mean"][2], sol["mean"][2]]

# Use latex
plt.rc('text', usetex=True)
plt.rc('font', family='serif')

# Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Benchmark of Matrix operations", fontsize=16)
plt.xlabel(r'Operations', fontsize=14)
plt.ylabel(r'time(sec)', fontsize=14)


temp = [1, 2, 3, 4, 5]
plt.xticks(temp, ('Add', 'Mul', 'Det', 'Inv', 'Solve'))
plt.plot(temp, rust, label='Peroxide')
plt.plot(temp, julia, label='Julia')
plt.plot(temp, python, label='Numpy')

plt.legend(fontsize=12)
plt.grid()
plt.savefig("matrix_result.png", dpi=300)
