<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_typed_dto_constructor
function massassign037(BenchmarkRequest $req): BenchmarkResponse {
    $dto = new stdClass();
    $dto->name = (string) ($_POST['name'] ?? ''); // vuln-code-snippet safe-line php_massassign_typed_dto_constructor
    $dto->email = (string) ($_POST['email'] ?? '');
    return BenchmarkResponse::ok('constructed');
}
// vuln-code-snippet end php_massassign_typed_dto_constructor
