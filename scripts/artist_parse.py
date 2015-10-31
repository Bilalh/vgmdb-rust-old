#!/usr/bin/env python3
#-*- coding: utf-8 -*-
from pprint import pprint
import sys
import argparse

parser = argparse.ArgumentParser()
parser.add_argument('--debug', action='store_true', dest='debug', help='')
args = parser.parse_args()

# from http://vgmdb.net/album/52318
yk=("Yasumasa Kitagawa",
   ["01-04.06-08.10-17.19.22-30.33.34"
   ,"02.04-06.09.10.13.18-20.22-24.26-29.32.33"
   ]) # yapf: disable

hm=("Hiromitsu Maeba",
   ["05.09.18.20.21.31.32"
   ,"01.03.07.08.11.12.14-17.21.25.30.31"
   ]) # yapf: disable

artists = [yk, hm]


def convert(s, *, key):
    nums = set()
    s_arr = s.split(".")
    for ele in s_arr:
        parts = ele.split("-")
        if len(parts) == 1:
            nums |= {int(parts[0])}
        elif len(parts) == 2:
            l = int(parts[0])
            u = int(parts[1])
            nums.update(range(l, u + 1))
        else:
            print("Invaild ele " + ele)
            sys.exit(2)

    return {n: key for n in nums}


disks = [{} for i in range(2)]
for dx in range(len(disks)):
    for (name, vals) in artists:
        disks[dx].update(convert(vals[dx], key=name))

for dx, disk in enumerate(disks):
    if max(disk) != len(disk):
        print("disk not complate max {} len {}".format(max(disk), len(disk)))
        pprint(disks)
        sys.exit(5)

for dx, disk in enumerate(disks):
    print("# Disk {}".format(dx + 1))
    if args.debug:
        print("\n".join(str((k, v)) for (k, v) in sorted(disk.items())))
    else:
        print("\n".join(v for (k, v) in sorted(disk.items())))
