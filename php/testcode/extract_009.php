<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_class_method
class Settings009 {
    public bool $debug = false;
    public string $mode = 'production';

    public function applyOverrides(array $data): void {
        extract($data); // vuln-code-snippet vuln-line php_extract_class_method
        $this->debug = $debug ?? $this->debug;
        $this->mode = $mode ?? $this->mode;
    }
}

function extract009(BenchmarkRequest $req): BenchmarkResponse {
    $settings = new Settings009();
    $settings->applyOverrides($req->postData);
    return BenchmarkResponse::json(['debug' => $settings->debug, 'mode' => $settings->mode]);
}
// vuln-code-snippet end php_extract_class_method
