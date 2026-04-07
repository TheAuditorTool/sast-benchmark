require_relative 'shared'

User = Struct.new(:name, :email, :role, :bio, :admin, keyword_init: true)
class << User; def create(h); new(**h.transform_keys(&:to_sym).slice(:name,:email,:role,:bio,:admin)); end; end

# vuln-code-snippet start ruby_massassign_dry_schema_filter
def handler(req)
  schema = { 'name' => String, 'email' => String }
  attrs = req.post_data.select { |k, v| schema.key?(k) && v.is_a?(schema[k]) }
  user = User.create(attrs) # vuln-code-snippet safe-line ruby_massassign_dry_schema_filter
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_massassign_dry_schema_filter
