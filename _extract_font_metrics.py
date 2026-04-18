"""Extract SMuFL glyph metrics from OTF/TTF font files for use in glyph.rs.

Extracts: advance widths, bounding boxes, and stem anchors for all glyphs
used by the renderer.
"""
import json
import sys
from fontTools.ttLib import TTFont
from fontTools.pens.boundsPen import BoundsPen

# SMuFL codepoints for the glyphs we need
GLYPHS = {
    "mensuralWhiteMaxima": 0xE95C,
    "mensuralWhiteLonga": 0xE95D,
    "noteheadDoubleWhole": 0xE0A0,
    "noteheadWhole": 0xE0A2,
    "noteheadHalf": 0xE0A3,
    "noteheadBlack": 0xE0A4,
    "restMaxima": 0xE4E0,
    "restLonga": 0xE4E1,
    "restDoubleWhole": 0xE4E2,
    "restWhole": 0xE4E3,
    "restHalf": 0xE4E4,
    "restQuarter": 0xE4E5,
    "rest8th": 0xE4E6,
    "rest16th": 0xE4E7,
    "rest32nd": 0xE4E8,
    "rest64th": 0xE4E9,
    "flag8thUp": 0xE240,
    "flag8thDown": 0xE241,
    "flag16thUp": 0xE242,
    "flag16thDown": 0xE243,
    "flag32ndUp": 0xE244,
    "flag32ndDown": 0xE245,
    "flag64thUp": 0xE246,
    "flag64thDown": 0xE247,
    "accidentalSharp": 0xE262,
    "accidentalFlat": 0xE260,
    "accidentalNatural": 0xE261,
    "accidentalDoubleSharp": 0xE263,
    "accidentalDoubleFlat": 0xE264,
    "gClef": 0xE050,
    "gClef8vb": 0xE052,
    "gClef8va": 0xE053,
    "gClef15ma": 0xE054,
    "gClef15mb": 0xE051,
    "fClef": 0xE062,
    "fClef8vb": 0xE064,
    "fClef8va": 0xE065,
    "fClef15ma": 0xE066,
    "fClef15mb": 0xE063,
    "cClef": 0xE05C,
    "unpitchedPercussionClef1": 0xE069,
    "timeSig0": 0xE080,
    "timeSig1": 0xE081,
    "timeSig2": 0xE082,
    "timeSig3": 0xE083,
    "timeSig4": 0xE084,
    "timeSig5": 0xE085,
    "timeSig6": 0xE086,
    "timeSig7": 0xE087,
    "timeSig8": 0xE088,
    "timeSig9": 0xE089,
    "timeSigCommon": 0xE08A,
    "timeSigCutCommon": 0xE08B,
    "ornamentTrill": 0xE566,
    "wiggleTrill": 0xEAA4,
    "breathMarkComma": 0xE4CE,
    "caesura": 0xE4D1,
    "segno": 0xE047,
    "coda": 0xE048,
    "brace": 0xE000,
}


def extract_metrics(font_path):
    """Extract advance widths, bboxes, and anchors from a font file."""
    font = TTFont(font_path)
    upm = font['head'].unitsPerEm
    # SMuFL staff space = UPM / 4
    staff_space = upm / 4.0
    cmap = font.getBestCmap()
    gs = font.getGlyphSet()
    hmtx = font['hmtx']

    results = {}
    for name, cp in GLYPHS.items():
        glyph_name = cmap.get(cp)
        if not glyph_name:
            continue

        # Advance width (in staff spaces)
        advance = hmtx[glyph_name][0] / staff_space
        
        # Bounding box (in staff spaces)
        bp = BoundsPen(gs)
        gs[glyph_name].draw(bp)
        bounds = bp.bounds
        if bounds:
            sw_x = bounds[0] / staff_space
            sw_y = bounds[1] / staff_space
            ne_x = bounds[2] / staff_space
            ne_y = bounds[3] / staff_space
        else:
            sw_x = sw_y = ne_x = ne_y = 0.0

        results[name] = {
            'advance': round(advance, 3),
            'sw_x': round(sw_x, 3),
            'sw_y': round(sw_y, 3),
            'ne_x': round(ne_x, 3),
            'ne_y': round(ne_y, 3),
        }

    return results


def extract_anchors(font_path):
    """Try to extract GPOS-based anchors for noteheads (stem attachment)."""
    font = TTFont(font_path)
    upm = font['head'].unitsPerEm
    cmap = font.getBestCmap()

    # The stem anchors we need
    notehead_cps = {
        "noteheadBlack": 0xE0A4,
        "noteheadHalf": 0xE0A3,
    }

    anchors = {}
    
    # Try to get anchors from GPOS table
    if 'GPOS' not in font:
        return anchors
    
    # For SMuFL fonts, stem attachment points are typically in the
    # glyphsWithAnchors section of the metadata JSON.
    # Since we don't have metadata JSONs, we'll derive them from bbox.
    return anchors


def fmt3(v):
    """Format a float to 3 decimal places, stripping trailing zeros."""
    s = f"{v:.3f}"
    # Keep at least one decimal
    if '.' in s:
        s = s.rstrip('0').rstrip('.')
        if '.' not in s:
            s += '.0'
    return s


def print_rust_code(font_id, metrics):
    """Generate Rust match arms for a font."""
    print(f"\n// ─── {font_id} ───")

    # Advance widths
    print(f"\nfn advance_width_{font_id.lower()}(g: &str) -> f64 {{")
    print("    match g {")
    
    # Group glyphs with same advance
    from collections import defaultdict
    adv_groups = defaultdict(list)
    for name, m in sorted(metrics.items()):
        adv_groups[m['advance']].append(name)
    
    for name, m in sorted(metrics.items()):
        if m['advance'] > 0:
            print(f'        "{name}" => {fmt3(m["advance"])},')
    
    print('        // Fallback for mensural glyphs')
    if 'mensuralWhiteMaxima' not in metrics:
        print('        "mensuralWhiteMaxima" | "mensuralWhiteLonga" => advance_width_bravura(g),')
    print("        _ => 0.0,")
    print("    }")
    print("}")

    # Bounding boxes
    print(f"\nfn bbox_{font_id.lower()}(g: &str) -> Option<BBox> {{")
    print("    let b = |sw_x, sw_y, ne_x, ne_y| Some(BBox { sw_x, sw_y, ne_x, ne_y });")
    print("    match g {")
    for name, m in sorted(metrics.items()):
        print(f'        "{name}" => b({fmt3(m["sw_x"])}, {fmt3(m["sw_y"])}, {fmt3(m["ne_x"])}, {fmt3(m["ne_y"])}),')
    if 'mensuralWhiteMaxima' not in metrics:
        print('        "mensuralWhiteMaxima" | "mensuralWhiteLonga" => bbox_bravura(g),')
    print("        _ => None,")
    print("    }")
    print("}")


if __name__ == "__main__":
    fonts = {
        "Sebastian": "fonts/Sebastian.otf",
        "FinaleAsh": "fonts/FinaleAsh.otf",
        "FinaleBroadway": "fonts/FinaleBroadway.otf",
        "FinaleEngraver": "fonts/FinaleEngraver.otf",
        "FinaleJazz": "fonts/FinaleJazz.otf",
        "FinaleLegacy": "fonts/FinaleLegacy.otf",
        "FinaleMaestro": "fonts/FinaleMaestro.otf",
    }
    
    for font_id, path in fonts.items():
        print(f"\n{'='*60}")
        print(f"  {font_id} ({path})")
        print(f"{'='*60}")
        try:
            metrics = extract_metrics(path)
            print(f"  Found {len(metrics)} glyphs out of {len(GLYPHS)}")
            missing = set(GLYPHS.keys()) - set(metrics.keys())
            if missing:
                print(f"  Missing: {', '.join(sorted(missing))}")
            print_rust_code(font_id, metrics)
        except Exception as e:
            print(f"  ERROR: {e}")
            import traceback
            traceback.print_exc()
