library("ggpubr")

dat_inv <- read.csv("../inv.csv")
dat_det <- read.csv("../det.csv")
dat_add <- read.csv("../add.csv")
dat_mul <- read.csv("../mul.csv")

dat <- rbind(dat_add, dat_mul, dat_det, dat_inv)

lib <- c("Peroxide(Rust)", "DNumeric(D)", "R", "Numpy(Python)", "Julia")
Library <- rep(lib, 4)

work <- c(rep("add", 5), rep("mul", 5), rep("det", 5), rep("inv", 5))

df <- cbind(dat, Library, work)

g <- ggline(df, "work", "mean", type='l', color="Library", ylab="time(s)", title="Benchmark for matrix operations")
g + yscale("log10", .format=TRUE)
