<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00970(BenchmarkRequest $req): BenchmarkResponse {
    $data = json_decode($req->bodyStr(), true);
    $url = $data['redirect'];
    header('Location: ' . $url);
    return BenchmarkResponse::ok('');
}
