use gpui::{div, IntoElement, ParentElement, Render, Styled, ViewContext, VisualContext, WindowContext};
use std::sync::Arc;

use crate::components::{
    // Import the factory type for registration
    editors::level::LevelEditorTabType,
    // Import the content provider to create an instance
    editors::level::LevelEditorContentProvider,
    menu_bar::AppMenuBar, 
    tab_system::{self, TabData}, 
    title_bar::TitleBar
};


pub struct App {
    tab_system: gpui::View<tab_system::TabSystem>,   
}

impl App {
    pub fn new(cx: &mut WindowContext) -> gpui::View<Self> {
        cx.new_view(|cx| {
            // 1. Create the tab system view first.
            let tab_system = tab_system::create_tab_system(cx);
            
            // 2. Register the tab type so it's available in the '+' dropdown.
            tab_system::register_tab_type(Arc::new(LevelEditorTabType));
            
            // 3. Create and add the default tab.
            //    First, create an instance of the content provider.
            let default_content = Arc::new(LevelEditorContentProvider::new("Default Level".to_string()));
            //    Then, create the TabData, adding an icon.
            let default_tab = TabData::new(default_content).with_icon("🎮".to_string());
            //    Finally, add the tab to the system.
            tab_system::TabSystem::add_tab(default_tab);

            // 4. Return the App struct.
            Self { tab_system }
        })
    }
}

impl Render for App {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .children(vec![
                TitleBar::new(cx).into_any_element(),
                AppMenuBar::new(cx).into_any_element(),
                self.tab_system.clone().into_any_element(),
            ])
    }
}
