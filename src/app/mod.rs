use anyhow::Result;

#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

mod action;
use action::Action;

mod app_model;
use app_model::AppModel;

mod updater;
use updater::update;
mod viewer;

use ratatui::crossterm::event as xEvent;
use ratatui::prelude::*;

//  //  //  //  //  //  //  //
pub fn run(terminal: &mut ratatui::Terminal<impl Backend>) -> Result<()> {
    trace!(" -> app::run()");
    let mut model = AppModel::new()?;

    while !model.is_exiting() {
        // DRAW
        terminal.draw(|frame| viewer::view(&mut model, frame.area(), frame.buffer_mut()))?;

        // UPDATE
        //      get inputs
        let raw_inputs = collect_events()?;
        check_terminate_sequence(&raw_inputs)?;
        //      updating loop
        for event in raw_inputs {
            invoke_update_loop(Action::TranslateRawEvent(event), &mut model)?;
        }
        invoke_update_loop(Action::UpdateTimer, &mut model)?;
    }
    trace!("normal exit");
    Ok(())
}

fn invoke_update_loop(first_action: Action, model: &mut AppModel) -> Result<()> {
    let mut current_action = first_action;
    while current_action != Action::Noop {
        current_action = update(model, &current_action)?;
    }
    Ok(())
}

//  //  //  //  //  //  //  //
fn check_terminate_sequence(events: &Vec<xEvent::Event>) -> Result<()> {
    for event in events {
        match event {
            xEvent::Event::Key(key) => {
                if key.modifiers.contains(xEvent::KeyModifiers::CONTROL) {
                    // <C-c>
                    if key.code == xEvent::KeyCode::Char('c') {
                        let msg = "exiting by <C-c>";
                        warn!("{}", msg);
                        return Err(anyhow::anyhow!(msg));
                    }
                    // <C-e>
                    if key.code == xEvent::KeyCode::Char('x') {
                        let msg = "exiting with TEST error by <C-x>";
                        error!("{}", msg);
                        return Err(anyhow::anyhow!(msg));
                    }
                    // <C-p>
                    if key.code == xEvent::KeyCode::Char('p') {
                        panic!("TEST panic by <C-p>");
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}

//  //  //  //  //  //  //  //
static POLL_WAIT_TIME: std::time::Duration = std::time::Duration::from_millis(16); //from_secs(0);
fn collect_events() -> Result<Vec<xEvent::Event>> {
    let mut result = Vec::new();
    while xEvent::poll(POLL_WAIT_TIME)? {
        result.push(xEvent::read()?);
    }
    Ok(result)
}
