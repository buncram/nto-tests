#! /usr/bin/env python3

import argparse

def main():
    parser = argparse.ArgumentParser(description="Merge CM7 and RV32 binaries for simulation")
    parser.add_argument(
        "--rv32", required=True, help="RV32 binary file", type=str
    )
    parser.add_argument(
        "--cm7", help="CM7 binary file", type=str
    )
    parser.add_argument(
        "--out-file", help="Output file", type=str, default="boot.bin"
    )
    args = parser.parse_args()

    with open(args.rv32, 'rb') as f:
        rv32 = f.read()
    with open(args.cm7, 'rb') as f:
        cm7 = f.read()

    with open(args.out_file, 'wb') as f:
        f.write(rv32)

        pad_to_target = 0x30_0000
        pad_len = pad_to_target - len(rv32)
        f.write(bytes(pad_len))
        f.write(cm7)

if __name__ == "__main__":
    main()
    exit(0)
