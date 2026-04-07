require_relative 'shared'

User = Struct.new(:name, :email, :role, :bio, :admin, keyword_init: true)
class << User; def create(h); new(**h.transform_keys(&:to_sym).slice(:name,:email,:role,:bio,:admin)); end; end

# vuln-code-snippet start ruby_massassign_slice_sym_taint
def handler(req)
  # User controls which fields are included via the 'fields' param
  fields = req.param('fields').split(',').map(&:to_sym)
  user = User.create(req.post_data.slice(*fields)) # vuln-code-snippet vuln-line ruby_massassign_slice_sym_taint
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_massassign_slice_sym_taint
