#!/usr/bin/env python3
#-*- coding: utf-8 -*-
from pprint import pprint, pformat
import sys
import argparse

parser = argparse.ArgumentParser()
parser.add_argument('--debug', action='store_true', dest='debug', help='')
args = parser.parse_args()

#from http://vgmdb.net/album/52818
artists = [
  ("Hayato Asano",
  ["04,07,11~14,25,29,30,36"
  ,"01,04,06,07,08,09,15,16,18,26~38"]
  ),

  ("Kazuki Yanagawa",
  ["01,02,09,16,19,21~24,26,31"
  ,"12~14,17,20~23,25"
  ]),

  ("Daisuke Achiwa",
  ["03,05,06,08,10,15,17,18,20,27,28,32,34"
  ,"02,03,05,10,11,19,24"
  ]),

  ("Toshiharu Yamanishi",
  [ "33,35"
  , None
  ])

  ]
num_disks = max( len(l) for (_,l) in artists )


def convert(s, *, key):
    if s is None:
        return {}
    nums = set()
    s_arr = s.split(",")
    for ele in s_arr:
        ele = ele.replace("~","-")
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


disks = [{} for i in range(num_disks)]
for dx in range(num_disks):
    for (name, vals) in artists:
        disks[dx].update(convert(vals[dx], key=name))

for dx, disk in enumerate(disks):
    if max(disk) != len(disk):
        print("disk {} not complete max {} len {}".format((dx+1), max(disk), len(disk)))
        missing = set()
        for i in range(1,max(disk)+1):
            if i not in disk:
                missing |= {i}
        print("missing tracks #'s {}".format(pformat(missing)))

        sys.exit(5)

for dx, disk in enumerate(disks):
    if args.debug:
        print("# Disk {}".format(dx + 1))
        print("\n".join(str((k, v)) for (k, v) in sorted(disk.items())))
    else:
        print("\n".join(v for (k, v) in sorted(disk.items())))
