<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01160(BenchmarkRequest $req): BenchmarkResponse {
    $msg = $req->post('message');
    $body = '<div class="alert">' . $msg . '</div>';
    return BenchmarkResponse::html($body);
}
