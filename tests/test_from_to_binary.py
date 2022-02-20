







def test_from_binary():
    from util.confirm_test_cases_with_instance import confirm_test_cases_with_instance
    from util.get_test_road_network_as_dict import get_test_road_network_as_dict
    from megalinref import Lookup
    road_network = get_test_road_network_as_dict()
    slk_lookup = Lookup.from_dict(road_network)
    bincode = slk_lookup.to_binary()
    slk_lookup = Lookup.from_binary(bincode)
    confirm_test_cases_with_instance(slk_lookup)





