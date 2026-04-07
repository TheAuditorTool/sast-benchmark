<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01172(BenchmarkRequest $req): BenchmarkResponse {
    $error = $req->param('error');
    $msg = htmlspecialchars($error, ENT_QUOTES, 'UTF-8');
    return BenchmarkResponse::html('<div class="error-box">' . $msg . '</div>');
}
