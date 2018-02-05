
use sdl2::rect::Rect;
use config::UI_CFG;
use super::commonuse::*;
use super::textwindow::TextWindow;
use super::choosewindow::ChooseWindow;
use super::winpos::{WindowPos, WindowHPos, WindowVPos};
use text;

pub struct ExitWindow {
    text_win: TextWindow,
    choose_win: ChooseWindow,
}

impl ExitWindow {
    pub fn new() -> ExitWindow {
        let rect: Rect = UI_CFG.exit_window.rect.into();
        let text_win = TextWindow::new(rect, text::ui_txt("dialog.exit"));
        let winpos = WindowPos::new(
            WindowHPos::RightX(rect.right()),
            WindowVPos::TopMargin(rect.bottom() + UI_CFG.gap_len_between_dialogs));
        ExitWindow {
            text_win: text_win,
            choose_win: ChooseWindow::with_yesno(winpos, None),
        }
    }
}

impl Window for ExitWindow {
    
    fn redraw(
        &mut self, canvas: &mut WindowCanvas, game: &Game, sv: &mut SdlValues,
        anim: Option<(&Animation, u32)>) {

        self.text_win.redraw(canvas, game, sv, anim);
        let rect = self.text_win.get_rect();
        let winpos = WindowPos::new(
            WindowHPos::RightX(rect.right()),
            WindowVPos::TopMargin(rect.bottom() + UI_CFG.gap_len_between_dialogs));
        self.choose_win.set_winpos(winpos);
        self.choose_win.redraw(canvas, game, sv, anim);
    }
}

impl DialogWindow for ExitWindow {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction) -> DialogResult {
        match *command {
            Command::Cancel => {
                return DialogResult::Close;
            }
            _ => (),
        }
        
        match self.choose_win.process_command(command, pa) {
            DialogResult::CloseWithValue(v) => { // An choice is choosed
                let n = *v.downcast::<u32>().unwrap();
                match n {
                    0 => { return DialogResult::Quit }
                    1 => { return DialogResult::Close }
                    _ => panic!(),
                }
            }
            _ => (),
        }
        return DialogResult::Continue;
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}


