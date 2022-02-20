import pytest


def test_from_dict_no_arguments():
    import megalinref as mlr
    with pytest.raises(TypeError):
        lookup = mlr.Lookup.from_dict()


def test_from_dict_wrong_type():
    import megalinref as mlr
    with pytest.raises(TypeError):
        lookup = mlr.Lookup.from_dict("a")


def test_from_dict_wrong_dict():
    import megalinref as mlr
    with pytest.raises(Exception, match="Unable to extract 'features' from input"):
        lookup = mlr.Lookup.from_dict(dict())


def test_from_dict_zero_features():
    import megalinref as mlr
    with pytest.raises(Exception, match="'features' list is empty"):
        lookup = mlr.Lookup.from_dict({"features":[]})


def test_from_dict_features_not_dict():
    import megalinref as mlr
    with pytest.raises(Exception, match="All items in 'features' must be of type dict"):
        lookup = mlr.Lookup.from_dict({"features":[1,2,3]})


def test_from_dict_features_no_properties():
    import megalinref as mlr
    with pytest.raises(Exception, match="Unable to find the 'properties' item on one of the features."):
        lookup = mlr.Lookup.from_dict({"features":[
            {
                "der":1
            }
        ]})
    

def test_from_dict_features_bad_properties():
    import megalinref as mlr
    with pytest.raises(Exception, match="'properties' must be of type dict"):
        lookup = mlr.Lookup.from_dict({"features":[
            {
                "properties":1
            }
        ]})





def test_from_dict_properties_missing():
    import megalinref as mlr

    properties = {
        'ROAD':           'X001',
        'START_SLK':      0.03,
        'END_SLK':        0.08,
        'CWY':            'Single',
        'NETWORK_TYPE':   'Crossover',
        'START_TRUE_DIST':0.03,
        'END_TRUE_DIST':  0.08
    }

    input_dict = {
        "features":[{
            'type': 'Feature',
            'geometry': {
                'type': 'LineString',
                'coordinates': [
                    [116.01613363394955, -31.90864177309936],
                    [116.01564263139306, -31.90885735305153]
                ]
            },
            'properties': properties
        }]
    }

    # confirm succeeds without error
    lookup = mlr.Lookup.from_dict(input_dict)

    for key in properties.keys():
        properties_copy = properties.copy()
        del properties_copy[key]
        input_dict["features"][0]["properties"] = properties_copy
        with pytest.raises(Exception, match="Item '{}' missing from 'properties'".format(key)):
            lookup = mlr.Lookup.from_dict(input_dict)


def test_from_dict_properties_wrong_type():
    import megalinref as mlr

    properties = {
        'ROAD':           'X001',
        'START_SLK':      0.03,
        'END_SLK':        0.08,
        'CWY':            'Single',
        'NETWORK_TYPE':   'Crossover',
        'START_TRUE_DIST':0.03,
        'END_TRUE_DIST':  0.08
    }

    properties_expected_type = {
        'ROAD':           "String",
        'START_SLK':      "f64",
        'END_SLK':        "f64",
        'CWY':            "Cwy",
        'NETWORK_TYPE':   "NetworkType",
        'START_TRUE_DIST':"f64",
        'END_TRUE_DIST':  "f64"
    }

    input_dict = {
        "features":[{
            'type': 'Feature',
            'geometry': {
                'type': 'LineString',
                'coordinates': [
                    [116.01613363394955, -31.90864177309936],
                    [116.01564263139306, -31.90885735305153]
                ]
            },
            'properties': properties
        }]
    }

    # confirm succeeds without error
    lookup = mlr.Lookup.from_dict(input_dict)

    for key, value in properties_expected_type.items():
        properties_copy = properties.copy()
        properties_copy[key] = {}
        input_dict["features"][0]["properties"] = properties_copy
        with pytest.raises(Exception, match=f"'{key}' must be of type {value}"):
            lookup = mlr.Lookup.from_dict(input_dict)


def test_from_dict_ok_one_feature():
    import megalinref as mlr

    lookup = mlr.Lookup.from_dict({
        "features":[{
            'type': 'Feature',
            'geometry': {
                'type': 'LineString',
                'coordinates': [
                    [116.01613363394955, -31.90864177309936],
                    [116.01564263139306, -31.90885735305153]
                ]
            },
            'properties': {
                'ROAD':           'X001',
                'START_SLK':      0.03,
                'END_SLK':        0.08,
                'CWY':            'Single',
                'NETWORK_TYPE':   'Crossover',
                'START_TRUE_DIST':0.03,
                'END_TRUE_DIST':  0.08
            }
        }]
    })

    assert lookup.get_feature_count() == 1
    
    
    result = lookup.road_slk_from_coordinate(
        lon           = 116.01613363394955, 
        lat           = -31.90864177309936,
        carriageways  = mlr.Cwy["All"],
        network_types = mlr.NetworkType["All"]
    )

    assert result["feature"]["ROAD"] == "X001"
    assert result["slk"] == 0.03


def test_from_dict_ok_all_features():
    import json
    import megalinref as mlr

    with open("./tests/temporary_test_data/road_network.json","r") as f:
        xx = json.load(f)

    lookup = mlr.Lookup.from_dict(xx)

    assert lookup.get_feature_count() == len(xx["features"])

    
    result = lookup.road_slk_from_coordinate(
        lon           = 116.01613363394955,
        lat           = -31.90864177309936,
        carriageways  = mlr.Cwy["All"],
        network_types = mlr.NetworkType["All"]
    )
    
    assert result["feature"]["ROAD"] == "X001"
    assert result["slk"] == 0.03

