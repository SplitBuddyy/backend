# TripSplit
"TripSplit: Effortlessly track and share expenses with friends on every adventure."

## Project Overview

TripSplit is a Rust-based application designed to help users track and share expenses with friends during trips. It provides a backend server using the Axum framework and a front-end interface built with Svelte.

## Getting Started

### Prerequisites

- Rust and Cargo installed on your system.
- Docker (optional, for containerized deployment).
- Node.js and npm (for front-end development).

### Building and Running the Backend

1. **Clone the repository:**

   ```bash
   git clone <repository-url>
   cd TripSplit
   ```

2. **Build the project:**

   ```bash
   cargo build --release
   ```

3. **Run the server:**

   ```bash
   cargo run --release
   ```

   The server will start on `http://localhost:3000`.

### Testing the Backend with Curl

- **Create a User:**

  ```bash
  curl -X POST http://127.0.0.1:3000/user/create_user \
       -H "Content-Type: application/json" \
       -d '{"name": "JohnDoe", "email": "john@example.com", "password": "password123"}'
  ```

- **Get Users:**

  ```bash
  curl -X GET http://127.0.0.1:3000/user/get_users
  ```

- **Create a Group:**

  ```bash
  curl -X POST http://127.0.0.1:3000/group/create_group \
       -H "Content-Type: application/json" \
       -d '{"name": "TripGroup", "owner": 1}'
  ```

- **Add Expense:**

  ```bash
  curl -X POST http://127.0.0.1:3000/group/add_expense \
       -H "Content-Type: application/json" \
       -d '{"group_info": {"owner": 1, "group_id": 1}, "expense": {"id": 1, "description": "Dinner", "amount": 100.0, "payer": {"id": 1, "name": "JohnDoe", "email": "john@example.com", "password": "password123"}, "participants": [{"id": 1, "name": "JohnDoe", "email": "john@example.com", "password": "password123"}], "date": "2023-10-01"}}'
  ```

### Frontend Development

1. **Navigate to the front-end directory:**

   ```bash
   cd mocked_front
   ```

2. **Install dependencies:**

   ```bash
   npm install
   ```

3. **Run the development server:**

   ```bash
   npm run dev
   ```

   The front-end will be available at `http://localhost:5000`.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
