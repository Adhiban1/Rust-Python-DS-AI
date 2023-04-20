from ctypes import *
import random
import numpy as np

# random.seed(0)
clib = CDLL("./target/release/liblinear_regression1.so")
clib.f.argtypes = [POINTER(c_double), c_int]
clib.f.restype = POINTER(c_double)

def random_points(m:int or float, c:int or float, d:tuple, n:int):
    y = lambda x: m * x + c + random.uniform(d[0], d[1])
    x_list = list(range(n))
    y_list = list(map(y, x_list))
    return x_list, y_list

def rustLr(x, y):
    combined_list = x + y
    l = ( c_double * len(combined_list) )(*combined_list)
    result = clib.f(l, len(combined_list))
    return result[0], result[1]
