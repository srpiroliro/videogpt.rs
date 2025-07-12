# YouTube Summarizer

A Rust command-line tool that automatically generates structured summaries of YouTube videos using AI. The tool extracts video transcripts and processes them with Anthropic's Claude AI to create comprehensive, actionable guides.

## Features

- **YouTube Transcript Extraction**: Automatically fetches transcripts from YouTube videos using the Supadata API
- **AI-Powered Summarization**: Uses Anthropic's Claude Sonnet 4 to generate structured, actionable summaries
- **Structured Output**: Creates well-formatted Markdown summaries with:
  - TL;DR overview
  - Key takeaways with actionable items
  - Step-by-step playbooks
  - Interesting nuggets and insights
  - Glossary of technical terms
  - Suggested follow-up tasks
- **Flexible Output**: Display results in terminal or save to file
- **Professional Format**: Generates business/marketing-focused summaries suitable for technical documentation

## Prerequisites

Before using this tool, you'll need:

1. **Supadata API Key**: Sign up at [Supadata](https://supadata.ai) to get an API key for YouTube transcript extraction
2. **Anthropic API Key**: Get an API key from [Anthropic](https://console.anthropic.com) to access Claude AI
3. **Rust**: Install Rust from [rustup.rs](https://rustup.rs/)

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd yt-summarizer
```

2. Create a `.env` file in the project root:
```env
SUPADATA_KEY=your_supadata_api_key_here
ANTHROPIC_KEY=your_anthropic_api_key_here
```

3. Build the project:
```bash
cargo build --release
```

## Usage

### Basic Usage

Summarize a YouTube video and display the result in the terminal:

```bash
cargo run -- "https://www.youtube.com/watch?v=VIDEO_ID"
```

### Save to File

Save the summary to a file:

```bash
cargo run -- "https://www.youtube.com/watch?v=VIDEO_ID" --output summary.md
```

### Command Line Options

- `video_url`: The YouTube video URL to summarize (required)
- `-o, --output <FILE>`: Save the summary to a file instead of displaying in terminal

### Example

```bash
# Summarize a video and save to file
cargo run -- "https://www.youtube.com/watch?v=dQw4w9WgXcQ" -o my_summary.md

# Summarize and display in terminal
cargo run -- "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
```

## Output Format

The tool generates a structured Markdown summary with the following sections:

1. **TL;DR**: Quick 3-sentence overview
2. **Key Takeaways**: 5-12 most important points with actionable items marked
3. **Actionable Playbook**: Step-by-step implementation instructions
4. **Interesting Nuggets**: Additional insights and context
5. **Glossary**: Definitions of technical terms and jargon
6. **Suggested Follow-Up Tasks**: Action items for further exploration

## Dependencies

- **reqwest**: HTTP client for API requests
- **tokio**: Async runtime
- **serde**: JSON serialization/deserialization
- **clap**: Command-line argument parsing
- **anthropic-ai-sdk**: Anthropic Claude AI integration
- **anyhow**: Error handling
- **dotenv**: Environment variable loading

## Architecture

The project consists of three main modules:

- **main.rs**: CLI interface and application entry point
- **supdata.rs**: Integration with Supadata API for transcript extraction
- **video_gpt.rs**: Anthropic Claude AI integration for summarization

## Error Handling

The application will panic and display helpful error messages if:
- Required environment variables are not set
- API keys are invalid
- Video URL is inaccessible or doesn't have transcripts
- Network requests fail

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

[Add your license information here]

## Acknowledgments

- [Supadata](https://supadata.ai) for YouTube transcript extraction API
- [Anthropic](https://anthropic.com) for Claude AI capabilities 