use crossterm::event::{Event, KeyCode};

use crate::player::Player;

pub fn handle_input(event: Event, p1: &mut Player, p2: &mut Player) {
    handler_user_1(&event, p1);
    handler_user_2(&event, p2);
}

fn handler_user_1(event: &Event, player: &mut Player) {
    if (event == &Event::Key(KeyCode::Char('s').into())) {
        if (player.y + player.height < player.gameheight) {
            player.y += 1;
        }
    } else if (event == &Event::Key(KeyCode::Char('w').into())) {
        if (player.y > 0) {
            player.y -= 1;
        }
    }
}

fn handler_user_2(event: &Event, player: &mut Player) {
    if event == &Event::Key(KeyCode::Down.into()) {
        if (player.y + player.height < player.gameheight) {
            player.y += 1;
        }
    } else if event == &Event::Key(KeyCode::Up.into()) {
        if (player.y > 0) {
            player.y -= 1;
        }
    }
}
 