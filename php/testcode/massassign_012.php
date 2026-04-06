<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_dto_readonly
class UserDTO012 {
    public function __construct(
        public readonly string $name,
        public readonly string $email, // vuln-code-snippet safe-line php_massassign_dto_readonly
    ) {}
}

function massassign012(BenchmarkRequest $req): BenchmarkResponse {
    $dto = new UserDTO012(
        name: $req->post('name'),
        email: $req->post('email'),
    );
    return BenchmarkResponse::json(['name' => $dto->name, 'email' => $dto->email]);
}
// vuln-code-snippet end php_massassign_dto_readonly
