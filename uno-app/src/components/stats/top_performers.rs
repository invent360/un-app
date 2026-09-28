//! Top performers podium component

use leptos::prelude::*;

/// Top performer data
#[derive(Debug, Clone)]
pub struct Performer {
    pub rank: u32,
    pub name: String,
    pub earnings: f64,
    pub device_type: String,
}

/// Top performers podium component
#[component]
pub fn TopPerformers(
    performers: Vec<Performer>,
) -> impl IntoView {
    // Reorder for podium display: 2nd, 1st, 3rd
    let podium_order = move || {
        let mut ordered = vec![];
        if performers.len() >= 2 {
            ordered.push(performers[1].clone()); // 2nd place
        }
        if !performers.is_empty() {
            ordered.push(performers[0].clone()); // 1st place
        }
        if performers.len() >= 3 {
            ordered.push(performers[2].clone()); // 3rd place
        }
        ordered
    };

    view! {
        <div class="top-performers">
            <h3 class="performers-title">"Top Earners"</h3>
            <div class="podium">
                {podium_order().into_iter().map(|performer| {
                    let rank_class = match performer.rank {
                        1 => "gold",
                        2 => "silver",
                        3 => "bronze",
                        _ => "",
                    };
                    let height = match performer.rank {
                        1 => "140px",
                        2 => "100px",
                        3 => "80px",
                        _ => "60px",
                    };

                    view! {
                        <div class=format!("podium-place {}", rank_class)>
                            <div class="performer-info">
                                <span class="performer-name">{performer.name.clone()}</span>
                                <span class="performer-earnings">
                                    {format!("${:.2}", performer.earnings)}
                                </span>
                                <span class="performer-device">{performer.device_type.clone()}</span>
                            </div>
                            <div class="podium-block" style=format!("height: {}", height)>
                                <span class="rank">{performer.rank}</span>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
