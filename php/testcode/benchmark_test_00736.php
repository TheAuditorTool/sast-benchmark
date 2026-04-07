<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00736(BenchmarkRequest $req): BenchmarkResponse {
    $prefs = $req->cookie('p');
    parse_str($prefs);
    if (!empty($isAdmin)) {
        return BenchmarkResponse::ok('Admin panel');
    }
    return BenchmarkResponse::ok('User panel');
}
