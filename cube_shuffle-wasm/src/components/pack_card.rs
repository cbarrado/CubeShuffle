use itertools::Itertools;
use yew::prelude::*;

use cube_shuffle_core::distribution_shuffle::Pack;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub index: usize,
    pub pack: Pack<String>,
    pub checked: bool,
    pub onclick: Callback<usize>,
    #[prop_or(1.0)]
    pub zoom_scale: f64,
}

pub enum Msg {
    Clicked,
}

/// Returns (background_color, text_color) CSS values for a pile name if it matches a known color.
fn get_pile_colors(name: &str) -> Option<(&'static str, &'static str)> {
    match name.to_lowercase().as_str() {
        "black" => Some(("#1a1a1a", "#ffffff")),         // Dark black with white text
        "blue" => Some(("#3273dc", "#ffffff")),          // Blue with white text
        "green" => Some(("#23d160", "#ffffff")),         // Green with white text
        "red" => Some(("#ff3860", "#ffffff")),           // Red with white text
        "white" => Some(("#f5f5f5", "#363636")),         // Light white/off-white with dark text
        "artifacts" => Some(("#8b4513", "#ffffff")),     // Brown (saddle brown) with white text
        "gold" => Some(("#ffd700", "#363636")),          // Gold/yellow with dark text
        _ => None,
    }
}

pub struct PackCard {}

impl Component for PackCard {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let props = ctx.props();
        match msg {
            Msg::Clicked => {
                props.onclick.emit(props.index);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let link = ctx.link();
        
        // Calculate font size based on zoom scale
        let base_font_size = 1.0; // 1rem
        let scaled_font_size = base_font_size * props.zoom_scale;
        
        let sources: Html = props
            .pack
            .card_sources
            .iter()
            .sorted_unstable_by_key(|(name, _)| name.as_str())
            .map(|(name, amount)| {
                let row_style = get_pile_colors(name)
                    .map(|(bg, text)| format!("background-color: {}; color: {};", bg, text))
                    .unwrap_or_default();
                html! {
                    <tr style={ row_style.clone() }>
                        <th style={ row_style.clone() }>{ name }</th>
                        <td style={ row_style }>{ amount }</td>
                    </tr>
                }
            })
            .collect();
        let checked_bg = if props.checked {
            " has-background-success"
        } else {
            ""
        };

        let on_click = link.callback(|_| Msg::Clicked);
        let mark_button = if props.checked {
            html! {
                <button class="delete"/>
            }
        } else {
            html! {
                <button class="button">{ "Done" }</button>
            }
        };

        // Apply zoom scale via CSS transform and font-size
        let card_style = format!(
            "transform: scale({}); transform-origin: top left; font-size: {}rem;",
            props.zoom_scale,
            scaled_font_size
        );

        html! {
            <div class="card" style={ card_style }>
                <div class={ "card-header".to_owned() + checked_bg }>
                    <label class="label card-header-title">{ props.index + 1 }</label>
                    <span class="card-header-icon" onclick={ on_click }>
                        { mark_button }
                    </span>
                </div>
                <div class="card-content">
                    <table class="table is-hoverable is-fullwidth is-striped">
                        <tbody>
                            { sources }
                        </tbody>
                    </table>
                </div>
            </div>
        }
    }
}
