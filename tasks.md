## tasks.md

### Implementation Plan

1. **Scanning**

   * Write script to run `nmap` and parse connected devices.
   * Add SSID switching logic for multiple routers.

2. **Backend**

   * Create API endpoints: `/upload-scan`, `/get-heatmap-data`.
   * Store results in PostgreSQL with timestamps.
   * Auto-expire data after X hours.

3. **Visualization**

   * Integrate Mapbox.
   * Show heatmap based on device counts per location.

4. **Filtering**

   * Add MAC vendor prefix check to ignore personal hotspots.
   * Apply basic confidence scoring (low, medium, high).

---

**MVP Output:**

* Real-time device count per router.
* Heatmap of library occupancy.
* Basic filtering for false positives.