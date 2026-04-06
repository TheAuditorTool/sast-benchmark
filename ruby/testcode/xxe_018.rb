require_relative 'shared'

# vuln-code-snippet start ruby_xxe_strip_dtd
def parse_stripped(req)
  xml_input = req.body_str
  clean_xml = xml_input.gsub(/<!DOCTYPE[^>]*>/i, '') # vuln-code-snippet safe-line ruby_xxe_strip_dtd
  BenchmarkResponse.ok(clean_xml)
end
# vuln-code-snippet end ruby_xxe_strip_dtd
