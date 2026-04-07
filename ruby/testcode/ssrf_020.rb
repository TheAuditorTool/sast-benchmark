require_relative 'shared'

begin; require 'patron'; rescue LoadError; end

# vuln-code-snippet start ruby_ssrf_patron
def fetch_patron(req)
  sess = Patron::Session.new
  sess.get(req.param('url')) # vuln-code-snippet vuln-line ruby_ssrf_patron
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ssrf_patron
