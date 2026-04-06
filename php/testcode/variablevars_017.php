<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_explicit_mapping
class UserProfile017 {
    public string $name = '';
    public string $email = '';
}

function variablevars017(BenchmarkRequest $req): BenchmarkResponse {
    $profile = new UserProfile017();
    $field = $req->param('field');
    $value = $req->param('value');
    switch ($field) { // vuln-code-snippet safe-line php_vv_explicit_mapping
        case 'name': $profile->name = $value; break;
        case 'email': $profile->email = $value; break;
        default: return BenchmarkResponse::badRequest('Invalid field');
    }
    return BenchmarkResponse::json(['name' => $profile->name, 'email' => $profile->email]);
}
// vuln-code-snippet end php_vv_explicit_mapping
