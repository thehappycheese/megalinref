from fixtures import road_network


def test_from_binary(road_network):
    from util.confirm_test_cases_with_instance import confirm_test_cases_with_instance
    from megalinref import Lookup
    slk_lookup = Lookup.from_dict(road_network)
    bincode = slk_lookup.to_binary()
    slk_lookup = Lookup.from_binary(bincode)
    confirm_test_cases_with_instance(slk_lookup)





