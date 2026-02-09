use itertools::Itertools;
use web_sys::WheelEvent;
use yew::prelude::*;

use cube_shuffle_core::distribution_shuffle::Pack;

use crate::components::pack_card::PackCard;
use crate::config::is_tauri;

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub packs: Vec<Pack<String>>,
}

pub struct PackItem {
    pub pack: Pack<String>,
    pub checked: bool,
}

pub enum Msg {
    Check(usize),
    Zoom(f64),
}

/// Zoom scale limits
const MIN_ZOOM: f64 = 0.5;
const MAX_ZOOM: f64 = 2.0;
const ZOOM_STEP: f64 = 0.1;

pub struct PackList {
    packs: Vec<PackItem>,
    zoom_scale: f64,
    is_desktop: bool,
}

impl Component for PackList {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            packs: ctx
                .props()
                .packs
                .iter()
                .map(|pack| PackItem {
                    pack: pack.to_owned(),
                    checked: false,
                })
                .collect(),
            zoom_scale: 1.0,
            is_desktop: is_tauri(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Check(index) => {
                self.packs[index].checked = !(self.packs[index].checked);
                true
            }
            Msg::Zoom(delta) => {
                // Only handle zoom if running in Tauri (Windows desktop)
                if self.is_desktop {
                    let new_scale = self.zoom_scale + delta;
                    self.zoom_scale = new_scale.clamp(MIN_ZOOM, MAX_ZOOM);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();
        
        // Create wheel event handler for zoom (only active on desktop)
        let on_wheel = {
            let is_desktop = self.is_desktop;
            Callback::from(move |e: WheelEvent| {
                if is_desktop {
                    // Prevent default scrolling when zooming
                    e.prevent_default();
                    
                    // Get scroll direction: negative delta = scroll up = zoom in
                    let delta_y = e.delta_y();
                    let zoom_delta = if delta_y < 0.0 {
                        ZOOM_STEP // Scroll up = zoom in
                    } else {
                        -ZOOM_STEP // Scroll down = zoom out
                    };
                    link.send_message(Msg::Zoom(zoom_delta));
                }
            })
        };

        let zoom_scale = self.zoom_scale;
        let packs: Html = self
            .packs
            .iter()
            .enumerate()
            .sorted_by_key(|(_, pi)| pi.checked)
            .map(|(index, pack_item)| {
                let on_click = ctx.link().callback(Msg::Check);
                html! {
                    <div class="column is-narrow">
                        <PackCard
                            index={ index }
                            pack={ pack_item.pack.clone() }
                            checked={ pack_item.checked }
                            onclick={ on_click }
                            zoom_scale={ zoom_scale }
                        />
                    </div>
                }
            })
            .collect();
        
        // Show zoom indicator only on desktop
        let zoom_indicator = if self.is_desktop {
            let zoom_percent = (self.zoom_scale * 100.0).round() as i32;
            html! {
                <div class="has-text-centered mb-2">
                    <span class="tag is-info">
                        { format!("Zoom: {}% (use mouse wheel)", zoom_percent) }
                    </span>
                </div>
            }
        } else {
            html! {}
        };

        html! {
            <div onwheel={ on_wheel }>
                { zoom_indicator }
                <div class="columns is-multiline is-centered">
                    { packs }
                </div>
            </div>
        }
    }
}
