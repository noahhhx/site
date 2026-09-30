use leptos::prelude::*;
use leptos_meta::Title;

use crate::app::{Ext, Prompt};

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <Title text="projects · noah"/>
        <Prompt command="ls ~/projects"/>
        <p class="fg-muted">
            "A few things I've built. Everything else is on "
            <Ext href="https://github.com/noahhhx?tab=repositories">"GitHub"</Ext>"."
        </p>

        <h2>"usb-hid-relay"</h2>
        <Meta lang="C" repo="https://github.com/noahhhx/usb-hid-relay"/>
        <p>
            "I use Linux for everything except League of Legends, which lives on an old Windows "
            "PC that I stream to my laptop with Sunshine and Moonlight. Everything is wired, so "
            "there's no noticeable latency. The catch is that Riot's Vanguard anti-cheat blocks "
            "the virtual mouse Moonlight uses."
        </p>
        <p>
            "The fix is to give Windows a real mouse. A Raspberry Pi Zero 2 W plugged into the PC "
            "runs as a USB HID gadget, so Windows just sees an ordinary USB mouse. On the laptop, "
            "a small C program reads mouse events from evdev and sends them over UDP to the Pi, "
            "which writes them out as HID reports. It asks "<code>"hyprctl"</code>" for the active "
            "window, so input is only forwarded while Moonlight has focus."
        </p>

        <h2>"nixos + pi-shop"</h2>
        <h3>"nixos"</h3>
        <Meta lang="Nix" repo="https://github.com/noahhhx/nixos"/>
        <p>
            "My NixOS config for my Framework laptop, built with the "
            <Ext href="https://github.com/vic/dendritic">"Dendritic pattern"</Ext>
            ": every file is a flake-parts module for one aspect of the system, like Hyprland, "
            "kitty or Tailscale. "<code>"verify.sh"</code>" formats and evaluates the whole thing, "
            "and can build and boot every host in a VM using the exact config the real machine "
            "runs. A fresh install is a single script that adopts the generated hardware config "
            "and switches over."
        </p>
        <p>
            "The look of this site comes from it: the wallpaper, kitty's colours and Hyprland's "
            "window borders."
        </p>
        <h3>"pi-shop"</h3>
        <Meta lang="Nix" repo="https://github.com/noahhhx/pi-shop"/>
        <p>
            "My setup for the "<Ext href="https://pi.dev/">"pi"</Ext>" coding agent: a global "
            <code>"AGENTS.md"</code>", portable skills and my own extensions. It installs as a "
            "home-manager module, a pi package or a plain git repo, and uses the same Dendritic "
            "layout. My NixOS config pulls it in and adds machine-specific instructions on top."
        </p>

        <h2>"learning rust"</h2>
        <p>"I write Java for a living. These are how I'm learning Rust."</p>
        <h3>"site"</h3>
        <Meta lang="Rust" repo="https://github.com/noahhhx/site"/>
        <p>
            "This site. It started out as Spring Boot and Thymeleaf; now it's "
            <Ext href="https://leptos.dev/">"Leptos"</Ext>" on Axum, rendered on the server and "
            "hydrated in the browser."
        </p>
        <h3>"rust-shell"</h3>
        <Meta lang="Rust" repo="https://github.com/noahhhx/rust-shell"/>
        <p>
            "A POSIX shell, built by working through CodeCrafters' "
            <Ext href="https://app.codecrafters.io/courses/shell/overview">"Build Your Own Shell"</Ext>
            " challenge: parsing, a REPL, builtins like "<code>"cd"</code>", "<code>"pwd"</code>
            " and "<code>"echo"</code>", and running external programs."
        </p>
        <h3>"powr"</h3>
        <Meta lang="Rust" repo="https://github.com/noahhhx/powr"/>
        <p>
            "A minimal, keyboard-driven power menu (lock, sleep, reboot, shutdown and so on) "
            "built with "<Ext href="https://iced.rs/">"iced"</Ext>". Still in progress."
        </p>
    }
}

/// `<language> · <repo link>` line under a project heading.
#[component]
fn Meta(lang: &'static str, repo: &'static str) -> impl IntoView {
    let name = repo.trim_start_matches("https://");
    view! {
        <p class="meta">
            <span class="fg-peach">{lang}</span>
            " · "
            <Ext href=repo>{name}</Ext>
        </p>
    }
}
