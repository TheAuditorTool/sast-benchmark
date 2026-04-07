<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00967(BenchmarkRequest $req): BenchmarkResponse {
    include(__DIR__ . "/templates/header.php");
    return BenchmarkResponse::ok("header loaded");
}
