<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00249(BenchmarkRequest $req): BenchmarkResponse {
    $allowed = ['lang', 'theme'];
    $vars = array_filter($_POST, fn($k) => in_array($k, $allowed, true), ARRAY_FILTER_USE_KEY);
    extract($vars);
    return BenchmarkResponse::ok("lang=$lang theme=$theme");
}
