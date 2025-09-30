# Testing Documentation

This document describes the comprehensive test suite for the Pushover notification tool.

## Test Structure

The project uses Cargo's built-in testing framework with multiple test categories:

```
tests/
├── config_tests.rs      # Configuration parsing and validation
├── integration_tests.rs # Command-line interface testing  
└── unit_tests.rs        # Pure function unit tests
src/
└── lib.rs              # Library unit tests
```

## Test Categories

### 1. Library Unit Tests (`src/lib.rs`)

**Location**: `cargo test --lib`
**Purpose**: Test core library functions in isolation

- **URL Encoding**: Validates proper encoding of special characters, Unicode, spaces
- **URL Parsing**: Tests HTTPS URL parsing, port extraction, path handling
- **Configuration Parsing**: TOML deserialization and structure validation
- **Default Values**: Ensures proper default behavior for optional fields

### 2. Configuration Tests (`tests/config_tests.rs`)

**Location**: `cargo test --test config_tests`
**Purpose**: Comprehensive TOML configuration validation

#### Valid Configuration Tests
- Minimal configuration (user + token only)
- Complete configuration with all optional fields
- Partial notification settings
- Unicode characters in configuration values
- Comments and formatting variations

#### Invalid Configuration Tests  
- Missing required fields (user, token)
- Invalid TOML syntax
- Empty configuration files
- Missing configuration sections

#### Edge Cases
- Empty string values
- Special characters in tokens/user keys
- Very long configuration values

### 3. Unit Tests (`tests/unit_tests.rs`)

**Location**: `cargo test --test unit_tests`  
**Purpose**: Test utility functions and logic components

#### URL Encoding Tests
- Basic character encoding (a-z, 0-9, safe characters)
- Special character encoding (@, !, %, etc.)
- Whitespace handling (spaces → `+`, newlines → `%0A`)
- Unicode character encoding (UTF-8 → percent-encoded)
- Boundary conditions (empty strings, very long strings)

#### URL Parsing Tests
- Valid HTTPS URLs with various formats
- Default port handling (443 for HTTPS)
- Complex paths with query parameters
- Invalid URL rejection (non-HTTPS, malformed)
- Edge cases (empty hosts, invalid ports)

#### Logic Tests
- Priority validation (-2 to 2 range)
- Token override functionality
- Form data encoding for API requests
- Configuration structure manipulation

### 4. Integration Tests (`tests/integration_tests.rs`)

**Location**: `cargo test --test integration_tests`
**Purpose**: End-to-end command-line interface testing

#### Argument Parsing Tests
- Help message display (`-h`, `--help`)
- Required argument validation (`-m` for message)
- Optional argument handling (`-t`, `-p`, `--app-token`)
- Invalid argument detection
- Priority range validation

#### Error Handling Tests  
- Missing message error
- Invalid priority values (non-numeric, out of range)
- Unknown command-line flags
- Missing arguments for flags

#### Network Integration Tests
- Configuration loading from system files
- Token override functionality verification
- Network error handling (expects 400/401 errors with test data)

## Running Tests

### Basic Test Execution

```bash
# Run all tests
cargo test

# Run with verbose output  
cargo test --verbose

# Run with test output displayed
cargo test -- --nocapture
```

### Category-Specific Tests

```bash
# Library unit tests only
cargo test --lib

# Configuration tests only  
cargo test --test config_tests

# Integration tests only
cargo test --test integration_tests  

# Utility unit tests only
cargo test --test unit_tests
```

### Individual Test Execution

```bash
# Run specific test function
cargo test test_url_encode

# Run tests matching pattern
cargo test url_encode

# Run tests in specific module
cargo test config_tests::test_valid_minimal_config
```

## Test Coverage

### Current Test Statistics

- **Total Tests**: 50 tests
- **Library Unit Tests**: 4 tests
- **Configuration Tests**: 14 tests  
- **Integration Tests**: 12 tests
- **Utility Unit Tests**: 20 tests

### Coverage Areas

✅ **Fully Covered**:
- URL encoding/decoding
- Configuration parsing
- Command-line argument parsing
- Error message generation
- Priority validation
- Token override logic

✅ **Well Covered**:
- TOML configuration validation
- Unicode handling
- Edge case handling
- Invalid input rejection

⚠️ **Limited Coverage**:
- Network request generation (tested indirectly)
- TLS connection handling (integration test only)
- File system operations (mocked in tests)

## Test Development Guidelines

### Writing New Tests

1. **Choose the Right Category**:
   - Pure functions → `unit_tests.rs` or `lib.rs`
   - Configuration logic → `config_tests.rs`  
   - CLI behavior → `integration_tests.rs`

2. **Test Naming Convention**:
   ```rust
   #[test]
   fn test_{component}_{scenario}() {
       // Test implementation
   }
   ```

3. **Test Structure**:
   ```rust
   #[test]  
   fn test_example() {
       // Arrange: Set up test data
       let input = "test data";
       
       // Act: Execute the function
       let result = function_under_test(input);
       
       // Assert: Verify expectations
       assert_eq!(result, expected_value);
   }
   ```

### Test Data Guidelines

- Use realistic but fake credentials in tests
- Test with various character encodings (ASCII, Unicode)
- Include boundary conditions (empty, very large inputs)
- Test error paths as thoroughly as success paths

### Integration Test Considerations

- Integration tests work with real system configuration files
- Network requests are expected to fail with test credentials
- Tests validate the complete pipeline up to the network boundary
- Environment isolation is limited due to system config dependencies

## Continuous Integration

The test suite is designed to run in CI environments:

```bash
# CI-friendly test execution
cargo test --verbose --no-fail-fast

# Check for warnings in test code
cargo test --tests 2>&1 | grep -i warning
```

### CI Test Requirements

- Tests must be deterministic (no random data)
- No external network dependencies for core functionality
- Minimal file system dependencies  
- Fast execution (all tests complete in <5 seconds)

## Debugging Test Failures

### Common Issues and Solutions

1. **Integration Test Failures**:
   ```bash
   # Debug actual vs expected output
   cargo test integration_test_name -- --nocapture
   ```

2. **Configuration Test Issues**:
   ```bash
   # Test TOML parsing directly
   cargo test config_tests --verbose
   ```

3. **Network-Related Failures**:
   - Check if system config files exist
   - Verify test credentials are invalid (should fail at network stage)
   - Ensure tests expect network errors, not success

### Test Debugging Tools

```bash
# Run single test with full output
cargo test test_name -- --exact --nocapture

# Show test execution times
cargo test --verbose

# Check test dependencies
cargo tree --tests
```

## Future Test Improvements

### Planned Enhancements

1. **Mock Network Layer**: Replace integration tests with proper network mocking
2. **Property-Based Testing**: Add fuzzing tests for URL encoding/parsing  
3. **Performance Tests**: Add benchmarks for critical functions
4. **Configuration File Mocking**: Better isolation for config tests

### Test Metrics Goals

- Maintain >90% line coverage for core functions
- All new features require corresponding tests
- Integration tests should complete in <3 seconds
- Zero test flakiness (all tests deterministic)