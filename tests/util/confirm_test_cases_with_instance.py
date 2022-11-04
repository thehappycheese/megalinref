from .dictdiffer_tools import assert_dictdiffer

example_result_cases = [
    {
        "args":{
            "lat"           : -31.89006203575722,
            "lon"           : 115.80183730752809,
            "carriageways"  : 0b1111_1111, # megalinref.Cwy["All"],
            "network_types" : 0b1111_1111, # megalinref.NetworkType["All"],
            "roads"         : []
        },
        "expected_result":{
            "feature":{
                "ROAD": "H016",
                "CWY": "Left",
                "NETWORK_TYPE": "State Road",
            },
            "slk":10,
            "true":10,
            "distance_metres":0,
        }
    }
]

def confirm_test_cases_with_instance(lookup_instance):
    for case in example_result_cases:
        assert_dictdiffer(
            result                    = lookup_instance.road_slk_from_coordinate(**case["args"]),
            expected_result           = case["expected_result"],
            check_unexpected_added    = False, # don't bother reporting additional outputs for now.
            absolute_tolerance        = 0.001
        )