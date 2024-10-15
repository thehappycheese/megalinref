from typing import Any, Optional, Dict
import requests
from arcgis2geojson import arcgis2geojson
import math

DATA_SOURCE_URL = (
    "https://gisservices.mainroads.wa.gov.au/arcgis/rest/services/OpenData/RoadAssets_DataPortal/MapServer/17/query"
    "?where=1%3D1"
    "&orderByFields=OBJECTID"
    "&outFields=ROAD,START_SLK,END_SLK,CWY,NETWORK_TYPE,START_TRUE_DIST,END_TRUE_DIST"
    "&outSR=4326"
    "&f=json"
)


def download_fresh_data_as_json(url:str=DATA_SOURCE_URL, chunk_limit:Optional[int]=None) -> Dict[str, Any]:

    response = requests.request("GET", f"{url}&returnCountOnly=true")
    record_count = response.json()["count"]

    print(f"Downloading {record_count} records" + (":" if chunk_limit is None else f", chunk_limit={chunk_limit}:"))
    if chunk_limit is not None:
        print("." * min(chunk_limit, math.floor(record_count/1000)))
    else:
        print("." * math.floor(record_count/1000))

    # TODO: make this download multiple chunks in parallel

    output=[]
    offset = 0
    chunk_counter = 0
    json = None
    while True:
        if chunk_limit is not None and chunk_counter >= chunk_limit:
            break
        chunk_counter += 1

        response = requests.request("GET", f"{url}&resultOffset={offset}")
        json = response.json()
        if json["geometryType"] != "esriGeometryPolyline":
            raise Exception("Rest service returned an object that did not have geometryType:esriGeometryPolyline")
        offset += len(json["features"])
        output.extend(json["features"])
        if "exceededTransferLimit" not in json or not json["exceededTransferLimit"]:
            break
        print(".", end="")
        
    if json is None or len(output) == 0:
        raise Exception("No data was returned")
    
    print(f"\nDownload Completed. received {len(output)} records")
    json["features"] = output
    json = arcgis2geojson(json)
    return json