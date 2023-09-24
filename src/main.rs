mod components;
use crate::components::*;

use rand::prelude::*;

use leptos::*;

use stylers::style_sheet;

struct Player {
    health: u8,
    strength: u8,
    agility: u8,
    endurance: u8
}

fn determine_attack(power: u8, agility: u8) -> u8 {
    let random_number = rand::thread_rng().gen::<f64>();
    let max_attack = (power + agility) as f64;
    (random_number * max_attack) as u8
}


#[component]
fn App() -> impl IntoView {
    let class_name = style_sheet!("./style.css");

    let (username, set_username) = create_signal("---".to_string());
    let (game_started, set_game_started) = create_signal(false);

    let (player_stats, set_player_stats) = create_signal(Player { health: 100, strength: 20, agility: 10, endurance: 5 });
    let (opponent_stats, set_opponent_stats) = create_signal(Player { health: 100, strength: 20, agility: 10, endurance: 5 });


    let attack = move |_| {
        let player_attack = determine_attack(player_stats.with(|x| x.strength),
                                               player_stats.with(|x| x.agility));
        logging::log!("{}", player_attack);
    };

    let render_game_buttons = move || {
        if game_started() {
            view! { class = class_name,
                <button id="attack-button" on:click=attack>Hit</button>
                <button id="protect-button" onclick="protect();">Block</button>
            }.into_view()
        } else {
            view! {""}.into_view()
        }
    };

    view! { class = class_name,
        <div id="gameZone">
            <div class="titleGame">"MIAMI FIGHT"</div>

            <Introduction game_started=game_started
                          set_game_started=set_game_started
                          set_username=set_username/>
            <div id="player">
	     	<h3><span id="playerNameEnter">{username}</span></h3>
	     	<img src="img/Player.jpg" alt="Your photo goes here" id="imgPlayer"/>
	     	<h4>Health: <span>{move || player_stats.with(|x| x.health)}</span></h4>
	     	<h4>Strength: <span>{move || player_stats.with(|x| x.strength)}</span></h4>
	   		<h4>Agility: <span>{move || player_stats.with(|x| x.agility)}</span></h4>
	        </div>

	        <div id="opponent">
	        <h3>Opponent</h3>
	        <img src="img/Tony.jpg" id="imgOpponent" alt="Тони" id="imgOpponent"/>
            <h4>Health: <span>{move || opponent_stats.with(|x| x.health)}</span></h4>
            <h4>Strength: <span>{move || opponent_stats.with(|x| x.strength)}</span></h4>
	   	    <h4>Agility: <span>{move || opponent_stats.with(|x| x.agility)}</span></h4>
            </div>

		    <br/>
		    <h3><span id="round">Round 1</span></h3>

            {render_game_buttons}

            <button id="next-opponent-button" hidden="true" onclick="nextOpponent();">Next opponent</button>
            <button id="restart-button" hidden="true" onclick="restart();">Restart</button>
            <h3 id="game-message">...</h3>

        </div>
    }

}


fn main() {
    mount_to_body(App)
}