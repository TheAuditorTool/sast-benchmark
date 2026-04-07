<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00190(BenchmarkRequest $req): BenchmarkResponse {
    $totpSecret = getenv('TOTP_SECRET');
    $userCode = $req->param('code');
    $timestamp = (int)(time() / 30);
    $hash = hash_hmac('sha1', pack('N*', 0, $timestamp), base32_decode($totpSecret), true);
    $offset = ord($hash[19]) & 0x0f;
    $otp = (unpack('N', substr($hash, $offset, 4))[1] & 0x7fffffff) % 1000000;
    return BenchmarkResponse::ok(str_pad((string)$otp, 6, '0', STR_PAD_LEFT) === $userCode ? 'valid' : 'invalid');
}
