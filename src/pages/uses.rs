use leptos::prelude::*;
use leptos_meta::Title;

use crate::app::{Ext, Prompt};

#[component]
pub fn Uses() -> impl IntoView {
    view! {
        <Title text="uses · noah"/>
        <Prompt command="cat uses.md"/>
        <p class="fg-muted">
            "What I use day to day. Nearly all of it is declared in my "
            <Ext href="https://github.com/noahhhx/nixos">"NixOS config"</Ext>"."
        </p>

        <h2>"hardware"</h2>
        <dl class="kv">
            <dt>"Laptop"</dt>
            <dd>"Framework Laptop 13 (AMD Ryzen AI 300)"</dd>
            <dt>"Gaming"</dt>
            <dd>"An old Windows PC, streamed over the LAN with Sunshine + Moonlight"</dd>
        </dl>

        <h2>"desktop"</h2>
        <dl class="kv">
            <dt>"OS"</dt>
            <dd>"NixOS"</dd>
            <dt>"WM"</dt>
            <dd>"Hyprland, with hyprlock, hypridle and hyprpaper"</dd>
            <dt>"Bar"</dt>
            <dd>"Waybar"</dd>
            <dt>"Launcher"</dt>
            <dd>"Walker"</dd>
            <dt>"Notifications"</dt>
            <dd>"mako"</dd>
            <dt>"Theme"</dt>
            <dd>"Catppuccin Mocha, adw-gtk3-dark"</dd>
            <dt>"Font"</dt>
            <dd>"JetBrains Mono Nerd Font"</dd>
        </dl>

        <h2>"terminal"</h2>
        <dl class="kv">
            <dt>"Terminal"</dt>
            <dd>"kitty"</dd>
            <dt>"Shell"</dt>
            <dd>"zsh, with carapace completions and fzf"</dd>
            <dt>"TUIs"</dt>
            <dd>"btop, lazydocker, wiremix, bluetui, wlctl"</dd>
        </dl>

        <h2>"development"</h2>
        <dl class="kv">
            <dt>"Editor"</dt>
            <dd>"Zed, and Neovim in the terminal"</dd>
            <dt>"IDE"</dt>
            <dd>"IntelliJ IDEA for Java"</dd>
            <dt>"Environments"</dt>
            <dd><Ext href="https://devenv.sh/">"devenv"</Ext></dd>
            <dt>"Agent"</dt>
            <dd>
                <Ext href="https://pi.dev/">"pi"</Ext>", set up with "
                <Ext href="https://github.com/noahhhx/pi-shop">"pi-shop"</Ext>
            </dd>
            <dt>"Containers"</dt>
            <dd>"Docker"</dd>
        </dl>

        <h2>"network & privacy"</h2>
        <dl class="kv">
            <dt>"Browser"</dt>
            <dd>"LibreWolf"</dd>
            <dt>"VPN"</dt>
            <dd>"Mullvad"</dd>
            <dt>"Mesh"</dt>
            <dd>"Tailscale"</dd>
            <dt>"Email"</dt>
            <dd>"Tuta"</dd>
            <dt>"Messaging"</dt>
            <dd>"Signal"</dd>
        </dl>
    }
}
