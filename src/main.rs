use swf_monitor::app::{App, AppResult};
use swf_monitor::event::{Event, EventHandler};
use swf_monitor::handler::handle_key_events;
use swf_monitor::tui::Tui;
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use std::path::Path;
use swf::workflow::{Workflow, load_workflows};

fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;

    println!("{:#?}", app.workflows);

    test_lib();

    Ok(())
}

fn test_lib() {
    // let yml_path = Path::new("./data/wf1/workflow.yaml");
    // let wf = Workflow::read_yml_file(&yml_path);
    // println!("{:#?}", wf);
    // println!("{:#?}", wf.get_step_logs(1));
    // println!("{:#?}", wf.get_step_logs(2));
    // println!("{:#?}", wf.get_step_logs(3));
    // println!("{:#?}", wf.get_step_logs(4));
    let wfs = load_workflows(Path::new("./data/"));
    if let Ok(wf) = wfs {
        for w in wf {
            println!("{:#?}", w.name);
        }
    }
    // println!("{:#?}", wfs);
}
