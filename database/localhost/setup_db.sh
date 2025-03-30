#!/bin/bash

echo "Starting container"
sudo docker compose up -d

echo "Copying sql script to container..."
sudo docker cp ../init.sql kartvisare_pg:/tmp/init.sql
sudo docker cp ./testdata.sql kartvisare_pg:/tmp/testdata.sql

echo "Running script..."
sudo docker exec kartvisare_pg psql -U kartvisare -d kartvisare -f /tmp/init.sql 
sudo docker exec kartvisare_pg psql -U kartvisare -d kartvisare -f /tmp/testdata.sql

echo "Removing script from container..."
sudo docker exec kartvisare_pg rm /tmp/init.sql /tmp/testdata.sql

echo "Database setup complete!"
