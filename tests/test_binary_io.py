
import json
import math
import os

from util.example_result_cases import example_result_cases


def test_init_from_dict():
    from megalinref import SLKLookup
    road_network = get_test_road_network_as_dict()
    slk_lookup = SLKLookup(road_network)
    confirm_test_cases_with_instance(slk_lookup, example_result_cases)


def test_cases_from_json_init():
    from megalinref import SLKLookup
    road_network = get_test_road_network_as_dict()
    slk_lookup = SLKLookup(road_network)
    bincode = slk_lookup.to_binary()
    slk_lookup = SLKLookup.from_binary(bincode)
    confirm_test_cases_with_instance(slk_lookup, example_result_cases)


# HELPER FUNCTIONS ##############################################

def get_test_road_network_as_dict():
    try:
        with open("./tests/temporary_test_data/road_network.json", "r") as f:
            json_data = json.load(f)
    except FileNotFoundError:
        cwd = os.getcwd()
        full_path = os.path.join(cwd, "temporary_test_data/road_network.json")
        raise Exception(f"Tests failed to locate {full_path}.\n Please run though `tests\setup temporary_test_data.ipynb` to download test data.")
    return json_data


def confirm_test_cases_with_instance(instance, test_result_cases):
    for case in test_result_cases:
        expected_result = case["expected_result"]

        result = instance.lookup(**case["args"])
        assert all(
            result["feature"][item] == expected_result["feature"][item]
            for item in ["ROAD", "CWY", "NETWORK_TYPE"]
        )
        assert all(
            math.isclose(
                result[item],
                expected_result[item],
                abs_tol=0.01
            )
            for item in [
                "slk",
                "true",
                # "distance_metres",
            ]
        )
