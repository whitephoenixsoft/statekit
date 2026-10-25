---
project: statekit
category: governance
status: active
review-status: accepted
version: 2
---
# Project Governance

## Purpose 

This document describes what rules are important when managing the project.

## Releases

**REL-001:** The default branch must accurately describe the latest published version of Statekit.

**REL-002:** The develop branch is to be used as the integration branch for all features. Direct commits related to planning and governance are acceptable.

**REL-003:** The release/\<version\> branches are for cleanup before release.

**REL-004:** The feature/\* branches are for development.

**REL-005:** The experiment/\* and research/\* branches are for experimentation and prototyping. Anything permanent must be migrated to a feature branch.

**REL-006:** Publishing must be done from the default branch after a tag is created to keep the default branch authoritive.

**REL-007:** Release tags must follow the pattern vX.Y.Z.