#!/usr/bin/pyhton

import sys
import matplotlib.pyplot as plt
import numpy as np

coords = np.loadtxt(sys.argv[1], delimiter=",", skiprows=1)
info = np.loadtxt(sys.argv[2], delimiter=",")
label = info[:,2]

assert(coords.shape[0] == info.shape[0])

plt.scatter(coords[:,0], coords[:,1], c=label, cmap='Pastel1')
plt.show()