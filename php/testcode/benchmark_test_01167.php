<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01167(BenchmarkRequest $req): BenchmarkResponse {
    $msg = $req->post('message');
    $encoded = htmlspecialchars($msg, ENT_QUOTES, 'UTF-8');
    $body = '<div class="alert">' . $encoded . '</div>';
    return BenchmarkResponse::html($body);
}
