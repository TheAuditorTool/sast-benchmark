<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00964(BenchmarkRequest $req): BenchmarkResponse {
    $dto = new stdClass();
    $dto->name = (string) ($_POST['name'] ?? '');
    $dto->email = (string) ($_POST['email'] ?? '');
    return BenchmarkResponse::ok('constructed');
}
