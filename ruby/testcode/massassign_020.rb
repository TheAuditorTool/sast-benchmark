require_relative 'shared'

User = Struct.new(:name, :email, :role, :bio, :admin, keyword_init: true)
class << User; def create(h); new(**h.transform_keys(&:to_sym).slice(:name,:email,:role,:bio,:admin)); end; end

# vuln-code-snippet start ruby_massassign_permit_bang_nested
def handler(req)
  # permit! used upstream -- all attributes allowed through
  profile = Profile.create(req.post_data) # vuln-code-snippet vuln-line ruby_massassign_permit_bang_nested
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_massassign_permit_bang_nested
