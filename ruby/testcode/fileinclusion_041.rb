require_relative 'shared'

# vuln-code-snippet start ruby_fi_no_user_in_require
def handler(req)
  require 'json'    # vuln-code-snippet safe-line ruby_fi_no_user_in_require
  require 'openssl'
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_no_user_in_require
