import matplotlib.pyplot as plt

with open('data.csv') as f:
    data = f.read()
    
data = data.split('\n')
del data[-1]
data = list(map(float, data))
plt.plot(data)
plt.savefig('graph.jpg')
plt.close()
print('Graph Created')