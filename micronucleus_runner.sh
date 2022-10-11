#!/bin/sh
SRC="$1"
echo "$@"
avr-objcopy -j .text -j .data -O ihex "$SRC" "$SRC.ihex"
avr-size --format=avr --mcu=$(DEVICE) "$SRC"
micronucleus --run "$SRC.ihex"
