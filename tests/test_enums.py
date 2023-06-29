
from .util.dictdiffer_tools import assert_dictdiffer


def test_cwy_enum():
    """
    This test will fail if we accidentally change the public interface for the Cwy enum
    """
    from megalinref import Cwy

    assert_dictdiffer(
        result          = Cwy,
        expected_result = {
            "L":      0b0100,
            "S":      0b0010,
            "R":      0b0001,
            "Left":   Cwy["L"],
            "Single": Cwy["S"],
            "Right":  Cwy["R"],
            "All":    0b0000_0111,
        }
    )


def test_network_type_enum():
    """
    This test will fail if we accidentally change the public interface for the NetworkType enum
    """
    from megalinref import NetworkType

    assert_dictdiffer(
        result          = NetworkType,
        expected_result = {
            "State Road":                 0b0000_0001,
            "Local Road":                 0b0000_0010,
            "Miscellaneous Road":         0b0000_0100,
            "Main Roads Controlled Path": 0b0000_1000,
            "Proposed Road":              0b0001_0000,
            "Crossover":                  0b0010_0000,
            "All":                        0b0011_1111,
        }
    )
