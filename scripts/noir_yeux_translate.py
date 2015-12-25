#!/usr/bin/env python3

lookup = {
"ラジオトーク①": "Radio Talk 1",
"maiden voyage": "maiden voyage",
"木漏れ日": "Streaming Sunlight",
"帰還": "Return",
"黒い瞳": "Dark Eye",
"南へ": "To the South",
"風の碑": "Monument of Wind",
"zodiac": "zodiac",
"roll out": "roll out",
"煙突讃歌": "Chimney Hymn",
"いざ！": "Now!",
"さるをとる": "Catch the Monkey",
"祈り": "Prayer",
"雨がやんだら": "If the Rain Stops",
"ヤ・ヤ": "Ya Ya",
"おやすみ": "Good Night",
"砂のパンドラ": "Pandora of Sand",
"ラジオトーク②": "Radio Talk 2",
"白虹": "White Rainbow",
"できるかな？": "Can We Do It?",
"蒼空祭": "Sky Festival",
"草笛ひとつ": "One Reed Pipe",
"兆し": "Signs",
"出発！": "Departure!",
"飛燕連峰": "Hien Mountain Range",
"カナヴィスの夢": "Dream of Canavis",
"月の心臓": "Heart of the Moon",
"雲を梳き": "Comb the Clouds",
"ジェノ": "Geno",
"夢追い": "Chasing Dreams",
"ひいらぎの帆": "Holly's Sail",
"死線": "Verge of Death",
"深層": "The Depths",
"exodus": "exodus",
"鳩の翼": "A Dove's Wing",
"ラジオトーク③": "Radio Talk 3",
}

jpn=[
"maiden voyage",
"風の碑",
"木洩れ日",
"煙突讃歌",
"雨がやんだら",
"白虹",
"草笛ひとつ",
"飛燕連峰",
"雲を梳き",
"ひいらぎの帆",
"砂のパンドラ",
"カナヴィスの夢",
"深層",
"zodiac",
"exodus",
"roll out",
"祈り",
"ヤ・ヤ",
"兆し",
"死線",
"夢追い",
"蒼空祭",
"黒い瞳",
"月の心臓",
"帰還",
"南へ",
"おやすみ",
"出発！",
"いざ！",
"できるかな？",
"ジェノ",
"さるをとる",
"鳩の翼",
"cielgris fantasm (dummytrack)",
]

eng = [x for x in jpn]
missing = set()
for (ix,val) in enumerate(jpn):
  try:
    eng[ix] = lookup[val]
  except Exception as e:
    print("Not found: new[%02d] %s" % (ix,val))
    missing.add(val)

print("\n Unused")
candidates = lookup.keys() - set(jpn) - missing
for c in candidates:
  print(c)

print("")

for (ix,val) in enumerate(eng):
  print("%02d %s" % (ix+1,val))

