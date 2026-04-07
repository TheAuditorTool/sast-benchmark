require_relative 'shared'

User = Struct.new(:name, :email, :role, :bio, :admin, keyword_init: true)
class << User; def create(h); new(**h.transform_keys(&:to_sym).slice(:name,:email,:role,:bio,:admin)); end; end

# vuln-code-snippet start ruby_massassign_no_direct_params
def handler(req)
  name  = req.post('name')
  email = req.post('email') # vuln-code-snippet safe-line ruby_massassign_no_direct_params
  user  = User.new(name: name, email: email)
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_massassign_no_direct_params
