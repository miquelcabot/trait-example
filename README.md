# Modular Balances & Voting System in Rust using Traits

This project is a step-by-step modular implementation of two core blockchain primitives: a **balances module** (for handling token transfers) and a **voting module** (for casting binary votes). The design gradually evolves from hardcoded implementations to fully generic and configurable modules.

---

## 📦 Project Structure

The project consists of five steps, each introducing more abstraction and flexibility:

| Step | File     | Description |
|------|----------|-------------|
| 1    | `step1.rs` | Basic balances module using fixed `u32` types for accounts and balances. |
| 2    | `step2.rs` | Uses `type` aliases for accounts and balances (`u16`, `u32`). |
| 3    | `step3.rs` | Introduces a voting module with fixed types. |
| 4    | `step4.rs` | Generic implementation of balances and voting modules with parameterized types. |
| 5    | `step5.rs` | Fully configurable runtime using a `Config` trait, similar to the Substrate framework. |

---

## 🚀 Running the Project

### 1. Install Rust

If you haven't already:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Clone the Repo

```bash
git clone https://github.com/miquelcabot/trait-example.git
cd trait-example
```
  
### 3. Run Tests

```bash
cargo test
```

This will execute unit tests defined in `main.rs` that cover the functionality of each step.

## 🧪 Example Test Cases

* Set and retrieve balances.
* Transfer funds between accounts.
* Cast and query votes.
* Ensure compatibility and correctness of generic modules.
