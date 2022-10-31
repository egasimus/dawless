# Dawless: AKAI

Support for AKAI S3000/S2000/S900 floppy disk images.

Based on s2kdie PHP5 script by Mick Kane.

## Overview

A S3000 floppy disk is 1638400 (0x190000) bytes long.
It's divided into 1600 (0x640) sectors of 1024 (0x400) bytes.
The first 16 (0x10) sectors are reserved, containing:
* a list of file headers, each 24 (0x3) bytes long,
  containing the name, type, size, and address of first block
* a list of block pointers, each 2 bytes long;
  if a file spans multiple blocks, the pointer
  corresponding to each block contains the address
  of the next block
