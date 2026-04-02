# CHANGELOG

## [1.2.0]

### Changed
- Credentials are now sent via HTTP Basic Auth header instead of URL.
- Single JSON parsing instead of double.

### Added
- Added Mobile ID authorization support: `send_mobile_id`, `mobile_id_status`, `verify_mobile_id`.
- HTTP timeouts: 30s request, 10s connect.
- Response size limit (10 MB).
- Custom gate URL support via `url_gate` parameter.

### Removed
- Unused `url` dependency.
- Unused `serde/derive` feature.

## [1.1.0]

### Added
- Added Telegram code sending functionality.

## [1.0.0]

### Added
- Initial release with SMS, Viber, HLR, contacts, groups, blacklist support.
