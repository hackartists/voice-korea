use dioxus::prelude::*;

use crate::models::pi::PiChart;

#[component]
pub fn PiGraph(chart_data: Vec<PiChart>) -> Element {
    rsx! {
        div {
            class: "chart-container",
            div {
                class: "labels rounded-lg border border-[#e7e8e8] h-min p-3",
                {chart_data.iter().map(|segment| rsx!(
                    div {
                        class: "label-item",
                        span {
                            class: "color-box",
                            style: "background-color: {segment.color};",
                        }
                        span {
                            class: "label-text",
                            "{segment.label} ({(segment.percentage * 100.0).round()}%)"
                        }
                    }
                ))}
            }
            div {
                class: "chart-wrapper",
                svg {
                    width: "300",
                    height: "300",
                    view_box: "-1 -1 2 2",
                    {chart_data.iter().enumerate().map(|(i, segment)| {
                        let (start_angle, end_angle) = calculate_angles(&chart_data, i);
                        let path = create_pie_slice_path(start_angle, end_angle);
                        rsx!(
                            path {
                                d: "{path}",
                                fill: "{segment.color}",
                            }
                        )
                    })}
                }
            },
        }
    }
}

fn calculate_angles(data: &Vec<PiChart>, index: usize) -> (f64, f64) {
    let total: f64 = data.iter().map(|d| d.percentage).sum();
    let cumulative: f64 = data.iter().take(index).map(|d| d.percentage).sum();
    let start_angle = cumulative / total * 360.0;
    let end_angle = (cumulative + data[index].percentage) / total * 360.0;
    (start_angle, end_angle)
}

fn create_pie_slice_path(start_angle: f64, end_angle: f64) -> String {
    let start = polar_to_cartesian(1.0, start_angle);
    let end = polar_to_cartesian(1.0, end_angle);
    let large_arc = if end_angle - start_angle > 180.0 {
        1
    } else {
        0
    };
    format!(
        "M 0 0 L {} {} A 1 1 0 {} 1 {} {} Z",
        start.0, start.1, large_arc, end.0, end.1
    )
}

fn polar_to_cartesian(radius: f64, angle: f64) -> (f64, f64) {
    let radians = angle.to_radians();
    (radius * radians.cos(), radius * radians.sin())
}
