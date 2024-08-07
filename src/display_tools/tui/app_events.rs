use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crate::display_tools::tui::app::App;
use crate::display_tools::tui::app_tabs::{BandsSelectorActive, SelectedTab};
use crate::modem_tools::modem::{modem_execute, save_bands_command};

impl App {

    pub(crate) fn switch_bands_selector_lists(&mut self) {
        match self.app_tabs.selected_tab {
            SelectedTab::BandsConfigTab => {
                match self.app_tabs.active_bands_selector {
                    BandsSelectorActive::UMTSBandsSelector => {
                        self.app_tabs.umts_bands_list_state.select(None);
                        self.app_tabs.lte_bands_list_state.select(Some(0));
                        self.app_tabs.active_bands_selector = BandsSelectorActive::LTEBandsSelector
                    },
                    BandsSelectorActive::LTEBandsSelector => {
                        self.app_tabs.lte_bands_list_state.select(None);
                        self.app_tabs.umts_bands_list_state.select(Some(0));
                        self.app_tabs.active_bands_selector = BandsSelectorActive::UMTSBandsSelector
                    },
                }
            }
            _ => {}
        }
    }
    pub(crate) fn handle_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Tab => self.next_tab(),
                    KeyCode::Left => {
                        match self.app_tabs.editing_mode {
                            true => self.move_cursor_left(),
                            false => self.switch_bands_selector_lists()
                        }
                    },
                    KeyCode::Right => {
                        match self.app_tabs.editing_mode {
                            true => self.move_cursor_right(),
                            false => self.switch_bands_selector_lists()
                        }
                    },
                    KeyCode::Esc => {
                        match self.app_tabs.selected_tab {
                            SelectedTab::TerminalTab => {
                                match self.app_tabs.editing_mode {
                                    true => self.app_tabs.editing_mode = false,
                                    false => self.exit()
                                }
                            },
                            _ => self.exit()
                        }
                    },
                    KeyCode::Down => {
                        match self.app_tabs.active_bands_selector {
                            BandsSelectorActive::UMTSBandsSelector => {
                                let current_selection = self.app_tabs.umts_bands_list_state.selected().unwrap_or(0);
                                if current_selection < self.app_tabs.modem_capabilities.spec.unwrap().supported_umts_bands.len() {
                                    self.app_tabs.umts_bands_list_state.select_next()
                                }
                            },
                            BandsSelectorActive::LTEBandsSelector => {
                                let current_selection = self.app_tabs.lte_bands_list_state.selected().unwrap_or(0);
                                if current_selection < self.app_tabs.modem_capabilities.spec.unwrap().supported_lte_bands.len()-1 {
                                    self.app_tabs.lte_bands_list_state.select_next()
                                }
                            }
                        }
                    },
                    KeyCode::Up => {
                        match self.app_tabs.active_bands_selector {
                            BandsSelectorActive::UMTSBandsSelector => self.app_tabs.umts_bands_list_state.select_previous(),
                            BandsSelectorActive::LTEBandsSelector => self.app_tabs.lte_bands_list_state.select_previous(),
                        }
                    },
                    KeyCode::Backspace => {
                        match self.app_tabs.editing_mode {
                            true => self.delete_char(),
                            false => {}
                        }
                    },
                    KeyCode::Char(ch) => {
                        match self.app_tabs.selected_tab {
                            SelectedTab::TerminalTab => {
                                match self.app_tabs.editing_mode {
                                    true => self.enter_char(ch),
                                    false => {
                                        match ch  {
                                            'i' | 'ш' => self.app_tabs.editing_mode = true,
                                            'q' | 'й' => self.exit = true,
                                            _ => {}
                                        }
                                    }
                                }
                            },
                            SelectedTab::BandsConfigTab => {
                                match ch {
                                    'q' | 'й' => self.exit = true,
                                    ' ' => {
                                        match self.app_tabs.active_bands_selector {
                                            BandsSelectorActive::UMTSBandsSelector => {
                                                let current_selection = self.app_tabs.umts_bands_list_state.selected().unwrap_or(0);
                                                let band = self.app_tabs.modem_capabilities.spec.unwrap().supported_umts_bands[current_selection];

                                                if self.app_tabs.config_umts_bands.contains(&band) {
                                                    self.app_tabs.config_umts_bands.retain(|&x| x != band);
                                                } else {
                                                    self.app_tabs.config_umts_bands.push(band);
                                                }
                                            },
                                            BandsSelectorActive::LTEBandsSelector => {
                                                let current_selection = self.app_tabs.lte_bands_list_state.selected().unwrap_or(0);
                                                let band = self.app_tabs.modem_capabilities.spec.unwrap().supported_lte_bands[current_selection];

                                                if self.app_tabs.config_lte_bands.contains(&band) {
                                                    self.app_tabs.config_lte_bands.retain(|&x| x != band);
                                                } else {
                                                    self.app_tabs.config_lte_bands.push(band);
                                                }
                                            }
                                        }
                                    },
                                    _ => {}
                                }
                            }
                            SelectedTab::MonitorTab => {
                                match ch {
                                    'q' | 'й' => self.exit = true,
                                    _ => {}
                                }
                            }
                        }
                    },
                    KeyCode::Enter => {
                        match self.app_tabs.editing_mode {
                            true => {
                                let command = self.app_tabs.terminal_data.input.trim();
                                if command.len() > 0 {
                                    let response = modem_execute(
                                        &self.port_name,
                                        &self.baud_rate,
                                        command
                                    ).unwrap_or_else(|err| {
                                        eprintln!("Error executing modem command: {}", err);
                                        "".to_string()
                                        });
                                    self.app_tabs.terminal_data.input = "".to_string();
                                    self.app_tabs.terminal_data.output.push_str(response.trim());
                                    self.app_tabs.terminal_data.output.push_str("\r\n----------\r\n");
                                }
                            },
                            false => {}
                        }
                    },
                    KeyCode::F(10) => {
                        let save_command  = save_bands_command(self.app_tabs.config_umts_bands.clone(), self.app_tabs.config_lte_bands.clone());
                        let response = modem_execute(&self.port_name, &self.baud_rate, save_command.as_str()).unwrap_or_else(
                            |err| {
                                format!("{:?}", err)
                            }
                        );
                        self.app_tabs.save_bands_command = response;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}