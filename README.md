# Kartvisare - Map Viewer
Kartvisare is a web application designed to display different map layers. The application includes user-specific access controls and idp integration.

## Local development environment
### Prerequisites
Rust (for backend development)
Node.js and npm (for frontend development)
Docker (for local database)

### Database Setup
1. Run the database setup script. 
``/databas/setup_db.sh``

### Backend Setup
1. In the ``/backend`` directory, create a ``.env`` file with the following content:
```
DB_HOST=localhost
DB_PORT=5432
DB_USER=kartvisare
DB_PASSWORD=kartvisare
DB_NAME=kartvisare
REDIS_URL=redis://localhost:6379
```
2. In the ``/backend`` directory, run the following command to start the backend:
``cargo run``

### Frontend
1. In the ``/frontend`` directory, create a ``.env`` file with the following content:
```
VITE_API_BASEURL=http://localhost:3000
```
2. In the ``/frontend`` directory run ``npm install`` to install dependencies
3. In the ``/frontend`` directory run ``npm run dev`` to start the frontend
