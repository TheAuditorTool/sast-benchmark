<?php
require_once __DIR__ . '/shared.php';

class SecureCookieFactory {
    public static function set(string $name, string $value, int $ttl = 3600): void {
        setcookie($name, $value, [
            'expires' => time() + $ttl,
            'path' => '/',
            'secure' => true,
            'httponly' => true,
            'samesite' => 'Strict',
        ]);
    }
}

function benchmarkTest00858(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(random_bytes(32));
    SecureCookieFactory::set('session', $token);
    return BenchmarkResponse::ok('Session started');
}
