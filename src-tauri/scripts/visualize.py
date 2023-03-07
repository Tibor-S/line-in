import matplotlib.pyplot as plt


with open("./test.out", "r") as f:
    inp = f.readlines()
    f.close()
Y = list(map(lambda s: float(s), inp))
plt.plot(Y)
plt.show()
