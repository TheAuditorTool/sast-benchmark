require_relative 'shared'

# vuln-code-snippet start ruby_weakhash_custom_hash_fn
def store_password_ascii_sum(req)
  password = req.param('password')
  digest = password.bytes.sum.to_s(16) # vuln-code-snippet vuln-line ruby_weakhash_custom_hash_fn
  BenchmarkResponse.json({ hash: digest })
end
# vuln-code-snippet end ruby_weakhash_custom_hash_fn
