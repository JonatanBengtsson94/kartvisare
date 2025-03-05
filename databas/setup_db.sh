#!/bin/bash

echo "Copying sql script to container..."
sudo docker cp ./init.sql kartvisare_pg:/tmp/init.sql

echo "Running script..."
sudo docker exec -it kartvisare_pg psql -U kartvisare -d kartvisare -f /tmp/init.sql

echo "Removing script from container..."
sudo docker exec -it kartvisare_pg rm /tmp/init.sql

echo "Database setup complete!"
