<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_object_vars
function extract028(BenchmarkRequest $req): BenchmarkResponse {
    $userObj = (object) [
        'name'    => $req->param('name'),
        'role'    => $req->param('role'),
        'isAdmin' => $req->param('isAdmin'),
    ];
    extract(get_object_vars($userObj)); // vuln-code-snippet vuln-line php_extract_object_vars
    return BenchmarkResponse::ok("name=$name role=$role");
}
// vuln-code-snippet end php_extract_object_vars
