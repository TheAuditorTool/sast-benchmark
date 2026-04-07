<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_dto_hydrate_all
function massassign029(BenchmarkRequest $req): BenchmarkResponse {
    $dto = new stdClass();
    $data = $_POST;
    foreach ($data as $k => $v) {
        $dto->$k = $v; // vuln-code-snippet vuln-line php_massassign_dto_hydrate_all
    }
    return BenchmarkResponse::ok('hydrated');
}
// vuln-code-snippet end php_massassign_dto_hydrate_all
