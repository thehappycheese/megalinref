import json

# run tests_notebooks/tests/temporary_test_data/road_network.json
# to prepare the data for fixtures

with open("tests/temporary_test_data/road_network.json") as f:
    road_network = json.load(f)