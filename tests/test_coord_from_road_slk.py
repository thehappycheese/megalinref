from .fixtures import road_network


def test_coord_from_road_slk(road_network):
    from megalinref import Lookup, Cwy
    import numpy as np
    lookup = Lookup.from_dict(road_network)
    examples = [
        [
            {"road":"H016", "slk":8.5, "carriageways":Cwy["Left"]},
            [[(115.81402235302504,-31.89749388845883)]]
        ]
    ]
    for args, expected_result in examples:
        assert np.isclose(lookup.coordinate_from_road_slk(**args), expected_result).all()
    
