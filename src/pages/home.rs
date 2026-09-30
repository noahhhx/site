use leptos::prelude::*;
use leptos_meta::Title;

use crate::app::{EMAIL, Ext, GITHUB, Prompt, SIGNAL, SIGNAL_HANDLE};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="noah"/>
        <Prompt command="fastfetch"/>
        <section class="fetch">
            <img src="/img/monstercat.webp" alt="A cat in a can of Monster Ultra" width="198" height="400"/>
            <div>
                <p class="fetch-title">
                    <span class="fg-mauve">"noah"</span>
                    "@"
                    <span class="fg-mauve">"noahhh.co.uk"</span>
                </p>
                <p class="fg-faint" aria-hidden="true">"-----------------"</p>
                <dl>
                    <dt>"OS"</dt>
                    <dd>"NixOS"</dd>
                    <dt>"Host"</dt>
                    <dd>"Framework Laptop 13"</dd>
                    <dt>"WM"</dt>
                    <dd>"Hyprland"</dd>
                    <dt>"Terminal"</dt>
                    <dd>"kitty"</dd>
                    <dt>"Editor"</dt>
                    <dd>"Zed"</dd>
                    <dt>"Theme"</dt>
                    <dd>"Catppuccin Mocha"</dd>
                    <dt>"Uptime"</dt>
                    <dd>"28 years"</dd>
                    <dt>"Init"</dt>
                    <dd>"Caffeine"</dd>
                </dl>
                <dl>
                    <dt>"GitHub"</dt>
                    <dd><Ext href=GITHUB>"noahhhx"</Ext></dd>
                    <dt>"Email"</dt>
                    <dd><a href=format!("mailto:{EMAIL}")>{EMAIL}</a></dd>
                    <dt>"Signal"</dt>
                    <dd><Ext href=SIGNAL>{SIGNAL_HANDLE}</Ext></dd>
                </dl>
                <p class="swatches" aria-hidden="true">
                    <span class="bg-red"></span>
                    <span class="bg-peach"></span>
                    <span class="bg-yellow"></span>
                    <span class="bg-green"></span>
                    <span class="bg-teal"></span>
                    <span class="bg-blue"></span>
                    <span class="bg-mauve"></span>
                    <span class="bg-pink"></span>
                </p>
            </div>
        </section>

        <Prompt command="cat about.md"/>
        <p>
            "I'm "<strong>"Noah"</strong>", a software engineer in the UK. I build mostly with "
            "Java, and I'm learning Rust on the side (this site is part of that). I feel most at "
            "home developing tools for other developers and technical people, but enjoy the "
            "problem solving aspect of any challenge. I have an interest in FOSS, self-hosting, "
            "local-first and privacy."
        </p>

        <p class="prompt cursor"></p>
    }
}
