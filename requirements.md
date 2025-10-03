## requirements.md

### User Stories (EARS format)

* WHEN a scan runs on a library router
  THE SYSTEM SHALL record connected devices (SSID, MAC, signal strength).

* WHEN scan results are stored
  THE SYSTEM SHALL update the device count for that location in real time.

* WHEN data is viewed
  THE SYSTEM SHALL display a heatmap showing density per zone.

* WHEN a hotspot or irrelevant device is detected
  THE SYSTEM SHALL filter it out using MAC vendor checks.

* WHEN data is older than the defined expiry period
  THE SYSTEM SHALL delete it automatically.