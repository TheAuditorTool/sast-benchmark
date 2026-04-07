<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00929(BenchmarkRequest $req): BenchmarkResponse {
    $chars = str_split('abcdefghijklmnopqrstuvwxyz0123456789');
    $resetToken = '';
    for ($i = 0; $i < 32; $i++) {
        $resetToken .= $chars[array_rand($chars)];
    }
    return BenchmarkResponse::json(['reset_token' => $resetToken]);
}
