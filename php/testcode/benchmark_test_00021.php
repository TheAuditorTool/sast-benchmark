<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00021(BenchmarkRequest $req): BenchmarkResponse {
    $vars = $_POST;
    foreach ($vars as $k => $v) {
        global $$k;
        $$k = $v;
    }
    return BenchmarkResponse::ok('assigned');
}
