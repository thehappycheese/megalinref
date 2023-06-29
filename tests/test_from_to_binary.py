from .fixtures import road_network


def test_from_binary(road_network):
    from .util.confirm_test_cases_with_instance import confirm_test_cases_with_instance
    from megalinref import Lookup
    slk_lookup = Lookup.from_dict(road_network)
    bincode = slk_lookup.to_binary()
    slk_lookup = Lookup.from_binary(bincode)
    confirm_test_cases_with_instance(slk_lookup)


def test_open_binary_file_bytearray(road_network):
    from .util.confirm_test_cases_with_instance import confirm_test_cases_with_instance
    from megalinref import Lookup, open_binary_file
    slk_lookup = Lookup.from_dict(road_network)
    bincode = bytearray(slk_lookup.to_binary())
    slk_lookup = open_binary_file(bincode)
    confirm_test_cases_with_instance(slk_lookup)

def test_open_binary_file_bytes(road_network):
    from .util.confirm_test_cases_with_instance import confirm_test_cases_with_instance
    from megalinref import Lookup, open_binary_file
    slk_lookup = Lookup.from_dict(road_network)
    bincode = slk_lookup.to_binary()
    slk_lookup = open_binary_file(bincode)
    confirm_test_cases_with_instance(slk_lookup)

def test_open_binary_file_file(road_network):
    from .util.confirm_test_cases_with_instance import confirm_test_cases_with_instance
    from megalinref import Lookup, open_binary_file
    from io import BytesIO
    slk_lookup = Lookup.from_dict(road_network)
    bincode = BytesIO(slk_lookup.to_binary())
    slk_lookup = open_binary_file(bincode)
    confirm_test_cases_with_instance(slk_lookup)