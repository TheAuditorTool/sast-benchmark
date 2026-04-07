<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00094(BenchmarkRequest $req): BenchmarkResponse {
    include __DIR__ . '/templates/header.php';
    return BenchmarkResponse::ok('Header rendered');
}
