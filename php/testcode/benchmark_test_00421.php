<?php
require_once __DIR__ . '/shared.php';

class UserProfile017 {
    public string $name = '';
    public string $email = '';
}

function benchmarkTest00421(BenchmarkRequest $req): BenchmarkResponse {
    $profile = new UserProfile017();
    $field = $req->param('field');
    $value = $req->param('value');
    switch ($field) {
        case 'name': $profile->name = $value; break;
        case 'email': $profile->email = $value; break;
        default: return BenchmarkResponse::badRequest('Invalid field');
    }
    return BenchmarkResponse::json(['name' => $profile->name, 'email' => $profile->email]);
}
