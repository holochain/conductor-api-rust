# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/). This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## \[Unreleased\]

### Added
### Changed
- Bump deps to holochain-0.3.0-beta-dev.23
### Fixed
### Removed

## 2023-10-20: v0.5.0-dev.0
### Changed
- Bump deps to holochain-0.3.0-beta-dev.22

## 2023-10-11: v0.4.5-rc.0
### Changed
- Remove unreachable code in `AppWebsocket::send`.
- Bump deps to holochain-0.2.3-beta-rc.0
### Fixed
- Upgrade to security patched version of `webpki`.

## 2023-10-02: v0.4.4
### Changed
- Pin serde to max v1.0.166 properly.

## 2023-09-28: v0.4.3
### Changed
- Pin serde to v1.0.166
- Upgrade holochain_serialized_bytes to v0.0.53

## 2023-09-13: v0.4.2
### Changed
- Upgrade to Holochain v0.2.2.

## 2023-09-11: v0.4.2-rc.3
### Changed
- Upgrade to Holochain v0.2.2-beta-rc.3.

## 2023-08-31: v0.4.2-rc.0
### Changed
- Upgrade to Holochain v0.2.2-beta-rc.0.

## 2023-08-07: v0.4.1
### Added
- Admin API call `graft_records`.
### Changed
- Upgrade to Holochain v0.2.1.

## 2023-04-21: v0.4.0
### Added
- Add `storage_info` to the admin websocket.
- Add `network_info` to the app websocket.
### Changed
- **BREAKING CHANGE**: Upgrade to Holochain 0.2 release candidate ahead of the holochain 0.2 release.

## 2023-02-15: v0.3.1
### Changed
- Upgrade to latest Holochain dependencies.
- Switch to Nix flake for develop environment. Run `nix develop` from now on instead of `nix-shell`. Pass on `--extra-experimental-features nix-command --extra-experimental-features flakes` or enable these features for your user in [`~/.config/nix/nix.conf`](https://nixos.org/manual/nix/stable/command-ref/conf-file.html#conf-experimental-features).

## 2023-01-23: v0.3.0
### Added
- Admin API call `get_dna_definition`
- Utility crate for authorizing credentials and signing zome calls
### Changed
- **BREAKING CHANGE**: Upgrade to Holochain 0.1.0-beta-rc.3
- **BREAKING CHANGE**: Require all zome calls to be signed.
- **BREAKING CHANGE**: Rename `install_app_bundle` to `install_app`.
- **BREAKING CHANGE**: Rename `archive_clone_cell` to `disable_clone_cell`.
- **BREAKING CHANGE**: Rename `restore_archived_clone_cell` to `enable_clone_cell`.
- **BREAKING CHANGE**: Move `enable_clone_cell` to App API.
- **BREAKING CHANGE**: Refactor `delete_clone_cell` to delete a single disabled clone cell.
- **BREAKING CHANGE**: Refactor `app_info` to return all cells and DNA modifiers.
- **BREAKING CHANGE**: Rename `request_agent_info` to `agent_info`.

## 2022-10-03: v0.2.0

Compatible with Holochain >= v0.0.165

### Added
- Added calls for clone cell management:
    - App API: create clone cell
    - App API: archive clone cell
    - Admin API: restore clone cell
    - Admin API: delete archived clone cells
- Added test fixture and tests for clone cells calls

### Changed
- Upgrade to Holochain v0.0.165

## 2022-08-18: v0.1.1

### Changed
- Upgrade to Holochain v0.0.154

## 2022-01-20: v0.1.0

### Changed
- Upgrade to latest Holochain v0.0.147

## 2022-01-20: v0.0.1

### Added
- Initial release & publication as a crate