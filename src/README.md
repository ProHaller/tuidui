# Tuidui

**Tuidui** is a life gamification to-do app designed to boost productivity and personal growth by making task management an engaging and rewarding experience. With Tuidui, users can track their tasks, earn rewards, and level up their personal character, all while enjoying the simplicity of a terminal-based interface. Designed for a close-knit group, Tuidui combines personalized AI insights, voice interaction, and gamified motivation to help you get things done.

## Features

- **Voice Interaction Mode** (Planned): Talk to Tuidui to add and manage your tasks seamlessly using voice commands.
- **CLI Quick Logging** (Planned): Quickly log tasks and manage your to-do list using a command-line interface powered by `clap`.
- **AI-Driven Task Breakdown** (Planned): Break down complex tasks into manageable steps, making productivity more approachable.
- **Reward System** (Planned): Earn experience points and level up your character as you complete tasks.
- **Character Sheet** (Planned): Track your progress and visualize your personal growth through an evolving character sheet.
- **Calendar Integration** (Planned): Sync your tasks with your calendar to better manage your time.
- **Obsidian Integration** (Planned): Import tasks from your Obsidian notes automatically, keeping everything in one place.

## Project Structure

Tuidui's codebase is organized in a modular manner to ensure scalability and maintainability:

- **Input Module**: Handles voice input and CLI interactions.
- **Task Management Module**: Manages task creation, editing, deletion, and breakdown.
- **Rewards Module**: Manages experience points and character leveling.
- **Character Sheet Module**: Displays user progress and skill development.
- **UI Module**: Provides the terminal interface for interacting with Tuidui.
- **Logging Module**: Tracks and logs important app activities for debugging and feedback.
- **Configuration Module**: Manages user preferences and app settings.

## Getting Started

### Prerequisites

- **Rust**: Ensure that Rust is installed on your system. You can download it from [rust-lang.org](https://www.rust-lang.org/).
- **Cargo**: Cargo should be installed along with Rust to manage dependencies.

### Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/yourusername/tuidui.git
   cd tuidui
   ```

2. Build the project:

   ```sh
   cargo build --release
   ```

3. Run Tuidui:
   ```sh
   cargo run
   ```

### Usage

- **Voice Mode**: Start a voice session with Tuidui and let it understand your tasks in a conversational way.
- **CLI Mode**: Use the command-line mode to quickly add and log tasks. Type `--help` for available commands.

### Configurations

You can modify the configuration file to customize Tuidui's settings. The configuration file can be found at `~/.tuidui/config.json` after the first run.

## Contributing

We welcome contributions from everyone! Here are some ways you can help:

1. **Report Issues**: If you find a bug or have a feature request, please create an issue on GitHub.
2. **Submit Pull Requests**: Feel free to fork the repository and submit pull requests for any improvements or bug fixes.
3. **Feature Suggestions**: Have an idea for a cool feature? Open an issue, and let's discuss it!

## Roadmap

- **Calendar Integration**: Sync tasks with user calendars for easier planning.
- **Periodic AI Check-ins**: Customize check-ins based on user preferences to keep tasks tailored to individual needs.
- **Obsidian Integration**: Automatically add tasks from Obsidian notes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

## Contact

For any inquiries, reach out to the project maintainer at `your.email@example.com`.
