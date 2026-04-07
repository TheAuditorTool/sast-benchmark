<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00329(BenchmarkRequest $req): BenchmarkResponse {
    $userObj = (object) [
        'name'    => $req->param('name'),
        'role'    => $req->param('role'),
        'isAdmin' => $req->param('isAdmin'),
    ];
    extract(get_object_vars($userObj));
    return BenchmarkResponse::ok("name=$name role=$role");
}
