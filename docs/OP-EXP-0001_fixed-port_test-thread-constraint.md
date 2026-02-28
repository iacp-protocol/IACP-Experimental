# OP-EXP-0001 — Fixed TCP Port & Test Thread Constraint Observed

## Context
Repository: IACP-Experimental  
Experiment type: minimal transactional roundtrip (IA → IA), harness-only, non-normative.  
Core dependency: iacp-protocol/Core tag core-v0.1 (validate-only).

## Observation
During local test execution, the experimental harness used a fixed TCP port (127.0.0.1:7878).  
When tests were executed concurrently, the server process could fail to bind the port (AddrInUse), causing nondeterministic failures.

## Evidence (mechanical)
- Symptom: OS error on bind (AddrInUse) during concurrent test execution.
- Mitigation applied (experimental): enforce single-threaded test execution.
  - CI: `cargo test -- --test-threads=1`
  - Local harness config: `.cargo/config.toml` sets `[test] threads = 1`

## Impact
- Test execution becomes deterministic for this harness design.
- This is a harness constraint only. It does not define any protocol rule and does not affect the Core specification.

## Status
Recorded as an experimental observation.
No normative change proposed.