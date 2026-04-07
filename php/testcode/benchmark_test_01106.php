<?php
require_once __DIR__ . '/shared.php';

class UserSession {
    public string $username;
    public string $role;
    public function __wakeup(): void {
        $this->role = $this->role ?: 'admin';
    }
}

function benchmarkTest01106(BenchmarkRequest $req): BenchmarkResponse {
    $payload = $req->param('session');
    $session = unserialize($payload);
    return BenchmarkResponse::json(['user' => $session->username, 'role' => $session->role]);
}
