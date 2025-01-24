import sys

try:
    r1 = float(int(sys.argv[1]))
    r2 = float(int(sys.argv[2]))
    y1 = int(sys.argv[3])
    y2 = int(sys.argv[4])
    n = (y2 - y1)
    d = (r2 - r1) / n
    r = r1
    for i in range(n-1):
        r = r + d
        print(r)
except Exception as e:
    print(e)
    print("$ python grad.py r1 r2 y1 y2")



