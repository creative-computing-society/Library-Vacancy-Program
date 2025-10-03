# Library Seat Vacancy Tracker

## Overview

This project estimates how crowded the library is using Wi-Fi data and mobile devices. It collects information from routers and user phones, then shows real-time seat availability on a heatmap.

---

## Goals

* Count devices connected to library Wi-Fi.
* Predict crowd density and trends.
* Visualize occupancy on a live map.
* Keep data private (anonymized, auto-deleted).

---

## How It Works

1. **Wi-Fi Scans**

   * Use `nmap` to find devices per router.
   * Switch between SSIDs to cover all areas.
   * Send results (SSID, MAC, signal strength) to backend.

2. **Mobile Data (Optional)**

   * Android/iOS app records connected SSID + background location.
   * Data is hashed, timestamped, and synced to backend.
   * Helps validate Wi-Fi scans.

3. **Filtering & Accuracy**

   * Ignore personal hotspots.
   * Add confidence scores where data is sparse.

---

## Outputs

* Real-time device count per zone.
* Heatmap of library occupancy.
* Occupancy confidence level.

---

## Tech Stack

* **Mobile App:** React Native (Expo, TypeScript)
* **Backend:** Node.js/Go + PostgreSQL
* **Scanning:** nmap, Wi-Fi scripts
* **Visualization:** Mapbox

---

## Next Steps

* Phase 1: Basic Wi-Fi scanning + backend storage
* Phase 2: Add mobile app for extra data
* Phase 3: Improve filtering and confidence