## design.md

### Architecture

* **Scanner**: Script using `nmap` to collect SSID, MAC, and signal strength.
* **Backend**: API server (Node.js/Go) with PostgreSQL for storing scan results.
* **Visualization**: Mapbox heatmap connected to backend API.

### Data Flow

1. Scanner runs → collects device info.
2. Data sent via API → stored in database.
3. Frontend requests data → displays real-time heatmap.

### Data Model (MVP)

* `ScanResult`: { id, ssid, mac, signal_strength, timestamp }
* `Location`: { id, router_name, coordinates }

### Error Handling

* Drop incomplete scan results.
* Mark zones with low confidence if fewer than N devices are detected.