#!/usr/bin/python

import sys, os

n = int(sys.argv[1]) if len(sys.argv) > 1 else 10
s = int(sys.argv[2]) if len(sys.argv) > 2 else 100
output = open(f"{os.path.basename(__file__).split('.')[0] + (('_' + sys.argv[1]) if len(sys.argv) > 1 else '')}.txt", "w")
for i in range(n):
    for j in range(n):
        output.write(f"{i * s} {j * s}\n")
