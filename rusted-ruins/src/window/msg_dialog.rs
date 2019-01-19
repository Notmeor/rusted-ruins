
use crate::config::UI_CFG;
use crate::game::DoPlayerAction;
use super::commonuse::*;
use super::text_window::TextWindow;
use super::choose_window::ChooseWindow;
use super::winpos::{WindowPos, WindowHPos, WindowVPos};

pub struct MsgDialog {
    text_win: TextWindow,
    choose_win: ChooseWindow,
    action_callback: Box<FnMut(&mut DoPlayerAction, u32) -> DialogResult + 'static>
}

impl MsgDialog {
    pub fn new<F>(msg: &str, choices: Vec<String>, f: F) -> MsgDialog
        where F: FnMut(&mut DoPlayerAction, u32) -> DialogResult + 'static {

        let rect = UI_CFG.msg_dialog.rect.into();
        let text_win = TextWindow::new(rect, msg);
        let winpos = WindowPos::new(
            WindowHPos::RightX(rect.right()),
            WindowVPos::TopMargin(rect.bottom() + UI_CFG.gap_len_between_dialogs));
        MsgDialog {
            text_win,
            choose_win: ChooseWindow::new(winpos, choices, None),
            action_callback: Box::new(f),
        }
    }
    
    pub fn with_yesno<F>(msg: &str, f: F) -> MsgDialog
        where F: FnMut(&mut DoPlayerAction, u32) -> DialogResult + 'static {

        let rect = UI_CFG.msg_dialog.rect.into();
        let text_win = TextWindow::new(rect, msg);
        let winpos = WindowPos::new(
            WindowHPos::RightX(rect.right()),
            WindowVPos::TopMargin(rect.bottom() + UI_CFG.gap_len_between_dialogs));
        MsgDialog {
            text_win,
            choose_win: ChooseWindow::with_yesno(winpos, None),
            action_callback: Box::new(f),
        }
    }
}

impl Window for MsgDialog {
    
    fn draw(&mut self, context: &mut Context, game: &Game, anim: Option<(&Animation, u32)>) {

        self.text_win.draw(context, game, anim);
        let rect = self.text_win.get_rect();
        let winpos = WindowPos::new(
            WindowHPos::RightX(rect.right()),
            WindowVPos::TopMargin(rect.bottom() + UI_CFG.gap_len_between_dialogs));
        self.choose_win.set_winpos(winpos);
        self.choose_win.draw(context, game, anim);
    }
}

impl DialogWindow for MsgDialog {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction) -> DialogResult {
        match *command {
            Command::Cancel => { return DialogResult::Close; }
            _ => (),
        }

        match self.choose_win.process_command(command, pa) {
            DialogResult::CloseWithValue(v) => { // An choice is choosed
                let n = *v.downcast::<u32>().unwrap();
                return (self.action_callback)(pa, n);
            }
            _ => (),
        }
        DialogResult::Continue
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}

