#functions
y = lambda x: x**2
loss = lambda x, yt: (yt - y(x))**2
grad = lambda x, yt: (loss(x+0.000001, yt) - loss(x-0.000001, yt))/(2*0.000001)

# Variables
x = 10
yt = 0
lr = 0.0001

# loop
for i in range(1, 1001):
    x -= lr*(i**3)*grad(x, yt)
    if i >= 995:
        print('Loss:',loss(x, yt))

# result
print('x:', x)