
import json
import os

from util.example_result_cases import example_result_cases
from util.dictdiffer_tools import assert_dictdiffer


def test_init_from_dict():
    from megalinref import Lookup
    road_network = get_test_road_network_as_dict()
    slk_lookup = Lookup.from_dict(road_network)
    confirm_test_cases_with_instance(slk_lookup, example_result_cases)


def test_cases_from_json_init():
    from megalinref import Lookup
    road_network = get_test_road_network_as_dict()
    slk_lookup = Lookup.from_dict(road_network)
    bincode = slk_lookup.to_binary()
    slk_lookup = Lookup.from_binary(bincode)
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
        assert_dictdiffer(
            result                    = instance.road_slk_from_coordinate(**case["args"]),
            expected_result           = case["expected_result"],
            check_unexpected_added    = False, # don't bother reporting additional outputs for now.
            absolute_tolerance        = 0.001
        )
