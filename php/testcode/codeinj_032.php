<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_wakeup_gadget
class EvalGadget032 {
    public string $code = '';
    public function __wakeup(): void {
        eval($this->code);
    }
}

function codeinj032(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $obj = unserialize($data); // vuln-code-snippet vuln-line php_codeinj_wakeup_gadget
    return BenchmarkResponse::ok('deserialized');
}
// vuln-code-snippet end php_codeinj_wakeup_gadget
