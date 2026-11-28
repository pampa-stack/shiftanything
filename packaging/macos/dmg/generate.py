#!/usr/bin/env python3
"""Regenerate background.tiff and dmg-layout.DS_Store for the macOS DMG window (see README.md here).

Needs resvg on PATH and:
    pip install 'pillow>=12' ds_store==1.3.3 mac_alias==2.2.3

Runs on any OS, no Mac needed. Both outputs come from this script alone, so they carry nothing
from the machine that made them: no local paths, user or disk names, dates or volume UUIDs.

    python3 packaging/macos/dmg/generate.py
"""

import argparse
import datetime
import os
import shutil
import subprocess
import sys
import tempfile

from ds_store import DSStore
from mac_alias import Alias, TargetInfo, VolumeInfo
from PIL import Image, ImageCms, ImageDraw, ImageFont

HERE = os.path.dirname(os.path.abspath(__file__))

# Keep in sync with package.sh (volume name) and README.md (layout).
VOLUME = "ArtCraft Launcher"
WIDTH, HEIGHT = 660, 400  # window content, pt; the background's 1x size
TITLE_BAR = 32
ICON_SIZE = 128
PALETTE = 16  # colours in the upstream background.tiff
SHIFT_WIDTH, SHIFT_HEIGHT = 748, 420
ICONS = {"ArtCraft Launcher.app": (326, 205), "Applications": (574, 205)}
# A fixed date for the alias and the colour profile. Finder never matches it against the image
# (each build is a new volume): it finds the background by volume name and path.
FIXED_DATE = datetime.datetime(2026, 10, 11, tzinfo=datetime.timezone.utc)


def render(svg, width, out):
    # The SVG's text is already outlined and no fonts are loaded, so every machine draws the
    # same pixels.
    subprocess.run(
        ["resvg", "--skip-system-fonts", "-w", str(width), svg, out],
        check=True,
    )
    with Image.open(out) as im:
        return im.convert("RGB")


def quantize(im, palette):
    # No dithering: dither noise compresses badly and the artwork has flat fills.
    return im.quantize(palette, method=Image.Quantize.FASTOCTREE, dither=Image.Dither.NONE)


def srgb_profile():
    # LittleCMS's built-in sRGB ("No copyright, use freely"), with its creation date fixed.
    icc = bytearray(ImageCms.ImageCmsProfile(ImageCms.createProfile("sRGB")).tobytes())
    d = FIXED_DATE
    icc[24:36] = b"".join(v.to_bytes(2, "big") for v in (d.year, d.month, d.day, 0, 0, 0))
    if icc[84:100] != bytes(16):
        sys.exit("error: the sRGB profile has an ID; recompute it after fixing the date")
    return bytes(icc)


def write_tiff(one, two, out, palette=PALETTE):
    """1x (72 dpi) and 2x (144 dpi) in one TIFF, as `tiffutil -cathidpicheck` writes them."""
    one, two = quantize(one, palette), quantize(two, palette)
    two.encoderinfo = {"dpi": (144, 144)}  # the rest as the first page
    # Indexed colour and Deflate keep the artwork compact.
    one.save(
        out,
        "TIFF",
        save_all=True,
        append_images=[two],
        compression="tiff_adobe_deflate",  # brand-ok: Pillow's name for TIFF compression 8 (Deflate)
        dpi=(72, 72),
        icc_profile=srgb_profile(),
    )
    with Image.open(out) as im:
        im.seek(1)
        if im.info.get("dpi") != (144, 144):
            sys.exit("error: the 2x page isn't 144 dpi (needs Pillow 12 or later)")
        strips = []
        for page in (0, 1):
            im.seek(page)
            strips += zip(im.tag_v2[273], im.tag_v2[279])  # StripOffsets, StripByteCounts
    # TIFF keeps directories and tag data on even offsets. When a strip ends on an odd one, libtiff
    # leaves the padding byte after it uninitialised, so the file changed from run to run with
    # identical pixels. Zero it: nothing refers to that byte.
    starts = {o for o, _ in strips}
    with open(out, "r+b") as f:
        for o, n in strips:
            if (o + n) % 2 and o + n not in starts:
                f.seek(o + n)
                f.write(bytes(1))


def background_alias(volume_name):
    # What Finder stores for /.background/background.tiff on the volume, minus anything about the
    # machine (dmgbuild writes the same kind of alias from a mounted image). Disk type 5 (ejectable),
    # the volume flags and the folder/file ids (25, 26) are what Finder wrote on a test image; the ids
    # are only hints, as the volume's creation date.
    volume = VolumeInfo(volume_name, FIXED_DATE, b"H+", 5, 0x0D02, b"\0\0")
    volume.posix_path = f"/Volumes/{volume_name}"
    target = TargetInfo(0, "background.tiff", 25, 26, FIXED_DATE, b"\0\0\0\0", b"\0\0\0\0")
    target.folder_name = ".background"
    target.cnid_path = [25]
    target.carbon_path = f"{volume_name}:.background:\0background.tiff"
    target.posix_path = "/.background/background.tiff"
    return Alias(volume=volume, target=target).to_bytes()


def write_ds_store(out, volume_name, icons, *, width=WIDTH, height=HEIGHT):
    if os.path.exists(out):
        os.remove(out)
    with DSStore.open(out, "w+") as d:
        d["."]["bwsp"] = {
            "ShowStatusBar": False,
            "ShowToolbar": False,
            "ShowTabView": False,
            "ContainerShowSidebar": False,
            "ShowSidebar": False,
            "WindowBounds": f"{{{{200, 528}}, {{{width}, {height + TITLE_BAR}}}}}",
        }
        d["."]["icvp"] = {
            "viewOptionsVersion": 1,
            "backgroundType": 2,
            "backgroundImageAlias": background_alias(volume_name),
            "backgroundColorRed": 1.0,
            "backgroundColorGreen": 1.0,
            "backgroundColorBlue": 1.0,
            "gridOffsetX": 0.0,
            "gridOffsetY": 0.0,
            "gridSpacing": 100.0,
            "arrangeBy": "none",
            "showIconPreview": True,
            "showItemInfo": False,
            "labelOnBottom": True,
            "textSize": 16.0,
            "iconSize": float(ICON_SIZE),
        }
        d["."]["vSrn"] = ("long", 1)
        for name, pos in icons.items():
            d[name]["Iloc"] = pos


def shiftanything_background(scale):
    image = Image.new("RGB", (SHIFT_WIDTH * scale, SHIFT_HEIGHT * scale), "#f0f4ff")
    draw = ImageDraw.Draw(image)
    root = os.path.join(HERE, "../../..")
    font_path = os.path.join(root, "assets/fonts/Inter-Variable.ttf")

    def coords(points):
        return tuple(value * scale for value in points)

    def label(x, y, text, size, weight, color, anchor="lt"):
        font = ImageFont.truetype(font_path, size * scale)
        font.set_variation_by_axes([weight])
        draw.text(coords((x, y)), text, font=font, fill=color, anchor=anchor)

    draw.rectangle(coords((0, 0, 219, SHIFT_HEIGHT)), fill="#101952")
    with Image.open(os.path.join(root, "assets/shiftanything/icon.png")) as original:
        badge = original.convert("RGB").resize(coords((150, 150)), Image.Resampling.LANCZOS)
    mask = Image.new("L", badge.size)
    ImageDraw.Draw(mask).rounded_rectangle((0, 0, badge.width - 1, badge.height - 1),
                                           radius=29 * scale, fill=255)
    image.paste(badge, coords((35, 126)), mask)
    with Image.open(os.path.join(root, "assets/shiftanything/promptica.png")) as original:
        mark = original.convert("RGBA").resize(coords((28, 28)), Image.Resampling.LANCZOS)
    image.paste(mark, coords((50, 343)), mark)
    label(87, 351, "PROMPTICA", 13, 700, "#d8e1ff")
    label(264, 53, "Sh*ftAnything", 39, 780, "#141b42")
    draw.rounded_rectangle(coords((266, 112, 307, 117)), radius=2 * scale, fill="#293af0")
    draw.line(coords((451, 213, 512, 213)), fill="#293af0", width=5 * scale)
    draw.line([coords(point) for point in ((497, 196), (515, 213), (497, 230))],
              fill="#293af0", width=5 * scale, joint="curve")
    label(266, 342, "Trascina l’app in Applications", 19, 670, "#141b42")
    label(266, 373, "Drag the app to Applications", 14, 450, "#354667")
    return image


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--shiftanything", action="store_true", help="generate the Sh*ftAnything Finder layout")
    args = parser.parse_args()
    if args.shiftanything:
        target = os.path.join(HERE, "shiftanything")
        os.makedirs(target, exist_ok=True)
        write_tiff(*(shiftanything_background(scale) for scale in (1, 2)), os.path.join(target, "background.tiff"), palette=192)
        write_ds_store(os.path.join(target, "dmg-layout.DS_Store"), "Sh*ftAnything",
                       {"Sh*ftAnything.app": (355, 213), "Applications": (617, 213)},
                       width=SHIFT_WIDTH, height=SHIFT_HEIGHT)
        return
    if not shutil.which("resvg"):
        sys.exit("error: resvg not found (brew install resvg / cargo install resvg --locked)")
    with tempfile.TemporaryDirectory() as tmp:
        images = [render(os.path.join(HERE, "background.svg"), WIDTH * scale,
                         os.path.join(tmp, f"{scale}x.png")) for scale in (1, 2)]
    write_tiff(*images, os.path.join(HERE, "background.tiff"))
    write_ds_store(os.path.join(HERE, "dmg-layout.DS_Store"), VOLUME, ICONS)


if __name__ == "__main__":
    main()
