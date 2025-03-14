# Kartvisare
En webapp för att visa olika kartlager

## Lokalmiljö
### Databas
Kör /databas/setup_db.sh

### Backend
Skapa en .env fil under /backend med följande innehåll:
```
DB_HOST=localhost
DB_PORT=5432
DB_USER=kartvisare
DB_PASSWORD=kartvisare
DB_NAME=kartvisare
```
Starta applikationen genom att köra ``cargo run`` i /backend

### Frontend
Skapa en .env fil under /frontend med följande innehåll:
```
VITE_API_BASEURL=http://localhost:3000
```
Starta frontend genoma att köra ``npm run dev`` i /frontend
