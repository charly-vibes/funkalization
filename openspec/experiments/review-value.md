# Maintainer review experiment

Status: protocol prepared; user volunteered as the first reviewer. No completed response or measured task times have been received. This is a product-validation experiment, not implementation acceptance or a claim that the tool saves time.

## Hypothesis and participants

For maintainers reviewing agent-proposed storage changes, source-linked obligations and evidence should reduce missed defects and total authoring-plus-review effort compared with existing OpenSpec/text review. A read-only view and an interactive editor are separate hypotheses; v1 can only evaluate the former.

The first user walkthrough is formative and already has prior exposure to the failure scenario. Do not count it as a blinded comparison or estimate a population effect from it. A later comparison should recruit at least five maintainers, rotate task order, use matched but different scenarios and keep answer keys separate from participant materials.

## First walkthrough

Run `cargo run --locked -- tracer target/pilot-review` in a new directory, then open `target/pilot-review/view.html`. Record the actual revision/profile and fixture hashes from `report.json` with the response.

Ask the reviewer to explain:
1. Why the bad backend was refused despite matching operation names.
2. What the good backend's pass establishes, and what it does not establish.
3. Why the stale decision could not change the selected binding.
4. What was hard to find, confusing or missing.

The answer key expects recognition of shipment before durable persistence, finite-corpus scope rather than production verification, and the base-revision guard. Record each response as given, then distinguish correct, partly correct and incorrect interpretations. Do not infer answers from selecting “use me as reviewer.”

## Comparative trial

Prepare matched tasks covering missing persistence, stale evidence, stronger assumptions and conflicting reports. Participants review one set with ordinary specs/text and one with workspace projections. Include time to encode the models, not just time to read the generated view. Capture elapsed time, missed seeded defects, false alarms, corrections and explanations of evidence scope. Do not reuse tasks whose answers participants have already seen.

Proposed exploratory success criteria, frozen before collecting comparative results: zero unsafe acceptances on the seeded cases; at least 20% lower median total authoring-plus-review time without worse defect detection; four of five reviewers completing the tasks unaided. These thresholds guide a small pilot, not a statistically established claim. If overhead rises or bounded evidence is misunderstood, revise the workflow before building an editor.

## Results record

Use a wai research artifact linked to funkalization-4e2. Record participant alias (no unnecessary personal data), consent to use feedback, fixture/source revision, condition/order, prior exposure, exact answers, timing method and observations. Unknown timing stays unknown. Keep author-generated checks, human interpretation and runtime observations separate.

Current results: participant volunteered; walkthrough requested; response and timing pending. No adoption, time-saving or commercial result is asserted.
