#!/usr/bin/python

from tkinter import *
import os

root = Tk()
points = []

canvas = Canvas(root, width=1000, height=1000, bd=0, bg="#ffffff")
point_size = 3


def click(event):
    if len(points) > 0:
        canvas.create_line(event.x, event.y, points[-1][0], points[-1][1])
    canvas.create_oval(event.x - point_size, event.y - point_size,
                       event.x + point_size, event.y + point_size, fill="#000000")
    points.append((event.x, event.y))


canvas.bind("<Button-1>", click)
canvas.pack()

root.mainloop()
output = open(f"{os.path.basename(__file__).split('.')[0]}.txt", "w")
for x, y in points:
    output.write(f"{x} {y}\n")
