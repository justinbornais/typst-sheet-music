import json

with open('data/bravura_metadata.json') as f:
    m = json.load(f)

needed = [
    'noteheadWhole','noteheadHalf','noteheadBlack',
    'restWhole','restHalf','restQuarter','rest8th','rest16th','rest32nd','rest64th',
    'flag8thUp','flag8thDown','flag16thUp','flag16thDown','flag32ndUp','flag32ndDown','flag64thUp','flag64thDown',
    'accidentalSharp','accidentalFlat','accidentalNatural','accidentalDoubleSharp','accidentalDoubleFlat',
    'gClef','fClef','cClef','gClef8va','gClef8vb','gClef15ma','gClef15mb','fClef8va','fClef8vb','fClef15ma','fClef15mb',
    'timeSig0','timeSig1','timeSig2','timeSig3','timeSig4','timeSig5','timeSig6','timeSig7','timeSig8','timeSig9','timeSigCommon','timeSigCutCommon',
    'ornamentTrill','wiggleTrill',
    'breathMarkComma','caesura','segno','coda','brace',
    'unpitchedPercussionClef1',
]

print("// Advance widths")
for g in needed:
    aw = m['glyphAdvanceWidths'].get(g)
    if aw is not None:
        print(f'("{g}", {aw}),')

print("\n// Bounding boxes (sw_x, sw_y, ne_x, ne_y)")
for g in needed:
    bb = m['glyphBBoxes'].get(g)
    if bb is not None:
        sw = bb['bBoxSW']
        ne = bb['bBoxNE']
        print(f'("{g}", ({sw[0]}, {sw[1]}, {ne[0]}, {ne[1]})),')

print("\n// Anchors")
anchor_glyphs = ['noteheadBlack','noteheadHalf','noteheadWhole']
for g in anchor_glyphs:
    a = m['glyphsWithAnchors'].get(g, {})
    for k, v in a.items():
        print(f'("{g}", "{k}", {v[0]}, {v[1]}),')
