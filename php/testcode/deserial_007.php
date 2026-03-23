<?php
require_once __DIR__ . '/shared.php';

class UserSession {
    public string $username;
    public string $role;
    public function __wakeup(): void {
        // Re-establish DB connection on deserialization
        $this->role = $this->role ?: 'admin';
    }
}

// vuln-code-snippet start php_deser_object_injection
function deserial_object_injection(BenchmarkRequest $req): BenchmarkResponse {
    $payload = $req->param('session');
    $session = unserialize($payload); // vuln-code-snippet vuln-line php_deser_object_injection
    return BenchmarkResponse::json(['user' => $session->username, 'role' => $session->role]);
}
// vuln-code-snippet end php_deser_object_injection
