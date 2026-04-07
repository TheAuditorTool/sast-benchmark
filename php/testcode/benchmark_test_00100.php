<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00100(BenchmarkRequest $req): BenchmarkResponse {
    extract(['lang' => 'en', 'theme' => 'default']);
    return BenchmarkResponse::ok("lang=$lang theme=$theme");
}
