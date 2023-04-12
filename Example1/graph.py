import matplotlib.pyplot as plt
import numpy as np

x = np.linspace(-12, 12, 100)
y = x**2

plt.plot([10, 10], [0, 100], '--', color='grey')
plt.plot(x, y)
plt.plot([10], [100], 'o', color='red')
plt.tight_layout()
plt.savefig('graph.png')
plt.close()

#functions
y = lambda x: x**2
loss = lambda x, yt: (yt - y(x))**2
grad = lambda x, yt: (loss(x+0.000001, yt) - loss(x-0.000001, yt))/(2*0.000001)

# Variables
x = 10
yt = 0
lr = 0.0001

# loop
x1 = [x]
for i in range(1, 101):
    x -= lr*(i**3)*grad(x, yt)
    x1.append(x)
    if i >= 95:
        print('Loss:',loss(x, yt))

# result
print('x:', x)

x2 = np.linspace(-12, 12, 100)
y2 = x2**2

plt.plot(x1, [y(i) for i in x1], 'o', color='red')
plt.plot(x2, y2)
plt.tight_layout()
plt.savefig('graph1.png')
plt.close()