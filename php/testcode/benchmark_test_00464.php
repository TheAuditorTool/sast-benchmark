<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00464(BenchmarkRequest $req): BenchmarkResponse {
    $allowed = array_flip(['lang', 'theme']);
    $safe = array_intersect_key($_POST, $allowed);
    $lang  = $safe['lang']  ?? 'en';
    $theme = $safe['theme'] ?? 'default';
    return BenchmarkResponse::ok("lang=$lang theme=$theme");
}
