#!/usr/bin/env python3
#-*- coding: utf-8 -*-
from pprint import pprint, pformat
import sys
import argparse

parser = argparse.ArgumentParser()
parser.add_argument('--debug', action='store_true', dest='debug', help='')
args = parser.parse_args()

# composer from http://vgmdb.net/album/59534
artists = [
  ("Kazuki Yanagawa",
      ["07, 10-11, 17-18, 21-22, 24-25, 27-29, 31-32"
      ,"01, 03, 14-16, 21, 23-24, 31"
      ,"05, 08-11, 13-14, 18-20, 23-27, 29-30"
      ,"02, 04, 08, 10-11, 14-20, 23-28, 30"
      ]
  ),
  ("Tatsuya Yano",
      ["02, 04-05, 08-09, 13-16, 20, 23"
      ,"04-07, 09-10, 12, 17, 19-20, 22, 25-30"
      ,"02-03, 06-07, 12, 15-17, 22"
      ,"01, 03, 05-07, 12-13, 21-22"
      ]
  ),
  ("Daisuke Achiwa",
      ["03, 06, 12, 19, 26, 30"
      ,"02, 08, 11, 13, 18"
      ,"01, 04, 21"
      ,"09"
      ]
  ),
  ("Daisuke Achiwa, Kazuki Yanagawa",
      [None
      ,None
      ,"28"
      ,"29"
      ]
  ),
  ("Atsushi Yuasa",
      ["01"
      ,None
      ,None
      ,None
      ]
  ),
  ("Ryudai Abe",
      [None
      ,None
      ,"31"
      ,None
      ]
  ),
  ]


num_disks = max( len(l) for (_,l) in artists )


def convert(s, *, key):
    if s is None:
        return {}
    nums = set()
    s_arr = s.split(",")
    for ele in s_arr:
        ele = ele.strip()
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
        print("disk {} not complete: max {}, len {}".format((dx+1), max(disk), len(disk)))
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
