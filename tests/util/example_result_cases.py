import megalinref as mlr

example_result_cases = [
    {
        "args":{
            "lat":          -31.89006203575722,
            "lon":          115.80183730752809,
            "cwy":          mlr.Cwy["All"],
            "network_type": mlr.NetworkType["All"],
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