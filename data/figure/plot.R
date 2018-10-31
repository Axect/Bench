library("ggpubr")

a <- read.csv("inv.csv")
lib <- c("Peroxide(Rust)", "R", "Julia", "DNumeric(D)", "Numpy(Python)")
a <- cbind(a, lib)

ggline(a, "lib", "mean", type='l', ylab="time(s)") + yscale("log10", .format=TRUE)
