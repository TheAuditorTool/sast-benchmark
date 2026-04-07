<?php
require_once __DIR__ . '/shared.php';

function createUser(string $name, string $email): stdClass {
    $m = new stdClass();
    $m->name = $name;
    $m->email = $email;
    return $m;
}

function benchmarkTest01091(BenchmarkRequest $req): BenchmarkResponse {
    $model = createUser045($_POST['name'] ?? '', $_POST['email'] ?? '');
    return BenchmarkResponse::ok('created');
}
