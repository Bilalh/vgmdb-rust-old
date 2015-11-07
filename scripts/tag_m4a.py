#!/usr/bin/env python3
#-*- coding: utf-8 -*-
import logging
import argparse
import json

from pprint import pprint
from pathlib import Path
from mutagen.mp4 import MP4

logger = logging.getLogger(__name__)

parser = argparse.ArgumentParser()
parser.add_argument("dir", help='dir of m4a')
parser.add_argument("json", help='json from vgmdb -J')
# args = parser.parse_args()
args = argparse.Namespace(dir="/Users/bilalh/aa", json="/Users/bilalh/aa/n.json")
pprint(args)

data = json.load(Path(args.json).expanduser().open())
mapping = dict(comment="©cmt", release_date="©day", name="©alb")

pprint(data)

for fp in Path(args.dir).expanduser().glob("*.m4a"):
    print("Processing {}".format(fp))
    cur = MP4(filename=str(fp))
    for key_json, key_m4a in mapping.items():
        try:
            value = data[key_json]
            if value is not None:
                print("Setting {}({})={}".format(key_json, key_m4a, value))
                cur[key_m4a] = value
        except KeyError as e:
            pass
    cur.save()
