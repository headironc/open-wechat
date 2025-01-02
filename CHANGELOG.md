# CHANGELOG

## [1.2.0](https://github.com/headironc/open-wechat/compare/v1.1.1...v1.2.0) - 2025-01-02

### Added

- Add GitHub Actions workflow for automated releases (#40)

## 1.1.1 - 2025-01-02

### Bug Fixes

- **[BUGFIX]**: Fixed the doc test error.

## 1.1.0 - 2025-01-02

### Breaking Changes

- **[BREAKING]**: Modified the fields of structure `Credential` serialization. `openid` and `unionid` are changed to `open_id` and `union_id` respectively.

### Features

- **[FEATURE]**: Added the `CheckSessionKey` trait for `GenericAccessToken<AccessToken>` and `GenericAccessToken<StableAccessToken>` to check the session key.
- **[FEATURE]**: Added the `ResetSessionKey` trait for `GenericAccessToken<AccessToken>` and `GenericAccessToken<StableAccessToken>` to reset the session key.

### Changes

- **[CHANGE]**: The parameter `force_refresh` of the `GetStableAccessToken` trait can directly use the `bool` type.
