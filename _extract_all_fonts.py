import json, sys

fonts = {
    'leipzig': 'fonts/leipzig_metadata.json',
    'leland': 'fonts/leland_metadata.json',
    'petaluma': 'fonts/petaluma_metadata.json',
}

needed_glyphs = [
    'mensuralWhiteMaxima', 'mensuralWhiteLonga',
    'noteheadDoubleWhole', 'noteheadWhole', 'noteheadHalf', 'noteheadBlack',
    'restMaxima', 'restLonga', 'restDoubleWhole', 'restWhole', 'restHalf',
    'restQuarter', 'rest8th', 'rest16th', 'rest32nd', 'rest64th',
    'flag8thUp', 'flag8thDown', 'flag16thUp', 'flag16thDown',
    'flag32ndUp', 'flag32ndDown', 'flag64thUp', 'flag64thDown',
    'accidentalSharp', 'accidentalFlat', 'accidentalNatural',
    'accidentalDoubleSharp', 'accidentalDoubleFlat',
    'gClef', 'fClef', 'cClef',
    'gClef8va', 'gClef8vb', 'gClef15ma', 'gClef15mb',
    'fClef8va', 'fClef8vb', 'fClef15ma', 'fClef15mb',
    'timeSig0', 'timeSig1', 'timeSig2', 'timeSig3', 'timeSig4',
    'timeSig5', 'timeSig6', 'timeSig7', 'timeSig8', 'timeSig9',
    'timeSigCommon', 'timeSigCutCommon',
    'ornamentTrill', 'wiggleTrill',
    'breathMarkComma', 'caesura', 'segno', 'coda', 'brace',
    'unpitchedPercussionClef1',
]

anchor_glyphs = ['noteheadBlack', 'noteheadHalf', 'noteheadWhole']
anchor_names = ['stemUpSE', 'stemDownNW']

for fname, path in fonts.items():
    with open(path) as f:
        m = json.load(f)
    
    print(f"\n=== {fname.upper()} ===")
    
    # Engraving defaults (key ones)
    ed = m.get('engravingDefaults', {})
    print(f"  stemThickness: {ed.get('stemThickness', 'N/A')}")
    print(f"  beamThickness: {ed.get('beamThickness', 'N/A')}")
    print(f"  beamSpacing: {ed.get('beamSpacing', 'N/A')}")
    print(f"  staffLineThickness: {ed.get('staffLineThickness', 'N/A')}")
    print(f"  thinBarlineThickness: {ed.get('thinBarlineThickness', 'N/A')}")
    print(f"  thickBarlineThickness: {ed.get('thickBarlineThickness', 'N/A')}")
    print(f"  legerLineExtension: {ed.get('legerLineExtension', 'N/A')}")
    
    bboxes = m.get('glyphBBoxes', {})
    anchors = m.get('glyphsWithAnchors', {})
    
    print(f"\n  --- Bounding Boxes (sw_x, sw_y, ne_x, ne_y) ---")
    for g in needed_glyphs:
        bb = bboxes.get(g)
        if bb:
            sw = bb['bBoxSW']
            ne = bb['bBoxNE']
            print(f'  "{g}" => b({sw[0]}, {sw[1]}, {ne[0]}, {ne[1]}),')
        else:
            print(f'  "{g}" => MISSING')
    
    print(f"\n  --- Anchors ---")
    for g in anchor_glyphs:
        a = anchors.get(g, {})
        for an in anchor_names:
            v = a.get(an)
            if v:
                print(f'  ("{g}", "{an}") => ({v[0]}, {v[1]}),')
            else:
                print(f'  ("{g}", "{an}") => MISSING')
    
    # Advance widths: check if glyphAdvanceWidths exists; if not, use ne_x from bbox
    adv = m.get('glyphAdvanceWidths', {})
    if adv:
        print(f"\n  --- Advance Widths (from glyphAdvanceWidths) ---")
        for g in needed_glyphs:
            w = adv.get(g)
            if w is not None:
                print(f'  "{g}" => {w},')
            else:
                print(f'  "{g}" => MISSING')
    else:
        print(f"\n  --- Advance Widths (from bbox ne_x, no glyphAdvanceWidths section) ---")
        for g in needed_glyphs:
            bb = bboxes.get(g)
            if bb:
                print(f'  "{g}" => {bb["bBoxNE"][0]},')
            else:
                print(f'  "{g}" => MISSING')
