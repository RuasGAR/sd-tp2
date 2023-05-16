import matplotlib.pyplot as plt
import numpy as np
from tabulate import tabulate
from math import pow
from matplotlib.ticker import ScalarFormatter


k = [1,2,4,8,16,32,64,128,256]
dif_n = [int(pow(10,7)), int(pow(10,8)), int(pow(10,9))]


def average(nums):
    sum = 0
    for i in nums:
      sum += i
    return sum/len(nums)

def read_file(n,k):
    with open(f"./results/{n}_{k}_bench.txt") as f:
       content = f.readlines()
       content = list(map(lambda x: float(x),content))
    return content

def generate_table(data, n):
    threads=data[0]
    periods=data[1]
    data = list(zip(threads, periods))

    headers = ["K Threads", "Tempo Exec.(s)"]
    table = tabulate(data, headers, tablefmt="fancy_grid")
    print(f"Tabela para N = {n}")
    print(table)


def generate_graph(data):
    
    x = np.array(k)
    y = data

    curve_fit = np.polyfit(x,y, deg=2)
    curve_func = np.poly1d(curve_fit)
    curve_x = np.linspace(x.min(), x.max(),100)
    curve_y = curve_func(curve_x)

    _, ax = plt.subplots()

    ax.scatter(x, y, color='blue', label='Data Points', s=100)
    ax.plot(curve_x, curve_y, color='red', label='Curve Fit')
    ax.set_xscale('log')

    ax.set_xticks(x) 
    ax.get_xaxis().set_major_formatter(ScalarFormatter())


    plt.xlabel('k threads')    
    plt.ylabel('Tempo de execução (s)')
    plt.title('T(k) (eixo x em log base 10)')
    plt.legend()
    plt.grid(True)
    plt.show()
    print("")
    print("")



def main():

    for number in dif_n:
    
        n_exec_times=[]

        for i in k:
            data = read_file(number, i)
            avg = average(data)
            n_exec_times.append(avg)

        generate_table([k,n_exec_times],number)
        generate_graph(n_exec_times)

main();