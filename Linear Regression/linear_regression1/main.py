
import matplotlib.pyplot as plt
from linear_reg import random_points, rustLr
import random
random.seed(10)
# import numpy as np

x, y = random_points(1, 1, (-50, 50), 100)
m, c = rustLr(x, y)

x1 = list(range(int(min(x))-1, int(max(x))+2))
y1 = [m*i+c for i in x1]

# k, d = np.polyfit(x, y, 1)

# x2 = x1
# y2 = [k*i+d for i in x1]

plt.subplots(figsize=(15,10))
plt.plot(x, y, 'o')
plt.plot(x1, y1, color='red')
# plt.plot(x2, y2, color='green')
plt.savefig('graph.jpg')
plt.close()