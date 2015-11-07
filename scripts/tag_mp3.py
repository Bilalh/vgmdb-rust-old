#!/usr/bin/env python3
#-*- coding: utf-8 -*-
import argparse
import json

from mutagen.id3 import ID3, TDRL, COMM
from pathlib import Path
from pprint import pprint

parser = argparse.ArgumentParser(description="tag mp3 using vgmdb -J")
parser.add_argument("dir", help='dir of mp3')
parser.add_argument("json", help='json from vgmdb -J')

args = parser.parse_args()
# args = argparse.Namespace(dir="/Users/bilalh/aa", json="/Users/bilalh/aa/n.json")
# pprint(args)

data = json.load(Path(args.json).expanduser().open())
allowed = {"comment", "release_date"}

pprint(data)

for fp in Path(args.dir).expanduser().glob("*.mp3"):
    print("Processing {}".format(fp))
    cur = ID3(filename=str(fp), v2_version=3)
    cur.update_to_v23()
    for key_json in allowed:
        try:
            value = data[key_json]
            if value is not None:
                print("Setting {}={}".format(key_json, value))

                if key_json == "release_date":
                    arr = reversed(value.split('.'))
                    val2 = "-".join(arr)

                    cur.delall('TDRL')
                    cur.add(TDRL(encoding=3, text=[val2]))
                elif key_json == "comment":
                    cur.delall('COMM')
                    cur.delall('COMM::ENG')
                    cur.delall('COMM::XXX')
                    cur.add(COMM(encoding=3, lang='eng', desc='', text=[value]))

        except KeyError as e:
            pass
    cur.save()
