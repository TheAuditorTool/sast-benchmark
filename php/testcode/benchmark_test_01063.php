<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01063(BenchmarkRequest $req): BenchmarkResponse {
    setcookie('session', getSessionId(), [
        'samesite' => 'Lax',
        'httponly' => true,
    ]);
    $userId = (int) $req->param('user');
    $newEmail = $req->param('email');
    updateEmail($userId, $newEmail);
    return BenchmarkResponse::ok('email updated');
}
