CREATE TABLE network_scans (
  id INTEGER NOT NULL PRIMARY KEY,
  network_id INTEGER NOT NULL,
  location_id INTEGER NOT NULL,
  FOREIGN KEY (network_id) REFERENCES networks(network_id),
  FOREIGN KEY (location_id) REFERENCES locations(location_id)
)
