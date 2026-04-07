<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00467(BenchmarkRequest $req): BenchmarkResponse {
    foreach ($_POST as $k => $v) {
        $$k = $v;
    }
    return BenchmarkResponse::ok('assigned');
}
