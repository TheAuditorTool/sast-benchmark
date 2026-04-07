<?php
require_once __DIR__ . '/shared.php';

class UserService {
    public function __construct(string $data = '') {}
}

// vuln-code-snippet start php_reflect_hardcoded_class_user_data
function unsafereflect037(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->param('input');
    $svc = new UserService($data); // vuln-code-snippet safe-line php_reflect_hardcoded_class_user_data
    return BenchmarkResponse::ok('created');
}
// vuln-code-snippet end php_reflect_hardcoded_class_user_data
