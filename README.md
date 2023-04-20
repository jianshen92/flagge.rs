# Flagge.rs
Inspired by https://github.com/openflagr/flagr

My hobby project to learn Rust. Do not use in production -- yet.

## What?
* Runs on Actix Web


### `/evaluate`
*Query Parameters*
* entity_id: String
* salt: String

returns a bucket id based on the TOTAL_BUCKET in distribution. Currently hardcoded to 2


