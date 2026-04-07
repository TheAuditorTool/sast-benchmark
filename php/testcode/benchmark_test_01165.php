<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01165(BenchmarkRequest $req): BenchmarkResponse {
    $error = $req->param('error');
    return BenchmarkResponse::html('<div class="error-box">' . $error . '</div>');
}
