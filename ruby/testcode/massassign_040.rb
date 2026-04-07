require_relative 'shared'

User = Struct.new(:name, :email, :role, :bio, :admin, keyword_init: true)
class << User; def create(h); new(**h.transform_keys(&:to_sym).slice(:name,:email,:role,:bio,:admin)); end; end

# vuln-code-snippet start ruby_massassign_update_columns_explicit
def handler(req)
  user = User.new(name: 'existing')
  user.update_columns(name: req.post('name'), bio: req.post('bio')) # vuln-code-snippet safe-line ruby_massassign_update_columns_explicit
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_massassign_update_columns_explicit
