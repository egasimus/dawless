# Dawless: AKAI

Support for AKAI S3000/S2000/S900 floppy disk images.

Based on s2kdie PHP5 script by Mick Kane.

## Overview

* AKAI-compatible floppy disks can be:

  * **double density:** 819200 (0x0C8000) bytes
    * S900 supports only these?
    * some vestigial support for them in the code, since this started as a rewrite of s2kdie
    * but I don't have a S900 to test, so get in touch if you want that to work

  * **high density:** 1638400 (0x190000) bytes. This project targets floppy emulators
    (Gotek/FlashFloppy) and the S3000XL, so it generates **high-density** images.
    * `0x000000:0x190000`: a 1638400 byte long disk image (a little over 1.44M).
      Divided into 1600 (0x640) sectors of 1024 (0x400) bytes. The first 16 or 17 blocks
      are reserved for filesystem metadata.
      * `0x0000:0x0600` - 64x 24-byte file headers (S900 backwards compatibility?)
      * `0x0017` = `0x11` (address of 1st non-reserved block?)
      * `0x0600:0x1280` - 1600x 2-byte block headers
        * `0x0600:0x0622`: `00 40`, marking reserved blocks
        * `0x0622:0x1280`: either `00 00` (marking a free block) space,
          or the id of next block in the file (addr = id * 0x200)
      * `0x1280:0x128C`: volume label in AKAI encoding
      * `0x128C:0x1400`: ???
      * `0x1400:0x4400`: 512x 24-byte file headers. When these are used,
        the section at 0x0000 is overwritten with the 1st one repeated?
        * Each file header contains the file name, file type, file size,
          and address of 1st block
      * `0x4400:EOF` - data area: 1583x 1024-byte blocks
      * Multiple file types are supported, such as Sample and Program, see `docs/`.
