require_relative 'shared'

User = Struct.new(:name, :email, :role, :bio, :admin, keyword_init: true)
class << User; def create(h); new(**h.transform_keys(&:to_sym).slice(:name,:email,:role,:bio,:admin)); end; end

# vuln-code-snippet start ruby_massassign_explicit_assign_all
def handler(req)
  u = User.new
  u.name = req.post('name') # vuln-code-snippet safe-line ruby_massassign_explicit_assign_all
  u.email = req.post('email')
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_massassign_explicit_assign_all
