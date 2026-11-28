#!/bin/bash
set -u
export HOME=/tmp/h; mkdir -p $HOME
Xvfb :90 -screen 0 1280x800x24 >/tmp/xvfb.log 2>&1 &
sleep 2
export DISPLAY=:90
xfwm4 >/tmp/xfwm.log 2>&1 &
sleep 2
cd /app && bash umi-ocr.sh >/tmp/umi.log 2>&1 &
for i in $(seq 1 90); do
  if xdotool search --onlyvisible --name "Umi-OCR" >/dev/null 2>&1; then echo "WINDOW after ${i}s"; break; fi
  sleep 1
done
sleep 8
xdotool search --onlyvisible --name "." getwindowname %@ 2>/dev/null | sort -u
import -window root /out/umi.png
cp /tmp/umi.log /out/umi.log
tail -5 /tmp/umi.log
