<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01010(BenchmarkRequest $req): BenchmarkResponse {
    $userId = (int) $req->param('user_id');
    $profile = fetchUserProfile($userId);
    return BenchmarkResponse::json($profile);
}
