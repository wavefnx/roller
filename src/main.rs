use clap::Parser;
use crossterm::event::{self, Event as CEvent, KeyCode};
use eventsource_client::SSE;
use futures::TryStreamExt;
use roller::{Client, Config, Tui};
use std::{error::Error, time::Duration};

// Currently we keep it single-threaded, since there's not much we get from multi.
// In the future, data processing could be offloaded to a separate thread.
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    // Parse the command-line configuration
    let config = Config::parse();
    // Create a new client instance with the specified API endpoint, or default
    let client = Client::new(config.api_endpoint);

    // Retrieve the list of networks from the API, this will give us the metadata.
    // At this point, the Data field of the Network struct will be `None` and will be populated later on through SSE.
    let networks = client.get_networks().await?;
    // Create a new Tui instance with the retrieved network metadata.
    let mut tui = Tui::new(networks);

    // Get the SSE Event Stream from the API.
    let mut stream = client.get_stream().await?;

    // Enter the terminal mode
    let mut terminal = roller::Terminal::enter()?;

    // Start the main event loop
    while let Ok(event) = stream.try_next().await {
        // Check if an event is received

        if let Some(SSE::Event(event)) = event {
            // Update the networks data in the Tui
            tui.update_networks(event);
            // Redraw the Tui on the terminal
            terminal.draw(|f| tui.render(f))?;
        }

        // This serves two purposes:
        // 1. Checking for user input
        // 2. Blocking the thread, lowering the interval in which we process events.
        //
        // In a multi-thread scenario, we can collect and process multiple events while waiting
        if crossterm::event::poll(Duration::from_millis(config.interval_ms))? {
            // Read the user input event
            if let CEvent::Key(key) = event::read()? {
                match key.code {
                    // Break the loop and exit if 'q' is received
                    KeyCode::Char('q') => break,
                    // For any other key, pass it to the Tui for handling
                    _ => tui.handle_input(key.code),
                }
            }
        }
    }

    // Exit the terminal mode and restore the previous terminal state
    terminal.exit()?;

    Ok(())
}
