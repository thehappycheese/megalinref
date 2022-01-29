

use geo::{line_string, polygon};
use geo::convex_hull::ConvexHull;

use reqwest;


pub fn test_geo()->String{
    // An L shape
    let poly:geo::Polygon<f64> = polygon![
        (x: 0.0, y: 0.0),
        (x: 4.0, y: 0.0),
        (x: 4.0, y: 1.0),
        (x: 1.0, y: 1.0),
        (x: 1.0, y: 4.0),
        (x: 0.0, y: 4.0),
        (x: 0.0, y: 0.0),
    ];

    // Calculate the polygon's convex hull
    let hull = poly.convex_hull();

    assert_eq!(
        hull.exterior(),
        &line_string![
            (x: 4.0, y: 0.0),
            (x: 4.0, y: 1.0),
            (x: 1.0, y: 4.0),
            (x: 0.0, y: 4.0),
            (x: 0.0, y: 0.0),
            (x: 4.0, y: 0.0),
        ].into()
    );
    "OK".into()
}

pub async fn fetch_data(url: &str) -> String{
    let client = reqwest::Client::new();
    let response = client.get(url).send().await.unwrap();
    response.text_with_charset("utf-8").await.unwrap()
}