<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_framework_factory
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

function securecookie017(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(random_bytes(32));
    SecureCookieFactory::set('session', $token); // vuln-code-snippet safe-line php_cookie_framework_factory
    return BenchmarkResponse::ok('Session started');
}
// vuln-code-snippet end php_cookie_framework_factory
