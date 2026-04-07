require_relative 'shared'

User = Struct.new(:name, :email, :role, :bio, :admin, keyword_init: true)
class << User; def create(h); new(**h.transform_keys(&:to_sym).slice(:name,:email,:role,:bio,:admin)); end; end

# vuln-code-snippet start ruby_massassign_sinatra_hash
def handler(req)
  user = User.create(req.post_data) # vuln-code-snippet vuln-line ruby_massassign_sinatra_hash
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_massassign_sinatra_hash
