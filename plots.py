import math
import matplotlib.pyplot as plt

x = [pow(10, i) for i in range(3, 15)]
ymax = [math.floor(math.log(n/2)/math.log(4 * math.e * n ** 2 / (n * (n-1)))) for n in x]
ymin = [math.floor(math.log(n/2)/math.log(math.e*math.sqrt(n)/4)) for n in x]


plt.clf()
plt.xlabel('n')
plt.ylabel("q")
plt.title("q value for m > 8 * n ^ (3/2)")
plt.xscale('log')


plt.plot(x, ymax, "-r")
plt.plot(x, ymin, "-r")
plt.fill_between(x, ymin, ymax, facecolor='red', alpha=0.5)
plt.savefig('q_plot.png')


x = [pow(10, i) for i in range(3, 15)]
ymax = [math.floor(math.log(n/2)/math.log(4 * math.e * n ** 2 / (n * (n-1)))) for n in x]
ymin = [math.floor(math.log(n/2)/math.log(math.e*math.sqrt(n)/4)) for n in x]


plt.clf()
plt.xlabel('n')
plt.ylabel("q")
plt.title("r value for m > 8 * n ^ (3/2)")
plt.xscale('log')


plt.plot(x, ymax, "-r")
plt.plot(x, ymin, "-r")
plt.fill_between(x, ymin, ymax, facecolor='red', alpha=0.5)
plt.savefig('q_plot.png')