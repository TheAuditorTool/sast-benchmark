<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00519(BenchmarkRequest $req): BenchmarkResponse {
    $pref = $req->param('preference');
    setcookie('user_pref', $pref, time() + 3600);
    return BenchmarkResponse::ok('Preference saved');
}
