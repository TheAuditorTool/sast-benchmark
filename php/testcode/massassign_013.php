<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_constructor
class CreateUserRequest013 {
    public function __construct(
        public string $name,
        public string $email,
    ) {}
}

function massassign013(BenchmarkRequest $req): BenchmarkResponse {
    $input = new CreateUserRequest013( // vuln-code-snippet safe-line php_massassign_constructor
        name: $req->post('name'),
        email: $req->post('email'),
    );
    return BenchmarkResponse::json(['name' => $input->name, 'email' => $input->email]);
}
// vuln-code-snippet end php_massassign_constructor
