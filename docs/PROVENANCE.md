# Provenance Index

Tracks the origin of all adapted or copied code in this project. Every adapted source file must have a corresponding entry here with:

- **Source URL**: The repository URL where the code was sourced from
- **Commit**: The specific commit hash at the time of adaptation
- **License**: The license of the source repository
- **Local modifications**: Summary of changes made to the adapted code

## Format

| Local path | Source repo | Commit | License | Modifications |
|---|---|---|---|---|
| *(no adapted files yet)* | | | | |

## Validation

The provenance check script (`scripts/check-provenance.sh`) validates that:
1. Every adapted file listed in source has a corresponding entry in this table
2. Each entry has all required fields (URL, commit, license, modifications)
