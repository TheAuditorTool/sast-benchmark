require_relative 'shared'

User = Struct.new(:name, :email, :role, :bio, :admin, keyword_init: true)
class << User; def create(h); new(**h.transform_keys(&:to_sym).slice(:name,:email,:role,:bio,:admin)); end; end

# vuln-code-snippet start ruby_massassign_jsonapi_deserialize
def handler(req)
  data = JSON.parse(req.body_str)
  attrs = data.fetch('attributes', {}).slice('name', 'title')
  resource = User.create(attrs) # vuln-code-snippet safe-line ruby_massassign_jsonapi_deserialize
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_massassign_jsonapi_deserialize
