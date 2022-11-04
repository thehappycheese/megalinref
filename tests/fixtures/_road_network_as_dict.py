from typing import Dict, Any
import pytest

@pytest.fixture()
def road_network() -> Dict[str, Any]:
    import json
    import os
    try:
        with open("./tests/temporary_test_data/road_network.json", "r") as f:
            json_data = json.load(f)
    except FileNotFoundError:
        cwd = os.getcwd()
        full_path = os.path.join(cwd, "temporary_test_data/road_network.json")
        raise Exception(f"Tests failed to locate {full_path}.\n Please run though `tests\setup temporary_test_data.ipynb` to download test data.")
    return json_data
