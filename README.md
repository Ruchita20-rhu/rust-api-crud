# Rust API with MongoDB CRUD Operations

This project is a simple REST API built using **Actix-web** and **MongoDB**. The API allows you to perform CRUD operations (Create, Read, Update, Delete) on items stored in a MongoDB database. 

## Features

- **Create an item**: Allows the user to add a new item to the database.
- **Get all items**: Retrieves a list of all items from the database.
- **Update an item**: Modifies the description of an existing item based on its name.
- **Delete an item**: Deletes an item from the database based on its name.

## Technologies Used:
- **Backend**: Rust (Actix Web)
- **Database**: MongoDB (Local)
- **Tools**: 
  - **MongoDB Compass** (for managing MongoDB database)
  - **Postman** (for testing API endpoints)

## Setup

### Prerequisites

1. **Install Rust**: Ensure that you have **Rust** installed on your system. If not, you can install it from [here](https://www.rust-lang.org/tools/install).

2. **Install MongoDB**: 
   - Download and install **MongoDB** on your local machine from the [MongoDB website](https://www.mongodb.com/try/download/community).
   - After installation, run MongoDB with the following command:
     ```bash
     mongod
     ```

3. **Install MongoDB Compass**: MongoDB Compass is a graphical interface to interact with your MongoDB instance. You can download it from [here](https://www.mongodb.com/try/download/compass).

### Running MongoDB Locally

- **Start MongoDB**: 
    After installing MongoDB, run the following command to start the MongoDB service on your local machine:
    ```bash
    mongod
    ```

    By default, MongoDB will run on `mongodb://localhost:27017`.

- **Connect using MongoDB Compass**: 
    1. Open **MongoDB Compass**.
    2. In the **Connection String** field, enter:
        ```
        mongodb://localhost:27017
        ```
    3. Click **Connect** to connect to your local MongoDB instance.

### Build and Run the API Server

1. **Clone the repository**:
    To start, clone this repository to your local machine:
```bash
git clone https://github.com/Ruchita20-rhu/rust-api-crud.git
cd rust-api-crud
    ```

2. **Set up environment variables**:
    Create a `.env` file in the root directory and add your MongoDB connection URI like so:
    ```
    MONGO_URI=mongodb://localhost:27017
    ```

3. **Build the project**:
    Use the following command to build the project:
    ```bash
    cargo build
    ```

4. **Run the application**:
    Start the server with:
    ```bash
    cargo run
    ```

    The server will start and bind to `127.0.0.1:8080`.

## API Endpoints

- **POST /create**: Create a new item
    - Request body:
        ```json
        {
            "name": "item_name",
            "description": "item_description"
        }
        ```
    - Response: 
        - Status 201 Created if successful.
        - Status 500 Internal Server Error if failed.

- **GET /get**: Retrieve all items
    - Response: 
        - Status 200 OK with a list of items in JSON format.
        - Status 500 Internal Server Error if failed.

- **PUT /update**: Update an item description
    - Request body:
        ```json
        {
            "name": "item_name",
            "description": "new_description"
        }
        ```
    - Response: 
        - Status 200 OK if successful.
        - Status 404 Not Found if item does not exist.
        - Status 500 Internal Server Error if failed.

- **DELETE /delete**: Delete an item
    - Request body:
        ```json
        {
            "name": "item_name"
        }
        ```
    - Response:
        - Status 200 OK if successful.
        - Status 404 Not Found if item does not exist.
        - Status 500 Internal Server Error if failed.

## Testing the API with Postman

1. **Create an item**:
    - Open **Postman** and set the method to **POST**.
    - Enter the following URL:
        ```
        http://127.0.0.1:8080/create
        ```
    - Set the **Body** to **raw** and **JSON**:
        ```json
        {
            "name": "Item1",
            "description": "Description of Item1"
        }
        ```
    - Click **Send** to create the item.

2. **Get all items**:
    - Set the method to **GET**.
    - Enter the following URL:
        ```
        http://127.0.0.1:8080/get
        ```
    - Click **Send** to retrieve the list of items.

3. **Update an item**:
    - Set the method to **PUT**.
    - Enter the following URL:
        ```
        http://127.0.0.1:8080/update
        ```
    - Set the **Body** to **raw** and **JSON**:
        ```json
        {
            "name": "Item1",
            "description": "Updated Description"
        }
        ```
    - Click **Send** to update the item.

4. **Delete an item**:
    - Set the method to **DELETE**.
    - Enter the following URL:
        ```
        http://127.0.0.1:8080/delete
        ```
    - Set the **Body** to **raw** and **JSON**:
        ```json
        {
            "name": "item1"
        }
        ```
    - Click **Send** to delete the item.

## Troubleshooting

- Ensure that **MongoDB** is running on your local machine or update the `MONGO_URI` in the `.env` file if using a remote MongoDB instance.
    ```


