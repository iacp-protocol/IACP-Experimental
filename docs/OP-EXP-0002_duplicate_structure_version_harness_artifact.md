\# OP-EXP-0002 — Duplicate `structure\_version` input (harness artifact)



\## Context

Repository: IACP-Experimental  

Scope: experimental harness only (non-normative).  

Core dependency: iacp\_core tag core-v0.1 (unchanged).



The TCP harness uses an ad-hoc, harness-only encoding:

`structure\_version=<value>`.



\## Observation

A payload containing two textual occurrences of `structure\_version=` was sent:



`structure\_version=0.1\\nstructure\_version=0.2`



The harness produced `OK` deterministically across two identical runs.



\## Evidence (mechanical)

Test output (local):

\- run1: "OK"

\- run2: "OK"



\## Analysis (mechanical)

The current harness parsing uses `strip\_prefix("structure\_version=")` on the full received buffer.

As a result, the second textual occurrence is included inside the value string, and the harness constructs a single Core field:

\- name: `structure\_version`

\- value: String("0.1\\nstructure\_version=0.2")



This does not represent a Core-level duplicate-field scenario.



\## Impact

This observation does not define any protocol rule and does not affect the Core specification.

It documents a harness limitation: the current ad-hoc encoding cannot express duplicate Core fields.



\## Status

Recorded as an experimental observation. No normative change proposed.

