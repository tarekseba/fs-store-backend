#!/bin/bash

diesel setup;
diesel migration redo --all;
cargo run;

