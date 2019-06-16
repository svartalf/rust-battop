use std::rc::Rc;
use std::sync::Arc;

use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;

use super::{Context, Painter, TabBar, View};
use crate::app::Config;
use crate::Result;

#[allow(clippy::redundant_closure)]
pub fn init(config: Arc<Config>, views: Vec<View>) -> Result<Interface<impl Backend>> {
    debug_assert!(!views.is_empty());

    let screen = crossterm::AlternateScreen::to_alternate(true)?;
    let backend = CrosstermBackend::with_alternate_screen(screen)?;
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    let tab_titles = views.iter().map(|view| view.title()).collect::<Vec<_>>();
    let tabs = TabBar::new(tab_titles);

    Ok(Interface {
        config,
        terminal,
        views,
        tabs,
    })
}

/// Interface is a group tabs and tab contents
pub struct Interface<B: Backend> {
    #[allow(unused)]
    config: Arc<Config>,
    terminal: Terminal<B>,
    views: Vec<View>,
    tabs: TabBar,
}

impl<B: Backend> Interface<B> {
    pub fn draw(&mut self) -> Result<()> {
        let context = Rc::new(Context {
            tabs: &self.tabs,
            view: &self.views[self.tabs.index()],
        });
        self.terminal.draw(|frame| {
            Painter::from_context(context.clone()).draw(frame);
        })?;

        Ok(())
    }

    pub fn views_mut(&mut self) -> &mut [View] {
        self.views.as_mut()
    }

    pub fn tabs_mut(&mut self) -> &mut TabBar {
        &mut self.tabs
    }
}

impl<B: Backend> Drop for Interface<B> {
    fn drop(&mut self) {
        let _ = self.terminal.show_cursor();
        let _ = self.terminal.clear();
    }
}
