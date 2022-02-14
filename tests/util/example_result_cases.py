
example_result_cases = [
    {
        "args":{
            "lat":          -31.89006203575722,
            "lon":          115.80183730752809,
            "cwy":          0b1111_1111, # megalinref.Cwy["All"],
            "network_type": 0b1111_1111, # megalinref.NetworkType["All"],
        },
        "expected_result":{
            "feature":{
                "ROAD": "H016",
                "CWY": 0b0000_0100, #"Left",
                "NETWORK_TYPE": 0b0000_0001,#"State Road",
            },
            "slk":10,
            "true":10,
            "distance_metres":0,
        }
    }
]

