#!/bin/bash

cd ./pjproject

# Mandatory Audio Codecs
# itu G 711
# itu G 722
# ISO-MPEG I/II
# L16
#
# Optional audio codecs
# MPEG III
# OPUS
# AMR

./configure --disable-video \
  --disable-small-filter \
  --disable-large-filter \
  --disable-speex-aec \
  --disable-gsm-codec \
  --disable-speex-codec \
  --disable-sdl \
  --disable-ffmpeg \
  --disable-v4l2 \
  --disable-vpx \
  --disable-silk \
  --disable-ilbc-codec \
  --disable-bcg729 \
  --disable-libyuv \
  --disable-libwebrtc \
  --disable-pjsua2 \
  --enable-shared

make dep
make

# rm -f ../pjsip-rs/*.so
# rm -f ../pjsip-rs/*.so.2
# rm -f ../pjsip-rs/*.a

# # copy compiled content to pjsip-rs
# cp ./pjsip-apps/bin/pjsua-* ../pjsip-rs/pjsua
# # copy pjlib
# cp ./pjlib/lib/* ../pjsip-rs/
# # copy pjlib-util
# cp ./pjlib-util/lib/* ../pjsip-rs/
# # copy pjnath
# cp ./pjnath/lib/* ../pjsip-rs/
# # copy pjmedia
# cp ./pjmedia/lib/* ../pjsip-rs/
# # copy pjsip
# cp ./pjsip/lib/* ../pjsip-rs/
# # copy third party lib
# cp ./third_party/lib/* ../pjsip-rs/
# # rm ../pjsip-rs/*.a
