



def test_cwy_enum():
    """
    This test will fail if we accidentally change the public interface for this enum
    """
    from megalinref import Cwy

    assert Cwy["L"] == 0b0100
    assert Cwy["S"] == 0b0010
    assert Cwy["R"] == 0b0001

    # assert Cwy["Left"]   == Cwy["L"]
    # assert Cwy["Single"] == Cwy["S"]
    # assert Cwy["Right"]  == Cwy["R"]

    # assert Cwy["All"]    == 0b0111


def test_network_type_enum():
    """
    This test will fail if we accidentally change the public interface for this enum
    """
    from megalinref import NetworkType

    assert NetworkType["State Road"]                 == 0b0000_0001
    assert NetworkType["Local Road"]                 == 0b0000_0010
    assert NetworkType["Miscellaneous Road"]         == 0b0000_0100
    assert NetworkType["Main Roads Controlled_Path"] == 0b0000_1000
    assert NetworkType["Proposed Road"]              == 0b0001_0000
    assert NetworkType["Crossover"]                  == 0b0010_0000

    # assert NetworkType["All"]                        == 0b0011_1111
