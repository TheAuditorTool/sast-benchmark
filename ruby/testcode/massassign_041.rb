require_relative 'shared'

User = Struct.new(:name, :email, :role, :bio, :admin, keyword_init: true)
class << User; def create(h); new(**h.transform_keys(&:to_sym).slice(:name,:email,:role,:bio,:admin)); end; end

# vuln-code-snippet start ruby_massassign_hash_transform
def handler(req)
  permitted = req.post_data.transform_keys(&:to_sym).slice(:name, :email)
  user = User.create(permitted) # vuln-code-snippet safe-line ruby_massassign_hash_transform
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_massassign_hash_transform
