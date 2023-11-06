#!/bin/bash

# Tears down USB gadgets per: https://www.kernel.org/doc/Documentation/usb/gadget_configfs.txt

# Exit on first error.
set -e

# Echo commands to stdout.
set -x

# Treat undefined environment variables as errors.
set -u

cd /sys/kernel/config/usb_gadget/

readonly DEVICE="g1"
if [ ! -d "${DEVICE}" ]; then
    echo "Gadget does not exist, quitting."
    exit 0
fi

pushd "${DEVICE}"

# Disable all gadgets
if [ -n "$(cat UDC)" ]; then
  echo "" > UDC
fi

readonly CONFIGS="configs/*"
readonly STRINGS="strings/0x409"
readonly FUNCTIONS="functions/*"

# Walk items in `configs`
for config in ${CONFIGS} ; do
    # Exit early if there are no entries
    [ "${config}" == "${CONFIGS}" ] && break

    # Remove all functions from config
    for function in ${FUNCTIONS} ; do
      file="${config}/$(basename "${function}")"
      [ -e "${file}" ] && rm "${file}"
    done

    # Remove strings in config
    [ -d "${config}/${STRINGS}" ] && rmdir "${config}/${STRINGS}"

    rmdir "${config}"
done

# Remove functions
for function in ${FUNCTIONS} ; do
    [ -d "${function}" ] && rmdir "${function}"
done

# Remove strings from device
[ -d "${STRINGS}" ] && rmdir "${STRINGS}"

popd

rmdir "${DEVICE}"