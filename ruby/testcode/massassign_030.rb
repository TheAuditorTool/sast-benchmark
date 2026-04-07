require_relative 'shared'

User = Struct.new(:name, :email, :role, :bio, :admin, keyword_init: true)
class << User; def create(h); new(**h.transform_keys(&:to_sym).slice(:name,:email,:role,:bio,:admin)); end; end

# vuln-code-snippet start ruby_massassign_admin_via_json_api
def handler(req)
  data = JSON.parse(req.body_str)
  user = User.create(data['attributes']) # vuln-code-snippet vuln-line ruby_massassign_admin_via_json_api
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_massassign_admin_via_json_api
