#!/usr/bin/python

import sys, random, os

n = int(sys.argv[1]) if len(sys.argv) > 1 else 50
s = int(sys.argv[2]) if len(sys.argv) > 2 else 1000
output = open(f"{os.path.basename(__file__).split('.')[0] + (('_' + sys.argv[1]) if len(sys.argv) > 1 else '')}.txt", "w")
for _ in range(n):
    output.write(f"{s * random.random()} {s * random.random()}\n")
