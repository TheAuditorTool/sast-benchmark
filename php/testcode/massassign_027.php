<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_magic_set_any
class MagicModel027 {
    private array $props = [];
    public function __set(string $name, mixed $value): void { $this->props[$name] = $value; }
}

function massassign027(BenchmarkRequest $req): BenchmarkResponse {
    $model = new MagicModel027();
    foreach ($_POST as $k => $v) {
        $model->$k = $v; // vuln-code-snippet vuln-line php_massassign_magic_set_any
    }
    return BenchmarkResponse::ok('set');
}
// vuln-code-snippet end php_massassign_magic_set_any
