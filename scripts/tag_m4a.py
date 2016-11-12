#!/usr/bin/env python3
#-*- coding: utf-8 -*-
import argparse
import json

from mutagen.mp4 import MP4
from pathlib import Path
from pprint import pprint

parser = argparse.ArgumentParser(description="tag a m4a using vgmdb -J")
parser.add_argument("dir", help='dir of m4a')
parser.add_argument("json", help='json from vgmdb -J')
parser.add_argument("-r", help='release_date only', action='store_true', dest='release_date_only')

args = parser.parse_args()
# args = argparse.Namespace(dir="/Users/bilalh/aa", json="/Users/bilalh/aa/n.json")
pprint(args)

data = json.load(Path(args.json).expanduser().open())
mapping = dict(comment="©cmt", release_date="©day", name="©alb")

pprint(data)

for fp in Path(args.dir).expanduser().glob("*.m4a"):
    print("Processing {}".format(fp))
    cur = MP4(filename=str(fp))
    for key_json, key_m4a in mapping.items():
        if args.release_date_only and key_json != 'release_date':
            continue

        try:
            value = data[key_json]
            if value is not None:
                print("Setting {}({})={}".format(key_json, key_m4a, value))
                cur[key_m4a] = value
        except KeyError as e:
            pass
    cur.save()
