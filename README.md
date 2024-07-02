# Rust Web API Project

This project is a simple web API built with [Axum](https://github.com/tokio-rs/axum) and uses PostgreSQL as the database. The project is structured to be modular, making it easy to manage and extend.

### Project Structure

.

        ├── Cargo.toml
        ├── .env
        ├── src
        │ ├── main.rs
        │ ├── db.rs
        │ ├── handlers.rs
        │ └── models.rs
        └── README.md

- `Cargo.toml`: Contains the project dependencies.
- `.env`: Stores environment variables for database configuration.
- `src/main.rs`: The entry point of the application.
- `src/db.rs`: Handles database connection setup.
- `src/handlers.rs`: Contains the route handlers for the API endpoints.
- `src/models.rs`: Defines the data models and validation logic.

## Setup Instructions

### Prerequisites

- Rust: Install from [rust-lang.org](https://www.rust-lang.org/)
- PostgreSQL: Install from [postgresql.org](https://www.postgresql.org/)
- `dotenv` crate: Used for managing environment variables

### Steps

1. **Clone the repository:**

   ```sh
   git clone https://github.com/your-username/your-repo.git
   cd your-repo

   ```

2. **Create a .env file:**

Create a .env file in the root directory of your project with the following content:

```env
DATABASE_HOST=your-host
DATABASE_USER=your-user
DATABASE_PASSWORD=your-password
DATABASE_NAME=Evergates
```

3.**Database Setup**

Ensure your PostgreSQL database server is running and accessible with the credentials provided in the .env file. Follow these steps to set up the database and table:

Create the Evergates Database:

```sql
CREATE DATABASE Evergates;
```

Connect to the Evergates Database:

Once the database is created, connect to it using:

```sh
psql -U postgres -d Evergates
```

Create the Details Table:

Within the Evergates database, create a table named Details with the following schema:

```sql
CREATE TABLE Details (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    value VARCHAR(100) NOT NULL
);
```

4.**Install dependencies:**

Run the following command to install the project dependencies:

```sh
cargo build
```

5.**Run the application:**

Start the application using:

```sh
cargo run
```

The server will start and listen on 127.0.0.1:3000.

## Testing with Postman

1.**Download and Install Postman:**

If you haven't already, download and install Postman from postman.com.

2.**Create a Postman Collection:**

Open Postman and create a new collection for your API tests.

3.**Add Requests to Postman:**

Add requests to your Postman collection to test your API endpoints:

**POST Request**

Endpoint: http://127.0.0.1:3000/details

Headers:

```bash
Content-Type: application/json
```

Body (raw JSON):

```json
{
  "name": "example name",
  "value": "example value"
}
```

**GET Request**

Endpoint: http://127.0.0.1:3000/details

Method: GET

Send Requests and Inspect Responses:

Click on the "Send" button for each request to send it to your API.
Postman will display the response body and status code.
Verify that the responses match your expected behavior based on the API logic.

4.**Example Screenshots**

Here are example screenshots of how your Postman requests might look:

POST Request in Postman:

```json
"Success"
```

GET Request in Postman:

```json
[
  {
    "name": "example name",
    "value": "example value"
  }
]
```

Notes
Ensure that your Rust web API server (cargo run) is running while you are testing with Postman.

This `README.md` file includes details about the project structure, setup instructions, endpoints, database setup, testing, contributing, and the license. Adjust the URL in the clone command and other details as needed to fit your actual repository.
