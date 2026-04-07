require_relative 'shared'
require 'digest'
require 'base64'

# vuln-code-snippet start ruby_authn_oauth_pkce
def oauth_token_exchange(req, db)
  code = req.post('code')
  code_verifier = req.post('code_verifier')
  row = db.execute('SELECT code_challenge FROM oauth_codes WHERE code = ?', [code]).first
  return BenchmarkResponse.error('Invalid code', 400) unless row
  stored_challenge = row[0]
  derived_challenge = Base64.urlsafe_encode64(Digest::SHA256.digest(code_verifier), padding: false)
  return BenchmarkResponse.error('PKCE verification failed', 401) unless derived_challenge == stored_challenge # vuln-code-snippet safe-line ruby_authn_oauth_pkce
  BenchmarkResponse.ok('Token issued')
end
# vuln-code-snippet end ruby_authn_oauth_pkce
