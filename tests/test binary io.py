from megalinref import SLKLookup
import json
import math


from .util.test_result_cases import test_result_cases


def init_from_json():
    with open("./temporary_test_data/road_network.json", "r") as f:
        json_data = json.read(f)
        slk_lookup = SLKLookup(json_data)
        del json_data
    return slk_lookup


def init_from_binary():
    with open("./temporary_test_data/binary_cache.bin", "rb") as f:
        bincode = f.read()
    slk_lookup_reloaded = SLKLookup.from_binary(bincode)
    return slk_lookup_reloaded


def test_cases_from_json_init():
    slk_lookup = init_from_json()
    confirm_test_cases_with_instance(slk_lookup, test_result_cases)


def test_cases_from_json_init():
    # Load the road network data from the JSON file
    slk_lookup = init_from_json()
    bincode = slk_lookup.to_binary()
    slk_lookup = SLKLookup.from_binary(bincode)
    confirm_test_cases_with_instance(slk_lookup, test_result_cases)


def confirm_test_cases_with_instance(instance:SLKLookup, test_result_cases):
    for case in test_result_cases:
        result = instance.lookup(**case["args"])
        assert all(
            result["feature"][item] == case["expected_result"]["feature"][item]
            for item in ["ROAD", "CWY", "NETWORK_TYPE"]
        )
        assert all(
            math.isclose(
                result["slk"],
                case["expected_result"]["slk"],
                abs_tol=0.01
            )
            for item in ["slk", "true", "distance_metres"]
        )


    

    