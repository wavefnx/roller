use crate::{network::Data, Network};
use crossterm::event::KeyCode;
use eventsource_client::Event;
use ratatui::style::palette::tailwind::SLATE;
use ratatui::{prelude::*, widgets::*};

/// Represents the sorting strategies for the network table, currently only `DESC`.
#[derive(Debug, Default)]
pub enum SortingStrategy {
    /// Sort by Gas Per Second (GPS).
    #[default]
    Gps,
    /// Sort by Transactions Per Second (TPS).
    Tps,
    /// Sort by Data Per Second (DPS).
    Dps,
}

// Currently since we're working with one screen, we can keep things more compact.
// Later on we can split this struct into multiple ones.
//
/// Represents the Text-based User Interface (TUI) for displaying network information.
pub struct Tui {
    /// The list of networks to display in the table.
    pub networks: Vec<Network>,
    /// {unstable} The current selection state of the table.
    pub selected: TableState,
    /// The current sorting strategy for the network table. GPS is default.
    pub sorting_strategy: SortingStrategy,
    ///  Flag to check if the info bar has been rendered to avoid re-rendering.
    pub info_rendered: bool,
}

impl Tui {
    // Offload constant values to the struct, so they aren't computed on runtime.
    const TABLE_STYLE: Style = Style::new().fg(SLATE.c400);

    const TABLE_HIGHLIGHT_STYLE: Style = Style::new().bg(SLATE.c500).fg(SLATE.c900);

    const TABLE_WIDTHS: [Constraint; 8] = [
        Constraint::Percentage(15),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(15),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
    ];

    /// Creates a new instance of the Tui.
    ///
    /// The Tui is initialized with the provided list of networks, an empty table selection state set to the first row,
    /// and the default sorting strategy (GPS).
    ///
    /// ### Arguments
    /// * `networks` - The list of networks to display in the table.
    ///
    /// ### Returns
    /// A new instance of the Tui.
    pub fn new(networks: Vec<Network>) -> Self {
        Self {
            networks,
            selected: TableState::new().with_selected(0),
            sorting_strategy: SortingStrategy::default(),
            info_rendered: false,
        }
    }

    /// Updates the networks data based on the incoming event.
    ///
    /// This method searches for the network in the `networks` list that matches the `event_type`
    /// of the incoming event.If a matching network is found, its data is updated with the data
    /// from the event. After updating the network data, the `sort_networks` method is called to
    /// re-sort the networks based on the current sorting strategy.
    ///
    /// A`HashMap` could be used for faster lookups, although in this case we'd have to `collect()`
    /// each time before rendering or sorting the networks.
    ///
    /// ### Arguments
    /// * `event` - The incoming SSE event containing the network data to update.
    pub fn update_networks(&mut self, event: Event) {
        if let Some(network) = self
            .networks
            .iter_mut()
            .find(|n| n.name == event.event_type)
        {
            // parse the string data into the Data struct
            let data = serde_json::from_str(&event.data).unwrap_or_default();
            // and update the network
            network.update_data(data);
        }

        self.sort_networks();
    }

    /// Sorts the networks based on the current sorting strategy.
    ///
    /// This method uses the `sort_by` function to sort the `networks` list based on the selected
    /// sorting strategy. The sorting is performed in descending order.
    fn sort_networks(&mut self) {
        self.networks.sort_by(|a, b| {
            let default = Data::default();
            let a_data = a.data.as_ref().unwrap_or(&default);
            let b_data = b.data.as_ref().unwrap_or(&default);

            let (a, b) = match self.sorting_strategy {
                SortingStrategy::Gps => (a_data.gps, b_data.gps),
                SortingStrategy::Tps => (a_data.tps, b_data.tps),
                SortingStrategy::Dps => (a_data.dps, b_data.dps),
            };

            b.partial_cmp(&a).unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    /// Handles user input related to the TUI functionality.
    /// The `char` 'q' is handled in the main loop for exiting.`
    ///
    /// This method responds to user input events to navigate the network table and change the
    /// sorting strategy. The supported navigation keys are:
    /// - Up arrow: Move the selection to the previous row.
    /// - Down arrow: Move the selection to the next row.
    ///
    /// The supported sorting strategy keys are:
    /// - 'g': Sort by Gas Per Second (GPS).
    /// - 't': Sort by Transactions Per Second (TPS).
    /// - 'k': Sort by Data Per Second (DPS).
    ///
    /// ### Arguments
    /// * `key` - The key code of the user input event.
    pub fn handle_input(&mut self, key: KeyCode) {
        if let Some(current) = self.selected.selected() {
            let network_size = self.networks.len().saturating_sub(1);
            match key {
                KeyCode::Up => {
                    self.selected
                        .select(Some(current.saturating_sub(1).min(network_size)));
                }
                KeyCode::Down => {
                    self.selected
                        .select(Some(current.saturating_add(1).min(network_size)));
                }
                KeyCode::Char('g') => {
                    self.sorting_strategy = SortingStrategy::Gps;
                    self.sort_networks();
                }
                KeyCode::Char('t') => {
                    self.sorting_strategy = SortingStrategy::Tps;
                    self.sort_networks();
                }
                KeyCode::Char('k') => {
                    self.sorting_strategy = SortingStrategy::Dps;
                    self.sort_networks();
                }
                _ => {}
            }
        }
    }

    /// Renders the network table and the info using the provided `Frame`.
    /// The layout is split vertically into two chunks: the network table and the information bar.
    ///
    /// The network table displays the list of networks with their corresponding data while the
    /// information bar displays the available user actions, such as quitting the application
    /// and changing the sorting strategy.
    ///
    /// ### Arguments
    /// * `f` - The `Frame` to render the TUI.
    pub fn render(&mut self, f: &mut Frame) {
        // Create the main layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100), Constraint::Min(3)])
            .split(f.size());

        // Split the layout into two chunks: the network table and the info bar
        let (network_layout, info_layout) = (chunks[0], chunks[1]);

        // Render the network table
        self.network_table(f, network_layout);
        self.render_info_bar(f, info_layout);
    }

    fn network_table(&mut self, f: &mut Frame, area: Rect) {
        // Create the network table block
        let network_block = Block::default()
            .borders(Borders::ALL)
            .style(Self::TABLE_STYLE)
            .border_type(BorderType::Rounded);

        // Generate and collect all rows for the network table
        let row_data = self.networks.iter().map(Network::to_row);

        // Initiate the Header row of the table
        let row_data_header = Row::new(vec![
            Cell::from("Network"),
            Cell::from("Block"),
            Cell::from("TPS"),
            Cell::from("MGas/s"),
            Cell::from("KB/s"),
            Cell::from("Stack"),
            Cell::from("DA"),
            Cell::from("Settlement"),
        ]);

        // Create the table widget
        let table = Table::new(row_data, &Self::TABLE_WIDTHS)
            .block(network_block)
            .header(row_data_header)
            .highlight_style(Self::TABLE_HIGHLIGHT_STYLE)
            .highlight_symbol(">> ");

        // Render it with access to the state, which allows to move through entries
        f.render_stateful_widget(table, area, &mut self.selected);
    }

    fn render_info_bar(&self, f: &mut Frame, area: Rect) {
        // Create the info bar block
        let info_block = Block::default()
            .borders(Borders::ALL)
            .style(Self::TABLE_STYLE)
            .border_type(BorderType::Rounded);

        // Create the info bar text
        let info_text = Paragraph::new(
            "[q] quit | sort: ([g] gas per second | [t] txs per second [k] kb per second)",
        )
        .alignment(Alignment::Center)
        .block(info_block);

        // Render the info bar
        f.render_widget(info_text, area);
    }
}
