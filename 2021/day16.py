import math
import unittest
from dataclasses import dataclass, field

DAY = 16


@dataclass
class Package:
    version: int = 0
    type_id: int = 0
    value: int = 0
    sub_pkgs: list["Package"] = field(default_factory=list)


def parse_pkg(num: int, bits: int) -> tuple[Package, int]:
    pkg = Package()
    pkg.version = (num >> (bits - 3)) & 0b111
    pkg.type_id = (num >> (bits - 6)) & 0b111
    remaining_bits = bits - 6
    if pkg.type_id == 4:  # literal value
        value = 0
        for block_count in range(1, (bits - 6) // 5 + 1):
            remaining_bits -= 5
            shift_num = num >> (bits - 6 - 5 * block_count)
            value = (value << 4) + (shift_num & 0b1111)
            if (shift_num >> 4) & 1 == 0:
                break
        pkg.value = value
    else:  # operator
        length_type_id = (num >> (bits - 7)) & 1
        remaining_bits -= 1
        if length_type_id == 0:
            sub_pkg_bits = (num >> (bits - 7 - 15)) & 0b111_1111_1111_1111
            masked_num = (num >> (bits - 7 - 15 - sub_pkg_bits)) & ~(~0 << sub_pkg_bits)
            remaining_bits -= 15 + sub_pkg_bits
            while sub_pkg_bits > 0:
                sub_pkg, sub_pkg_bits = parse_pkg(masked_num, sub_pkg_bits)
                pkg.sub_pkgs.append(sub_pkg)
                masked_num = masked_num & ~(~0 << sub_pkg_bits)
        elif length_type_id == 1:
            sub_pkg_num = (num >> (bits - 7 - 11)) & 0b111_1111_1111
            remaining_bits -= 11
            masked_num = num & ~(~0 << (bits - 7 - 11))
            for _ in range(sub_pkg_num):
                sub_pkg, remaining_bits = parse_pkg(masked_num, remaining_bits)
                pkg.sub_pkgs.append(sub_pkg)
                masked_num = masked_num & ~(~0 << remaining_bits)
    return pkg, remaining_bits


def sum_pkg_version(pkg: Package) -> int:
    version_sum = pkg.version
    for sub_pkg in pkg.sub_pkgs:
        version_sum += sum_pkg_version(sub_pkg)
    return version_sum


def part_one(hex_str: str) -> int:
    pkg, _ = parse_pkg(int(hex_str, 16), len(hex_str) * 4)
    return sum_pkg_version(pkg)


def get_pkg_value(pkg: Package) -> int:
    match pkg.type_id:
        case 0:
            return sum(get_pkg_value(sub_pkg) for sub_pkg in pkg.sub_pkgs)
        case 1:
            return math.prod(get_pkg_value(sub_pkg) for sub_pkg in pkg.sub_pkgs)
        case 2:
            return min(get_pkg_value(sub_pkg) for sub_pkg in pkg.sub_pkgs)
        case 3:
            return max(get_pkg_value(sub_pkg) for sub_pkg in pkg.sub_pkgs)
        case 4:
            return pkg.value
        case 5:
            return (
                1
                if get_pkg_value(pkg.sub_pkgs[0]) > get_pkg_value(pkg.sub_pkgs[1])
                else 0
            )
        case 6:
            return (
                1
                if get_pkg_value(pkg.sub_pkgs[0]) < get_pkg_value(pkg.sub_pkgs[1])
                else 0
            )
        case 7:
            return (
                1
                if get_pkg_value(pkg.sub_pkgs[0]) == get_pkg_value(pkg.sub_pkgs[1])
                else 0
            )
        case _:
            return 0


def part_two(hex_str: str) -> int:
    pkg, _ = parse_pkg(int(hex_str, 16), len(hex_str) * 4)
    return get_pkg_value(pkg)


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one("D2FE28"), 6)
        self.assertEqual(part_one("38006F45291200"), 9)
        self.assertEqual(part_one("EE00D40C823060"), 14)
        self.assertEqual(part_one("8A004A801A8002F478"), 16)
        self.assertEqual(part_one("620080001611562C8802118E34"), 12)
        self.assertEqual(part_one("C0015000016115A2E0802F182340"), 23)
        self.assertEqual(part_one("A0016C880162017C3686B18A3D4780"), 31)

    def test_part_two(self):
        self.assertEqual(part_two("C200B40A82"), 3)
        self.assertEqual(part_two("04005AC33890"), 54)
        self.assertEqual(part_two("880086C3E88112"), 7)
        self.assertEqual(part_two("CE00C43D881120"), 9)
        self.assertEqual(part_two("D8005AC2A8F0"), 1)
        self.assertEqual(part_two("F600BC2D8F"), 0)
        self.assertEqual(part_two("9C005AC2F8F0"), 0)
        self.assertEqual(part_two("9C0141080250320F1802104A08"), 1)


if __name__ == "__main__":
    with open(f"input/day{DAY}_input") as f:
        hex_str = f.read().strip()
        print(part_one(hex_str))
        print(part_two(hex_str))
