require_relative 'shared'

User = Struct.new(:name, :email, :role, :bio, :admin, keyword_init: true)
class << User; def create(h); new(**h.transform_keys(&:to_sym).slice(:name,:email,:role,:bio,:admin)); end; end

def find_user(id)
  User.new(name: 'target', role: 'user')
end

# vuln-code-snippet start ruby_massassign_role_elevation
def handler(req)
  current_user_attrs = req.post_data
  user = find_user(req.param('id'))
  user.update(current_user_attrs) # vuln-code-snippet vuln-line ruby_massassign_role_elevation
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_massassign_role_elevation
