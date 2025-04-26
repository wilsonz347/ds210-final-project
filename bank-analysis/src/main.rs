mod parser;
mod analysis;
mod models;

// Charming for visualization
use charming::{
    component::{Axis, Feature, Grid, Title},
    datatype::{CompositeValue, Dataset, NumericValue},
    element::{AxisLabel, AxisType, NameLocation, Tooltip, ItemStyle},
    series::Scatter,
    Chart, HtmlRenderer,
};
use std::fs::write;

use crate::parser::load_csv_file;
use crate::analysis::{compute_region_stats, aggregate_by_date, aggregate_by_domain};
use crate::models::RegionStats;

fn create_charts(region_stats: Vec<RegionStats>) -> Vec<Chart> {
    // Prepare data for the dataset
    let data: Vec<Vec<CompositeValue>> = region_stats
        .into_iter()
        .map(|stat| {
            vec![
                CompositeValue::String(stat.region),
                CompositeValue::Number(charming::datatype::NumericValue::Float(stat.total as f64)),
                CompositeValue::Number(charming::datatype::NumericValue::Float(stat.average)),    
                CompositeValue::Number(charming::datatype::NumericValue::Float(stat.median)),
                CompositeValue::Number(charming::datatype::NumericValue::Float(stat.count as f64)),
            ]
        })
        .collect();

    let mut charts = Vec::new();

    // Chart 1: Total
    charts.push(
        Chart::new()
            .title(Title::new().text("Total Transaction Value by City").left("center"))
            .tooltip(Tooltip::new())
            .x_axis(
                Axis::new()
                    .type_(AxisType::Category)
                    .name("City")
                    .data(
                        data.iter()
                            .map(|row| match &row[0] {
                                CompositeValue::String(s) => s.clone(),
                                _ => panic!("Expected string for city"),
                            })
                            .collect::<Vec<_>>(),
                    )
                    .axis_label(AxisLabel::new().rotate(45).interval(0)),
            )
            .y_axis(
                Axis::new()
                    .type_(AxisType::Value)
                    .name("Total Transaction Value")
                    .name_location(NameLocation::Middle)
                    .name_gap(100),
            )
            .series(
                Scatter::new()
                    .name("Transaction Value")
                    .data(
                        data.iter()
                            .map(|row| row[1].clone())
                            .collect::<Vec<_>>(),
                    )
                    .item_style(ItemStyle::new().color("orange"))
            ),
    );

    // Chart 2: Average
    charts.push(
        Chart::new()
            .title(Title::new().text("Average Transaction Value by City").left("center"))
            .tooltip(Tooltip::new())
            .x_axis(
                Axis::new()
                    .type_(AxisType::Category)
                    .name("City")
                    .data(
                        data.iter()
                            .map(|row| match &row[0] {
                                CompositeValue::String(s) => s.clone(),
                                _ => panic!("Expected string for city"),
                            })
                            .collect::<Vec<_>>(),
                    )
                    .axis_label(AxisLabel::new().rotate(45).interval(0)),
            )
            .y_axis(
                Axis::new()
                    .type_(AxisType::Value)
                    .name("Average Transaction Value")
                    .name_location(NameLocation::Middle)
                    .name_gap(70),
            )
            .series(
                Scatter::new()
                    .name("Transaction Value")
                    .data(
                        data.iter()
                            .map(|row| row[2].clone())
                            .collect::<Vec<_>>(),
                    )
                    .item_style(ItemStyle::new().color("purple"))
            ),
    );

    // Chart 3: Median
    charts.push(
        Chart::new()
            .title(Title::new().text("Median Transaction Value by City").left("center"))
            .tooltip(Tooltip::new())
            .x_axis(
                Axis::new()
                    .type_(AxisType::Category)
                    .name("City")
                    .data(
                        data.iter()
                            .map(|row| match &row[0] {
                                CompositeValue::String(s) => s.clone(),
                                _ => panic!("Expected string for city"),
                            })
                            .collect::<Vec<_>>(),
                    )
                    .axis_label(AxisLabel::new().rotate(45).interval(0)),
            )
            .y_axis(
                Axis::new()
                    .type_(AxisType::Value)
                    .name("Median Transaction Value")
                    .name_location(NameLocation::Middle)
                    .name_gap(70),
            )
            .series(
                Scatter::new()
                    .name("Transaction Value")
                    .data(
                        data.iter()
                            .map(|row| row[3].clone())
                            .collect::<Vec<_>>(),
                    )
                    .item_style(ItemStyle::new().color("green"))
            ),
    );

    // Chart 4: Count
    charts.push(
        Chart::new()
            .title(Title::new().text("Total Number of Transactions by City").left("center"))
            .tooltip(Tooltip::new())
            .x_axis(
                Axis::new()
                    .type_(AxisType::Category)
                    .name("City")
                    .data(
                        data.iter()
                            .map(|row| match &row[0] {
                                CompositeValue::String(s) => s.clone(),
                                _ => panic!("Expected string for city"),
                            })
                            .collect::<Vec<_>>(),
                    )
                    .axis_label(AxisLabel::new().rotate(45).interval(0)),
            )
            .y_axis(
                Axis::new()
                    .type_(AxisType::Value)
                    .name("Total Transactions")
                    .name_location(NameLocation::Middle)
                    .name_gap(60),
            )
            .series(
                Scatter::new()
                    .name("Transactions")
                    .data(
                        data.iter()
                            .map(|row| row[4].clone())
                            .collect::<Vec<_>>(),
                    )
                    .item_style(ItemStyle::new().color("red"))
            ),
    );

    charts
}

fn main() {
    let transactions = load_csv_file("../data/bankdataset.csv").expect("Failed to load");

    let region_stats = compute_region_stats(&transactions);

    for stat in &region_stats {
        println!("{:?}", stat);
    }

    // Create 4 subplots for region statistics
    let charts = create_charts(region_stats);

    // Render each chart and combine HTML outputs
    let renderer = HtmlRenderer::new("Region Statistics", 1200, 800);
    let mut html_output = String::new();
    for (i, chart) in charts.iter().enumerate() {
        let chart_id = format!("chart{}", i + 1);
        let html = renderer
            .render(chart)
            .expect(&format!("Failed to render chart {}", i + 1));
        // Extract only the <div> and <script> parts, removing <!DOCTYPE> and <html>
        let start = html.find("<div class=\"container\"").unwrap_or(0);
        let end = html.rfind("</script>").unwrap_or(html.len()) + 9;
        let chart_html = &html[start..end];
        // Replace default 'chart' ID with unique chart_id
        let modified_html = chart_html
            .replace(r#"id="chart""#, &format!(r#"id="{}""#, chart_id))
            .replace(
                r#"document.getElementById('chart')"#, 
                &format!(r#"document.getElementById('{}')"#, chart_id),
            );
        html_output.push_str(&modified_html);
        html_output.push_str("<div style='margin-bottom: 50px;'></div>");
    }

    // Wrap in a single HTML structure
    let final_html = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="utf-8">
            <title>Region Statistics</title>
            <script src="https://cdn.jsdelivr.net/npm/echarts@5.4.2/dist/echarts.min.js"></script>
            <script src="https://cdn.jsdelivr.net/npm/echarts-gl@2.0.9/dist/echarts-gl.min.js"></script>
            <style>
                .container {{ display: flex; justify-content: center; align-items: center; }}
                .item {{ margin: auto; }}
            </style>
        </head>
        <body>
            {}
            <div style="text-align: center; font-size: 14px; color: gray; margin-top: 30px;">
                This report was AI-generated using Rust and Charming.
            </div>
        </body>
        </html>
        "#,
        html_output
    );

    // Write to file
    write("region_stats.html", final_html).expect("Failed to write HTML file");
}