from typing import Any
from . import megalinref as mlr
import pandas as pd
import numpy as np
import requests
from arcgis2geojson import arcgis2geojson
import math
import os

DATA_SOURCE_URL = "https://mrgis.mainroads.wa.gov.au/arcgis/rest/services/OpenData/RoadAssets_DataPortal/MapServer/17/query?where=1%3D1&outFields=ROAD,START_SLK,END_SLK,CWY,NETWORK_TYPE,START_TRUE_DIST,END_TRUE_DIST&outSR=4326&f=json"



def _download_fresh_data_as_json(url=DATA_SOURCE_URL, chunk_limit=None):

    response = requests.request("GET", f"{url}&returnCountOnly=true")
    record_count = response.json()["count"]

    print(f"Downloading {record_count} records" + (":" if chunk_limit is None else f", {chunk_limit=}:"))
    if chunk_limit is not None:
        print("." * min(chunk_limit, math.floor(record_count/1000)))
    else:
        print("." * math.floor(record_count/1000))

    # TODO: make this download multiple chunks in parallel

    output=[]
    offset = 0
    chunk_counter = 0
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
    print(f"\nDownload Completed. received {len(output)} records")
    json["features"] = output
    json = arcgis2geojson(json)
    return json

def pass_data_to_rust(geojson:dict[str, Any]):
    new_object = mlr.pass_data_to_rust(geojson)


def _convert_geo_json_to_pandas_dataframe(json):
    if not json["type"] == "FeatureCollection":
        raise Exception("Error converting geojson, expected a FeatureCollection")

    features = json["features"]
    # check there is at least 1 feature
    if len(features) == 0:
        raise Exception("No features in the FeatureCollection")
    
    # check that all features are the expected type
    if not all(
            feature["geometry"]["type"] == "LineString"
        for feature in features
    ):
        raise Exception("Error converting geojson, expected only LineString features")
    
    columns = features[0]["properties"].keys()
    # confirm all features have the same properties
    if not all(column in {"ROAD", "START_SLK", "END_SLK", "CWY", "NETWORK_TYPE", "START_TRUE_DIST", "END_TRUE_DIST"} for column in columns):
        raise Exception("Data contains unexpected columns")

    flat_coords            :list[list[float]] = []
    road              :list[str]   = []
    slk_from          :list[float] = []
    slk_to            :list[float] = []
    true_from         :list[float] = []
    true_to           :list[float] = []
    network_type      :list[int]   = []
    cwy               :list[int]   = []
    point_index_start :list[int]   = []
    point_index_end   :list[int]   = []

    for feature in features:
        # this next line is hard to read sure... but it has good performance, and that matters a little bit here.
        feature_points = [
            sub_item 
            for item in feature["geometry"]["coordinates"]
            for sub_item in item
        ]

        # TODO: to be tested
        # point_index_start .append(len(points))
        # points            .extend(feature_points)
        # point_index_end   .append(len(points))
        
        flat_coords       .append(np.array(feature_points))
        road              .append(feature["properties"]["ROAD"])
        slk_from          .append(feature["properties"]["START_SLK"])
        slk_to            .append(feature["properties"]["END_SLK"])
        true_from         .append(feature["properties"]["START_TRUE_DIST"])
        true_to           .append(feature["properties"]["END_TRUE_DIST"])
        network_type      .append(mlr.NETWORK_TYPE.get(feature["properties"]["NETWORK_TYPE"], 0))
        cwy               .append(mlr.CWY.get(feature["properties"]["CWY"], 0))
    
    return pd.DataFrame({
        "road": road,
        "slk_from": slk_from,
        "slk_to": slk_to,
        "true_from": true_from,
        "true_to": true_to,
        "network_type": network_type,
        "cwy": cwy,
        "points": flat_coords,
    })
    



def obtain_fresh_data():
    json = _download_fresh_data_as_json(
        DATA_SOURCE_URL,
        chunk_limit=3
    )    
    json = arcgis2geojson(json)
    return _convert_geo_json_to_pandas_dataframe(json)






# def convert_lat_lng_to_slk(self, df:pd.DataFrame, lat_col:str, lon_col:str, state_roads_only:bool=True) -> pd.DataFrame:
#     coords = gpd.points_from_xy(df[lon_col], df[lat_col])
#     dat = self.data
#     if state_roads_only:
#         dat = dat[dat["NETWORK_TYPE"]=="State Road"]
    
#     nearest_features = self.data.sindex.nearest(coords.data)

#     dat = dat.loc[nearest_features[1]]
    
#     result = dat.loc[:,["ROAD", "CWY"]]
#     result["SLK"] = (
#         dat
#         .project(
#             gpd.GeoSeries(coords),
#             align=False,
#             normalized=False
#         )
#         / dat.length
#         * (dat["END_SLK"] - dat["START_SLK"])
#         + dat["START_SLK"]
#     )
#     result.index = df.index

#     return result

