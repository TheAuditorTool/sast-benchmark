<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00030(BenchmarkRequest $req): BenchmarkResponse {
    $theme = htmlspecialchars($req->param('theme'), ENT_QUOTES, 'UTF-8');
    setcookie('pref', $theme, ['secure' => true, 'httponly' => true, 'samesite' => 'Lax']);
    return BenchmarkResponse::ok('pref saved');
}
