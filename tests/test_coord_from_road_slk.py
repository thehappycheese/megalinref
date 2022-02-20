





def test_coord_from_road_slk():
    from megalinref import Lookup, Cwy
    from util.get_test_road_network_as_dict import get_test_road_network_as_dict
    import numpy as np

    road_network = get_test_road_network_as_dict()
    lookup = Lookup.from_dict(road_network)

    examples = [
        [
            {"road":"H016", "slk":8.5, "carriageways":Cwy["Left"]},
            [[(115.81402235302504,-31.89749388845883)]]
        ]
    ]
    for args, expected_result in examples:
        assert np.isclose(lookup.coordinate_from_road_slk(**args), expected_result).all()
    
