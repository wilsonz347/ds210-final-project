# ds210-final-project
📈 Bank Transaction Analysis with Rust & Charming

This project explores a large-scale bank transactions dataset (over 1 million rows) to extract trends, detect anomalies, and visualize financial behaviors by month and region. It uses Rust and the charming library for graph generation, with modularized architecture and automated unit tests to ensure code quality.

---
🚀 Project Structure
- models.rs — Defines core structs (Transaction, RegionStats, MonthStats)
- parser.rs — Loads and parses CSV data into structured Rust types
- analysis.rs — Core data analysis: aggregation, anomaly detection, median/percentile calculations
- main.rs — Graph generation, rendering HTML dashboard using Charming

The project is split into clear, reusable modules to maximize maintainability and readability.

---
📚 Dataset
- Source: Massive Bank Dataset ([Kaggle](https://www.kaggle.com/datasets/ksabishek/massive-bank-dataset-1-million-rows))
- Size: ~1,000,000 transactions across multiple domains and cities
- Fields: Date, Domain, Location, Transaction Value, Transaction Count

---
🧠 AI Transparency
- Some portions of the project (e.g., Charming graph debugging, edge case handling) were accelerated with AI assistance (GPT-4o).
- AI was used responsibly for troubleshooting, not for entire code generation.

---
📊 Visual Output
Running the project produces an interactive HTML dashboard that displays:
- Transaction Value by Month (Line Chart)
- Number of Transactions by Month (Line Chart)
- Total Transaction Value by City (Scatter Plot)
- Average Transaction Value by City (Scatter Plot)
- Median Transaction Value by City (Scatter Plot)
- Number of Transactions by City (Scatter Plot)

---
🛠️ How to Run
```bash
# Navigate to the project
cd bank-analysis

# Build and Run
cargo run
```

- This will generate an output file: stats.html
- Open stats.html in any browser to view your charts.

---
🧪 Testing
The project includes unit tests for:
- calculate_median
- percentile
- parsing the csv file

Run tests via:
```bash
cargo test
```

---
📜 License
- This project is for educational purposes.
- Charming is under MIT License.
- Dataset is subject to Kaggle's user data agreements.
